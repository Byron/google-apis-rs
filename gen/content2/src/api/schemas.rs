use super::*;
/// Account data. After the creation of a new account it may take a few minutes before it is fully operational. The methods delete, insert, and update require the admin role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authinfo accounts](AccountAuthinfoCall) (none)
/// * [claimwebsite accounts](AccountClaimwebsiteCall) (none)
/// * [custombatch accounts](AccountCustombatchCall) (none)
/// * [delete accounts](AccountDeleteCall) (none)
/// * [get accounts](AccountGetCall) (response)
/// * [insert accounts](AccountInsertCall) (request|response)
/// * [link accounts](AccountLinkCall) (none)
/// * [list accounts](AccountListCall) (none)
/// * [update accounts](AccountUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Indicates whether the merchant sells adult content.
    #[serde(rename="adultContent")]
    
    pub adult_content: Option<bool>,
    /// List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list.
    #[serde(rename="adwordsLinks")]
    
    pub adwords_links: Option<Vec<AccountAdwordsLink>>,
    /// The business information of the account.
    #[serde(rename="businessInformation")]
    
    pub business_information: Option<AccountBusinessInformation>,
    /// The GMB account which is linked or in the process of being linked with the Merchant Center account.
    #[serde(rename="googleMyBusinessLink")]
    
    pub google_my_business_link: Option<AccountGoogleMyBusinessLink>,
    /// Required for update. Merchant Center account ID.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#account`"
    
    pub kind: Option<String>,
    /// Required. Display name for the account.
    
    pub name: Option<String>,
    /// [DEPRECATED] This field is never returned and will be ignored if provided.
    #[serde(rename="reviewsUrl")]
    
    pub reviews_url: Option<String>,
    /// Client-specific, locally-unique, internal ID for the child account.
    #[serde(rename="sellerId")]
    
    pub seller_id: Option<String>,
    /// Users with access to the account. Every account (except for subaccounts) must have at least one admin user.
    
    pub users: Option<Vec<AccountUser>>,
    /// The merchant's website.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
    /// List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list.
    #[serde(rename="youtubeChannelLinks")]
    
    pub youtube_channel_links: Option<Vec<AccountYouTubeChannelLink>>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountAddress {
    /// CLDR country code (e.g. "US"). This value cannot be set for a sub-account of an MCA. All MCA sub-accounts inherit the country of their parent MCA.
    
    pub country: Option<String>,
    /// City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs).
    
    pub locality: Option<String>,
    /// Postal code or ZIP (e.g. "94043").
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Top-level administrative subdivision of the country. For example, a state like California ("CA") or a province like Quebec ("QC").
    
    pub region: Option<String>,
    /// Street-level part of the address.
    #[serde(rename="streetAddress")]
    
    pub street_address: Option<String>,
}

impl client::Part for AccountAddress {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountAdwordsLink {
    /// Customer ID of the AdWords account.
    #[serde(rename="adwordsId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub adwords_id: Option<u64>,
    /// Status of the link between this Merchant Center account and the AdWords account. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in Google AdWords or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending. Acceptable values are: - "`active`" - "`pending`" 
    
    pub status: Option<String>,
}

impl client::Part for AccountAdwordsLink {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountBusinessInformation {
    /// The address of the business.
    
    pub address: Option<AccountAddress>,
    /// The customer service information of the business.
    #[serde(rename="customerService")]
    
    pub customer_service: Option<AccountCustomerService>,
    /// The 10-digit [Korean business registration number](https://support.google.com/merchants/answer/9037766) separated with dashes in the format: XXX-XX-XXXXX. This field will only be updated if explicitly set.
    #[serde(rename="koreanBusinessRegistrationNumber")]
    
    pub korean_business_registration_number: Option<String>,
    /// The phone number of the business.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for AccountBusinessInformation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountCustomerService {
    /// Customer service email.
    
    pub email: Option<String>,
    /// Customer service phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Customer service URL.
    
    pub url: Option<String>,
}

impl client::Part for AccountCustomerService {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountGoogleMyBusinessLink {
    /// The GMB email address of which a specific account within a GMB account. A sample account within a GMB account could be a business account with set of locations, managed under the GMB account.
    #[serde(rename="gmbEmail")]
    
    pub gmb_email: Option<String>,
    /// Status of the link between this Merchant Center account and the GMB account. Acceptable values are: - "`active`" - "`pending`" 
    
    pub status: Option<String>,
}

impl client::Part for AccountGoogleMyBusinessLink {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountIdentifier {
    /// The aggregator ID, set for aggregators and subaccounts (in that case, it represents the aggregator of the subaccount).
    #[serde(rename="aggregatorId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub aggregator_id: Option<u64>,
    /// The merchant account ID, set for individual accounts and subaccounts.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
}

impl client::Part for AccountIdentifier {}


/// The status of an account, i.e., information about its products, which is computed offline and not returned immediately at insertion time.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get accountstatuses](AccountstatusGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatus {
    /// The ID of the account for which the status is reported.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// A list of account level issues.
    #[serde(rename="accountLevelIssues")]
    
    pub account_level_issues: Option<Vec<AccountStatusAccountLevelIssue>>,
    /// DEPRECATED - never populated.
    #[serde(rename="dataQualityIssues")]
    
    pub data_quality_issues: Option<Vec<AccountStatusDataQualityIssue>>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#accountStatus`"
    
    pub kind: Option<String>,
    /// List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes.
    
    pub products: Option<Vec<AccountStatusProducts>>,
    /// Whether the account's website is claimed or not.
    #[serde(rename="websiteClaimed")]
    
    pub website_claimed: Option<bool>,
}

impl client::ResponseResult for AccountStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusAccountLevelIssue {
    /// Country for which this issue is reported.
    
    pub country: Option<String>,
    /// The destination the issue applies to. If this field is empty then the issue applies to all available destinations.
    
    pub destination: Option<String>,
    /// Additional details about the issue.
    
    pub detail: Option<String>,
    /// The URL of a web page to help resolving this issue.
    
    pub documentation: Option<String>,
    /// Issue identifier.
    
    pub id: Option<String>,
    /// Severity of the issue. Acceptable values are: - "`critical`" - "`error`" - "`suggestion`" 
    
    pub severity: Option<String>,
    /// Short description of the issue.
    
    pub title: Option<String>,
}

impl client::Part for AccountStatusAccountLevelIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusDataQualityIssue {
    /// no description provided
    
    pub country: Option<String>,
    /// no description provided
    
    pub destination: Option<String>,
    /// no description provided
    
    pub detail: Option<String>,
    /// no description provided
    #[serde(rename="displayedValue")]
    
    pub displayed_value: Option<String>,
    /// no description provided
    #[serde(rename="exampleItems")]
    
    pub example_items: Option<Vec<AccountStatusExampleItem>>,
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    #[serde(rename="lastChecked")]
    
    pub last_checked: Option<String>,
    /// no description provided
    
    pub location: Option<String>,
    /// no description provided
    #[serde(rename="numItems")]
    
    pub num_items: Option<u32>,
    ///  Acceptable values are: - "`critical`" - "`error`" - "`suggestion`" 
    
    pub severity: Option<String>,
    /// no description provided
    #[serde(rename="submittedValue")]
    
    pub submitted_value: Option<String>,
}

impl client::Part for AccountStatusDataQualityIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusExampleItem {
    /// no description provided
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// no description provided
    
    pub link: Option<String>,
    /// no description provided
    #[serde(rename="submittedValue")]
    
    pub submitted_value: Option<String>,
    /// no description provided
    
    pub title: Option<String>,
    /// no description provided
    #[serde(rename="valueOnLandingPage")]
    
    pub value_on_landing_page: Option<String>,
}

impl client::Part for AccountStatusExampleItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusItemLevelIssue {
    /// The attribute's name, if the issue is caused by a single attribute.
    #[serde(rename="attributeName")]
    
    pub attribute_name: Option<String>,
    /// The error code of the issue.
    
    pub code: Option<String>,
    /// A short issue description in English.
    
    pub description: Option<String>,
    /// A detailed issue description in English.
    
    pub detail: Option<String>,
    /// The URL of a web page to help with resolving this issue.
    
    pub documentation: Option<String>,
    /// Number of items with this issue.
    #[serde(rename="numItems")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_items: Option<i64>,
    /// Whether the issue can be resolved by the merchant.
    
    pub resolution: Option<String>,
    /// How this issue affects serving of the offer.
    
    pub servability: Option<String>,
}

impl client::Part for AccountStatusItemLevelIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusProducts {
    /// The channel the data applies to. Acceptable values are: - "`local`" - "`online`" 
    
    pub channel: Option<String>,
    /// The country the data applies to.
    
    pub country: Option<String>,
    /// The destination the data applies to.
    
    pub destination: Option<String>,
    /// List of item-level issues.
    #[serde(rename="itemLevelIssues")]
    
    pub item_level_issues: Option<Vec<AccountStatusItemLevelIssue>>,
    /// Aggregated product statistics.
    
    pub statistics: Option<AccountStatusStatistics>,
}

impl client::Part for AccountStatusProducts {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountStatusStatistics {
    /// Number of active offers.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active: Option<i64>,
    /// Number of disapproved offers.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disapproved: Option<i64>,
    /// Number of expiring offers.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiring: Option<i64>,
    /// Number of pending offers.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pending: Option<i64>,
}

impl client::Part for AccountStatusStatistics {}


/// The tax settings of a merchant account. All methods require the admin role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get accounttax](AccounttaxGetCall) (response)
/// * [update accounttax](AccounttaxUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountTax {
    /// Required. The ID of the account to which these account tax settings belong.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountTax".
    
    pub kind: Option<String>,
    /// Tax rules. Updating the tax rules will enable US taxes (not reversible). Defining no rules is equivalent to not charging tax at all.
    
    pub rules: Option<Vec<AccountTaxTaxRule>>,
}

impl client::RequestValue for AccountTax {}
impl client::ResponseResult for AccountTax {}


/// Tax calculation rule to apply in a state or province (USA only).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountTaxTaxRule {
    /// Country code in which tax is applicable.
    
    pub country: Option<String>,
    /// Required. State (or province) is which the tax is applicable, described by its location ID (also called criteria ID).
    #[serde(rename="locationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub location_id: Option<u64>,
    /// Explicit tax rate in percent, represented as a floating point number without the percentage character. Must not be negative.
    #[serde(rename="ratePercent")]
    
    pub rate_percent: Option<String>,
    /// If true, shipping charges are also taxed.
    #[serde(rename="shippingTaxed")]
    
    pub shipping_taxed: Option<bool>,
    /// Whether the tax rate is taken from a global tax table or specified explicitly.
    #[serde(rename="useGlobalRate")]
    
    pub use_global_rate: Option<bool>,
}

impl client::Part for AccountTaxTaxRule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountUser {
    /// Whether user is an admin.
    
    pub admin: Option<bool>,
    /// User's email address.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Whether user is an order manager.
    #[serde(rename="orderManager")]
    
    pub order_manager: Option<bool>,
    /// Whether user can access payment statements.
    #[serde(rename="paymentsAnalyst")]
    
    pub payments_analyst: Option<bool>,
    /// Whether user can manage payment settings.
    #[serde(rename="paymentsManager")]
    
    pub payments_manager: Option<bool>,
}

impl client::Part for AccountUser {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountYouTubeChannelLink {
    /// Channel ID.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Status of the link between this Merchant Center account and the YouTube channel. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in YT Creator Studio or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending.
    
    pub status: Option<String>,
}

impl client::Part for AccountYouTubeChannelLink {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [authinfo accounts](AccountAuthinfoCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsAuthInfoResponse {
    /// The account identifiers corresponding to the authenticated user. - For an individual account: only the merchant ID is defined - For an aggregator: only the aggregator ID is defined - For a subaccount of an MCA: both the merchant ID and the aggregator ID are defined. 
    #[serde(rename="accountIdentifiers")]
    
    pub account_identifiers: Option<Vec<AccountIdentifier>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountsAuthInfoResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountsAuthInfoResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [claimwebsite accounts](AccountClaimwebsiteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsClaimWebsiteResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountsClaimWebsiteResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountsClaimWebsiteResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accounts](AccountCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<AccountsCustomBatchRequestEntry>>,
}

impl client::RequestValue for AccountsCustomBatchRequest {}


/// A batch entry encoding a single non-batch accounts request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsCustomBatchRequestEntry {
    /// The account to create or update. Only defined if the method is `insert` or `update`.
    
    pub account: Option<Account>,
    /// The ID of the targeted account. Only defined if the method is not `insert`.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// Whether the account should be deleted if the account has offers. Only applicable if the method is `delete`.
    
    pub force: Option<bool>,
    /// Label IDs for the 'updatelabels' request.
    #[serde(rename="labelIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub label_ids: Option<Vec<u64>>,
    /// Details about the `link` request.
    #[serde(rename="linkRequest")]
    
    pub link_request: Option<AccountsCustomBatchRequestEntryLinkRequest>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`claimWebsite`" - "`delete`" - "`get`" - "`insert`" - "`link`" - "`update`" 
    
    pub method: Option<String>,
    /// Only applicable if the method is `claimwebsite`. Indicates whether or not to take the claim from another account in case there is a conflict.
    
    pub overwrite: Option<bool>,
}

impl client::Part for AccountsCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsCustomBatchRequestEntryLinkRequest {
    /// Action to perform for this link. The `"request"` action is only available to select merchants. Acceptable values are: - "`approve`" - "`remove`" - "`request`" 
    
    pub action: Option<String>,
    /// Type of the link between the two accounts. Acceptable values are: - "`channelPartner`" - "`eCommercePlatform`" 
    #[serde(rename="linkType")]
    
    pub link_type: Option<String>,
    /// The ID of the linked account.
    #[serde(rename="linkedAccountId")]
    
    pub linked_account_id: Option<String>,
}

impl client::Part for AccountsCustomBatchRequestEntryLinkRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accounts](AccountCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<AccountsCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountsCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountsCustomBatchResponse {}


/// A batch entry encoding a single non-batch accounts response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsCustomBatchResponseEntry {
    /// The retrieved, created, or updated account. Not defined if the method was `delete`, `claimwebsite` or `link`.
    
    pub account: Option<Account>,
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#accountsCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// Deprecated. This field is never set. Acceptable values are: - "`active`" - "`inactive`" - "`pending`" 
    #[serde(rename="linkStatus")]
    
    pub link_status: Option<String>,
}

impl client::Part for AccountsCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [link accounts](AccountLinkCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsLinkRequest {
    /// Action to perform for this link. The `"request"` action is only available to select merchants. Acceptable values are: - "`approve`" - "`remove`" - "`request`" 
    
    pub action: Option<String>,
    /// Type of the link between the two accounts. Acceptable values are: - "`channelPartner`" - "`eCommercePlatform`" 
    #[serde(rename="linkType")]
    
    pub link_type: Option<String>,
    /// The ID of the linked account.
    #[serde(rename="linkedAccountId")]
    
    pub linked_account_id: Option<String>,
}

impl client::RequestValue for AccountsLinkRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [link accounts](AccountLinkCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsLinkResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountsLinkResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountsLinkResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of accounts.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Account>>,
}

impl client::ResponseResult for AccountsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accountstatuses](AccountstatusCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountstatusesCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<AccountstatusesCustomBatchRequestEntry>>,
}

impl client::RequestValue for AccountstatusesCustomBatchRequest {}


/// A batch entry encoding a single non-batch accountstatuses request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountstatusesCustomBatchRequestEntry {
    /// The ID of the (sub-)account whose status to get.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination.
    
    pub destinations: Option<Vec<String>>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" 
    
    pub method: Option<String>,
}

impl client::Part for AccountstatusesCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accountstatuses](AccountstatusCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountstatusesCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<AccountstatusesCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountstatusesCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccountstatusesCustomBatchResponse {}


/// A batch entry encoding a single non-batch accountstatuses response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountstatusesCustomBatchResponseEntry {
    /// The requested account status. Defined if and only if the request was successful.
    #[serde(rename="accountStatus")]
    
    pub account_status: Option<AccountStatus>,
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
}

impl client::Part for AccountstatusesCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accountstatuses](AccountstatusListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountstatusesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#accountstatusesListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of account statuses.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<AccountStatus>>,
}

impl client::ResponseResult for AccountstatusesListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accounttax](AccounttaxCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccounttaxCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<AccounttaxCustomBatchRequestEntry>>,
}

impl client::RequestValue for AccounttaxCustomBatchRequest {}


/// A batch entry encoding a single non-batch accounttax request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccounttaxCustomBatchRequestEntry {
    /// The ID of the account for which to get/update account tax settings.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// The account tax settings to update. Only defined if the method is `update`.
    #[serde(rename="accountTax")]
    
    pub account_tax: Option<AccountTax>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" - "`update`" 
    
    pub method: Option<String>,
}

impl client::Part for AccounttaxCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch accounttax](AccounttaxCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccounttaxCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<AccounttaxCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#accounttaxCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AccounttaxCustomBatchResponse {}


/// A batch entry encoding a single non-batch accounttax response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccounttaxCustomBatchResponseEntry {
    /// The retrieved or updated account tax settings.
    #[serde(rename="accountTax")]
    
    pub account_tax: Option<AccountTax>,
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#accounttaxCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
}

impl client::Part for AccounttaxCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounttax](AccounttaxListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccounttaxListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#accounttaxListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of account tax settings.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<AccountTax>>,
}

impl client::ResponseResult for AccounttaxListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    /// Required. Top-level administrative subdivision of the country. For example, a state like California ("CA") or a province like Quebec ("QC").
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Required. City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs).
    
    pub city: Option<String>,
    /// Required. [CLDR country code](http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml)(e.g. "US").
    
    pub country: Option<String>,
    /// Required. Postal code or ZIP (e.g. "94043"). Required.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Street-level part of the address.
    #[serde(rename="streetAddress")]
    
    pub street_address: Option<String>,
}

impl client::Part for Address {}


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
pub struct BusinessDayConfig {
    /// Regular business days, such as '"monday"'. May not be empty.
    #[serde(rename="businessDays")]
    
    pub business_days: Option<Vec<String>>,
}

impl client::Part for BusinessDayConfig {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CarrierRate {
    /// Carrier service, such as `"UPS"` or `"Fedex"`. The list of supported carriers can be retrieved via the `getSupportedCarriers` method. Required.
    #[serde(rename="carrierName")]
    
    pub carrier_name: Option<String>,
    /// Carrier service, such as `"ground"` or `"2 days"`. The list of supported services for a carrier can be retrieved via the `getSupportedCarriers` method. Required.
    #[serde(rename="carrierService")]
    
    pub carrier_service: Option<String>,
    /// Additive shipping rate modifier. Can be negative. For example `{ "value": "1", "currency" : "USD" }` adds $1 to the rate, `{ "value": "-3", "currency" : "USD" }` removes $3 from the rate. Optional.
    #[serde(rename="flatAdjustment")]
    
    pub flat_adjustment: Option<Price>,
    /// Name of the carrier rate. Must be unique per rate group. Required.
    
    pub name: Option<String>,
    /// Shipping origin for this carrier rate. Required.
    #[serde(rename="originPostalCode")]
    
    pub origin_postal_code: Option<String>,
    /// Multiplicative shipping rate modifier as a number in decimal notation. Can be negative. For example `"5.4"` increases the rate by 5.4%, `"-3"` decreases the rate by 3%. Optional.
    #[serde(rename="percentageAdjustment")]
    
    pub percentage_adjustment: Option<String>,
}

impl client::Part for CarrierRate {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CarriersCarrier {
    /// The CLDR country code of the carrier (e.g., "US"). Always present.
    
    pub country: Option<String>,
    /// A list of services supported for EDD (Estimated Delivery Date) calculation. This is the list of valid values for WarehouseBasedDeliveryTime.carrierService.
    #[serde(rename="eddServices")]
    
    pub edd_services: Option<Vec<String>>,
    /// The name of the carrier (e.g., `"UPS"`). Always present.
    
    pub name: Option<String>,
    /// A list of supported services (e.g., `"ground"`) for that carrier. Contains at least one service. This is the list of valid values for CarrierRate.carrierService.
    
    pub services: Option<Vec<String>>,
}

impl client::Part for CarriersCarrier {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttribute {
    /// The name of the attribute. Underscores will be replaced by spaces upon insertion.
    
    pub name: Option<String>,
    /// The type of the attribute. Acceptable values are: - "`boolean`" - "`datetimerange`" - "`float`" - "`group`" - "`int`" - "`price`" - "`text`" - "`time`" - "`url`" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Free-form unit of the attribute. Unit can only be used for values of type int, float, or price.
    
    pub unit: Option<String>,
    /// The value of the attribute.
    
    pub value: Option<String>,
}

impl client::Part for CustomAttribute {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomGroup {
    /// The sub-attributes.
    
    pub attributes: Option<Vec<CustomAttribute>>,
    /// The name of the group. Underscores will be replaced by spaces upon insertion.
    
    pub name: Option<String>,
}

impl client::Part for CustomGroup {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomerReturnReason {
    /// Description of the reason.
    
    pub description: Option<String>,
    /// Code of the return reason. Acceptable values are: - "`betterPriceFound`" - "`changedMind`" - "`damagedOrDefectiveItem`" - "`didNotMatchDescription`" - "`doesNotFit`" - "`expiredItem`" - "`incorrectItemReceived`" - "`noLongerNeeded`" - "`notSpecified`" - "`orderedWrongItem`" - "`other`" - "`qualityNotExpected`" - "`receivedTooLate`" - "`undeliverable`" 
    #[serde(rename="reasonCode")]
    
    pub reason_code: Option<String>,
}

impl client::Part for CustomerReturnReason {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CutoffTime {
    /// Hour of the cutoff time until which an order has to be placed to be processed in the same day. Required.
    
    pub hour: Option<u32>,
    /// Minute of the cutoff time until which an order has to be placed to be processed in the same day. Required.
    
    pub minute: Option<u32>,
    /// Timezone identifier for the cutoff time. A list of identifiers can be found in the AdWords API documentation. E.g. "Europe/Zurich". Required.
    
    pub timezone: Option<String>,
}

impl client::Part for CutoffTime {}


/// Datafeed configuration data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch datafeeds](DatafeedCustombatchCall) (none)
/// * [delete datafeeds](DatafeedDeleteCall) (none)
/// * [fetchnow datafeeds](DatafeedFetchnowCall) (none)
/// * [get datafeeds](DatafeedGetCall) (response)
/// * [insert datafeeds](DatafeedInsertCall) (request|response)
/// * [list datafeeds](DatafeedListCall) (none)
/// * [update datafeeds](DatafeedUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Datafeed {
    /// The two-letter ISO 639-1 language in which the attributes are defined in the data feed.
    #[serde(rename="attributeLanguage")]
    
    pub attribute_language: Option<String>,
    /// [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`" 
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// Fetch schedule for the feed file.
    #[serde(rename="fetchSchedule")]
    
    pub fetch_schedule: Option<DatafeedFetchSchedule>,
    /// Required. The filename of the feed. All feeds must have a unique file name.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// Format of the feed file.
    
    pub format: Option<DatafeedFormat>,
    /// Required for update. The ID of the data feed.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center).
    #[serde(rename="intendedDestinations")]
    
    pub intended_destinations: Option<Vec<String>>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`"
    
    pub kind: Option<String>,
    /// Required for insert. A descriptive name of the data feed.
    
    pub name: Option<String>,
    /// [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// The targets this feed should apply to (country, language, destinations).
    
    pub targets: Option<Vec<DatafeedTarget>>,
}

impl client::RequestValue for Datafeed {}
impl client::Resource for Datafeed {}
impl client::ResponseResult for Datafeed {}


/// The required fields vary based on the frequency of fetching. For a monthly fetch schedule, day_of_month and hour are required. For a weekly fetch schedule, weekday and hour are required. For a daily fetch schedule, only hour is required.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedFetchSchedule {
    /// The day of the month the feed file should be fetched (1-31).
    #[serde(rename="dayOfMonth")]
    
    pub day_of_month: Option<u32>,
    /// The URL where the feed file can be fetched. Google Merchant Center will support automatic scheduled uploads using the HTTP, HTTPS, FTP, or SFTP protocols, so the value will need to be a valid link using one of those four protocols.
    #[serde(rename="fetchUrl")]
    
    pub fetch_url: Option<String>,
    /// The hour of the day the feed file should be fetched (0-23).
    
    pub hour: Option<u32>,
    /// The minute of the hour the feed file should be fetched (0-59). Read-only.
    #[serde(rename="minuteOfHour")]
    
    pub minute_of_hour: Option<u32>,
    /// An optional password for fetch_url.
    
    pub password: Option<String>,
    /// Whether the scheduled fetch is paused or not.
    
    pub paused: Option<bool>,
    /// Time zone used for schedule. UTC by default. E.g., "America/Los_Angeles".
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// An optional user name for fetch_url.
    
    pub username: Option<String>,
    /// The day of the week the feed file should be fetched. Acceptable values are: - "`monday`" - "`tuesday`" - "`wednesday`" - "`thursday`" - "`friday`" - "`saturday`" - "`sunday`" 
    
    pub weekday: Option<String>,
}

impl client::Part for DatafeedFetchSchedule {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedFormat {
    /// Delimiter for the separation of values in a delimiter-separated values feed. If not specified, the delimiter will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - "`pipe`" - "`tab`" - "`tilde`" 
    #[serde(rename="columnDelimiter")]
    
    pub column_delimiter: Option<String>,
    /// Character encoding scheme of the data feed. If not specified, the encoding will be auto-detected. Acceptable values are: - "`latin-1`" - "`utf-16be`" - "`utf-16le`" - "`utf-8`" - "`windows-1252`" 
    #[serde(rename="fileEncoding")]
    
    pub file_encoding: Option<String>,
    /// Specifies how double quotes are interpreted. If not specified, the mode will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - "`normal character`" - "`value quoting`" 
    #[serde(rename="quotingMode")]
    
    pub quoting_mode: Option<String>,
}

impl client::Part for DatafeedFormat {}


/// The status of a datafeed, i.e., the result of the last retrieval of the datafeed computed asynchronously when the feed processing is finished.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get datafeedstatuses](DatafeedstatusGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedStatus {
    /// The country for which the status is reported, represented as a CLDR territory code.
    
    pub country: Option<String>,
    /// The ID of the feed for which the status is reported.
    #[serde(rename="datafeedId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub datafeed_id: Option<u64>,
    /// The list of errors occurring in the feed.
    
    pub errors: Option<Vec<DatafeedStatusError>>,
    /// The number of items in the feed that were processed.
    #[serde(rename="itemsTotal")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub items_total: Option<u64>,
    /// The number of items in the feed that were valid.
    #[serde(rename="itemsValid")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub items_valid: Option<u64>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#datafeedStatus`"
    
    pub kind: Option<String>,
    /// The two-letter ISO 639-1 language for which the status is reported.
    
    pub language: Option<String>,
    /// The last date at which the feed was uploaded.
    #[serde(rename="lastUploadDate")]
    
    pub last_upload_date: Option<String>,
    /// The processing status of the feed. Acceptable values are: - "`"`failure`": The feed could not be processed or all items had errors.`" - "`in progress`": The feed is being processed. - "`none`": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - "`success`": The feed was processed successfully, though some items might have had errors. 
    #[serde(rename="processingStatus")]
    
    pub processing_status: Option<String>,
    /// The list of errors occurring in the feed.
    
    pub warnings: Option<Vec<DatafeedStatusError>>,
}

impl client::ResponseResult for DatafeedStatus {}


/// An error occurring in the feed, like "invalid price".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedStatusError {
    /// The code of the error, e.g., "validation/invalid_value".
    
    pub code: Option<String>,
    /// The number of occurrences of the error in the feed.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<u64>,
    /// A list of example occurrences of the error, grouped by product.
    
    pub examples: Option<Vec<DatafeedStatusExample>>,
    /// The error message, e.g., "Invalid price".
    
    pub message: Option<String>,
}

impl client::Part for DatafeedStatusError {}


/// An example occurrence for a particular error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedStatusExample {
    /// The ID of the example item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Line number in the data feed where the example is found.
    #[serde(rename="lineNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub line_number: Option<u64>,
    /// The problematic value.
    
    pub value: Option<String>,
}

impl client::Part for DatafeedStatusExample {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedTarget {
    /// The country where the items in the feed will be included in the search index, represented as a CLDR territory code.
    
    pub country: Option<String>,
    /// The list of destinations to exclude for this target (corresponds to unchecked check boxes in Merchant Center).
    #[serde(rename="excludedDestinations")]
    
    pub excluded_destinations: Option<Vec<String>>,
    /// The list of destinations to include for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. List of supported destinations (if available to the account): - DisplayAds - Shopping - ShoppingActions - SurfacesAcrossGoogle 
    #[serde(rename="includedDestinations")]
    
    pub included_destinations: Option<Vec<String>>,
    /// The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targets[].country`.
    
    pub language: Option<String>,
}

impl client::Part for DatafeedTarget {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch datafeeds](DatafeedCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<DatafeedsCustomBatchRequestEntry>>,
}

impl client::RequestValue for DatafeedsCustomBatchRequest {}


/// A batch entry encoding a single non-batch datafeeds request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The data feed to insert.
    
    pub datafeed: Option<Datafeed>,
    /// The ID of the data feed to get, delete or fetch.
    #[serde(rename="datafeedId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub datafeed_id: Option<u64>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`delete`" - "`fetchNow`" - "`get`" - "`insert`" - "`update`" 
    
    pub method: Option<String>,
}

impl client::Part for DatafeedsCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch datafeeds](DatafeedCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<DatafeedsCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#datafeedsCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DatafeedsCustomBatchResponse {}


/// A batch entry encoding a single non-batch datafeeds response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The requested data feed. Defined if and only if the request was successful.
    
    pub datafeed: Option<Datafeed>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
}

impl client::Part for DatafeedsCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [fetchnow datafeeds](DatafeedFetchnowCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsFetchNowResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#datafeedsFetchNowResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DatafeedsFetchNowResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list datafeeds](DatafeedListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#datafeedsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of datafeeds.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Datafeed>>,
}

impl client::ResponseResult for DatafeedsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch datafeedstatuses](DatafeedstatusCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedstatusesCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<DatafeedstatusesCustomBatchRequestEntry>>,
}

impl client::RequestValue for DatafeedstatusesCustomBatchRequest {}


/// A batch entry encoding a single non-batch datafeedstatuses request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedstatusesCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The country for which to get the datafeed status. If this parameter is provided then language must also be provided. Note that for multi-target datafeeds this parameter is required.
    
    pub country: Option<String>,
    /// The ID of the data feed to get.
    #[serde(rename="datafeedId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub datafeed_id: Option<u64>,
    /// The language for which to get the datafeed status. If this parameter is provided then country must also be provided. Note that for multi-target datafeeds this parameter is required.
    
    pub language: Option<String>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" 
    
    pub method: Option<String>,
}

impl client::Part for DatafeedstatusesCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch datafeedstatuses](DatafeedstatusCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedstatusesCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<DatafeedstatusesCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#datafeedstatusesCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for DatafeedstatusesCustomBatchResponse {}


/// A batch entry encoding a single non-batch datafeedstatuses response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedstatusesCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The requested data feed status. Defined if and only if the request was successful.
    #[serde(rename="datafeedStatus")]
    
    pub datafeed_status: Option<DatafeedStatus>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
}

impl client::Part for DatafeedstatusesCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list datafeedstatuses](DatafeedstatusListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatafeedstatusesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#datafeedstatusesListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of datafeed statuses.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<DatafeedStatus>>,
}

impl client::ResponseResult for DatafeedstatusesListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryTime {
    /// Business days cutoff time definition. If not configured the cutoff time will be defaulted to 8AM PST.
    #[serde(rename="cutoffTime")]
    
    pub cutoff_time: Option<CutoffTime>,
    /// The business days during which orders can be handled. If not provided, Monday to Friday business days will be assumed.
    #[serde(rename="handlingBusinessDayConfig")]
    
    pub handling_business_day_config: Option<BusinessDayConfig>,
    /// Holiday cutoff definitions. If configured, they specify order cutoff times for holiday-specific shipping.
    #[serde(rename="holidayCutoffs")]
    
    pub holiday_cutoffs: Option<Vec<HolidayCutoff>>,
    /// Maximum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped. Must be greater than or equal to `minHandlingTimeInDays`.
    #[serde(rename="maxHandlingTimeInDays")]
    
    pub max_handling_time_in_days: Option<u32>,
    /// Maximum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Must be greater than or equal to `minTransitTimeInDays`.
    #[serde(rename="maxTransitTimeInDays")]
    
    pub max_transit_time_in_days: Option<u32>,
    /// Minimum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped.
    #[serde(rename="minHandlingTimeInDays")]
    
    pub min_handling_time_in_days: Option<u32>,
    /// Minimum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Either `{min,max}TransitTimeInDays` or `transitTimeTable` must be set, but not both.
    #[serde(rename="minTransitTimeInDays")]
    
    pub min_transit_time_in_days: Option<u32>,
    /// The business days during which orders can be in-transit. If not provided, Monday to Friday business days will be assumed.
    #[serde(rename="transitBusinessDayConfig")]
    
    pub transit_business_day_config: Option<BusinessDayConfig>,
    /// Transit time table, number of business days spent in transit based on row and column dimensions. Either `{min,max}TransitTimeInDays` or `transitTimeTable` can be set, but not both.
    #[serde(rename="transitTimeTable")]
    
    pub transit_time_table: Option<TransitTable>,
    /// Indicates that the delivery time should be calculated per warehouse (shipping origin location) based on the settings of the selected carrier. When set, no other transit time related field in DeliveryTime should be set.
    #[serde(rename="warehouseBasedDeliveryTimes")]
    
    pub warehouse_based_delivery_times: Option<Vec<WarehouseBasedDeliveryTime>>,
}

impl client::Part for DeliveryTime {}


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
    /// The HTTP status of the first error in `errors`.
    
    pub code: Option<u32>,
    /// A list of errors.
    
    pub errors: Option<Vec<Error>>,
    /// The message of the first error in `errors`.
    
    pub message: Option<String>,
}

impl client::Part for Errors {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GmbAccounts {
    /// The ID of the Merchant Center account.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// A list of GMB accounts which are available to the merchant.
    #[serde(rename="gmbAccounts")]
    
    pub gmb_accounts: Option<Vec<GmbAccountsGmbAccount>>,
}

impl client::Part for GmbAccounts {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GmbAccountsGmbAccount {
    /// The email which identifies the GMB account.
    
    pub email: Option<String>,
    /// Number of listings under this account.
    #[serde(rename="listingCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub listing_count: Option<u64>,
    /// The name of the GMB account.
    
    pub name: Option<String>,
    /// The type of the GMB account (User or Business).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GmbAccountsGmbAccount {}


/// A non-empty list of row or column headers for a table. Exactly one of `prices`, `weights`, `numItems`, `postalCodeGroupNames`, or `location` must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Headers {
    /// A list of location ID sets. Must be non-empty. Can only be set if all other fields are not set.
    
    pub locations: Option<Vec<LocationIdSet>>,
    /// A list of inclusive number of items upper bounds. The last value can be `"infinity"`. For example `["10", "50", "infinity"]` represents the headers "<= 10 items", "<= 50 items", and "> 50 items". Must be non-empty. Can only be set if all other fields are not set.
    #[serde(rename="numberOfItems")]
    
    pub number_of_items: Option<Vec<String>>,
    /// A list of postal group names. The last value can be `"all other locations"`. Example: `["zone 1", "zone 2", "all other locations"]`. The referred postal code groups must match the delivery country of the service. Must be non-empty. Can only be set if all other fields are not set.
    #[serde(rename="postalCodeGroupNames")]
    
    pub postal_code_group_names: Option<Vec<String>>,
    /// A list of inclusive order price upper bounds. The last price's value can be `"infinity"`. For example `[{"value": "10", "currency": "USD"}, {"value": "500", "currency": "USD"}, {"value": "infinity", "currency": "USD"}]` represents the headers "<= $10", "<= $500", and "> $500". All prices within a service must have the same currency. Must be non-empty. Can only be set if all other fields are not set.
    
    pub prices: Option<Vec<Price>>,
    /// A list of inclusive order weight upper bounds. The last weight's value can be `"infinity"`. For example `[{"value": "10", "unit": "kg"}, {"value": "50", "unit": "kg"}, {"value": "infinity", "unit": "kg"}]` represents the headers "<= 10kg", "<= 50kg", and "> 50kg". All weights within a service must have the same unit. Must be non-empty. Can only be set if all other fields are not set.
    
    pub weights: Option<Vec<Weight>>,
}

impl client::Part for Headers {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HolidayCutoff {
    /// Date of the order deadline, in ISO 8601 format. E.g. "2016-11-29" for 29th November 2016. Required.
    #[serde(rename="deadlineDate")]
    
    pub deadline_date: Option<String>,
    /// Hour of the day on the deadline date until which the order has to be placed to qualify for the delivery guarantee. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Required.
    #[serde(rename="deadlineHour")]
    
    pub deadline_hour: Option<u32>,
    /// Timezone identifier for the deadline hour. A list of identifiers can be found in the AdWords API documentation. E.g. "Europe/Zurich". Required.
    #[serde(rename="deadlineTimezone")]
    
    pub deadline_timezone: Option<String>,
    /// Unique identifier for the holiday. Required.
    #[serde(rename="holidayId")]
    
    pub holiday_id: Option<String>,
    /// Date on which the deadline will become visible to consumers in ISO 8601 format. E.g. "2016-10-31" for 31st October 2016. Required.
    #[serde(rename="visibleFromDate")]
    
    pub visible_from_date: Option<String>,
}

impl client::Part for HolidayCutoff {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HolidaysHoliday {
    /// The CLDR territory code of the country in which the holiday is available. E.g. "US", "DE", "GB". A holiday cutoff can only be configured in a shipping settings service with matching delivery country. Always present.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Date of the holiday, in ISO 8601 format. E.g. "2016-12-25" for Christmas 2016. Always present.
    
    pub date: Option<String>,
    /// Date on which the order has to arrive at the customer's, in ISO 8601 format. E.g. "2016-12-24" for 24th December 2016. Always present.
    #[serde(rename="deliveryGuaranteeDate")]
    
    pub delivery_guarantee_date: Option<String>,
    /// Hour of the day in the delivery location's timezone on the guaranteed delivery date by which the order has to arrive at the customer's. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Always present.
    #[serde(rename="deliveryGuaranteeHour")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub delivery_guarantee_hour: Option<u64>,
    /// Unique identifier for the holiday to be used when configuring holiday cutoffs. Always present.
    
    pub id: Option<String>,
    /// The holiday type. Always present. Acceptable values are: - "`Christmas`" - "`Easter`" - "`Father's Day`" - "`Halloween`" - "`Independence Day (USA)`" - "`Mother's Day`" - "`Thanksgiving`" - "`Valentine's Day`" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for HolidaysHoliday {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Installment {
    /// The amount the buyer has to pay per month.
    
    pub amount: Option<Price>,
    /// The number of installments the buyer has to pay.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub months: Option<i64>,
}

impl client::Part for Installment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    /// The availability of the product. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`" 
    
    pub availability: Option<String>,
    /// Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel0")]
    
    pub custom_label0: Option<String>,
    /// Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel1")]
    
    pub custom_label1: Option<String>,
    /// Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel2")]
    
    pub custom_label2: Option<String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel3")]
    
    pub custom_label3: Option<String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel4")]
    
    pub custom_label4: Option<String>,
    /// Number and amount of installments to pay for an item. Brazil only.
    
    pub installment: Option<Installment>,
    /// The instore product location. Supported only for local products.
    #[serde(rename="instoreProductLocation")]
    
    pub instore_product_location: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#inventory`"
    
    pub kind: Option<String>,
    /// Loyalty points that users receive after purchasing the item. Japan only.
    #[serde(rename="loyaltyPoints")]
    
    pub loyalty_points: Option<LoyaltyPoints>,
    /// Store pickup information. Only supported for local inventory. Not setting `pickup` means "don't update" while setting it to the empty value (`{}` in JSON) means "delete". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is "not supported".
    
    pub pickup: Option<InventoryPickup>,
    /// The price of the product.
    
    pub price: Option<Price>,
    /// The quantity of the product. Must be equal to or greater than zero. Supported only for local products.
    
    pub quantity: Option<u32>,
    /// The sale price of the product. Mandatory if `sale_price_effective_date` is defined.
    #[serde(rename="salePrice")]
    
    pub sale_price: Option<Price>,
    /// A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided.
    #[serde(rename="salePriceEffectiveDate")]
    
    pub sale_price_effective_date: Option<String>,
    /// The quantity of the product that is available for selling on Google. Supported only for online products.
    #[serde(rename="sellOnGoogleQuantity")]
    
    pub sell_on_google_quantity: Option<u32>,
}

impl client::Part for Inventory {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch inventory](InventoryCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<InventoryCustomBatchRequestEntry>>,
}

impl client::RequestValue for InventoryCustomBatchRequest {}


/// A batch entry encoding a single non-batch inventory request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// Price and availability of the product.
    
    pub inventory: Option<Inventory>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The ID of the product for which to update price and availability.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The code of the store for which to update price and availability. Use `online` to update price and availability of an online product.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
}

impl client::Part for InventoryCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch inventory](InventoryCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<InventoryCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#inventoryCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for InventoryCustomBatchResponse {}


/// A batch entry encoding a single non-batch inventory response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#inventoryCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
}

impl client::Part for InventoryCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventoryPickup {
    /// Whether store pickup is available for this offer and whether the pickup option should be shown as buy, reserve, or not supported. Only supported for local inventory. Unless the value is "not supported", must be submitted together with `pickupSla`. Acceptable values are: - "`buy`" - "`not supported`" - "`reserve`" - "`ship to store`" 
    #[serde(rename="pickupMethod")]
    
    pub pickup_method: Option<String>,
    /// The expected date that an order will be ready for pickup, relative to when the order is placed. Only supported for local inventory. Must be submitted together with `pickupMethod`. Acceptable values are: - "`five day`" - "`four day`" - "`multi day`" - "`multi week`" - "`next day`" - "`same day`" - "`seven day`" - "`six day`" - "`three day`" - "`two day`" 
    #[serde(rename="pickupSla")]
    
    pub pickup_sla: Option<String>,
}

impl client::Part for InventoryPickup {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set inventory](InventorySetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySetRequest {
    /// The availability of the product. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`" 
    
    pub availability: Option<String>,
    /// Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel0")]
    
    pub custom_label0: Option<String>,
    /// Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel1")]
    
    pub custom_label1: Option<String>,
    /// Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel2")]
    
    pub custom_label2: Option<String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel3")]
    
    pub custom_label3: Option<String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products.
    #[serde(rename="customLabel4")]
    
    pub custom_label4: Option<String>,
    /// Number and amount of installments to pay for an item. Brazil only.
    
    pub installment: Option<Installment>,
    /// The instore product location. Supported only for local products.
    #[serde(rename="instoreProductLocation")]
    
    pub instore_product_location: Option<String>,
    /// Loyalty points that users receive after purchasing the item. Japan only.
    #[serde(rename="loyaltyPoints")]
    
    pub loyalty_points: Option<LoyaltyPoints>,
    /// Store pickup information. Only supported for local inventory. Not setting `pickup` means "don't update" while setting it to the empty value (`{}` in JSON) means "delete". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is "not supported".
    
    pub pickup: Option<InventoryPickup>,
    /// The price of the product.
    
    pub price: Option<Price>,
    /// The quantity of the product. Must be equal to or greater than zero. Supported only for local products.
    
    pub quantity: Option<u32>,
    /// The sale price of the product. Mandatory if `sale_price_effective_date` is defined.
    #[serde(rename="salePrice")]
    
    pub sale_price: Option<Price>,
    /// A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided.
    #[serde(rename="salePriceEffectiveDate")]
    
    pub sale_price_effective_date: Option<String>,
    /// The quantity of the product that is available for selling on Google. Supported only for online products.
    #[serde(rename="sellOnGoogleQuantity")]
    
    pub sell_on_google_quantity: Option<u32>,
}

impl client::RequestValue for InventorySetRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set inventory](InventorySetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InventorySetResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#inventorySetResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for InventorySetResponse {}


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
    /// Deprecated.
    #[serde(rename="customerBalance")]
    
    pub customer_balance: Option<Amount>,
    /// Deprecated.
    #[serde(rename="googleBalance")]
    
    pub google_balance: Option<Amount>,
    /// Deprecated.
    #[serde(rename="merchantBalance")]
    
    pub merchant_balance: Option<Amount>,
    /// [required] Total price for the product.
    #[serde(rename="productTotal")]
    
    pub product_total: Option<Amount>,
    /// Deprecated.
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
    /// [required] Type of the additional charge. Acceptable values are: - "`shipping`" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for InvoiceSummaryAdditionalChargeSummary {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaAboutPageSettings {
    /// The status of the verification process for the About page. Acceptable values are: - "`active`" - "`inactive`" - "`pending`" 
    
    pub status: Option<String>,
    /// The URL for the About page.
    
    pub url: Option<String>,
}

impl client::Part for LiaAboutPageSettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaCountrySettings {
    /// The settings for the About page.
    
    pub about: Option<LiaAboutPageSettings>,
    /// Required. CLDR country code (e.g. "US").
    
    pub country: Option<String>,
    /// The status of the "Merchant hosted local storefront" feature.
    #[serde(rename="hostedLocalStorefrontActive")]
    
    pub hosted_local_storefront_active: Option<bool>,
    /// LIA inventory verification settings.
    
    pub inventory: Option<LiaInventorySettings>,
    /// LIA "On Display To Order" settings.
    #[serde(rename="onDisplayToOrder")]
    
    pub on_display_to_order: Option<LiaOnDisplayToOrderSettings>,
    /// The POS data provider linked with this country.
    #[serde(rename="posDataProvider")]
    
    pub pos_data_provider: Option<LiaPosDataProvider>,
    /// The status of the "Store pickup" feature.
    #[serde(rename="storePickupActive")]
    
    pub store_pickup_active: Option<bool>,
}

impl client::Part for LiaCountrySettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaInventorySettings {
    /// The email of the contact for the inventory verification process.
    #[serde(rename="inventoryVerificationContactEmail")]
    
    pub inventory_verification_contact_email: Option<String>,
    /// The name of the contact for the inventory verification process.
    #[serde(rename="inventoryVerificationContactName")]
    
    pub inventory_verification_contact_name: Option<String>,
    /// The status of the verification contact. Acceptable values are: - "`active`" - "`inactive`" - "`pending`" 
    #[serde(rename="inventoryVerificationContactStatus")]
    
    pub inventory_verification_contact_status: Option<String>,
    /// The status of the inventory verification process. Acceptable values are: - "`active`" - "`inactive`" - "`pending`" 
    
    pub status: Option<String>,
}

impl client::Part for LiaInventorySettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaOnDisplayToOrderSettings {
    /// Shipping cost and policy URL.
    #[serde(rename="shippingCostPolicyUrl")]
    
    pub shipping_cost_policy_url: Option<String>,
    /// The status of the ?On display to order? feature. Acceptable values are: - "`active`" - "`inactive`" - "`pending`" 
    
    pub status: Option<String>,
}

impl client::Part for LiaOnDisplayToOrderSettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaPosDataProvider {
    /// The ID of the POS data provider.
    #[serde(rename="posDataProviderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pos_data_provider_id: Option<u64>,
    /// The account ID by which this merchant is known to the POS data provider.
    #[serde(rename="posExternalAccountId")]
    
    pub pos_external_account_id: Option<String>,
}

impl client::Part for LiaPosDataProvider {}


/// Local Inventory ads (LIA) settings. All methods except listposdataproviders require the admin role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get liasettings](LiasettingGetCall) (response)
/// * [update liasettings](LiasettingUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiaSettings {
    /// The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// The LIA settings for each country.
    #[serde(rename="countrySettings")]
    
    pub country_settings: Option<Vec<LiaCountrySettings>>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#liaSettings`"
    
    pub kind: Option<String>,
}

impl client::RequestValue for LiaSettings {}
impl client::ResponseResult for LiaSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch liasettings](LiasettingCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<LiasettingsCustomBatchRequestEntry>>,
}

impl client::RequestValue for LiasettingsCustomBatchRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsCustomBatchRequestEntry {
    /// The ID of the account for which to get/update account LIA settings.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// Inventory validation contact email. Required only for SetInventoryValidationContact.
    #[serde(rename="contactEmail")]
    
    pub contact_email: Option<String>,
    /// Inventory validation contact name. Required only for SetInventoryValidationContact.
    #[serde(rename="contactName")]
    
    pub contact_name: Option<String>,
    /// The country code. Required only for RequestInventoryVerification.
    
    pub country: Option<String>,
    /// The GMB account. Required only for RequestGmbAccess.
    #[serde(rename="gmbEmail")]
    
    pub gmb_email: Option<String>,
    /// The account Lia settings to update. Only defined if the method is `update`.
    #[serde(rename="liaSettings")]
    
    pub lia_settings: Option<LiaSettings>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" - "`getAccessibleGmbAccounts`" - "`requestGmbAccess`" - "`requestInventoryVerification`" - "`setInventoryVerificationContact`" - "`update`" 
    
    pub method: Option<String>,
    /// The ID of POS data provider. Required only for SetPosProvider.
    #[serde(rename="posDataProviderId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pos_data_provider_id: Option<u64>,
    /// The account ID by which this merchant is known to the POS provider.
    #[serde(rename="posExternalAccountId")]
    
    pub pos_external_account_id: Option<String>,
}

impl client::Part for LiasettingsCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch liasettings](LiasettingCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<LiasettingsCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsCustomBatchResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsCustomBatchResponseEntry {
    /// The ID of the request entry to which this entry responds.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if, and only if, the request failed.
    
    pub errors: Option<Errors>,
    /// The list of accessible GMB accounts.
    #[serde(rename="gmbAccounts")]
    
    pub gmb_accounts: Option<GmbAccounts>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#liasettingsCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The retrieved or updated Lia settings.
    #[serde(rename="liaSettings")]
    
    pub lia_settings: Option<LiaSettings>,
    /// The list of POS data providers.
    #[serde(rename="posDataProviders")]
    
    pub pos_data_providers: Option<Vec<PosDataProviders>>,
}

impl client::Part for LiasettingsCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getaccessiblegmbaccounts liasettings](LiasettingGetaccessiblegmbaccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsGetAccessibleGmbAccountsResponse {
    /// The ID of the Merchant Center account.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// A list of GMB accounts which are available to the merchant.
    #[serde(rename="gmbAccounts")]
    
    pub gmb_accounts: Option<Vec<GmbAccountsGmbAccount>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsGetAccessibleGmbAccountsResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsGetAccessibleGmbAccountsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listposdataproviders liasettings](LiasettingListposdataproviderCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsListPosDataProvidersResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsListPosDataProvidersResponse".
    
    pub kind: Option<String>,
    /// The list of POS data providers for each eligible country
    #[serde(rename="posDataProviders")]
    
    pub pos_data_providers: Option<Vec<PosDataProviders>>,
}

impl client::ResponseResult for LiasettingsListPosDataProvidersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list liasettings](LiasettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of LIA settings.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<LiaSettings>>,
}

impl client::ResponseResult for LiasettingsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [requestgmbaccess liasettings](LiasettingRequestgmbaccesCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsRequestGmbAccessResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsRequestGmbAccessResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsRequestGmbAccessResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [requestinventoryverification liasettings](LiasettingRequestinventoryverificationCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsRequestInventoryVerificationResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsRequestInventoryVerificationResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsRequestInventoryVerificationResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setinventoryverificationcontact liasettings](LiasettingSetinventoryverificationcontactCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsSetInventoryVerificationContactResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsSetInventoryVerificationContactResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsSetInventoryVerificationContactResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [setposdataprovider liasettings](LiasettingSetposdataproviderCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiasettingsSetPosDataProviderResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#liasettingsSetPosDataProviderResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for LiasettingsSetPosDataProviderResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationIdSet {
    /// A non-empty list of location IDs. They must all be of the same location type (e.g., state).
    #[serde(rename="locationIds")]
    
    pub location_ids: Option<Vec<String>>,
}

impl client::Part for LocationIdSet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyPoints {
    /// Name of loyalty points program. It is recommended to limit the name to 12 full-width characters or 24 Roman characters.
    
    pub name: Option<String>,
    /// The retailer's loyalty points in absolute value.
    #[serde(rename="pointsValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub points_value: Option<i64>,
    /// The ratio of a point when converted to currency. Google assumes currency based on Merchant Center settings. If ratio is left out, it defaults to 1.0.
    
    pub ratio: Option<f64>,
}

impl client::Part for LoyaltyPoints {}


/// Order return. Production access (all methods) requires the order manager role. Sandbox access does not.
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
    /// The date of creation of the return, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// Merchant defined order ID.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// Google order ID.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Order return ID generated by Google.
    #[serde(rename="orderReturnId")]
    
    pub order_return_id: Option<String>,
    /// Items of the return.
    #[serde(rename="returnItems")]
    
    pub return_items: Option<Vec<MerchantOrderReturnItem>>,
    /// Shipments of the return.
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
    /// The reason that the customer chooses to return an item.
    #[serde(rename="customerReturnReason")]
    
    pub customer_return_reason: Option<CustomerReturnReason>,
    /// Product level item ID. If the returned items are of the same product, they will have the same ID.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// The reason that merchant chooses to accept a return item.
    #[serde(rename="merchantReturnReason")]
    
    pub merchant_return_reason: Option<RefundReason>,
    /// Product data from the time of the order placement.
    
    pub product: Option<OrderLineItemProduct>,
    /// IDs of the return shipments that this return item belongs to.
    #[serde(rename="returnShipmentIds")]
    
    pub return_shipment_ids: Option<Vec<String>>,
    /// State of the item. Acceptable values are: - "`canceled`" - "`new`" - "`received`" - "`refunded`" - "`rejected`" 
    
    pub state: Option<String>,
}

impl client::Part for MerchantOrderReturnItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MinimumOrderValueTable {
    /// no description provided
    #[serde(rename="storeCodeSetWithMovs")]
    
    pub store_code_set_with_movs: Option<Vec<MinimumOrderValueTableStoreCodeSetWithMov>>,
}

impl client::Part for MinimumOrderValueTable {}


/// A list of store code sets sharing the same minimum order value. At least two sets are required and the last one must be empty, which signifies 'MOV for all other stores'. Each store code can only appear once across all the sets. All prices within a service must have the same currency.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MinimumOrderValueTableStoreCodeSetWithMov {
    /// A list of unique store codes or empty for the catch all.
    #[serde(rename="storeCodes")]
    
    pub store_codes: Option<Vec<String>>,
    /// The minimum order value for the given stores.
    
    pub value: Option<Price>,
}

impl client::Part for MinimumOrderValueTableStoreCodeSetWithMov {}


/// Order. Production access (all methods) requires the order manager role. Sandbox access does not.
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
    /// Deprecated. Acceptable values are: - "`googleExpress`" - "`purchasesOnGoogle`" 
    #[serde(rename="channelType")]
    
    pub channel_type: Option<String>,
    /// The details of the customer who placed the order.
    
    pub customer: Option<OrderCustomer>,
    /// Delivery details for shipments of type `delivery`.
    #[serde(rename="deliveryDetails")]
    
    pub delivery_details: Option<OrderDeliveryDetails>,
    /// The REST ID of the order. Globally unique.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#order`"
    
    pub kind: Option<String>,
    /// Line items that are ordered.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderLineItem>>,
    /// no description provided
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// Merchant-provided ID of the order.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80.
    #[serde(rename="netAmount")]
    
    pub net_amount: Option<Price>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    
    pub payment_method: Option<OrderPaymentMethod>,
    /// The status of the payment. Acceptable values are: - "`paymentCaptured`" - "`paymentRejected`" - "`paymentSecured`" - "`pendingAuthorization`" 
    #[serde(rename="paymentStatus")]
    
    pub payment_status: Option<String>,
    /// Pickup details for shipments of type `pickup`.
    #[serde(rename="pickupDetails")]
    
    pub pickup_details: Option<OrderPickupDetails>,
    /// The date when the order was placed, in ISO 8601 format.
    #[serde(rename="placedDate")]
    
    pub placed_date: Option<String>,
    /// The details of the merchant provided promotions applied to the order. To determine which promotions apply to which products, check the `Promotions[].Benefits[].OfferIds` field against the `LineItems[].Product.OfferId` field for each promotion. If a promotion is applied to more than 1 `offerId`, divide the discount value by the number of affected offers to determine how much discount to apply to each `offerId`. Examples: 1. To calculate the line item level discount for a single specific item: For each promotion, subtract the `Promotions[].Benefits[].Discount.value` amount from the `LineItems[].Price.value`. 2. To calculate the line item level discount for multiple quantity of a specific item: For each promotion, divide the `Promotions[].Benefits[].Discount.value` by the quantity of products and substract it from `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here.
    
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
    /// Deprecated. Shipping details are provided with line items instead. Acceptable values are: - "`economy`" - "`expedited`" - "`oneDay`" - "`sameDay`" - "`standard`" - "`twoDay`" 
    #[serde(rename="shippingOption")]
    
    pub shipping_option: Option<String>,
    /// The status of the order. Acceptable values are: - "`canceled`" - "`delivered`" - "`inProgress`" - "`partiallyDelivered`" - "`partiallyReturned`" - "`partiallyShipped`" - "`pendingShipment`" - "`returned`" - "`shipped`" 
    
    pub status: Option<String>,
    /// The party responsible for collecting and remitting taxes. Acceptable values are: - "`marketplaceFacilitator`" - "`merchant`" 
    #[serde(rename="taxCollector")]
    
    pub tax_collector: Option<String>,
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
    /// Strings representing the lines of the printed label for mailing the order, for example: John Smith 1600 Amphitheatre Parkway Mountain View, CA, 94043 United States 
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
    /// The actor that created the cancellation. Acceptable values are: - "`customer`" - "`googleBot`" - "`googleCustomerService`" - "`googlePayments`" - "`googleSabre`" - "`merchant`" 
    
    pub actor: Option<String>,
    /// Date on which the cancellation has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// The quantity that was canceled.
    
    pub quantity: Option<u32>,
    /// The reason for the cancellation. Orders that are canceled with a noInventory reason will lead to the removal of the product from Buy on Google until you make an update to that product. This will not affect your Shopping ads. Acceptable values are: - "`autoPostInternal`" - "`autoPostInvalidBillingAddress`" - "`autoPostNoInventory`" - "`autoPostPriceError`" - "`autoPostUndeliverableShippingAddress`" - "`couponAbuse`" - "`customerCanceled`" - "`customerInitiatedCancel`" - "`customerSupportRequested`" - "`failToPushOrderGoogleError`" - "`failToPushOrderMerchantError`" - "`failToPushOrderMerchantFulfillmentError`" - "`failToPushOrderToMerchant`" - "`failToPushOrderToMerchantOutOfStock`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`merchantDidNotShipOnTime`" - "`noInventory`" - "`orderTimeout`" - "`other`" - "`paymentAbuse`" - "`paymentDeclined`" - "`priceError`" - "`returnRefundAbuse`" - "`shippingPriceError`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" 
    
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
    /// Email address for the merchant to send value-added tax or invoice documentation of the order. Only the last document sent is made available to the customer. For more information, see About automated VAT invoicing for Buy on Google.
    #[serde(rename="invoiceReceivingEmail")]
    
    pub invoice_receiving_email: Option<String>,
    /// Customer's marketing preferences. Contains the marketing opt-in information that is current at the time that the merchant call. User preference selections can change from one order to the next so preferences must be checked with every order.
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
    /// Last known customer selection regarding marketing preferences. In certain cases this selection might not be known, so this field would be empty. If a customer selected `granted` in their most recent order, they can be subscribed to marketing emails. Customers who have chosen `denied` must not be subscribed, or must be unsubscribed if already opted-in. Acceptable values are: - "`denied`" - "`granted`" 
    #[serde(rename="explicitMarketingPreference")]
    
    pub explicit_marketing_preference: Option<String>,
    /// Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet.
    #[serde(rename="lastUpdatedTimestamp")]
    
    pub last_updated_timestamp: Option<String>,
    /// Email address that can be used for marketing purposes. The field may be empty even if `explicitMarketingPreference` is 'granted'. This happens when retrieving an old order from the customer who deleted their account.
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
    /// The date and time frame when the promotion is active and ready for validation review. Note that the promotion live time may be delayed for a few hours due to the validation review. Start date and end date are separated by a forward slash (/). The start date is specified by the format (YYYY-MM-DD), followed by the letter ?T?, the time of the day when the sale starts (in Greenwich Mean Time, GMT), followed by an expression of the time zone for the sale. The end date is in the same format.
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
    /// Whether the promotion is applicable to all products or only specific products. Acceptable values are: - "`allProducts`" - "`specificProducts`" 
    #[serde(rename="productApplicability")]
    
    pub product_applicability: Option<String>,
    /// Indicates that the promotion is valid online. Acceptable values are: - "`online`" 
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
    /// Further describes the benefit of the promotion. Note that we will expand on this enumeration as we support new promotion sub-types. Acceptable values are: - "`buyMGetMoneyOff`" - "`buyMGetNMoneyOff`" - "`buyMGetNPercentOff`" - "`buyMGetPercentOff`" - "`freeGift`" - "`freeGiftWithItemId`" - "`freeGiftWithValue`" - "`freeOvernightShipping`" - "`freeShipping`" - "`freeTwoDayShipping`" - "`moneyOff`" - "`percentageOff`" - "`rewardPoints`" - "`salePrice`" 
    #[serde(rename="subType")]
    
    pub sub_type: Option<String>,
    /// The impact on tax when the promotion is applied.
    #[serde(rename="taxImpact")]
    
    pub tax_impact: Option<Price>,
    /// Describes whether the promotion applies to products (e.g. 20% off) or to shipping (e.g. Free Shipping). Acceptable values are: - "`product`" - "`shipping`" 
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
    /// The ID of the line item.
    
    pub id: Option<String>,
    /// Total price for the line item. For example, if two items for $10 are purchased, the total price will be $20.
    
    pub price: Option<Price>,
    /// Product data as seen by customer from the time of the order placement. Note that certain attributes values (e.g. title or gtin) might be reformatted and no longer match values submitted via product feed.
    
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
    /// Number of items ready for pickup.
    #[serde(rename="quantityReadyForPickup")]
    
    pub quantity_ready_for_pickup: Option<u32>,
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
    /// The item's channel (online or local). Acceptable values are: - "`local`" - "`online`" 
    
    pub channel: Option<String>,
    /// Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`" 
    
    pub condition: Option<String>,
    /// The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Associated fees at order creation time.
    
    pub fees: Option<Vec<OrderLineItemProductFee>>,
    /// Global Trade Item Number (GTIN) of the item.
    
    pub gtin: Option<String>,
    /// The REST ID of the product.
    
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
    /// The CLDR territory // code of the target country of the product.
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
pub struct OrderLineItemProductFee {
    /// Amount of the fee.
    
    pub amount: Option<Price>,
    /// Name of the fee.
    
    pub name: Option<String>,
}

impl client::Part for OrderLineItemProductFee {}


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
    /// Required. How many days later the item can be returned.
    #[serde(rename="daysToReturn")]
    
    pub days_to_return: Option<i32>,
    /// Required. Whether the item is returnable.
    #[serde(rename="isReturnable")]
    
    pub is_returnable: Option<bool>,
    /// Required. URL of the item return policy.
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
    /// Required. The delivery by date, in ISO 8601 format.
    #[serde(rename="deliverByDate")]
    
    pub deliver_by_date: Option<String>,
    /// Required. Details of the shipping method.
    
    pub method: Option<OrderLineItemShippingDetailsMethod>,
    /// Required. The ship by date, in ISO 8601 format.
    #[serde(rename="shipByDate")]
    
    pub ship_by_date: Option<String>,
    /// Type of shipment. Indicates whether `deliveryDetails` or `pickupDetails` is applicable for this shipment. Acceptable values are: - "`delivery`" - "`pickup`" 
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for OrderLineItemShippingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderLineItemShippingDetailsMethod {
    /// The carrier for the shipping. Optional. See `shipments[].carrier` for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Required. Maximum transit time.
    #[serde(rename="maxDaysInTransit")]
    
    pub max_days_in_transit: Option<u32>,
    /// Required. The name of the shipping method.
    #[serde(rename="methodName")]
    
    pub method_name: Option<String>,
    /// Required. Minimum transit time.
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
    /// The type of instrument. Acceptable values are: - "`AMEX`" - "`DISCOVER`" - "`JCB`" - "`MASTERCARD`" - "`UNIONPAY`" - "`VISA`" - "``" 
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
pub struct OrderPickupDetails {
    /// Address of the pickup location where the shipment should be sent. Note that `recipientName` in the address is the name of the business at the pickup location.
    
    pub address: Option<OrderAddress>,
    /// Collectors authorized to pick up shipment from the pickup location.
    
    pub collectors: Option<Vec<OrderPickupDetailsCollector>>,
    /// ID of the pickup location.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::Part for OrderPickupDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderPickupDetailsCollector {
    /// Name of the person picking up the shipment.
    
    pub name: Option<String>,
    /// Phone number of the person picking up the shipment.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
}

impl client::Part for OrderPickupDetailsCollector {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderRefund {
    /// The actor that created the refund. Acceptable values are: - "`customer`" - "`googleBot`" - "`googleCustomerService`" - "`googlePayments`" - "`googleSabre`" - "`merchant`" 
    
    pub actor: Option<String>,
    /// The amount that is refunded.
    
    pub amount: Option<Price>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// The reason for the refund. Acceptable values are: - "`adjustment`" - "`autoPostInternal`" - "`autoPostInvalidBillingAddress`" - "`autoPostNoInventory`" - "`autoPostPriceError`" - "`autoPostUndeliverableShippingAddress`" - "`couponAbuse`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`customerSupportRequested`" - "`deliveredLateByCarrier`" - "`deliveredTooLate`" - "`expiredItem`" - "`failToPushOrderGoogleError`" - "`failToPushOrderMerchantError`" - "`failToPushOrderMerchantFulfillmentError`" - "`failToPushOrderToMerchant`" - "`failToPushOrderToMerchantOutOfStock`" - "`feeAdjustment`" - "`invalidCoupon`" - "`lateShipmentCredit`" - "`malformedShippingAddress`" - "`merchantDidNotShipOnTime`" - "`noInventory`" - "`orderTimeout`" - "`other`" - "`paymentAbuse`" - "`paymentDeclined`" - "`priceAdjustment`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`promoReallocation`" - "`qualityNotAsExpected`" - "`returnRefundAbuse`" - "`shippingCostAdjustment`" - "`shippingPriceError`" - "`taxAdjustment`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
    pub reason: Option<String>,
    /// The explanation of the reason.
    #[serde(rename="reasonText")]
    
    pub reason_text: Option<String>,
}

impl client::Part for OrderRefund {}


/// Order disbursement. All methods require the payment analyst role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderReportDisbursement {
    /// The disbursement amount.
    #[serde(rename="disbursementAmount")]
    
    pub disbursement_amount: Option<Price>,
    /// The disbursement date, in ISO 8601 format.
    #[serde(rename="disbursementCreationDate")]
    
    pub disbursement_creation_date: Option<String>,
    /// The date the disbursement was initiated, in ISO 8601 format.
    #[serde(rename="disbursementDate")]
    
    pub disbursement_date: Option<String>,
    /// The ID of the disbursement.
    #[serde(rename="disbursementId")]
    
    pub disbursement_id: Option<String>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
}

impl client::Part for OrderReportDisbursement {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderReportTransaction {
    /// The disbursement amount.
    #[serde(rename="disbursementAmount")]
    
    pub disbursement_amount: Option<Price>,
    /// The date the disbursement was created, in ISO 8601 format.
    #[serde(rename="disbursementCreationDate")]
    
    pub disbursement_creation_date: Option<String>,
    /// The date the disbursement was initiated, in ISO 8601 format.
    #[serde(rename="disbursementDate")]
    
    pub disbursement_date: Option<String>,
    /// The ID of the disbursement.
    #[serde(rename="disbursementId")]
    
    pub disbursement_id: Option<String>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// Merchant-provided ID of the order.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The ID of the order.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Total amount for the items.
    #[serde(rename="productAmount")]
    
    pub product_amount: Option<Amount>,
    /// Total amount with remitted tax for the items.
    #[serde(rename="productAmountWithRemittedTax")]
    
    pub product_amount_with_remitted_tax: Option<ProductAmount>,
    /// The date of the transaction, in ISO 8601 format.
    #[serde(rename="transactionDate")]
    
    pub transaction_date: Option<String>,
}

impl client::Part for OrderReportTransaction {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderReturn {
    /// The actor that created the refund. Acceptable values are: - "`customer`" - "`googleBot`" - "`googleCustomerService`" - "`googlePayments`" - "`googleSabre`" - "`merchant`" 
    
    pub actor: Option<String>,
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// Quantity that is returned.
    
    pub quantity: Option<u32>,
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The carrier handling the shipment. For supported carriers, Google includes the carrier name and tracking URL in emails to customers. For select supported carriers, Google also automatically updates the shipment status based on the provided shipment ID. *Note:* You can also use unsupported carriers, but emails to customers will not include the carrier name or tracking URL, and there will be no automatic order status updates. Supported carriers for US are: - "`ups`" (United Parcel Service) *automatic status updates* - "`usps`" (United States Postal Service) *automatic status updates* - "`fedex`" (FedEx) *automatic status updates * - "`dhl`" (DHL eCommerce) *automatic status updates* (US only) - "`ontrac`" (OnTrac) *automatic status updates * - "`dhl express`" (DHL Express) - "`deliv`" (Deliv) - "`dynamex`" (TForce) - "`lasership`" (LaserShip) - "`mpx`" (Military Parcel Xpress) - "`uds`" (United Delivery Service) - "`efw`" (Estes Forwarding Worldwide) - "`jd logistics`" (JD Logistics) - "`yunexpress`" (YunExpress) - "`china post`" (China Post) - "`china ems`" (China Post Express Mail Service) - "`singapore post`" (Singapore Post) - "`pos malaysia`" (Pos Malaysia) - "`postnl`" (PostNL) - "`ptt`" (PTT Turkish Post) - "`eub`" (ePacket) - "`chukou1`" (Chukou1 Logistics) - "`bestex`" (Best Express) - "`canada post`" (Canada Post) - "`purolator`" (Purolator) - "`canpar`" (Canpar) - "`india post`" (India Post) - "`blue dart`" (Blue Dart) - "`delhivery`" (Delhivery) - "`dtdc`" (DTDC) - "`tpc india`" (TPC India) Supported carriers for FR are: - "`la poste`" (La Poste) *automatic status updates * - "`colissimo`" (Colissimo by La Poste) *automatic status updates* - "`ups`" (United Parcel Service) *automatic status updates * - "`chronopost`" (Chronopost by La Poste) - "`gls`" (General Logistics Systems France) - "`dpd`" (DPD Group by GeoPost) - "`bpost`" (Belgian Post Group) - "`colis prive`" (Colis Privé) - "`boxtal`" (Boxtal) - "`geodis`" (GEODIS) - "`tnt`" (TNT) - "`db schenker`" (DB Schenker) - "`aramex`" (Aramex) 
    
    pub carrier: Option<String>,
    /// Date on which the shipment has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Present only if `status` is `delivered`
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The ID of the shipment.
    
    pub id: Option<String>,
    /// The line items that are shipped.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<OrderShipmentLineItemShipment>>,
    /// Delivery details of the shipment if scheduling is needed.
    #[serde(rename="scheduledDeliveryDetails")]
    
    pub scheduled_delivery_details: Option<OrderShipmentScheduledDeliveryDetails>,
    /// The status of the shipment. Acceptable values are: - "`delivered`" - "`readyForPickup`" - "`shipped`" - "`undeliverable`" 
    
    pub status: Option<String>,
    /// The tracking ID for the shipment.
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
    /// The ID of the line item that is shipped. This value is assigned by Google when an order is created. Either lineItemId or productId is required.
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
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderShipmentScheduledDeliveryDetails {
    /// The phone number of the carrier fulfilling the delivery. The phone number is formatted as the international notation in ITU-T Recommendation E.123 (e.g., "+41 44 668 1800").
    #[serde(rename="carrierPhoneNumber")]
    
    pub carrier_phone_number: Option<String>,
    /// The date a shipment is scheduled for delivery, in ISO 8601 format.
    #[serde(rename="scheduledDate")]
    
    pub scheduled_date: Option<String>,
}

impl client::Part for OrderShipmentScheduledDeliveryDetails {}


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
    /// [required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges.
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// Option to create a refund-only invoice. Exactly one of `refundOnlyOption` or `returnOption` must be provided.
    #[serde(rename="refundOnlyOption")]
    
    pub refund_only_option: Option<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption>,
    /// Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of `refundOnlyOption` or `returnOption` must be provided.
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// [required] Reason for the refund. Acceptable values are: - "`adjustment`" - "`autoPostInternal`" - "`autoPostInvalidBillingAddress`" - "`autoPostNoInventory`" - "`autoPostPriceError`" - "`autoPostUndeliverableShippingAddress`" - "`couponAbuse`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`customerSupportRequested`" - "`deliveredLateByCarrier`" - "`deliveredTooLate`" - "`expiredItem`" - "`failToPushOrderGoogleError`" - "`failToPushOrderMerchantError`" - "`failToPushOrderMerchantFulfillmentError`" - "`failToPushOrderToMerchant`" - "`failToPushOrderToMerchantOutOfStock`" - "`feeAdjustment`" - "`invalidCoupon`" - "`lateShipmentCredit`" - "`malformedShippingAddress`" - "`merchantDidNotShipOnTime`" - "`noInventory`" - "`orderTimeout`" - "`other`" - "`paymentAbuse`" - "`paymentDeclined`" - "`priceAdjustment`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`promoReallocation`" - "`qualityNotAsExpected`" - "`returnRefundAbuse`" - "`shippingCostAdjustment`" - "`shippingPriceError`" - "`taxAdjustment`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// [required] Reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
/// * [listdisbursements orderreports](OrderreportListdisbursementCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderreportsListDisbursementsResponse {
    /// The list of disbursements.
    
    pub disbursements: Option<Vec<OrderReportDisbursement>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderreportsListDisbursementsResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of disbursements.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for OrderreportsListDisbursementsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [listtransactions orderreports](OrderreportListtransactionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderreportsListTransactionsResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#orderreportsListTransactionsResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of transactions.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of transactions.
    
    pub transactions: Option<Vec<OrderReportTransaction>>,
}

impl client::ResponseResult for OrderreportsListTransactionsResponse {}


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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided.
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
    /// The reason for the cancellation. Acceptable values are: - "`customerInitiatedCancel`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`noInventory`" - "`other`" - "`priceError`" - "`shippingPriceError`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The reason for the cancellation. Acceptable values are: - "`customerInitiatedCancel`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`noInventory`" - "`other`" - "`priceError`" - "`shippingPriceError`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The reason for the cancellation. Acceptable values are: - "`changedMind`" - "`orderedWrongItem`" - "`other`" 
    
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
    /// The CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via `template_name`, or the addresses of orders created via `test_order`. Acceptable values are: - "`US`" - "`FR`" Defaults to `US`.
    
    pub country: Option<String>,
    /// The test order template to use. Specify as an alternative to `testOrder` as a shortcut for retrieving a template and then creating an order using that template. Acceptable values are: - "`template1`" - "`template1a`" - "`template1b`" - "`template2`" - "`template3`" 
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
    /// Required for `cancel` method.
    
    pub cancel: Option<OrdersCustomBatchRequestEntryCancel>,
    /// Required for `cancelLineItem` method.
    #[serde(rename="cancelLineItem")]
    
    pub cancel_line_item: Option<OrdersCustomBatchRequestEntryCancelLineItem>,
    /// Required for `inStoreReturnLineItem` method.
    #[serde(rename="inStoreRefundLineItem")]
    
    pub in_store_refund_line_item: Option<OrdersCustomBatchRequestEntryInStoreRefundLineItem>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The merchant order ID. Required for `updateMerchantOrderId` and `getByMerchantOrderId` methods.
    #[serde(rename="merchantOrderId")]
    
    pub merchant_order_id: Option<String>,
    /// The method of the batch entry. Acceptable values are: - "`acknowledge`" - "`cancel`" - "`cancelLineItem`" - "`get`" - "`getByMerchantOrderId`" - "`inStoreRefundLineItem`" - "`refund`" - "`rejectReturnLineItem`" - "`returnLineItem`" - "`returnRefundLineItem`" - "`setLineItemMetadata`" - "`shipLineItems`" - "`updateLineItemShippingDetails`" - "`updateMerchantOrderId`" - "`updateShipment`" 
    
    pub method: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order. Required for all methods beside `get` and `getByMerchantOrderId`.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the order. Required for all methods beside `getByMerchantOrderId`.
    #[serde(rename="orderId")]
    
    pub order_id: Option<String>,
    /// Required for `refund` method.
    
    pub refund: Option<OrdersCustomBatchRequestEntryRefund>,
    /// Required for `rejectReturnLineItem` method.
    #[serde(rename="rejectReturnLineItem")]
    
    pub reject_return_line_item: Option<OrdersCustomBatchRequestEntryRejectReturnLineItem>,
    /// Required for `returnLineItem` method.
    #[serde(rename="returnLineItem")]
    
    pub return_line_item: Option<OrdersCustomBatchRequestEntryReturnLineItem>,
    /// Required for `returnRefundLineItem` method.
    #[serde(rename="returnRefundLineItem")]
    
    pub return_refund_line_item: Option<OrdersCustomBatchRequestEntryReturnRefundLineItem>,
    /// Required for `setLineItemMetadata` method.
    #[serde(rename="setLineItemMetadata")]
    
    pub set_line_item_metadata: Option<OrdersCustomBatchRequestEntrySetLineItemMetadata>,
    /// Required for `shipLineItems` method.
    #[serde(rename="shipLineItems")]
    
    pub ship_line_items: Option<OrdersCustomBatchRequestEntryShipLineItems>,
    /// Required for `updateLineItemShippingDate` method.
    #[serde(rename="updateLineItemShippingDetails")]
    
    pub update_line_item_shipping_details: Option<OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails>,
    /// Required for `updateShipment` method.
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
    /// The reason for the cancellation. Acceptable values are: - "`customerInitiatedCancel`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`noInventory`" - "`other`" - "`priceError`" - "`shippingPriceError`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" 
    
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
    /// Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided.
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
    /// The reason for the cancellation. Acceptable values are: - "`customerInitiatedCancel`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`noInventory`" - "`other`" - "`priceError`" - "`shippingPriceError`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" 
    
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
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The amount that is refunded. Either amount or amountPretax should be filled.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The reason for the refund. Acceptable values are: - "`adjustment`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`deliveredLateByCarrier`" - "`feeAdjustment`" - "`lateShipmentCredit`" - "`noInventory`" - "`other`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`shippingCostAdjustment`" - "`taxAdjustment`" - "`undeliverableShippingAddress`" - "`wrongProductShipped`" 
    
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
    /// The reason for the return. Acceptable values are: - "`damagedOrUsed`" - "`missingComponent`" - "`notEligible`" - "`other`" - "`outOfReturnWindow`" 
    
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
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method).
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided.
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
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values.
    
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
    /// Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment.
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
    /// The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Required. The ID of the shipment. This is assigned by the merchant and is unique to each shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// The tracking ID for the shipment.
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
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past.
    #[serde(rename="deliverByDate")]
    
    pub deliver_by_date: Option<String>,
    /// The ID of the line item to set metadata. Either lineItemId or productId is required.
    #[serde(rename="lineItemId")]
    
    pub line_item_id: Option<String>,
    /// The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past.
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
    /// The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`.
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// New status for the shipment. Not updated if missing. Acceptable values are: - "`delivered`" - "`undeliverable`" - "`readyForPickup`" 
    
    pub status: Option<String>,
    /// The tracking ID for the shipment. Not updated if missing.
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
    /// The status of the execution. Only defined if 1. the request was successful; and 2. the method is not `get`, `getByMerchantOrderId`, or one of the test methods. Acceptable values are: - "`duplicate`" - "`executed`" 
    #[serde(rename="executionStatus")]
    
    pub execution_status: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#ordersCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The retrieved order. Only defined if the method is `get` and if the request was successful.
    
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
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The amount that is refunded. Either amount or amountPretax should be filled.
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided.
    #[serde(rename="amountTax")]
    
    pub amount_tax: Option<Price>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The reason for the refund. Acceptable values are: - "`adjustment`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`deliveredLateByCarrier`" - "`feeAdjustment`" - "`lateShipmentCredit`" - "`noInventory`" - "`other`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`shippingCostAdjustment`" - "`taxAdjustment`" - "`undeliverableShippingAddress`" - "`wrongProductShipped`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The reason for the return. Acceptable values are: - "`damagedOrUsed`" - "`missingComponent`" - "`notEligible`" - "`other`" - "`outOfReturnWindow`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method).
    #[serde(rename="amountPretax")]
    
    pub amount_pretax: Option<Price>,
    /// Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided.
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
    /// The quantity to return and refund. Quantity is required.
    
    pub quantity: Option<u32>,
    /// The reason for the return. Acceptable values are: - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`deliveredTooLate`" - "`expiredItem`" - "`invalidCoupon`" - "`malformedShippingAddress`" - "`other`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`qualityNotAsExpected`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
    
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values.
    
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
    /// Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment.
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past.
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
    /// Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past.
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
    /// The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values.
    
    pub carrier: Option<String>,
    /// Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`.
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// The ID of the operation. Unique across all operations for a given order.
    #[serde(rename="operationId")]
    
    pub operation_id: Option<String>,
    /// The ID of the shipment.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// New status for the shipment. Not updated if missing. Acceptable values are: - "`delivered`" - "`undeliverable`" - "`readyForPickup`" 
    
    pub status: Option<String>,
    /// The tracking ID for the shipment. Not updated if missing.
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
    /// The status of the execution. Acceptable values are: - "`duplicate`" - "`executed`" 
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
pub struct PickupCarrierService {
    /// The name of the pickup carrier (e.g., `"UPS"`). Required.
    #[serde(rename="carrierName")]
    
    pub carrier_name: Option<String>,
    /// The name of the pickup service (e.g., `"Access point"`). Required.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for PickupCarrierService {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PickupServicesPickupService {
    /// The name of the carrier (e.g., `"UPS"`). Always present.
    #[serde(rename="carrierName")]
    
    pub carrier_name: Option<String>,
    /// The CLDR country code of the carrier (e.g., "US"). Always present.
    
    pub country: Option<String>,
    /// The name of the pickup service (e.g., `"Access point"`). Always present.
    #[serde(rename="serviceName")]
    
    pub service_name: Option<String>,
}

impl client::Part for PickupServicesPickupService {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch pos](PoCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<PosCustomBatchRequestEntry>>,
}

impl client::RequestValue for PosCustomBatchRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The inventory to submit. This should be set only if the method is `inventory`.
    
    pub inventory: Option<PosInventory>,
    /// The ID of the POS data provider.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`delete`" - "`get`" - "`insert`" - "`inventory`" - "`sale`" 
    
    pub method: Option<String>,
    /// The sale information to submit. This should be set only if the method is `sale`.
    
    pub sale: Option<PosSale>,
    /// The store information to submit. This should be set only if the method is `insert`.
    
    pub store: Option<PosStore>,
    /// The store code. This should be set only if the method is `delete` or `get`.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// The ID of the account for which to get/submit data.
    #[serde(rename="targetMerchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub target_merchant_id: Option<u64>,
}

impl client::Part for PosCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch pos](PoCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<PosCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#posCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for PosCustomBatchResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosCustomBatchResponseEntry {
    /// The ID of the request entry to which this entry responds.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if, and only if, the request failed.
    
    pub errors: Option<Errors>,
    /// The updated inventory information.
    
    pub inventory: Option<PosInventory>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#posCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The updated sale information.
    
    pub sale: Option<PosSale>,
    /// The retrieved or updated store information.
    
    pub store: Option<PosStore>,
}

impl client::Part for PosCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosDataProviders {
    /// Country code.
    
    pub country: Option<String>,
    /// A list of POS data providers.
    #[serde(rename="posDataProviders")]
    
    pub pos_data_providers: Option<Vec<PosDataProvidersPosDataProvider>>,
}

impl client::Part for PosDataProviders {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosDataProvidersPosDataProvider {
    /// The display name of Pos data Provider.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The full name of this POS data Provider.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// The ID of the account.
    #[serde(rename="providerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub provider_id: Option<u64>,
}

impl client::Part for PosDataProvidersPosDataProvider {}


/// The absolute quantity of an item available at the given store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosInventory {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#posInventory`"
    
    pub kind: Option<String>,
    /// Required. The current price of the item.
    
    pub price: Option<Price>,
    /// Required. The available quantity of the item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::Part for PosInventory {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inventory pos](PoInventoryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosInventoryRequest {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Required. The current price of the item.
    
    pub price: Option<Price>,
    /// Required. The available quantity of the item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::RequestValue for PosInventoryRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [inventory pos](PoInventoryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosInventoryResponse {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#posInventoryResponse".
    
    pub kind: Option<String>,
    /// Required. The current price of the item.
    
    pub price: Option<Price>,
    /// Required. The available quantity of the item.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::ResponseResult for PosInventoryResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list pos](PoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#posListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<PosStore>>,
}

impl client::ResponseResult for PosListResponse {}


/// The change of the available quantity of an item at the given store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosSale {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#posSale`"
    
    pub kind: Option<String>,
    /// Required. The price of the item.
    
    pub price: Option<Price>,
    /// Required. The relative change of the available quantity. Negative for items returned.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// A unique ID to group items from the same sale event.
    #[serde(rename="saleId")]
    
    pub sale_id: Option<String>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::Part for PosSale {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sale pos](PoSaleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosSaleRequest {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Required. The price of the item.
    
    pub price: Option<Price>,
    /// Required. The relative change of the available quantity. Negative for items returned.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// A unique ID to group items from the same sale event.
    #[serde(rename="saleId")]
    
    pub sale_id: Option<String>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::RequestValue for PosSaleRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sale pos](PoSaleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosSaleResponse {
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Global Trade Item Number.
    
    pub gtin: Option<String>,
    /// Required. A unique identifier for the item.
    #[serde(rename="itemId")]
    
    pub item_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#posSaleResponse".
    
    pub kind: Option<String>,
    /// Required. The price of the item.
    
    pub price: Option<Price>,
    /// Required. The relative change of the available quantity. Negative for items returned.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub quantity: Option<i64>,
    /// A unique ID to group items from the same sale event.
    #[serde(rename="saleId")]
    
    pub sale_id: Option<String>,
    /// Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The inventory timestamp, in ISO 8601 format.
    
    pub timestamp: Option<String>,
}

impl client::ResponseResult for PosSaleResponse {}


/// Store resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get pos](PoGetCall) (response)
/// * [insert pos](PoInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PosStore {
    /// The business type of the store.
    #[serde(rename="gcidCategory")]
    
    pub gcid_category: Option<Vec<String>>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#posStore`"
    
    pub kind: Option<String>,
    /// The store phone number.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// The Google Place Id of the store location.
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Required. The street address of the store.
    #[serde(rename="storeAddress")]
    
    pub store_address: Option<String>,
    /// Required. A store identifier that is unique for the given merchant.
    #[serde(rename="storeCode")]
    
    pub store_code: Option<String>,
    /// The merchant or store name.
    #[serde(rename="storeName")]
    
    pub store_name: Option<String>,
    /// The website url for the store or merchant.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::RequestValue for PosStore {}
impl client::ResponseResult for PosStore {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalCodeGroup {
    /// The CLDR territory code of the country the postal code group applies to. Required.
    
    pub country: Option<String>,
    /// The name of the postal code group, referred to in headers. Required.
    
    pub name: Option<String>,
    /// A range of postal codes. Required.
    #[serde(rename="postalCodeRanges")]
    
    pub postal_code_ranges: Option<Vec<PostalCodeRange>>,
}

impl client::Part for PostalCodeGroup {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalCodeRange {
    /// A postal code or a pattern of the form `prefix*` denoting the inclusive lower bound of the range defining the area. Examples values: `"94108"`, `"9410*"`, `"9*"`. Required.
    #[serde(rename="postalCodeRangeBegin")]
    
    pub postal_code_range_begin: Option<String>,
    /// A postal code or a pattern of the form `prefix*` denoting the inclusive upper bound of the range defining the area. It must have the same length as `postalCodeRangeBegin`: if `postalCodeRangeBegin` is a postal code then `postalCodeRangeEnd` must be a postal code too; if `postalCodeRangeBegin` is a pattern then `postalCodeRangeEnd` must be a pattern with the same prefix length. Optional: if not set, then the area is defined as being all the postal codes matching `postalCodeRangeBegin`.
    #[serde(rename="postalCodeRangeEnd")]
    
    pub postal_code_range_end: Option<String>,
}

impl client::Part for PostalCodeRange {}


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


/// Required product attributes are primarily defined by the products data specification. See the Products Data Specification Help Center article for information. Product data. After inserting, updating, or deleting a product, it may take several minutes before changes take effect.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch products](ProductCustombatchCall) (none)
/// * [delete products](ProductDeleteCall) (none)
/// * [get products](ProductGetCall) (response)
/// * [insert products](ProductInsertCall) (request|response)
/// * [list products](ProductListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// Additional URLs of images of the item.
    #[serde(rename="additionalImageLinks")]
    
    pub additional_image_links: Option<Vec<String>>,
    /// Additional categories of the item (formatted as in products data specification).
    #[serde(rename="additionalProductTypes")]
    
    pub additional_product_types: Option<Vec<String>>,
    /// Should be set to true if the item is targeted towards adults.
    
    pub adult: Option<bool>,
    /// Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise.
    #[serde(rename="adwordsGrouping")]
    
    pub adwords_grouping: Option<String>,
    /// Similar to adwords_grouping, but only works on CPC.
    #[serde(rename="adwordsLabels")]
    
    pub adwords_labels: Option<Vec<String>>,
    /// Allows advertisers to override the item URL when the product is shown within the context of Product Ads.
    #[serde(rename="adwordsRedirect")]
    
    pub adwords_redirect: Option<String>,
    /// Target age group of the item. Acceptable values are: - "`adult`" - "`infant`" - "`kids`" - "`newborn`" - "`toddler`" - "`youngAdult`" 
    #[serde(rename="ageGroup")]
    
    pub age_group: Option<String>,
    /// Deprecated. Do not use.
    
    pub aspects: Option<Vec<ProductAspect>>,
    /// Availability status of the item. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`" 
    
    pub availability: Option<String>,
    /// The day a pre-ordered product becomes available for delivery, in ISO 8601 format.
    #[serde(rename="availabilityDate")]
    
    pub availability_date: Option<String>,
    /// Brand of the item.
    
    pub brand: Option<String>,
    /// URL for the canonical version of your item's landing page.
    #[serde(rename="canonicalLink")]
    
    pub canonical_link: Option<String>,
    /// Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`" 
    
    pub channel: Option<String>,
    /// Color of the item.
    
    pub color: Option<String>,
    /// Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`" 
    
    pub condition: Option<String>,
    /// Required. The two-letter ISO 639-1 language code for the item.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Cost of goods sold. Used for gross profit reporting.
    #[serde(rename="costOfGoodsSold")]
    
    pub cost_of_goods_sold: Option<Price>,
    /// A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions).
    #[serde(rename="customAttributes")]
    
    pub custom_attributes: Option<Vec<CustomAttribute>>,
    /// A list of custom (merchant-provided) custom attribute groups.
    #[serde(rename="customGroups")]
    
    pub custom_groups: Option<Vec<CustomGroup>>,
    /// Custom label 0 for custom grouping of items in a Shopping campaign.
    #[serde(rename="customLabel0")]
    
    pub custom_label0: Option<String>,
    /// Custom label 1 for custom grouping of items in a Shopping campaign.
    #[serde(rename="customLabel1")]
    
    pub custom_label1: Option<String>,
    /// Custom label 2 for custom grouping of items in a Shopping campaign.
    #[serde(rename="customLabel2")]
    
    pub custom_label2: Option<String>,
    /// Custom label 3 for custom grouping of items in a Shopping campaign.
    #[serde(rename="customLabel3")]
    
    pub custom_label3: Option<String>,
    /// Custom label 4 for custom grouping of items in a Shopping campaign.
    #[serde(rename="customLabel4")]
    
    pub custom_label4: Option<String>,
    /// Description of the item.
    
    pub description: Option<String>,
    /// Specifies the intended destinations for the product.
    
    pub destinations: Option<Vec<ProductDestination>>,
    /// An identifier for an item for dynamic remarketing campaigns.
    #[serde(rename="displayAdsId")]
    
    pub display_ads_id: Option<String>,
    /// URL directly to your item's landing page for dynamic remarketing campaigns.
    #[serde(rename="displayAdsLink")]
    
    pub display_ads_link: Option<String>,
    /// Advertiser-specified recommendations.
    #[serde(rename="displayAdsSimilarIds")]
    
    pub display_ads_similar_ids: Option<Vec<String>>,
    /// Title of an item for dynamic remarketing campaigns.
    #[serde(rename="displayAdsTitle")]
    
    pub display_ads_title: Option<String>,
    /// Offer margin for dynamic remarketing campaigns.
    #[serde(rename="displayAdsValue")]
    
    pub display_ads_value: Option<f64>,
    /// The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`" 
    #[serde(rename="energyEfficiencyClass")]
    
    pub energy_efficiency_class: Option<String>,
    /// Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future.
    #[serde(rename="expirationDate")]
    
    pub expiration_date: Option<String>,
    /// Target gender of the item. Acceptable values are: - "`female`" - "`male`" - "`unisex`" 
    
    pub gender: Option<String>,
    /// Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API.
    #[serde(rename="googleProductCategory")]
    
    pub google_product_category: Option<String>,
    /// Global Trade Item Number (GTIN) of the item.
    
    pub gtin: Option<String>,
    /// The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId.
    
    pub id: Option<String>,
    /// False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada.
    #[serde(rename="identifierExists")]
    
    pub identifier_exists: Option<bool>,
    /// URL of an image of the item.
    #[serde(rename="imageLink")]
    
    pub image_link: Option<String>,
    /// Number and amount of installments to pay for an item.
    
    pub installment: Option<Installment>,
    /// Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price.
    #[serde(rename="isBundle")]
    
    pub is_bundle: Option<bool>,
    /// Shared identifier for all variants of the same product.
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#product`"
    
    pub kind: Option<String>,
    /// URL directly linking to your item's page on your website.
    
    pub link: Option<String>,
    /// Loyalty points that users receive after purchasing the item. Japan only.
    #[serde(rename="loyaltyPoints")]
    
    pub loyalty_points: Option<LoyaltyPoints>,
    /// The material of which the item is made.
    
    pub material: Option<String>,
    /// The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`" 
    #[serde(rename="maxEnergyEfficiencyClass")]
    
    pub max_energy_efficiency_class: Option<String>,
    /// Maximal product handling time (in business days).
    #[serde(rename="maxHandlingTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_handling_time: Option<i64>,
    /// The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`" 
    #[serde(rename="minEnergyEfficiencyClass")]
    
    pub min_energy_efficiency_class: Option<String>,
    /// Minimal product handling time (in business days).
    #[serde(rename="minHandlingTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_handling_time: Option<i64>,
    /// URL for the mobile-optimized version of your item's landing page.
    #[serde(rename="mobileLink")]
    
    pub mobile_link: Option<String>,
    /// Manufacturer Part Number (MPN) of the item.
    
    pub mpn: Option<String>,
    /// The number of identical products in a merchant-defined multipack.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub multipack: Option<i64>,
    /// Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// Deprecated.
    #[serde(rename="onlineOnly")]
    
    pub online_only: Option<bool>,
    /// The item's pattern (e.g. polka dots).
    
    pub pattern: Option<String>,
    /// Price of the item.
    
    pub price: Option<Price>,
    /// Your category of the item (formatted as in products data specification).
    #[serde(rename="productType")]
    
    pub product_type: Option<String>,
    /// The unique ID of a promotion.
    #[serde(rename="promotionIds")]
    
    pub promotion_ids: Option<Vec<String>>,
    /// Advertised sale price of the item.
    #[serde(rename="salePrice")]
    
    pub sale_price: Option<Price>,
    /// Date range during which the item is on sale (see products data specification ).
    #[serde(rename="salePriceEffectiveDate")]
    
    pub sale_price_effective_date: Option<String>,
    /// The quantity of the product that is available for selling on Google. Supported only for online products.
    #[serde(rename="sellOnGoogleQuantity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sell_on_google_quantity: Option<i64>,
    /// Shipping rules.
    
    pub shipping: Option<Vec<ProductShipping>>,
    /// Height of the item for shipping.
    #[serde(rename="shippingHeight")]
    
    pub shipping_height: Option<ProductShippingDimension>,
    /// The shipping label of the product, used to group product in account-level shipping rules.
    #[serde(rename="shippingLabel")]
    
    pub shipping_label: Option<String>,
    /// Length of the item for shipping.
    #[serde(rename="shippingLength")]
    
    pub shipping_length: Option<ProductShippingDimension>,
    /// Weight of the item for shipping.
    #[serde(rename="shippingWeight")]
    
    pub shipping_weight: Option<ProductShippingWeight>,
    /// Width of the item for shipping.
    #[serde(rename="shippingWidth")]
    
    pub shipping_width: Option<ProductShippingDimension>,
    /// System in which the size is specified. Recommended for apparel items. Acceptable values are: - "`AU`" - "`BR`" - "`CN`" - "`DE`" - "`EU`" - "`FR`" - "`IT`" - "`JP`" - "`MEX`" - "`UK`" - "`US`" 
    #[serde(rename="sizeSystem")]
    
    pub size_system: Option<String>,
    /// The cut of the item. Recommended for apparel items. Acceptable values are: - "`big and tall`" - "`maternity`" - "`oversize`" - "`petite`" - "`plus`" - "`regular`" 
    #[serde(rename="sizeType")]
    
    pub size_type: Option<String>,
    /// Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition).
    
    pub sizes: Option<Vec<String>>,
    /// The source of the offer, i.e., how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`" 
    
    pub source: Option<String>,
    /// Required. The CLDR territory code for the item.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Tax information.
    
    pub taxes: Option<Vec<ProductTax>>,
    /// Title of the item.
    
    pub title: Option<String>,
    /// The preference of the denominator of the unit price.
    #[serde(rename="unitPricingBaseMeasure")]
    
    pub unit_pricing_base_measure: Option<ProductUnitPricingBaseMeasure>,
    /// The measure and dimension of an item.
    #[serde(rename="unitPricingMeasure")]
    
    pub unit_pricing_measure: Option<ProductUnitPricingMeasure>,
    /// Deprecated. The read-only list of intended destinations which passed validation.
    #[serde(rename="validatedDestinations")]
    
    pub validated_destinations: Option<Vec<String>>,
    /// Read-only warnings.
    
    pub warnings: Option<Vec<Error>>,
}

impl client::RequestValue for Product {}
impl client::Resource for Product {}
impl client::ResponseResult for Product {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductAmount {
    /// The pre-tax or post-tax price depending on the location of the order.
    #[serde(rename="priceAmount")]
    
    pub price_amount: Option<Price>,
    /// Remitted tax value.
    #[serde(rename="remittedTaxAmount")]
    
    pub remitted_tax_amount: Option<Price>,
    /// Tax value.
    #[serde(rename="taxAmount")]
    
    pub tax_amount: Option<Price>,
}

impl client::Part for ProductAmount {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductAspect {
    /// Deprecated.
    #[serde(rename="aspectName")]
    
    pub aspect_name: Option<String>,
    /// Deprecated.
    #[serde(rename="destinationName")]
    
    pub destination_name: Option<String>,
    /// Deprecated.
    
    pub intention: Option<String>,
}

impl client::Part for ProductAspect {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductDestination {
    /// The name of the destination.
    #[serde(rename="destinationName")]
    
    pub destination_name: Option<String>,
    /// Whether the destination is required, excluded or should be validated. Acceptable values are: - "`default`" - "`excluded`" - "`optional`" - "`required`" 
    
    pub intention: Option<String>,
}

impl client::Part for ProductDestination {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductShipping {
    /// The CLDR territory code of the country to which an item will ship.
    
    pub country: Option<String>,
    /// The location where the shipping is applicable, represented by a location group name.
    #[serde(rename="locationGroupName")]
    
    pub location_group_name: Option<String>,
    /// The numeric ID of a location that the shipping rate applies to as defined in the AdWords API.
    #[serde(rename="locationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub location_id: Option<i64>,
    /// The postal code range that the shipping rate applies to, represented by a postal code, a postal code prefix followed by a * wildcard, a range between two postal codes or two postal code prefixes of equal length.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Fixed shipping price, represented as a number.
    
    pub price: Option<Price>,
    /// The geographic region to which a shipping rate applies.
    
    pub region: Option<String>,
    /// A free-form description of the service class or delivery speed.
    
    pub service: Option<String>,
}

impl client::Part for ProductShipping {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductShippingDimension {
    /// The unit of value.
    
    pub unit: Option<String>,
    /// The dimension of the product used to calculate the shipping cost of the item.
    
    pub value: Option<f64>,
}

impl client::Part for ProductShippingDimension {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductShippingWeight {
    /// The unit of value.
    
    pub unit: Option<String>,
    /// The weight of the product used to calculate the shipping cost of the item.
    
    pub value: Option<f64>,
}

impl client::Part for ProductShippingWeight {}


/// The status of a product, i.e., information about a product computed asynchronously.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get productstatuses](ProductstatusGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductStatus {
    /// Date on which the item has been created, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// DEPRECATED - never populated
    #[serde(rename="dataQualityIssues")]
    
    pub data_quality_issues: Option<Vec<ProductStatusDataQualityIssue>>,
    /// The intended destinations for the product.
    #[serde(rename="destinationStatuses")]
    
    pub destination_statuses: Option<Vec<ProductStatusDestinationStatus>>,
    /// Date on which the item expires in Google Shopping, in ISO 8601 format.
    #[serde(rename="googleExpirationDate")]
    
    pub google_expiration_date: Option<String>,
    /// A list of all issues associated with the product.
    #[serde(rename="itemLevelIssues")]
    
    pub item_level_issues: Option<Vec<ProductStatusItemLevelIssue>>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#productStatus`"
    
    pub kind: Option<String>,
    /// Date on which the item has been last updated, in ISO 8601 format.
    #[serde(rename="lastUpdateDate")]
    
    pub last_update_date: Option<String>,
    /// The link to the product.
    
    pub link: Option<String>,
    /// Product data after applying all the join inputs.
    
    pub product: Option<Product>,
    /// The ID of the product for which status is reported.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The title of the product.
    
    pub title: Option<String>,
}

impl client::ResponseResult for ProductStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductStatusDataQualityIssue {
    /// no description provided
    
    pub destination: Option<String>,
    /// no description provided
    
    pub detail: Option<String>,
    /// no description provided
    #[serde(rename="fetchStatus")]
    
    pub fetch_status: Option<String>,
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub location: Option<String>,
    /// no description provided
    
    pub severity: Option<String>,
    /// no description provided
    
    pub timestamp: Option<String>,
    /// no description provided
    #[serde(rename="valueOnLandingPage")]
    
    pub value_on_landing_page: Option<String>,
    /// no description provided
    #[serde(rename="valueProvided")]
    
    pub value_provided: Option<String>,
}

impl client::Part for ProductStatusDataQualityIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductStatusDestinationStatus {
    /// Whether the approval status might change due to further processing.
    #[serde(rename="approvalPending")]
    
    pub approval_pending: Option<bool>,
    /// The destination's approval status. Acceptable values are: - "`approved`" - "`disapproved`" 
    #[serde(rename="approvalStatus")]
    
    pub approval_status: Option<String>,
    /// The name of the destination
    
    pub destination: Option<String>,
    /// Provided for backward compatibility only. Always set to "required". Acceptable values are: - "`default`" - "`excluded`" - "`optional`" - "`required`" 
    
    pub intention: Option<String>,
}

impl client::Part for ProductStatusDestinationStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductStatusItemLevelIssue {
    /// The attribute's name, if the issue is caused by a single attribute.
    #[serde(rename="attributeName")]
    
    pub attribute_name: Option<String>,
    /// The error code of the issue.
    
    pub code: Option<String>,
    /// A short issue description in English.
    
    pub description: Option<String>,
    /// The destination the issue applies to.
    
    pub destination: Option<String>,
    /// A detailed issue description in English.
    
    pub detail: Option<String>,
    /// The URL of a web page to help with resolving this issue.
    
    pub documentation: Option<String>,
    /// Whether the issue can be resolved by the merchant.
    
    pub resolution: Option<String>,
    /// How this issue affects serving of the offer.
    
    pub servability: Option<String>,
}

impl client::Part for ProductStatusItemLevelIssue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductTax {
    /// The country within which the item is taxed, specified as a CLDR territory code.
    
    pub country: Option<String>,
    /// The numeric ID of a location that the tax rate applies to as defined in the AdWords API.
    #[serde(rename="locationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub location_id: Option<i64>,
    /// The postal code range that the tax rate applies to, represented by a ZIP code, a ZIP code prefix using * wildcard, a range between two ZIP codes or two ZIP code prefixes of equal length. Examples: 94114, 94*, 94002-95460, 94*-95*.
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// The percentage of tax rate that applies to the item price.
    
    pub rate: Option<f64>,
    /// The geographic region to which the tax rate applies.
    
    pub region: Option<String>,
    /// Should be set to true if tax is charged on shipping.
    #[serde(rename="taxShip")]
    
    pub tax_ship: Option<bool>,
}

impl client::Part for ProductTax {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductUnitPricingBaseMeasure {
    /// The unit of the denominator.
    
    pub unit: Option<String>,
    /// The denominator of the unit price.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for ProductUnitPricingBaseMeasure {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductUnitPricingMeasure {
    /// The unit of the measure.
    
    pub unit: Option<String>,
    /// The measure of an item.
    
    pub value: Option<f64>,
}

impl client::Part for ProductUnitPricingMeasure {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch products](ProductCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<ProductsCustomBatchRequestEntry>>,
}

impl client::RequestValue for ProductsCustomBatchRequest {}


/// A batch entry encoding a single non-batch products request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`delete`" - "`get`" - "`insert`" 
    
    pub method: Option<String>,
    /// The product to insert. Only required if the method is `insert`.
    
    pub product: Option<Product>,
    /// The ID of the product to get or delete. Only defined if the method is `get` or `delete`.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for ProductsCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch products](ProductCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<ProductsCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#productsCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ProductsCustomBatchResponse {}


/// A batch entry encoding a single non-batch products response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if and only if the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#productsCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The inserted product. Only defined if the method is `insert` and if the request was successful.
    
    pub product: Option<Product>,
}

impl client::Part for ProductsCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list products](ProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#productsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of products.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<Product>>,
}

impl client::ResponseResult for ProductsListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch productstatuses](ProductstatusCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductstatusesCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<ProductstatusesCustomBatchRequestEntry>>,
}

impl client::RequestValue for ProductstatusesCustomBatchRequest {}


/// A batch entry encoding a single non-batch productstatuses request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductstatusesCustomBatchRequestEntry {
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination.
    
    pub destinations: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="includeAttributes")]
    
    pub include_attributes: Option<bool>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" 
    
    pub method: Option<String>,
    /// The ID of the product whose status to get.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
}

impl client::Part for ProductstatusesCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch productstatuses](ProductstatusCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductstatusesCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<ProductstatusesCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#productstatusesCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ProductstatusesCustomBatchResponse {}


/// A batch entry encoding a single non-batch productstatuses response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductstatusesCustomBatchResponseEntry {
    /// The ID of the request entry this entry responds to.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors, if the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#productstatusesCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The requested product status. Only defined if the request was successful.
    #[serde(rename="productStatus")]
    
    pub product_status: Option<ProductStatus>,
}

impl client::Part for ProductstatusesCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list productstatuses](ProductstatusListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductstatusesListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#productstatusesListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of products statuses.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<ProductStatus>>,
}

impl client::ResponseResult for ProductstatusesListResponse {}


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
pub struct RateGroup {
    /// A list of shipping labels defining the products to which this rate group applies to. This is a disjunction: only one of the labels has to match for the rate group to apply. May only be empty for the last rate group of a service. Required.
    #[serde(rename="applicableShippingLabels")]
    
    pub applicable_shipping_labels: Option<Vec<String>>,
    /// A list of carrier rates that can be referred to by `mainTable` or `singleValue`.
    #[serde(rename="carrierRates")]
    
    pub carrier_rates: Option<Vec<CarrierRate>>,
    /// A table defining the rate group, when `singleValue` is not expressive enough. Can only be set if `singleValue` is not set.
    #[serde(rename="mainTable")]
    
    pub main_table: Option<Table>,
    /// Name of the rate group. Optional. If set has to be unique within shipping service.
    
    pub name: Option<String>,
    /// The value of the rate group (e.g. flat rate $10). Can only be set if `mainTable` and `subtables` are not set.
    #[serde(rename="singleValue")]
    
    pub single_value: Option<Value>,
    /// A list of subtables referred to by `mainTable`. Can only be set if `mainTable` is set.
    
    pub subtables: Option<Vec<Table>>,
}

impl client::Part for RateGroup {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefundReason {
    /// Description of the reason.
    
    pub description: Option<String>,
    /// Code of the refund reason. Acceptable values are: - "`adjustment`" - "`autoPostInternal`" - "`autoPostInvalidBillingAddress`" - "`autoPostNoInventory`" - "`autoPostPriceError`" - "`autoPostUndeliverableShippingAddress`" - "`couponAbuse`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`customerInitiatedMerchantCancel`" - "`customerSupportRequested`" - "`deliveredLateByCarrier`" - "`deliveredTooLate`" - "`expiredItem`" - "`failToPushOrderGoogleError`" - "`failToPushOrderMerchantError`" - "`failToPushOrderMerchantFulfillmentError`" - "`failToPushOrderToMerchant`" - "`failToPushOrderToMerchantOutOfStock`" - "`feeAdjustment`" - "`invalidCoupon`" - "`lateShipmentCredit`" - "`malformedShippingAddress`" - "`merchantDidNotShipOnTime`" - "`noInventory`" - "`orderTimeout`" - "`other`" - "`paymentAbuse`" - "`paymentDeclined`" - "`priceAdjustment`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`promoReallocation`" - "`qualityNotAsExpected`" - "`returnRefundAbuse`" - "`shippingCostAdjustment`" - "`shippingPriceError`" - "`taxAdjustment`" - "`taxError`" - "`undeliverableShippingAddress`" - "`unsupportedPoBoxAddress`" - "`wrongProductShipped`" 
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
    /// The date of creation of the shipment, in ISO 8601 format.
    #[serde(rename="creationDate")]
    
    pub creation_date: Option<String>,
    /// The date of delivery of the shipment, in ISO 8601 format.
    #[serde(rename="deliveryDate")]
    
    pub delivery_date: Option<String>,
    /// Type of the return method. Acceptable values are: - "`byMail`" - "`contactCustomerSupport`" - "`returnless`" 
    #[serde(rename="returnMethodType")]
    
    pub return_method_type: Option<String>,
    /// Shipment ID generated by Google.
    #[serde(rename="shipmentId")]
    
    pub shipment_id: Option<String>,
    /// Tracking information of the shipment. One return shipment might be handled by several shipping carriers sequentially.
    #[serde(rename="shipmentTrackingInfos")]
    
    pub shipment_tracking_infos: Option<Vec<ShipmentTrackingInfo>>,
    /// The date of shipping of the shipment, in ISO 8601 format.
    #[serde(rename="shippingDate")]
    
    pub shipping_date: Option<String>,
    /// State of the shipment. Acceptable values are: - "`completed`" - "`new`" - "`shipped`" - "`undeliverable`" - "`pending`" 
    
    pub state: Option<String>,
}

impl client::Part for ReturnShipment {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// The list of cells that constitute the row. Must have the same length as `columnHeaders` for two-dimensional tables, a length of 1 for one-dimensional tables. Required.
    
    pub cells: Option<Vec<Value>>,
}

impl client::Part for Row {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// A boolean exposing the active status of the shipping service. Required.
    
    pub active: Option<bool>,
    /// The CLDR code of the currency to which this service applies. Must match that of the prices in rate groups.
    
    pub currency: Option<String>,
    /// The CLDR territory code of the country to which the service applies. Required.
    #[serde(rename="deliveryCountry")]
    
    pub delivery_country: Option<String>,
    /// Time spent in various aspects from order to the delivery of the product. Required.
    #[serde(rename="deliveryTime")]
    
    pub delivery_time: Option<DeliveryTime>,
    /// Eligibility for this service. Acceptable values are: - "`All scenarios`" - "`All scenarios except Shopping Actions`" - "`Shopping Actions`" 
    
    pub eligibility: Option<String>,
    /// Minimum order value for this service. If set, indicates that customers will have to spend at least this amount. All prices within a service must have the same currency. Cannot be set together with minimum_order_value_table.
    #[serde(rename="minimumOrderValue")]
    
    pub minimum_order_value: Option<Price>,
    /// Table of per store minimum order values for the pickup fulfillment type. Cannot be set together with minimum_order_value.
    #[serde(rename="minimumOrderValueTable")]
    
    pub minimum_order_value_table: Option<MinimumOrderValueTable>,
    /// Free-form name of the service. Must be unique within target account. Required.
    
    pub name: Option<String>,
    /// The carrier-service pair delivering items to collection points. The list of supported pickup services can be retrieved via the `getSupportedPickupServices` method. Required if and only if the service delivery type is `pickup`.
    #[serde(rename="pickupService")]
    
    pub pickup_service: Option<PickupCarrierService>,
    /// Shipping rate group definitions. Only the last one is allowed to have an empty `applicableShippingLabels`, which means "everything else". The other `applicableShippingLabels` must not overlap.
    #[serde(rename="rateGroups")]
    
    pub rate_groups: Option<Vec<RateGroup>>,
    /// Type of locations this service ships orders to. Acceptable values are: - "`delivery`" - "`pickup`" 
    #[serde(rename="shipmentType")]
    
    pub shipment_type: Option<String>,
}

impl client::Part for Service {}


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
    /// [required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges.
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
    /// [required] The shipment unit ID is assigned by the merchant and defines individual quantities within a line item. The same ID can be assigned to units that are the same while units that differ must be assigned a different ID (for example: free or promotional units).
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
    /// The shipping carrier that handles the package. Acceptable values are: - "`boxtal`" - "`bpost`" - "`chronopost`" - "`colisPrive`" - "`colissimo`" - "`cxt`" - "`deliv`" - "`dhl`" - "`dpd`" - "`dynamex`" - "`eCourier`" - "`easypost`" - "`efw`" - "`fedex`" - "`fedexSmartpost`" - "`geodis`" - "`gls`" - "`googleCourier`" - "`gsx`" - "`jdLogistics`" - "`laPoste`" - "`lasership`" - "`manual`" - "`mpx`" - "`onTrac`" - "`other`" - "`tnt`" - "`uds`" - "`ups`" - "`usps`" 
    
    pub carrier: Option<String>,
    /// The tracking number for the package.
    #[serde(rename="trackingNumber")]
    
    pub tracking_number: Option<String>,
}

impl client::Part for ShipmentTrackingInfo {}


/// The merchant account’s shipping settings. All methods except getsupportedcarriers and getsupportedholidays require the admin role.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get shippingsettings](ShippingsettingGetCall) (response)
/// * [update shippingsettings](ShippingsettingUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingSettings {
    /// The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// A list of postal code groups that can be referred to in `services`. Optional.
    #[serde(rename="postalCodeGroups")]
    
    pub postal_code_groups: Option<Vec<PostalCodeGroup>>,
    /// The target account's list of services. Optional.
    
    pub services: Option<Vec<Service>>,
    /// Optional. A list of warehouses which can be referred to in `services`.
    
    pub warehouses: Option<Vec<Warehouse>>,
}

impl client::RequestValue for ShippingSettings {}
impl client::ResponseResult for ShippingSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch shippingsettings](ShippingsettingCustombatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsCustomBatchRequest {
    /// The request entries to be processed in the batch.
    
    pub entries: Option<Vec<ShippingsettingsCustomBatchRequestEntry>>,
}

impl client::RequestValue for ShippingsettingsCustomBatchRequest {}


/// A batch entry encoding a single non-batch shippingsettings request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsCustomBatchRequestEntry {
    /// The ID of the account for which to get/update account shipping settings.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<u64>,
    /// An entry ID, unique within the batch request.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// The ID of the managing account.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// The method of the batch entry. Acceptable values are: - "`get`" - "`update`" 
    
    pub method: Option<String>,
    /// The account shipping settings to update. Only defined if the method is `update`.
    #[serde(rename="shippingSettings")]
    
    pub shipping_settings: Option<ShippingSettings>,
}

impl client::Part for ShippingsettingsCustomBatchRequestEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custombatch shippingsettings](ShippingsettingCustombatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsCustomBatchResponse {
    /// The result of the execution of the batch requests.
    
    pub entries: Option<Vec<ShippingsettingsCustomBatchResponseEntry>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#shippingsettingsCustomBatchResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ShippingsettingsCustomBatchResponse {}


/// A batch entry encoding a single non-batch shipping settings response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsCustomBatchResponseEntry {
    /// The ID of the request entry to which this entry responds.
    #[serde(rename="batchId")]
    
    pub batch_id: Option<u32>,
    /// A list of errors defined if, and only if, the request failed.
    
    pub errors: Option<Errors>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#shippingsettingsCustomBatchResponseEntry`"
    
    pub kind: Option<String>,
    /// The retrieved or updated account shipping settings.
    #[serde(rename="shippingSettings")]
    
    pub shipping_settings: Option<ShippingSettings>,
}

impl client::Part for ShippingsettingsCustomBatchResponseEntry {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getsupportedcarriers shippingsettings](ShippingsettingGetsupportedcarrierCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsGetSupportedCarriersResponse {
    /// A list of supported carriers. May be empty.
    
    pub carriers: Option<Vec<CarriersCarrier>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#shippingsettingsGetSupportedCarriersResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ShippingsettingsGetSupportedCarriersResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getsupportedholidays shippingsettings](ShippingsettingGetsupportedholidayCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsGetSupportedHolidaysResponse {
    /// A list of holidays applicable for delivery guarantees. May be empty.
    
    pub holidays: Option<Vec<HolidaysHoliday>>,
    /// Identifies what kind of resource this is. Value: the fixed string "content#shippingsettingsGetSupportedHolidaysResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ShippingsettingsGetSupportedHolidaysResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getsupportedpickupservices shippingsettings](ShippingsettingGetsupportedpickupserviceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsGetSupportedPickupServicesResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#shippingsettingsGetSupportedPickupServicesResponse".
    
    pub kind: Option<String>,
    /// A list of supported pickup services. May be empty.
    #[serde(rename="pickupServices")]
    
    pub pickup_services: Option<Vec<PickupServicesPickupService>>,
}

impl client::ResponseResult for ShippingsettingsGetSupportedPickupServicesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list shippingsettings](ShippingsettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShippingsettingsListResponse {
    /// Identifies what kind of resource this is. Value: the fixed string "content#shippingsettingsListResponse".
    
    pub kind: Option<String>,
    /// The token for the retrieval of the next page of shipping settings.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    
    pub resources: Option<Vec<ShippingSettings>>,
}

impl client::ResponseResult for ShippingsettingsListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Headers of the table's columns. Optional: if not set then the table has only one dimension.
    #[serde(rename="columnHeaders")]
    
    pub column_headers: Option<Headers>,
    /// Name of the table. Required for subtables, ignored for the main table.
    
    pub name: Option<String>,
    /// Headers of the table's rows. Required.
    #[serde(rename="rowHeaders")]
    
    pub row_headers: Option<Headers>,
    /// The list of rows that constitute the table. Must have the same length as `rowHeaders`. Required.
    
    pub rows: Option<Vec<Row>>,
}

impl client::Part for Table {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestOrder {
    /// Required. The details of the customer who placed the order.
    
    pub customer: Option<TestOrderCustomer>,
    /// Whether the orderinvoices service should support this order.
    #[serde(rename="enableOrderinvoices")]
    
    pub enable_orderinvoices: Option<bool>,
    /// Identifies what kind of resource this is. Value: the fixed string "`content#testOrder`"
    
    pub kind: Option<String>,
    /// Required. Line items that are ordered. At least one line item must be provided.
    #[serde(rename="lineItems")]
    
    pub line_items: Option<Vec<TestOrderLineItem>>,
    /// Restricted. Do not use.
    #[serde(rename="notificationMode")]
    
    pub notification_mode: Option<String>,
    /// The details of the payment method.
    #[serde(rename="paymentMethod")]
    
    pub payment_method: Option<TestOrderPaymentMethod>,
    /// Required. Identifier of one of the predefined delivery addresses for the delivery. Acceptable values are: - "`dwight`" - "`jim`" - "`pam`" 
    #[serde(rename="predefinedDeliveryAddress")]
    
    pub predefined_delivery_address: Option<String>,
    /// Identifier of one of the predefined pickup details. Required for orders containing line items with shipping type `pickup`. Acceptable values are: - "`dwight`" - "`jim`" - "`pam`" 
    #[serde(rename="predefinedPickupDetails")]
    
    pub predefined_pickup_details: Option<String>,
    /// Deprecated. Ignored if provided.
    
    pub promotions: Option<Vec<OrderLegacyPromotion>>,
    /// Required. The price of shipping for all items. Shipping tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied. Note that shipping is not taxed in certain states.
    #[serde(rename="shippingCost")]
    
    pub shipping_cost: Option<Price>,
    /// Deprecated. Ignored if provided.
    #[serde(rename="shippingCostTax")]
    
    pub shipping_cost_tax: Option<Price>,
    /// Required. The requested shipping option. Acceptable values are: - "`economy`" - "`expedited`" - "`oneDay`" - "`sameDay`" - "`standard`" - "`twoDay`" 
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
    /// Required. Email address of the customer. Acceptable values are: - "`pog.dwight.schrute@gmail.com`" - "`pog.jim.halpert@gmail.com`" - "`penpog.pam.beesly@gmail.comding`" 
    
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
    /// Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty. Acceptable values are: - "`denied`" - "`granted`" 
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
    /// Required. Product data from the time of the order placement.
    
    pub product: Option<TestOrderLineItemProduct>,
    /// Required. Number of items ordered.
    #[serde(rename="quantityOrdered")]
    
    pub quantity_ordered: Option<u32>,
    /// Required. Details of the return policy for the line item.
    #[serde(rename="returnInfo")]
    
    pub return_info: Option<OrderLineItemReturnInfo>,
    /// Required. Details of the requested shipping for the line item.
    #[serde(rename="shippingDetails")]
    
    pub shipping_details: Option<OrderLineItemShippingDetails>,
    /// Deprecated. Ignored if provided.
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
    /// Required. Brand of the item.
    
    pub brand: Option<String>,
    /// Deprecated. Acceptable values are: - "`online`" 
    
    pub channel: Option<String>,
    /// Required. Condition or state of the item. Acceptable values are: - "`new`" 
    
    pub condition: Option<String>,
    /// Required. The two-letter ISO 639-1 language code for the item. Acceptable values are: - "`en`" - "`fr`" 
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Fees for the item. Optional.
    
    pub fees: Option<Vec<OrderLineItemProductFee>>,
    /// Global Trade Item Number (GTIN) of the item. Optional.
    
    pub gtin: Option<String>,
    /// Required. URL of an image of the item.
    #[serde(rename="imageLink")]
    
    pub image_link: Option<String>,
    /// Shared identifier for all variants of the same product. Optional.
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// Manufacturer Part Number (MPN) of the item. Optional.
    
    pub mpn: Option<String>,
    /// Required. An identifier of the item.
    #[serde(rename="offerId")]
    
    pub offer_id: Option<String>,
    /// Required. The price for the product. Tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied.
    
    pub price: Option<Price>,
    /// Required. The CLDR territory // code of the target country of the product.
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
    /// Required. The title of the product.
    
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
    /// The billing address. Acceptable values are: - "`dwight`" - "`jim`" - "`pam`" 
    #[serde(rename="predefinedBillingAddress")]
    
    pub predefined_billing_address: Option<String>,
    /// The type of instrument. Note that real orders might have different values than the four values accepted by `createTestOrder`. Acceptable values are: - "`AMEX`" - "`DISCOVER`" - "`MASTERCARD`" - "`VISA`" 
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
pub struct TransitTable {
    /// A list of postal group names. The last value can be `"all other locations"`. Example: `["zone 1", "zone 2", "all other locations"]`. The referred postal code groups must match the delivery country of the service.
    #[serde(rename="postalCodeGroupNames")]
    
    pub postal_code_group_names: Option<Vec<String>>,
    /// no description provided
    
    pub rows: Option<Vec<TransitTableTransitTimeRow>>,
    /// A list of transit time labels. The last value can be `"all other labels"`. Example: `["food", "electronics", "all other labels"]`.
    #[serde(rename="transitTimeLabels")]
    
    pub transit_time_labels: Option<Vec<String>>,
}

impl client::Part for TransitTable {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitTableTransitTimeRow {
    /// no description provided
    
    pub values: Option<Vec<TransitTableTransitTimeRowTransitTimeValue>>,
}

impl client::Part for TransitTableTransitTimeRow {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitTableTransitTimeRowTransitTimeValue {
    /// Must be greater than or equal to `minTransitTimeInDays`.
    #[serde(rename="maxTransitTimeInDays")]
    
    pub max_transit_time_in_days: Option<u32>,
    /// Transit time range (min-max) in business days. 0 means same day delivery, 1 means next day delivery.
    #[serde(rename="minTransitTimeInDays")]
    
    pub min_transit_time_in_days: Option<u32>,
}

impl client::Part for TransitTableTransitTimeRowTransitTimeValue {}


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
    /// Deprecated.
    
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
    /// Deprecated.
    #[serde(rename="additionalChargePromotions")]
    
    pub additional_charge_promotions: Option<Vec<Promotion>>,
    /// [required] Type of the additional charge. Acceptable values are: - "`shipping`" 
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
    /// Optional name of the tax type. This should only be provided if `taxType` is `otherFeeTax`.
    #[serde(rename="taxName")]
    
    pub tax_name: Option<String>,
    /// [required] Type of the tax. Acceptable values are: - "`otherFee`" - "`otherFeeTax`" - "`sales`" 
    #[serde(rename="taxType")]
    
    pub tax_type: Option<String>,
}

impl client::Part for UnitInvoiceTaxLine {}


/// The single value of a rate group or the value of a rate group table's cell. Exactly one of `noShipping`, `flatRate`, `pricePercentage`, `carrierRateName`, `subtableName` must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// The name of a carrier rate referring to a carrier rate defined in the same rate group. Can only be set if all other fields are not set.
    #[serde(rename="carrierRateName")]
    
    pub carrier_rate_name: Option<String>,
    /// A flat rate. Can only be set if all other fields are not set.
    #[serde(rename="flatRate")]
    
    pub flat_rate: Option<Price>,
    /// If true, then the product can't ship. Must be true when set, can only be set if all other fields are not set.
    #[serde(rename="noShipping")]
    
    pub no_shipping: Option<bool>,
    /// A percentage of the price represented as a number in decimal notation (e.g., `"5.4"`). Can only be set if all other fields are not set.
    #[serde(rename="pricePercentage")]
    
    pub price_percentage: Option<String>,
    /// The name of a subtable. Can only be set in table cells (i.e., not for single values), and only if all other fields are not set.
    #[serde(rename="subtableName")]
    
    pub subtable_name: Option<String>,
}

impl client::Part for Value {}


/// A fulfillment warehouse, which stores and handles inventory.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Warehouse {
    /// Business days of the warehouse. If not set, will be Monday to Friday by default.
    #[serde(rename="businessDayConfig")]
    
    pub business_day_config: Option<BusinessDayConfig>,
    /// Required. The latest time of day that an order can be accepted and begin processing. Later orders will be processed in the next day. The time is based on the warehouse postal code.
    #[serde(rename="cutoffTime")]
    
    pub cutoff_time: Option<WarehouseCutoffTime>,
    /// Required. The number of days it takes for this warehouse to pack up and ship an item. This is on the warehouse level, but can be overridden on the offer level based on the attributes of an item.
    #[serde(rename="handlingDays")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub handling_days: Option<i64>,
    /// Required. The name of the warehouse. Must be unique within account.
    
    pub name: Option<String>,
    /// Required. Shipping address of the warehouse.
    #[serde(rename="shippingAddress")]
    
    pub shipping_address: Option<Address>,
}

impl client::Part for Warehouse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WarehouseBasedDeliveryTime {
    /// Required. Carrier, such as `"UPS"` or `"Fedex"`. The list of supported carriers can be retrieved via the `listSupportedCarriers` method.
    
    pub carrier: Option<String>,
    /// Required. Carrier service, such as `"ground"` or `"2 days"`. The list of supported services for a carrier can be retrieved via the `listSupportedCarriers` method. The name of the service must be in the eddSupportedServices list.
    #[serde(rename="carrierService")]
    
    pub carrier_service: Option<String>,
    /// Shipping origin's state.
    #[serde(rename="originAdministrativeArea")]
    
    pub origin_administrative_area: Option<String>,
    /// Shipping origin's city.
    #[serde(rename="originCity")]
    
    pub origin_city: Option<String>,
    /// Shipping origin's country represented as a [CLDR territory code](http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml).
    #[serde(rename="originCountry")]
    
    pub origin_country: Option<String>,
    /// Shipping origin.
    #[serde(rename="originPostalCode")]
    
    pub origin_postal_code: Option<String>,
    /// Shipping origin's street address
    #[serde(rename="originStreetAddress")]
    
    pub origin_street_address: Option<String>,
    /// The name of the warehouse. Warehouse name need to be matched with name. If warehouseName is set, the below fields will be ignored. The warehouse info will be read from warehouse.
    #[serde(rename="warehouseName")]
    
    pub warehouse_name: Option<String>,
}

impl client::Part for WarehouseBasedDeliveryTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WarehouseCutoffTime {
    /// Required. Hour (24-hour clock) of the cutoff time until which an order has to be placed to be processed in the same day by the warehouse. Hour is based on the timezone of warehouse.
    
    pub hour: Option<i32>,
    /// Required. Minute of the cutoff time until which an order has to be placed to be processed in the same day by the warehouse. Minute is based on the timezone of warehouse.
    
    pub minute: Option<i32>,
}

impl client::Part for WarehouseCutoffTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Weight {
    /// Required. The weight unit. Acceptable values are: - "`kg`" - "`lb`" 
    
    pub unit: Option<String>,
    /// Required. The weight represented as a number. The weight can have a maximum precision of four decimal places.
    
    pub value: Option<String>,
}

impl client::Part for Weight {}


