use super::*;
/// Request message for CloudChannelService.ActivateEntitlement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements activate accounts](AccountCustomerEntitlementActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ActivateEntitlementRequest {
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ActivateEntitlementRequest {}


/// Information needed to create an Admin User for Google Workspace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1AdminUser {
    /// Primary email of the admin user.
    
    pub email: Option<String>,
    /// Family name of the admin user.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// Given name of the admin user.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
}

impl client::Part for GoogleCloudChannelV1AdminUser {}


/// Association links that an entitlement has to other entitlements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1AssociationInfo {
    /// The name of the base entitlement, for which this entitlement is an add-on.
    #[serde(rename="baseEntitlement")]
    
    pub base_entitlement: Option<String>,
}

impl client::Part for GoogleCloudChannelV1AssociationInfo {}


/// Request message for CloudChannelService.CancelEntitlement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements cancel accounts](AccountCustomerEntitlementCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CancelEntitlementRequest {
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1CancelEntitlementRequest {}


/// Request message for CloudChannelService.ChangeOffer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements change offer accounts](AccountCustomerEntitlementChangeOfferCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChangeOfferRequest {
    /// Required. New Offer. Format: accounts/{account_id}/offers/{offer_id}.
    
    pub offer: Option<String>,
    /// Optional. Parameters needed to purchase the Offer. To view the available Parameters refer to the Offer.parameter_definitions from the desired offer.
    
    pub parameters: Option<Vec<GoogleCloudChannelV1Parameter>>,
    /// Optional. Purchase order id provided by the reseller.
    #[serde(rename="purchaseOrderId")]
    
    pub purchase_order_id: Option<String>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ChangeOfferRequest {}


/// Request message for CloudChannelService.ChangeParametersRequest.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements change parameters accounts](AccountCustomerEntitlementChangeParameterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChangeParametersRequest {
    /// Required. Entitlement parameters to update. You can only change editable parameters. To view the available Parameters for a request, refer to the Offer.parameter_definitions from the desired offer.
    
    pub parameters: Option<Vec<GoogleCloudChannelV1Parameter>>,
    /// Optional. Purchase order ID provided by the reseller.
    #[serde(rename="purchaseOrderId")]
    
    pub purchase_order_id: Option<String>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ChangeParametersRequest {}


/// Request message for CloudChannelService.ChangeRenewalSettings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements change renewal settings accounts](AccountCustomerEntitlementChangeRenewalSettingCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChangeRenewalSettingsRequest {
    /// Required. New renewal settings.
    #[serde(rename="renewalSettings")]
    
    pub renewal_settings: Option<GoogleCloudChannelV1RenewalSettings>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ChangeRenewalSettingsRequest {}


/// Entity representing a link between distributors and their indirect resellers in an n-tier resale channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links create accounts](AccountChannelPartnerLinkCreateCall) (request|response)
/// * [channel partner links get accounts](AccountChannelPartnerLinkGetCall) (response)
/// * [channel partner links patch accounts](AccountChannelPartnerLinkPatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChannelPartnerLink {
    /// Output only. Cloud Identity info of the channel partner (IR).
    #[serde(rename="channelPartnerCloudIdentityInfo")]
    
    pub channel_partner_cloud_identity_info: Option<GoogleCloudChannelV1CloudIdentityInfo>,
    /// Output only. Timestamp of when the channel partner link is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. URI of the web page where partner accepts the link invitation.
    #[serde(rename="inviteLinkUri")]
    
    pub invite_link_uri: Option<String>,
    /// Required. State of the channel partner link.
    #[serde(rename="linkState")]
    
    pub link_state: Option<GoogleCloudChannelV1ChannelPartnerLinkLinkStateEnum>,
    /// Output only. Resource name for the channel partner link, in the format accounts/{account_id}/channelPartnerLinks/{id}.
    
    pub name: Option<String>,
    /// Output only. Public identifier that a customer must use to generate a transfer token to move to this distributor-reseller combination.
    #[serde(rename="publicId")]
    
    pub public_id: Option<String>,
    /// Required. Cloud Identity ID of the linked reseller.
    #[serde(rename="resellerCloudIdentityId")]
    
    pub reseller_cloud_identity_id: Option<String>,
    /// Output only. Timestamp of when the channel partner link is updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudChannelV1ChannelPartnerLink {}
impl client::ResponseResult for GoogleCloudChannelV1ChannelPartnerLink {}


/// Configuration for how a distributor will rebill a channel partner (also known as a distributor-authorized reseller).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links channel partner repricing configs create accounts](AccountChannelPartnerLinkChannelPartnerRepricingConfigCreateCall) (request|response)
/// * [channel partner links channel partner repricing configs get accounts](AccountChannelPartnerLinkChannelPartnerRepricingConfigGetCall) (response)
/// * [channel partner links channel partner repricing configs patch accounts](AccountChannelPartnerLinkChannelPartnerRepricingConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChannelPartnerRepricingConfig {
    /// Output only. Resource name of the ChannelPartnerRepricingConfig. Format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}/channelPartnerRepricingConfigs/{id}.
    
    pub name: Option<String>,
    /// Required. The configuration for bill modifications made by a reseller before sending it to ChannelPartner.
    #[serde(rename="repricingConfig")]
    
    pub repricing_config: Option<GoogleCloudChannelV1RepricingConfig>,
    /// Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudChannelV1ChannelPartnerRepricingConfig {}
impl client::ResponseResult for GoogleCloudChannelV1ChannelPartnerRepricingConfig {}


/// Request message for CloudChannelService.CheckCloudIdentityAccountsExist.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check cloud identity accounts exist accounts](AccountCheckCloudIdentityAccountsExistCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest {
    /// Required. Domain to fetch for Cloud Identity account customer.
    
    pub domain: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest {}


/// Response message for CloudChannelService.CheckCloudIdentityAccountsExist.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check cloud identity accounts exist accounts](AccountCheckCloudIdentityAccountsExistCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponse {
    /// The Cloud Identity accounts associated with the domain.
    #[serde(rename="cloudIdentityAccounts")]
    
    pub cloud_identity_accounts: Option<Vec<GoogleCloudChannelV1CloudIdentityCustomerAccount>>,
}

impl client::ResponseResult for GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponse {}


/// Entity representing a Cloud Identity account that may be associated with a Channel Services API partner.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CloudIdentityCustomerAccount {
    /// If existing = true, the Cloud Identity ID of the customer.
    #[serde(rename="customerCloudIdentityId")]
    
    pub customer_cloud_identity_id: Option<String>,
    /// If owned = true, the name of the customer that owns the Cloud Identity account. Customer_name uses the format: accounts/{account_id}/customers/{customer_id}
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// Returns true if a Cloud Identity account exists for a specific domain.
    
    pub existing: Option<bool>,
    /// Returns true if the Cloud Identity account is associated with a customer of the Channel Services partner.
    
    pub owned: Option<bool>,
}

impl client::Part for GoogleCloudChannelV1CloudIdentityCustomerAccount {}


/// Cloud Identity information for the Cloud Channel Customer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CloudIdentityInfo {
    /// Output only. URI of Customer's Admin console dashboard.
    #[serde(rename="adminConsoleUri")]
    
    pub admin_console_uri: Option<String>,
    /// The alternate email.
    #[serde(rename="alternateEmail")]
    
    pub alternate_email: Option<String>,
    /// CustomerType indicates verification type needed for using services.
    #[serde(rename="customerType")]
    
    pub customer_type: Option<GoogleCloudChannelV1CloudIdentityInfoCustomerTypeEnum>,
    /// Edu information about the customer.
    #[serde(rename="eduData")]
    
    pub edu_data: Option<GoogleCloudChannelV1EduData>,
    /// Output only. Whether the domain is verified. This field is not returned for a Customer's cloud_identity_info resource. Partners can use the domains.get() method of the Workspace SDK's Directory API, or listen to the PRIMARY_DOMAIN_VERIFIED Pub/Sub event in to track domain verification of their resolve Workspace customers.
    #[serde(rename="isDomainVerified")]
    
    pub is_domain_verified: Option<bool>,
    /// Language code.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Phone number associated with the Cloud Identity.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Output only. The primary domain name.
    #[serde(rename="primaryDomain")]
    
    pub primary_domain: Option<String>,
}

impl client::Part for GoogleCloudChannelV1CloudIdentityInfo {}


/// The definition of a report column. Specifies the data properties in the corresponding position of the report rows.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Column {
    /// The unique name of the column (for example, customer_domain, channel_partner, customer_cost). You can use column IDs in RunReportJobRequest.filter. To see all reports and their columns, call CloudChannelReportsService.ListReports.
    #[serde(rename="columnId")]
    
    pub column_id: Option<String>,
    /// The type of the values for this column.
    #[serde(rename="dataType")]
    
    pub data_type: Option<GoogleCloudChannelV1ColumnDataTypeEnum>,
    /// The column's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Column {}


/// Commitment settings for commitment-based offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CommitmentSettings {
    /// Output only. Commitment end timestamp.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Renewal settings applicable for a commitment-based Offer.
    #[serde(rename="renewalSettings")]
    
    pub renewal_settings: Option<GoogleCloudChannelV1RenewalSettings>,
    /// Output only. Commitment start timestamp.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudChannelV1CommitmentSettings {}


/// Specifies the override to conditionally apply.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ConditionalOverride {
    /// Required. Information about the applied override's adjustment.
    
    pub adjustment: Option<GoogleCloudChannelV1RepricingAdjustment>,
    /// Required. The RebillingBasis to use for the applied override. Shows the relative cost based on your repricing costs.
    #[serde(rename="rebillingBasis")]
    
    pub rebilling_basis: Option<GoogleCloudChannelV1ConditionalOverrideRebillingBasisEnum>,
    /// Required. Specifies the condition which, if met, will apply the override.
    #[serde(rename="repricingCondition")]
    
    pub repricing_condition: Option<GoogleCloudChannelV1RepricingCondition>,
}

impl client::Part for GoogleCloudChannelV1ConditionalOverride {}


/// Represents the constraints for buying the Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Constraints {
    /// Represents constraints required to purchase the Offer for a customer.
    #[serde(rename="customerConstraints")]
    
    pub customer_constraints: Option<GoogleCloudChannelV1CustomerConstraints>,
}

impl client::Part for GoogleCloudChannelV1Constraints {}


/// Contact information for a customer account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ContactInfo {
    /// Output only. The customer account contact's display name, formatted as a combination of the customer's first and last name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The customer account's contact email. Required for entitlements that create admin.google.com accounts, and serves as the customer's username for those accounts. Use this email to invite Team customers.
    
    pub email: Option<String>,
    /// The customer account contact's first name. Optional for Team customers.
    #[serde(rename="firstName")]
    
    pub first_name: Option<String>,
    /// The customer account contact's last name. Optional for Team customers.
    #[serde(rename="lastName")]
    
    pub last_name: Option<String>,
    /// The customer account's contact phone number.
    
    pub phone: Option<String>,
    /// Optional. The customer account contact's job title.
    
    pub title: Option<String>,
}

impl client::Part for GoogleCloudChannelV1ContactInfo {}


/// Request message for CloudChannelService.CreateEntitlement
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements create accounts](AccountCustomerEntitlementCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CreateEntitlementRequest {
    /// Required. The entitlement to create.
    
    pub entitlement: Option<GoogleCloudChannelV1Entitlement>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1CreateEntitlementRequest {}


/// Entity representing a customer of a reseller or distributor.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links customers create accounts](AccountChannelPartnerLinkCustomerCreateCall) (request|response)
/// * [channel partner links customers get accounts](AccountChannelPartnerLinkCustomerGetCall) (response)
/// * [channel partner links customers import accounts](AccountChannelPartnerLinkCustomerImportCall) (response)
/// * [channel partner links customers patch accounts](AccountChannelPartnerLinkCustomerPatchCall) (request|response)
/// * [customers create accounts](AccountCustomerCreateCall) (request|response)
/// * [customers get accounts](AccountCustomerGetCall) (response)
/// * [customers import accounts](AccountCustomerImportCall) (response)
/// * [customers patch accounts](AccountCustomerPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Customer {
    /// Secondary contact email. You need to provide an alternate email to create different domains if a primary contact email already exists. Users will receive a notification with credentials when you create an admin.google.com account. Secondary emails are also recovery email addresses. Alternate emails are optional when you create Team customers.
    #[serde(rename="alternateEmail")]
    
    pub alternate_email: Option<String>,
    /// Cloud Identity ID of the customer's channel partner. Populated only if a channel partner exists for this customer.
    #[serde(rename="channelPartnerId")]
    
    pub channel_partner_id: Option<String>,
    /// Output only. The customer's Cloud Identity ID if the customer has a Cloud Identity resource.
    #[serde(rename="cloudIdentityId")]
    
    pub cloud_identity_id: Option<String>,
    /// Output only. Cloud Identity information for the customer. Populated only if a Cloud Identity account exists for this customer.
    #[serde(rename="cloudIdentityInfo")]
    
    pub cloud_identity_info: Option<GoogleCloudChannelV1CloudIdentityInfo>,
    /// Output only. Time when the customer was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. The customer's primary domain. Must match the primary contact email's domain.
    
    pub domain: Option<String>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    
    pub name: Option<String>,
    /// Required. Name of the organization that the customer entity represents.
    #[serde(rename="orgDisplayName")]
    
    pub org_display_name: Option<String>,
    /// Required. The organization address for the customer. To enforce US laws and embargoes, we require a region and zip code. You must provide valid addresses for every customer. To set the customer's language, use the Customer-level language code.
    #[serde(rename="orgPostalAddress")]
    
    pub org_postal_address: Option<GoogleTypePostalAddress>,
    /// Primary contact info.
    #[serde(rename="primaryContactInfo")]
    
    pub primary_contact_info: Option<GoogleCloudChannelV1ContactInfo>,
    /// Output only. Time when the customer was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudChannelV1Customer {}
impl client::ResponseResult for GoogleCloudChannelV1Customer {}


/// Represents constraints required to purchase the Offer for a customer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CustomerConstraints {
    /// Allowed Customer Type.
    #[serde(rename="allowedCustomerTypes")]
    
    pub allowed_customer_types: Option<Vec<GoogleCloudChannelV1CustomerConstraintAllowedCustomerTypesEnum>>,
    /// Allowed geographical regions of the customer.
    #[serde(rename="allowedRegions")]
    
    pub allowed_regions: Option<Vec<String>>,
    /// Allowed Promotional Order Type. Present for Promotional offers.
    #[serde(rename="promotionalOrderTypes")]
    
    pub promotional_order_types: Option<Vec<GoogleCloudChannelV1CustomerConstraintPromotionalOrderTypesEnum>>,
}

impl client::Part for GoogleCloudChannelV1CustomerConstraints {}


/// Configuration for how a reseller will reprice a Customer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers customer repricing configs create accounts](AccountCustomerCustomerRepricingConfigCreateCall) (request|response)
/// * [customers customer repricing configs get accounts](AccountCustomerCustomerRepricingConfigGetCall) (response)
/// * [customers customer repricing configs patch accounts](AccountCustomerCustomerRepricingConfigPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CustomerRepricingConfig {
    /// Output only. Resource name of the CustomerRepricingConfig. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}.
    
    pub name: Option<String>,
    /// Required. The configuration for bill modifications made by a reseller before sending it to customers.
    #[serde(rename="repricingConfig")]
    
    pub repricing_config: Option<GoogleCloudChannelV1RepricingConfig>,
    /// Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudChannelV1CustomerRepricingConfig {}
impl client::ResponseResult for GoogleCloudChannelV1CustomerRepricingConfig {}


/// A representation of usage or invoice date ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1DateRange {
    /// The latest invoice date (exclusive). If your product uses monthly invoices, and this value is not the beginning of a month, this will adjust the date to the first day of the following month.
    #[serde(rename="invoiceEndDate")]
    
    pub invoice_end_date: Option<GoogleTypeDate>,
    /// The earliest invoice date (inclusive). If your product uses monthly invoices, and this value is not the beginning of a month, this will adjust the date to the first day of the given month.
    #[serde(rename="invoiceStartDate")]
    
    pub invoice_start_date: Option<GoogleTypeDate>,
    /// The latest usage date time (exclusive). If you use time groupings (daily, weekly, etc), each group uses midnight to midnight (Pacific time). The usage end date is rounded down to include all usage from the specified date. We recommend that clients pass `usage_start_date_time` in Pacific time.
    #[serde(rename="usageEndDateTime")]
    
    pub usage_end_date_time: Option<GoogleTypeDateTime>,
    /// The earliest usage date time (inclusive). If you use time groupings (daily, weekly, etc), each group uses midnight to midnight (Pacific time). The usage start date is rounded down to include all usage from the specified date. We recommend that clients pass `usage_start_date_time` in Pacific time.
    #[serde(rename="usageStartDateTime")]
    
    pub usage_start_date_time: Option<GoogleTypeDateTime>,
}

impl client::Part for GoogleCloudChannelV1DateRange {}


/// Required Edu Attributes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1EduData {
    /// Size of the institute.
    #[serde(rename="instituteSize")]
    
    pub institute_size: Option<GoogleCloudChannelV1EduDataInstituteSizeEnum>,
    /// Designated institute type of customer.
    #[serde(rename="instituteType")]
    
    pub institute_type: Option<GoogleCloudChannelV1EduDataInstituteTypeEnum>,
    /// Web address for the edu customer's institution.
    
    pub website: Option<String>,
}

impl client::Part for GoogleCloudChannelV1EduData {}


/// An entitlement is a representation of a customer’s ability to use a service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements get accounts](AccountCustomerEntitlementGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Entitlement {
    /// Association information to other entitlements.
    #[serde(rename="associationInfo")]
    
    pub association_info: Option<GoogleCloudChannelV1AssociationInfo>,
    /// Commitment settings for a commitment-based Offer. Required for commitment based offers.
    #[serde(rename="commitmentSettings")]
    
    pub commitment_settings: Option<GoogleCloudChannelV1CommitmentSettings>,
    /// Output only. The time at which the entitlement is created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Resource name of an entitlement in the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}.
    
    pub name: Option<String>,
    /// Required. The offer resource name for which the entitlement is to be created. Takes the form: accounts/{account_id}/offers/{offer_id}.
    
    pub offer: Option<String>,
    /// Extended entitlement parameters. When creating an entitlement, valid parameter names and values are defined in the Offer.parameter_definitions. For Google Workspace, the following Parameters may be accepted as input: - max_units: The maximum assignable units for a flexible offer OR - num_units: The total commitment for commitment-based offers The response may additionally include the following output-only Parameters: - assigned_units: The number of licenses assigned to users. For GCP billing accounts, the following Parameter may be accepted as input: - display_name: The display name of the billing account.
    
    pub parameters: Option<Vec<GoogleCloudChannelV1Parameter>>,
    /// Output only. Service provisioning details for the entitlement.
    #[serde(rename="provisionedService")]
    
    pub provisioned_service: Option<GoogleCloudChannelV1ProvisionedService>,
    /// Output only. Current provisioning state of the entitlement.
    #[serde(rename="provisioningState")]
    
    pub provisioning_state: Option<GoogleCloudChannelV1EntitlementProvisioningStateEnum>,
    /// Optional. This purchase order (PO) information is for resellers to use for their company tracking usage. If a purchaseOrderId value is given, it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters. This is only supported for Google Workspace entitlements.
    #[serde(rename="purchaseOrderId")]
    
    pub purchase_order_id: Option<String>,
    /// Output only. Enumerable of all current suspension reasons for an entitlement.
    #[serde(rename="suspensionReasons")]
    
    pub suspension_reasons: Option<Vec<GoogleCloudChannelV1EntitlementSuspensionReasonsEnum>>,
    /// Output only. Settings for trial offers.
    #[serde(rename="trialSettings")]
    
    pub trial_settings: Option<GoogleCloudChannelV1TrialSettings>,
    /// Output only. The time at which the entitlement is updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudChannelV1Entitlement {}


/// Request message for CloudChannelReportsService.FetchReportResults.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report jobs fetch report results accounts](AccountReportJobFetchReportResultCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1FetchReportResultsRequest {
    /// Optional. Requested page size of the report. The server may return fewer results than requested. If you don't specify a page size, the server uses a sensible default (may change over time). The maximum value is 30,000; the server will change larger values to 30,000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. A token that specifies a page of results beyond the first page. Obtained through FetchReportResultsResponse.next_page_token of the previous CloudChannelReportsService.FetchReportResults call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1FetchReportResultsRequest {}


/// Response message for CloudChannelReportsService.FetchReportResults. Contains a tabular representation of the report results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [report jobs fetch report results accounts](AccountReportJobFetchReportResultCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1FetchReportResultsResponse {
    /// Pass this token to FetchReportResultsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The metadata for the report results (display name, columns, row count, and date ranges).
    #[serde(rename="reportMetadata")]
    
    pub report_metadata: Option<GoogleCloudChannelV1ReportResultsMetadata>,
    /// The report's lists of values. Each row follows the settings and ordering of the columns from `report_metadata`.
    
    pub rows: Option<Vec<GoogleCloudChannelV1Row>>,
}

impl client::ResponseResult for GoogleCloudChannelV1FetchReportResultsResponse {}


/// Request message for CloudChannelService.ImportCustomer
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links customers import accounts](AccountChannelPartnerLinkCustomerImportCall) (request)
/// * [customers import accounts](AccountCustomerImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ImportCustomerRequest {
    /// Optional. The super admin of the resold customer generates this token to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. You can omit this token after authorization. See https://support.google.com/a/answer/7643790 for more details.
    #[serde(rename="authToken")]
    
    pub auth_token: Option<String>,
    /// Optional. Cloud Identity ID of a channel partner who will be the direct reseller for the customer's order. This field is required for 2-tier transfer scenarios and can be provided via the request Parent binding as well.
    #[serde(rename="channelPartnerId")]
    
    pub channel_partner_id: Option<String>,
    /// Required. Customer's Cloud Identity ID
    #[serde(rename="cloudIdentityId")]
    
    pub cloud_identity_id: Option<String>,
    /// Optional. Specifies the customer that will receive imported Cloud Identity information. Format: accounts/{account_id}/customers/{customer_id}
    
    pub customer: Option<String>,
    /// Required. Customer domain.
    
    pub domain: Option<String>,
    /// Required. Choose to overwrite an existing customer if found. This must be set to true if there is an existing customer with a conflicting region code or domain.
    #[serde(rename="overwriteIfExists")]
    
    pub overwrite_if_exists: Option<bool>,
}

impl client::RequestValue for GoogleCloudChannelV1ImportCustomerRequest {}


/// Response message for CloudChannelService.ListChannelPartnerLinks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links list accounts](AccountChannelPartnerLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListChannelPartnerLinksResponse {
    /// The Channel partner links for a reseller.
    #[serde(rename="channelPartnerLinks")]
    
    pub channel_partner_links: Option<Vec<GoogleCloudChannelV1ChannelPartnerLink>>,
    /// A token to retrieve the next page of results. Pass to ListChannelPartnerLinksRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListChannelPartnerLinksResponse {}


/// Response message for CloudChannelService.ListChannelPartnerRepricingConfigs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links channel partner repricing configs list accounts](AccountChannelPartnerLinkChannelPartnerRepricingConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListChannelPartnerRepricingConfigsResponse {
    /// The repricing configs for this channel partner.
    #[serde(rename="channelPartnerRepricingConfigs")]
    
    pub channel_partner_repricing_configs: Option<Vec<GoogleCloudChannelV1ChannelPartnerRepricingConfig>>,
    /// A token to retrieve the next page of results. Pass to ListChannelPartnerRepricingConfigsRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListChannelPartnerRepricingConfigsResponse {}


/// Response message for CloudChannelService.ListCustomerRepricingConfigs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers customer repricing configs list accounts](AccountCustomerCustomerRepricingConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListCustomerRepricingConfigsResponse {
    /// The repricing configs for this channel partner.
    #[serde(rename="customerRepricingConfigs")]
    
    pub customer_repricing_configs: Option<Vec<GoogleCloudChannelV1CustomerRepricingConfig>>,
    /// A token to retrieve the next page of results. Pass to ListCustomerRepricingConfigsRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListCustomerRepricingConfigsResponse {}


/// Response message for CloudChannelService.ListCustomers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links customers list accounts](AccountChannelPartnerLinkCustomerListCall) (response)
/// * [customers list accounts](AccountCustomerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListCustomersResponse {
    /// The customers belonging to a reseller or distributor.
    
    pub customers: Option<Vec<GoogleCloudChannelV1Customer>>,
    /// A token to retrieve the next page of results. Pass to ListCustomersRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListCustomersResponse {}


/// Response message for CloudChannelService.ListEntitlements.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements list accounts](AccountCustomerEntitlementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListEntitlementsResponse {
    /// The reseller customer's entitlements.
    
    pub entitlements: Option<Vec<GoogleCloudChannelV1Entitlement>>,
    /// A token to list the next page of results. Pass to ListEntitlementsRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListEntitlementsResponse {}


/// Response message for ListOffers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [offers list accounts](AccountOfferListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListOffersResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Offers requested.
    
    pub offers: Option<Vec<GoogleCloudChannelV1Offer>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListOffersResponse {}


/// Response message for ListProducts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list products](ProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListProductsResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of Products requested.
    
    pub products: Option<Vec<GoogleCloudChannelV1Product>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListProductsResponse {}


/// Response message for ListPurchasableOffers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers list purchasable offers accounts](AccountCustomerListPurchasableOfferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListPurchasableOffersResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Offers requested.
    #[serde(rename="purchasableOffers")]
    
    pub purchasable_offers: Option<Vec<GoogleCloudChannelV1PurchasableOffer>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListPurchasableOffersResponse {}


/// Response message for ListPurchasableSkus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers list purchasable skus accounts](AccountCustomerListPurchasableSkuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListPurchasableSkusResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of SKUs requested.
    #[serde(rename="purchasableSkus")]
    
    pub purchasable_skus: Option<Vec<GoogleCloudChannelV1PurchasableSku>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListPurchasableSkusResponse {}


/// Response message for CloudChannelReportsService.ListReports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports list accounts](AccountReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListReportsResponse {
    /// Pass this token to FetchReportResultsRequest.page_token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The reports available to the partner.
    
    pub reports: Option<Vec<GoogleCloudChannelV1Report>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListReportsResponse {}


/// Response message for ListSkus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [skus list products](ProductSkuListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListSkusResponse {
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of SKUs requested.
    
    pub skus: Option<Vec<GoogleCloudChannelV1Sku>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListSkusResponse {}


/// Response Message for ListSubscribers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subscribers accounts](AccountListSubscriberCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListSubscribersResponse {
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of service accounts which have subscriber access to the topic.
    #[serde(rename="serviceAccounts")]
    
    pub service_accounts: Option<Vec<String>>,
    /// Name of the topic registered with the reseller.
    
    pub topic: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListSubscribersResponse {}


/// Request message for CloudChannelService.ListTransferableOffers
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transferable offers accounts](AccountListTransferableOfferCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableOffersRequest {
    /// Customer's Cloud Identity ID
    #[serde(rename="cloudIdentityId")]
    
    pub cloud_identity_id: Option<String>,
    /// A reseller should create a customer and use the resource name of that customer here.
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 offers. The maximum value is 1000; the server will coerce values above 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A token for a page of results other than the first page. Obtained using ListTransferableOffersResponse.next_page_token of the previous CloudChannelService.ListTransferableOffers call.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. The SKU to look up Offers for.
    
    pub sku: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ListTransferableOffersRequest {}


/// Response message for CloudChannelService.ListTransferableOffers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transferable offers accounts](AccountListTransferableOfferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableOffersResponse {
    /// A token to retrieve the next page of results. Pass to ListTransferableOffersRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Information about Offers for a customer that can be used for transfer.
    #[serde(rename="transferableOffers")]
    
    pub transferable_offers: Option<Vec<GoogleCloudChannelV1TransferableOffer>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListTransferableOffersResponse {}


/// Request message for CloudChannelService.ListTransferableSkus
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transferable skus accounts](AccountListTransferableSkuCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableSkusRequest {
    /// Optional. The super admin of the resold customer generates this token to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. You can omit this token after authorization. See https://support.google.com/a/answer/7643790 for more details.
    #[serde(rename="authToken")]
    
    pub auth_token: Option<String>,
    /// Customer's Cloud Identity ID
    #[serde(rename="cloudIdentityId")]
    
    pub cloud_identity_id: Option<String>,
    /// A reseller is required to create a customer and use the resource name of the created customer here. Customer_name uses the format: accounts/{account_id}/customers/{customer_id}
    #[serde(rename="customerName")]
    
    pub customer_name: Option<String>,
    /// The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US". Optional.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 SKUs. The maximum value is 1000; the server will coerce values above 1000. Optional.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// A token for a page of results other than the first page. Obtained using ListTransferableSkusResponse.next_page_token of the previous CloudChannelService.ListTransferableSkus call. Optional.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ListTransferableSkusRequest {}


/// Response message for CloudChannelService.ListTransferableSkus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transferable skus accounts](AccountListTransferableSkuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableSkusResponse {
    /// A token to retrieve the next page of results. Pass to ListTransferableSkusRequest.page_token to obtain that page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Information about existing SKUs for a customer that needs a transfer.
    #[serde(rename="transferableSkus")]
    
    pub transferable_skus: Option<Vec<GoogleCloudChannelV1TransferableSku>>,
}

impl client::ResponseResult for GoogleCloudChannelV1ListTransferableSkusResponse {}


/// Represents the marketing information for a Product, SKU or Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1MarketingInfo {
    /// Default logo.
    #[serde(rename="defaultLogo")]
    
    pub default_logo: Option<GoogleCloudChannelV1Media>,
    /// Human readable description. Description can contain HTML.
    
    pub description: Option<String>,
    /// Human readable name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for GoogleCloudChannelV1MarketingInfo {}


/// Represents media information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Media {
    /// URL of the media.
    
    pub content: Option<String>,
    /// Title of the media.
    
    pub title: Option<String>,
    /// Type of the media.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleCloudChannelV1MediaTypeEnum>,
}

impl client::Part for GoogleCloudChannelV1Media {}


/// Represents an offer made to resellers for purchase. An offer is associated with a Sku, has a plan for payment, a price, and defines the constraints for buying.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements lookup offer accounts](AccountCustomerEntitlementLookupOfferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Offer {
    /// Constraints on transacting the Offer.
    
    pub constraints: Option<GoogleCloudChannelV1Constraints>,
    /// The deal code of the offer to get a special promotion or discount.
    #[serde(rename="dealCode")]
    
    pub deal_code: Option<String>,
    /// Output only. End of the Offer validity time.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Marketing information for the Offer.
    #[serde(rename="marketingInfo")]
    
    pub marketing_info: Option<GoogleCloudChannelV1MarketingInfo>,
    /// Resource Name of the Offer. Format: accounts/{account_id}/offers/{offer_id}
    
    pub name: Option<String>,
    /// Parameters required to use current Offer to purchase.
    #[serde(rename="parameterDefinitions")]
    
    pub parameter_definitions: Option<Vec<GoogleCloudChannelV1ParameterDefinition>>,
    /// Describes the payment plan for the Offer.
    
    pub plan: Option<GoogleCloudChannelV1Plan>,
    /// Price for each monetizable resource type.
    #[serde(rename="priceByResources")]
    
    pub price_by_resources: Option<Vec<GoogleCloudChannelV1PriceByResource>>,
    /// SKU the offer is associated with.
    
    pub sku: Option<GoogleCloudChannelV1Sku>,
    /// Start of the Offer validity time.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for GoogleCloudChannelV1Offer {}


/// Definition for extended entitlement parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Parameter {
    /// Output only. Specifies whether this parameter is allowed to be changed. For example, for a Google Workspace Business Starter entitlement in commitment plan, num_units is editable when entitlement is active.
    
    pub editable: Option<bool>,
    /// Name of the parameter.
    
    pub name: Option<String>,
    /// Value of the parameter.
    
    pub value: Option<GoogleCloudChannelV1Value>,
}

impl client::Part for GoogleCloudChannelV1Parameter {}


/// Parameter's definition. Specifies what parameter is required to use the current Offer to purchase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ParameterDefinition {
    /// If not empty, parameter values must be drawn from this list. For example, [us-west1, us-west2, ...] Applicable to STRING parameter type.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<GoogleCloudChannelV1Value>>,
    /// Maximum value of the parameter, if applicable. Inclusive. For example, maximum seats when purchasing Google Workspace Business Standard. Applicable to INT64 and DOUBLE parameter types.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<GoogleCloudChannelV1Value>,
    /// Minimal value of the parameter, if applicable. Inclusive. For example, minimal commitment when purchasing Anthos is 0.01. Applicable to INT64 and DOUBLE parameter types.
    #[serde(rename="minValue")]
    
    pub min_value: Option<GoogleCloudChannelV1Value>,
    /// Name of the parameter.
    
    pub name: Option<String>,
    /// If set to true, parameter is optional to purchase this Offer.
    
    pub optional: Option<bool>,
    /// Data type of the parameter. Minimal value, Maximum value and allowed values will use specified data type here.
    #[serde(rename="parameterType")]
    
    pub parameter_type: Option<GoogleCloudChannelV1ParameterDefinitionParameterTypeEnum>,
}

impl client::Part for GoogleCloudChannelV1ParameterDefinition {}


/// An adjustment that applies a flat markup or markdown to an entire bill.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PercentageAdjustment {
    /// The percentage of the bill to adjust. For example: Mark down by 1% => "-1.00" Mark up by 1% => "1.00" Pass-Through => "0.00"
    
    pub percentage: Option<GoogleTypeDecimal>,
}

impl client::Part for GoogleCloudChannelV1PercentageAdjustment {}


/// Represents period in days/months/years.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Period {
    /// Total duration of Period Type defined.
    
    pub duration: Option<i32>,
    /// Period Type.
    #[serde(rename="periodType")]
    
    pub period_type: Option<GoogleCloudChannelV1PeriodPeriodTypeEnum>,
}

impl client::Part for GoogleCloudChannelV1Period {}


/// The payment plan for the Offer. Describes how to make a payment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Plan {
    /// Reseller Billing account to charge after an offer transaction. Only present for Google Cloud Platform offers.
    #[serde(rename="billingAccount")]
    
    pub billing_account: Option<String>,
    /// Describes how frequently the reseller will be billed, such as once per month.
    #[serde(rename="paymentCycle")]
    
    pub payment_cycle: Option<GoogleCloudChannelV1Period>,
    /// Describes how a reseller will be billed.
    #[serde(rename="paymentPlan")]
    
    pub payment_plan: Option<GoogleCloudChannelV1PlanPaymentPlanEnum>,
    /// Specifies when the payment needs to happen.
    #[serde(rename="paymentType")]
    
    pub payment_type: Option<GoogleCloudChannelV1PlanPaymentTypeEnum>,
    /// Present for Offers with a trial period. For trial-only Offers, a paid service needs to start before the trial period ends for continued service. For Regular Offers with a trial period, the regular pricing goes into effect when trial period ends, or if paid service is started before the end of the trial period.
    #[serde(rename="trialPeriod")]
    
    pub trial_period: Option<GoogleCloudChannelV1Period>,
}

impl client::Part for GoogleCloudChannelV1Plan {}


/// Represents the price of the Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Price {
    /// Base price.
    #[serde(rename="basePrice")]
    
    pub base_price: Option<GoogleTypeMoney>,
    /// Discount percentage, represented as decimal. For example, a 20% discount will be represent as 0.2.
    
    pub discount: Option<f64>,
    /// Effective Price after applying the discounts.
    #[serde(rename="effectivePrice")]
    
    pub effective_price: Option<GoogleTypeMoney>,
    /// Link to external price list, such as link to Google Voice rate card.
    #[serde(rename="externalPriceUri")]
    
    pub external_price_uri: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Price {}


/// Represents price by resource type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PriceByResource {
    /// Price of the Offer. Present if there are no price phases.
    
    pub price: Option<GoogleCloudChannelV1Price>,
    /// Specifies the price by time range.
    #[serde(rename="pricePhases")]
    
    pub price_phases: Option<Vec<GoogleCloudChannelV1PricePhase>>,
    /// Resource Type. Example: SEAT
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<GoogleCloudChannelV1PriceByResourceResourceTypeEnum>,
}

impl client::Part for GoogleCloudChannelV1PriceByResource {}


/// Specifies the price by the duration of months. For example, a 20% discount for the first six months, then a 10% discount starting on the seventh month.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PricePhase {
    /// Defines first period for the phase.
    #[serde(rename="firstPeriod")]
    
    pub first_period: Option<i32>,
    /// Defines first period for the phase.
    #[serde(rename="lastPeriod")]
    
    pub last_period: Option<i32>,
    /// Defines the phase period type.
    #[serde(rename="periodType")]
    
    pub period_type: Option<GoogleCloudChannelV1PricePhasePeriodTypeEnum>,
    /// Price of the phase. Present if there are no price tiers.
    
    pub price: Option<GoogleCloudChannelV1Price>,
    /// Price by the resource tiers.
    #[serde(rename="priceTiers")]
    
    pub price_tiers: Option<Vec<GoogleCloudChannelV1PriceTier>>,
}

impl client::Part for GoogleCloudChannelV1PricePhase {}


/// Defines price at resource tier level. For example, an offer with following definition : * Tier 1: Provide 25% discount for all seats between 1 and 25. * Tier 2: Provide 10% discount for all seats between 26 and 100. * Tier 3: Provide flat 15% discount for all seats above 100. Each of these tiers is represented as a PriceTier.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PriceTier {
    /// First resource for which the tier price applies.
    #[serde(rename="firstResource")]
    
    pub first_resource: Option<i32>,
    /// Last resource for which the tier price applies.
    #[serde(rename="lastResource")]
    
    pub last_resource: Option<i32>,
    /// Price of the tier.
    
    pub price: Option<GoogleCloudChannelV1Price>,
}

impl client::Part for GoogleCloudChannelV1PriceTier {}


/// A Product is the entity a customer uses when placing an order. For example, Google Workspace, Google Voice, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Product {
    /// Marketing information for the product.
    #[serde(rename="marketingInfo")]
    
    pub marketing_info: Option<GoogleCloudChannelV1MarketingInfo>,
    /// Resource Name of the Product. Format: products/{product_id}
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Product {}


/// Request message for CloudChannelService.ProvisionCloudIdentity
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers provision cloud identity accounts](AccountCustomerProvisionCloudIdentityCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ProvisionCloudIdentityRequest {
    /// CloudIdentity-specific customer information.
    #[serde(rename="cloudIdentityInfo")]
    
    pub cloud_identity_info: Option<GoogleCloudChannelV1CloudIdentityInfo>,
    /// Admin user information.
    
    pub user: Option<GoogleCloudChannelV1AdminUser>,
    /// Validate the request and preview the review, but do not post it.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for GoogleCloudChannelV1ProvisionCloudIdentityRequest {}


/// Service provisioned for an entitlement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ProvisionedService {
    /// Output only. The product pertaining to the provisioning resource as specified in the Offer.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Output only. Provisioning ID of the entitlement. For Google Workspace, this is the underlying Subscription ID. For Google Cloud Platform, this is the Billing Account ID of the billing subaccount."
    #[serde(rename="provisioningId")]
    
    pub provisioning_id: Option<String>,
    /// Output only. The SKU pertaining to the provisioning resource as specified in the Offer.
    #[serde(rename="skuId")]
    
    pub sku_id: Option<String>,
}

impl client::Part for GoogleCloudChannelV1ProvisionedService {}


/// Offer that you can purchase for a customer. This is used in the ListPurchasableOffer API response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PurchasableOffer {
    /// Offer.
    
    pub offer: Option<GoogleCloudChannelV1Offer>,
}

impl client::Part for GoogleCloudChannelV1PurchasableOffer {}


/// SKU that you can purchase. This is used in ListPurchasableSku API response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PurchasableSku {
    /// SKU
    
    pub sku: Option<GoogleCloudChannelV1Sku>,
}

impl client::Part for GoogleCloudChannelV1PurchasableSku {}


/// Request Message for RegisterSubscriber.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register accounts](AccountRegisterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RegisterSubscriberRequest {
    /// Required. Service account that provides subscriber access to the registered topic.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1RegisterSubscriberRequest {}


/// Response Message for RegisterSubscriber.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [register accounts](AccountRegisterCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RegisterSubscriberResponse {
    /// Name of the topic the subscriber will listen to.
    
    pub topic: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1RegisterSubscriberResponse {}


/// Renewal settings for renewable Offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RenewalSettings {
    /// If false, the plan will be completed at the end date.
    #[serde(rename="enableRenewal")]
    
    pub enable_renewal: Option<bool>,
    /// Describes how frequently the reseller will be billed, such as once per month.
    #[serde(rename="paymentCycle")]
    
    pub payment_cycle: Option<GoogleCloudChannelV1Period>,
    /// Describes how a reseller will be billed.
    #[serde(rename="paymentPlan")]
    
    pub payment_plan: Option<GoogleCloudChannelV1RenewalSettingPaymentPlanEnum>,
    /// If true and enable_renewal = true, the unit (for example seats or licenses) will be set to the number of active units at renewal time.
    #[serde(rename="resizeUnitCount")]
    
    pub resize_unit_count: Option<bool>,
}

impl client::Part for GoogleCloudChannelV1RenewalSettings {}


/// The ID and description of a report that was used to generate report data. For example, "GCP Daily Spend", "Google Workspace License Activity", etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Report {
    /// The list of columns included in the report. This defines the schema of the report results.
    
    pub columns: Option<Vec<GoogleCloudChannelV1Column>>,
    /// A description of other aspects of the report, such as the products it supports.
    
    pub description: Option<String>,
    /// A human-readable name for this report.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Required. The report's resource name. Specifies the account and report used to generate report data. The report_id identifier is a UID (for example, `613bf59q`). Name uses the format: accounts/{account_id}/reports/{report_id}
    
    pub name: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Report {}


/// The features describing the data. Returned by CloudChannelReportsService.RunReportJob and CloudChannelReportsService.FetchReportResults.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ReportResultsMetadata {
    /// The date range of reported usage.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<GoogleCloudChannelV1DateRange>,
    /// The usage dates immediately preceding `date_range` with the same duration. Use this to calculate trending usage and costs. This is only populated if you request trending data. For example, if `date_range` is July 1-15, `preceding_date_range` will be June 16-30.
    #[serde(rename="precedingDateRange")]
    
    pub preceding_date_range: Option<GoogleCloudChannelV1DateRange>,
    /// Details of the completed report.
    
    pub report: Option<GoogleCloudChannelV1Report>,
    /// The total number of rows of data in the final report.
    #[serde(rename="rowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count: Option<i64>,
}

impl client::Part for GoogleCloudChannelV1ReportResultsMetadata {}


/// A single report value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ReportValue {
    /// A value of type `google.type.DateTime` (year, month, day, hour, minute, second, and UTC offset or timezone.)
    #[serde(rename="dateTimeValue")]
    
    pub date_time_value: Option<GoogleTypeDateTime>,
    /// A value of type `google.type.Date` (year, month, day).
    #[serde(rename="dateValue")]
    
    pub date_value: Option<GoogleTypeDate>,
    /// A value of type `google.type.Decimal`, representing non-integer numeric values.
    #[serde(rename="decimalValue")]
    
    pub decimal_value: Option<GoogleTypeDecimal>,
    /// A value of type `int`.
    #[serde(rename="intValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int_value: Option<i64>,
    /// A value of type `google.type.Money` (currency code, whole units, decimal units).
    #[serde(rename="moneyValue")]
    
    pub money_value: Option<GoogleTypeMoney>,
    /// A value of type `string`.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for GoogleCloudChannelV1ReportValue {}


/// A type that represents the various adjustments you can apply to a bill.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RepricingAdjustment {
    /// Flat markup or markdown on an entire bill.
    #[serde(rename="percentageAdjustment")]
    
    pub percentage_adjustment: Option<GoogleCloudChannelV1PercentageAdjustment>,
}

impl client::Part for GoogleCloudChannelV1RepricingAdjustment {}


/// Represents the various repricing conditions you can use for a conditional override.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RepricingCondition {
    /// SKU Group condition for override.
    #[serde(rename="skuGroupCondition")]
    
    pub sku_group_condition: Option<GoogleCloudChannelV1SkuGroupCondition>,
}

impl client::Part for GoogleCloudChannelV1RepricingCondition {}


/// Configuration for repricing a Google bill over a period of time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RepricingConfig {
    /// Required. Information about the adjustment.
    
    pub adjustment: Option<GoogleCloudChannelV1RepricingAdjustment>,
    /// Applies the repricing configuration at the channel partner level. This is the only supported value for ChannelPartnerRepricingConfig.
    #[serde(rename="channelPartnerGranularity")]
    
    pub channel_partner_granularity: Option<GoogleCloudChannelV1RepricingConfigChannelPartnerGranularity>,
    /// The conditional overrides to apply for this configuration. If you list multiple overrides, only the first valid override is used. If you don't list any overrides, the API uses the normal adjustment and rebilling basis.
    #[serde(rename="conditionalOverrides")]
    
    pub conditional_overrides: Option<Vec<GoogleCloudChannelV1ConditionalOverride>>,
    /// Required. The YearMonth when these adjustments activate. The Day field needs to be "0" since we only accept YearMonth repricing boundaries.
    #[serde(rename="effectiveInvoiceMonth")]
    
    pub effective_invoice_month: Option<GoogleTypeDate>,
    /// Applies the repricing configuration at the entitlement level. This is the only supported value for CustomerRepricingConfig.
    #[serde(rename="entitlementGranularity")]
    
    pub entitlement_granularity: Option<GoogleCloudChannelV1RepricingConfigEntitlementGranularity>,
    /// Required. The RebillingBasis to use for this bill. Specifies the relative cost based on repricing costs you will apply.
    #[serde(rename="rebillingBasis")]
    
    pub rebilling_basis: Option<GoogleCloudChannelV1RepricingConfigRebillingBasisEnum>,
}

impl client::Part for GoogleCloudChannelV1RepricingConfig {}


/// Applies the repricing configuration at the channel partner level. The channel partner value is derived from the resource name. Takes an empty json object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RepricingConfigChannelPartnerGranularity { _never_set: Option<bool> }

impl client::Part for GoogleCloudChannelV1RepricingConfigChannelPartnerGranularity {}


/// Applies the repricing configuration at the entitlement level.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RepricingConfigEntitlementGranularity {
    /// Resource name of the entitlement. Format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    
    pub entitlement: Option<String>,
}

impl client::Part for GoogleCloudChannelV1RepricingConfigEntitlementGranularity {}


/// A row of report values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Row {
    /// The list of values in the row.
    
    pub values: Option<Vec<GoogleCloudChannelV1ReportValue>>,
}

impl client::Part for GoogleCloudChannelV1Row {}


/// Request message for CloudChannelReportsService.RunReportJob.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports run accounts](AccountReportRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1RunReportJobRequest {
    /// Optional. The range of usage or invoice dates to include in the result.
    #[serde(rename="dateRange")]
    
    pub date_range: Option<GoogleCloudChannelV1DateRange>,
    /// Optional. A structured string that defines conditions on dimension columns to restrict the report output. Filters support logical operators (AND, OR, NOT) and conditional operators (=, !=, <, >, <=, and >=) using `column_id` as keys. For example: `(customer:"accounts/C123abc/customers/S456def" OR customer:"accounts/C123abc/customers/S789ghi") AND invoice_start_date.year >= 2022`
    
    pub filter: Option<String>,
    /// Optional. The BCP-47 language code, such as "en-US". If specified, the response is localized to the corresponding language code if the original data sources support it. Default is "en-US".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1RunReportJobRequest {}


/// Represents a product's purchasable Stock Keeping Unit (SKU). SKUs represent the different variations of the product. For example, Google Workspace Business Standard and Google Workspace Business Plus are Google Workspace product SKUs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Sku {
    /// Marketing information for the SKU.
    #[serde(rename="marketingInfo")]
    
    pub marketing_info: Option<GoogleCloudChannelV1MarketingInfo>,
    /// Resource Name of the SKU. Format: products/{product_id}/skus/{sku_id}
    
    pub name: Option<String>,
    /// Product the SKU is associated with.
    
    pub product: Option<GoogleCloudChannelV1Product>,
}

impl client::Part for GoogleCloudChannelV1Sku {}


/// A condition that applies the override if a line item SKU is found in the SKU group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1SkuGroupCondition {
    /// Specifies a SKU group (https://cloud.google.com/skus/sku-groups). Resource name of SKU group. Format: accounts/{account}/skuGroups/{sku_group}. Example: "accounts/C01234/skuGroups/3d50fd57-3157-4577-a5a9-a219b8490041".
    #[serde(rename="skuGroup")]
    
    pub sku_group: Option<String>,
}

impl client::Part for GoogleCloudChannelV1SkuGroupCondition {}


/// Request message for CloudChannelService.StartPaidService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements start paid service accounts](AccountCustomerEntitlementStartPaidServiceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1StartPaidServiceRequest {
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1StartPaidServiceRequest {}


/// Request message for CloudChannelService.SuspendEntitlement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements suspend accounts](AccountCustomerEntitlementSuspendCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1SuspendEntitlementRequest {
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1SuspendEntitlementRequest {}


/// Specifies transfer eligibility of a SKU.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferEligibility {
    /// Localized description if reseller is not eligible to transfer the SKU.
    
    pub description: Option<String>,
    /// Specified the reason for ineligibility.
    #[serde(rename="ineligibilityReason")]
    
    pub ineligibility_reason: Option<GoogleCloudChannelV1TransferEligibilityIneligibilityReasonEnum>,
    /// Whether reseller is eligible to transfer the SKU.
    #[serde(rename="isEligible")]
    
    pub is_eligible: Option<bool>,
}

impl client::Part for GoogleCloudChannelV1TransferEligibility {}


/// Request message for CloudChannelService.TransferEntitlements.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers transfer entitlements accounts](AccountCustomerTransferEntitlementCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferEntitlementsRequest {
    /// The super admin of the resold customer generates this token to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. You can omit this token after authorization. See https://support.google.com/a/answer/7643790 for more details.
    #[serde(rename="authToken")]
    
    pub auth_token: Option<String>,
    /// Required. The new entitlements to create or transfer.
    
    pub entitlements: Option<Vec<GoogleCloudChannelV1Entitlement>>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1TransferEntitlementsRequest {}


/// Request message for CloudChannelService.TransferEntitlementsToGoogle.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers transfer entitlements to google accounts](AccountCustomerTransferEntitlementsToGoogleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferEntitlementsToGoogleRequest {
    /// Required. The entitlements to transfer to Google.
    
    pub entitlements: Option<Vec<GoogleCloudChannelV1Entitlement>>,
    /// Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`).
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1TransferEntitlementsToGoogleRequest {}


/// TransferableOffer represents an Offer that can be used in Transfer. Read-only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferableOffer {
    /// Offer with parameter constraints updated to allow the Transfer.
    
    pub offer: Option<GoogleCloudChannelV1Offer>,
}

impl client::Part for GoogleCloudChannelV1TransferableOffer {}


/// TransferableSku represents information a reseller needs to view existing provisioned services for a customer that they do not own. Read-only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferableSku {
    /// Optional. The customer to transfer has an entitlement with the populated legacy SKU.
    #[serde(rename="legacySku")]
    
    pub legacy_sku: Option<GoogleCloudChannelV1Sku>,
    /// The SKU pertaining to the provisioning resource as specified in the Offer.
    
    pub sku: Option<GoogleCloudChannelV1Sku>,
    /// Describes the transfer eligibility of a SKU.
    #[serde(rename="transferEligibility")]
    
    pub transfer_eligibility: Option<GoogleCloudChannelV1TransferEligibility>,
}

impl client::Part for GoogleCloudChannelV1TransferableSku {}


/// Settings for trial offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TrialSettings {
    /// Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example [Epoch converter](https://www.epochconverter.com).
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Determines if the entitlement is in a trial or not: * `true` - The entitlement is in trial. * `false` - The entitlement is not in trial.
    
    pub trial: Option<bool>,
}

impl client::Part for GoogleCloudChannelV1TrialSettings {}


/// Request Message for UnregisterSubscriber.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unregister accounts](AccountUnregisterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1UnregisterSubscriberRequest {
    /// Required. Service account to unregister from subscriber access to the topic.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1UnregisterSubscriberRequest {}


/// Response Message for UnregisterSubscriber.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unregister accounts](AccountUnregisterCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1UnregisterSubscriberResponse {
    /// Name of the topic the service account subscriber access was removed from.
    
    pub topic: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1UnregisterSubscriberResponse {}


/// Request message for CloudChannelService.UpdateChannelPartnerLink
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links patch accounts](AccountChannelPartnerLinkPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {
    /// Required. The channel partner link to update. Only channel_partner_link.link_state is allowed for updates.
    #[serde(rename="channelPartnerLink")]
    
    pub channel_partner_link: Option<GoogleCloudChannelV1ChannelPartnerLink>,
    /// Required. The update mask that applies to the resource. The only allowable value for an update mask is channel_partner_link.link_state.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {}


/// Data type and value of a parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Value {
    /// Represents a boolean value.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Represents a double value.
    #[serde(rename="doubleValue")]
    
    pub double_value: Option<f64>,
    /// Represents an int64 value.
    #[serde(rename="int64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int64_value: Option<i64>,
    /// Represents an 'Any' proto value.
    #[serde(rename="protoValue")]
    
    pub proto_value: Option<HashMap<String, json::Value>>,
    /// Represents a string value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Value {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningCancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleLongrunningCancelOperationRequest {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunningOperation>>,
}

impl client::ResponseResult for GoogleLongrunningListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements activate accounts](AccountCustomerEntitlementActivateCall) (response)
/// * [customers entitlements cancel accounts](AccountCustomerEntitlementCancelCall) (response)
/// * [customers entitlements change offer accounts](AccountCustomerEntitlementChangeOfferCall) (response)
/// * [customers entitlements change parameters accounts](AccountCustomerEntitlementChangeParameterCall) (response)
/// * [customers entitlements change renewal settings accounts](AccountCustomerEntitlementChangeRenewalSettingCall) (response)
/// * [customers entitlements create accounts](AccountCustomerEntitlementCreateCall) (response)
/// * [customers entitlements start paid service accounts](AccountCustomerEntitlementStartPaidServiceCall) (response)
/// * [customers entitlements suspend accounts](AccountCustomerEntitlementSuspendCall) (response)
/// * [customers provision cloud identity accounts](AccountCustomerProvisionCloudIdentityCall) (response)
/// * [customers transfer entitlements accounts](AccountCustomerTransferEntitlementCall) (response)
/// * [customers transfer entitlements to google accounts](AccountCustomerTransferEntitlementsToGoogleCall) (response)
/// * [reports run accounts](AccountReportRunCall) (response)
/// * [get operations](OperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links channel partner repricing configs delete accounts](AccountChannelPartnerLinkChannelPartnerRepricingConfigDeleteCall) (response)
/// * [channel partner links customers delete accounts](AccountChannelPartnerLinkCustomerDeleteCall) (response)
/// * [customers customer repricing configs delete accounts](AccountCustomerCustomerRepricingConfigDeleteCall) (response)
/// * [customers delete accounts](AccountCustomerDeleteCall) (response)
/// * [cancel operations](OperationCancelCall) (response)
/// * [delete operations](OperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}


/// Represents civil time (or occasionally physical time). This type can represent a civil time in one of a few possible ways: * When utc_offset is set and time_zone is unset: a civil time on a calendar day with a particular offset from UTC. * When time_zone is set and utc_offset is unset: a civil time on a calendar day in a particular time zone. * When neither time_zone nor utc_offset is set: a civil time on a calendar day in local time. The date is relative to the Proleptic Gregorian Calendar. If year, month, or day are 0, the DateTime is considered not to have a specific year, month, or day respectively. This type may also be used to represent a physical time if all the date and time fields are set and either case of the `time_offset` oneof is set. Consider using `Timestamp` message for physical time instead. If your use case also would like to store the user's timezone, that can be done in another field. This type is more flexible than some applications may want. Make sure to document and validate your application's limitations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDateTime {
    /// Optional. Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a datetime without a day.
    
    pub day: Option<i32>,
    /// Optional. Hours of day in 24 hour format. Should be from 0 to 23, defaults to 0 (midnight). An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Optional. Minutes of hour of day. Must be from 0 to 59, defaults to 0.
    
    pub minutes: Option<i32>,
    /// Optional. Month of year. Must be from 1 to 12, or 0 if specifying a datetime without a month.
    
    pub month: Option<i32>,
    /// Optional. Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999, defaults to 0.
    
    pub nanos: Option<i32>,
    /// Optional. Seconds of minutes of the time. Must normally be from 0 to 59, defaults to 0. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
    /// Time zone.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<GoogleTypeTimeZone>,
    /// UTC offset. Must be whole seconds, between -18 hours and +18 hours. For example, a UTC offset of -4:00 would be represented as { seconds: -14400 }.
    #[serde(rename="utcOffset")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub utc_offset: Option<client::chrono::Duration>,
    /// Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a datetime without a year.
    
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDateTime {}


/// A representation of a decimal value, such as 2.5. Clients may convert values into language-native decimal formats, such as Java's BigDecimal or Python's decimal.Decimal. [BigDecimal]: https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/math/BigDecimal.html [decimal.Decimal]: https://docs.python.org/3/library/decimal.html
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDecimal {
    /// The decimal value, as a string. The string representation consists of an optional sign, `+` (`U+002B`) or `-` (`U+002D`), followed by a sequence of zero or more decimal digits ("the integer"), optionally followed by a fraction, optionally followed by an exponent. An empty string **should** be interpreted as `0`. The fraction consists of a decimal point followed by zero or more decimal digits. The string must contain at least one digit in either the integer or the fraction. The number formed by the sign, the integer and the fraction is referred to as the significand. The exponent consists of the character `e` (`U+0065`) or `E` (`U+0045`) followed by one or more decimal digits. Services **should** normalize decimal values before storing them by: - Removing an explicitly-provided `+` sign (`+2.5` -> `2.5`). - Replacing a zero-length integer value with `0` (`.5` -> `0.5`). - Coercing the exponent character to upper-case, with explicit sign (`2.5e8` -> `2.5E+8`). - Removing an explicitly-provided zero exponent (`2.5E0` -> `2.5`). Services **may** perform additional normalization based on its own needs and the internal decimal implementation selected, such as shifting the decimal point and exponent value together (example: `2.5E-1` <-> `0.25`). Additionally, services **may** preserve trailing zeroes in the fraction to indicate increased precision, but are not required to do so. Note that only the `.` character is supported to divide the integer and the fraction; `,` **should not** be supported regardless of locale. Additionally, thousand separators **should not** be supported. If a service does support them, values **must** be normalized. The ENBF grammar is: DecimalString = '' | [Sign] Significand [Exponent]; Sign = '+' | '-'; Significand = Digits '.' | [Digits] '.' Digits; Exponent = ('e' | 'E') [Sign] Digits; Digits = { '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' }; Services **should** clearly document the range of supported values, the maximum supported precision (total number of digits), and, if applicable, the scale (number of digits after the decimal point), as well as how it behaves when receiving out-of-bounds values. Services **may** choose to accept values passed as input even when the value has a higher precision or scale than the service supports, and **should** round the value to fit the supported scale. Alternatively, the service **may** error with `400 Bad Request` (`INVALID_ARGUMENT` in gRPC) if precision would be lost. Services **should** error with `400 Bad Request` (`INVALID_ARGUMENT` in gRPC) if the service receives a value outside of the supported range.
    
    pub value: Option<String>,
}

impl client::Part for GoogleTypeDecimal {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for GoogleTypeMoney {}


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypePostalAddress {
    /// Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[serde(rename="addressLines")]
    
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain "care of" information.
    
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions.
    
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number alone, representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts.
    
    pub sublocality: Option<String>,
}

impl client::Part for GoogleTypePostalAddress {}


/// Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeTimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    
    pub id: Option<String>,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    
    pub version: Option<String>,
}

impl client::Part for GoogleTypeTimeZone {}


