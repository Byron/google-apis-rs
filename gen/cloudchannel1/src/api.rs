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

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Manage users on your domain
    AppOrder,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppOrder => "https://www.googleapis.com/auth/apps.order",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppOrder
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Cloudchannel related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ActivateEntitlementRequest;
/// use cloudchannel1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use cloudchannel1::Cloudchannel;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ActivateEntitlementRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_activate(req, "name")
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
pub struct Cloudchannel<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for Cloudchannel<C> {}

impl<'a, C> Cloudchannel<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Cloudchannel<C> {
        Cloudchannel {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://cloudchannel.googleapis.com/".to_string(),
            _root_url: "https://cloudchannel.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C> {
        AccountMethods { hub: &self }
    }
    pub fn operations(&'a self) -> OperationMethods<'a, C> {
        OperationMethods { hub: &self }
    }
    pub fn products(&'a self) -> ProductMethods<'a, C> {
        ProductMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://cloudchannel.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://cloudchannel.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Request message for CloudChannelService.ActivateEntitlement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements activate accounts](AccountCustomerEntitlementActivateCall) (request)
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChangeOfferRequest {
    /// Required. New Offer. Format: accounts/{account_id}/offers/{offer_id}.
    pub offer: Option<String>,
    /// Optional. Parameters needed to purchase the Offer.
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChangeParametersRequest {
    /// Required. Entitlement parameters to update. You can only change editable parameters.
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ChannelPartnerLink {
    /// Output only. Cloud Identity info of the channel partner (IR).
    #[serde(rename="channelPartnerCloudIdentityInfo")]
    pub channel_partner_cloud_identity_info: Option<GoogleCloudChannelV1CloudIdentityInfo>,
    /// Output only. Timestamp of when the channel partner link is created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// Output only. URI of the web page where partner accepts the link invitation.
    #[serde(rename="inviteLinkUri")]
    pub invite_link_uri: Option<String>,
    /// Required. State of the channel partner link.
    #[serde(rename="linkState")]
    pub link_state: Option<String>,
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
    pub update_time: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1ChannelPartnerLink {}
impl client::ResponseResult for GoogleCloudChannelV1ChannelPartnerLink {}


/// Request message for CloudChannelService.CheckCloudIdentityAccountsExist.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check cloud identity accounts exist accounts](AccountCheckCloudIdentityAccountsExistCall) (request)
/// 
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
/// 
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
    pub customer_type: Option<String>,
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


/// Commitment settings for commitment-based offers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CommitmentSettings {
    /// Output only. Commitment end timestamp.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// Optional. Renewal settings applicable for a commitment-based Offer.
    #[serde(rename="renewalSettings")]
    pub renewal_settings: Option<GoogleCloudChannelV1RenewalSettings>,
    /// Output only. Commitment start timestamp.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
}

impl client::Part for GoogleCloudChannelV1CommitmentSettings {}


/// Represents the constraints for buying the Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ContactInfo {
    /// Output only. Display name of the contact in the customer account. Populated by combining customer first name and last name.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Email of the contact in the customer account. Email is required for entitlements that need creation of admin.google.com accounts. The email will be the username used in credentials to access the admin.google.com account.
    pub email: Option<String>,
    /// First name of the contact in the customer account.
    #[serde(rename="firstName")]
    pub first_name: Option<String>,
    /// Last name of the contact in the customer account.
    #[serde(rename="lastName")]
    pub last_name: Option<String>,
    /// Phone number of the contact in the customer account.
    pub phone: Option<String>,
    /// Optional. Job title of the contact in the customer account.
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
/// 
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
/// * [channel partner links customers patch accounts](AccountChannelPartnerLinkCustomerPatchCall) (request|response)
/// * [customers create accounts](AccountCustomerCreateCall) (request|response)
/// * [customers get accounts](AccountCustomerGetCall) (response)
/// * [customers patch accounts](AccountCustomerPatchCall) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Customer {
    /// Secondary contact email. Alternate email and primary contact email are required to have different domains if primary contact email is present. When creating admin.google.com accounts, users get notified credentials at this email. This email address is also used as a recovery email.
    #[serde(rename="alternateEmail")]
    pub alternate_email: Option<String>,
    /// Cloud Identity ID of the customer's channel partner. Populated only if a channel partner exists for this customer.
    #[serde(rename="channelPartnerId")]
    pub channel_partner_id: Option<String>,
    /// Output only. Customer's cloud_identity_id. Populated only if a Cloud Identity resource exists for this customer.
    #[serde(rename="cloudIdentityId")]
    pub cloud_identity_id: Option<String>,
    /// Output only. Cloud Identity information for the customer. Populated only if a Cloud Identity account exists for this customer.
    #[serde(rename="cloudIdentityInfo")]
    pub cloud_identity_info: Option<GoogleCloudChannelV1CloudIdentityInfo>,
    /// Output only. The time at which the customer is created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// Required. Primary domain used by the customer. Domain of primary contact email is required to be same as the provided domain.
    pub domain: Option<String>,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub name: Option<String>,
    /// Required. Name of the organization that the customer entity represents.
    #[serde(rename="orgDisplayName")]
    pub org_display_name: Option<String>,
    /// Required. Address of the organization of the customer entity. Region and zip codes are required to enforce US laws and embargoes. Valid address lines are required for all customers. Language code is discarded. Use the Customer-level language code to set the customer's language.
    #[serde(rename="orgPostalAddress")]
    pub org_postal_address: Option<GoogleTypePostalAddress>,
    /// Primary contact info.
    #[serde(rename="primaryContactInfo")]
    pub primary_contact_info: Option<GoogleCloudChannelV1ContactInfo>,
    /// Output only. The time at which the customer is updated.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1Customer {}
impl client::ResponseResult for GoogleCloudChannelV1Customer {}


/// Represents constraints required to purchase the Offer for a customer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1CustomerConstraints {
    /// Allowed Customer Type.
    #[serde(rename="allowedCustomerTypes")]
    pub allowed_customer_types: Option<Vec<String>>,
    /// Allowed geographical regions of the customer.
    #[serde(rename="allowedRegions")]
    pub allowed_regions: Option<Vec<String>>,
    /// Allowed Promotional Order Type. Present for Promotional offers.
    #[serde(rename="promotionalOrderTypes")]
    pub promotional_order_types: Option<Vec<String>>,
}

impl client::Part for GoogleCloudChannelV1CustomerConstraints {}


/// Required Edu Attributes
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1EduData {
    /// Size of the institute.
    #[serde(rename="instituteSize")]
    pub institute_size: Option<String>,
    /// Designated institute type of customer.
    #[serde(rename="instituteType")]
    pub institute_type: Option<String>,
    /// Web address for the edu customer's institution.
    pub website: Option<String>,
}

impl client::Part for GoogleCloudChannelV1EduData {}


/// An entitlement is a representation of a customer's ability to use a service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements get accounts](AccountCustomerEntitlementGetCall) (response)
/// 
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
    pub create_time: Option<String>,
    /// Output only. Resource name of an entitlement in the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}.
    pub name: Option<String>,
    /// Required. The offer resource name for which the entitlement is to be created. Takes the form: accounts/{account_id}/offers/{offer_id}.
    pub offer: Option<String>,
    /// Extended entitlement parameters. When creating an entitlement, valid parameters' names and values are defined in the offer's parameter definitions.
    pub parameters: Option<Vec<GoogleCloudChannelV1Parameter>>,
    /// Output only. Service provisioning details for the entitlement.
    #[serde(rename="provisionedService")]
    pub provisioned_service: Option<GoogleCloudChannelV1ProvisionedService>,
    /// Output only. Current provisioning state of the entitlement.
    #[serde(rename="provisioningState")]
    pub provisioning_state: Option<String>,
    /// Optional. This purchase order (PO) information is for resellers to use for their company tracking usage. If a purchaseOrderId value is given, it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters.
    #[serde(rename="purchaseOrderId")]
    pub purchase_order_id: Option<String>,
    /// Output only. Enumerable of all current suspension reasons for an entitlement.
    #[serde(rename="suspensionReasons")]
    pub suspension_reasons: Option<Vec<String>>,
    /// Output only. Settings for trial offers.
    #[serde(rename="trialSettings")]
    pub trial_settings: Option<GoogleCloudChannelV1TrialSettings>,
    /// Output only. The time at which the entitlement is updated.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
}

impl client::ResponseResult for GoogleCloudChannelV1Entitlement {}


/// Response message for CloudChannelService.ListChannelPartnerLinks.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links list accounts](AccountChannelPartnerLinkListCall) (response)
/// 
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


/// Response message for CloudChannelService.ListCustomers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links customers list accounts](AccountChannelPartnerLinkCustomerListCall) (response)
/// * [customers list accounts](AccountCustomerListCall) (response)
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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
/// 
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


/// Response message for ListSkus.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [skus list products](ProductSkuListCall) (response)
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableOffersRequest {
    /// Customer's Cloud Identity ID
    #[serde(rename="cloudIdentityId")]
    pub cloud_identity_id: Option<String>,
    /// A reseller should create a customer and use the resource name of that customer here.
    #[serde(rename="customerName")]
    pub customer_name: Option<String>,
    /// The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ListTransferableSkusRequest {
    /// The super admin of the resold customer generates this token to authorize a reseller to access their Cloud Identity and purchase entitlements on their behalf. You can omit this token after authorization. See https://support.google.com/a/answer/7643790 for more details.
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Media {
    /// URL of the media.
    pub content: Option<String>,
    /// Title of the media.
    pub title: Option<String>,
    /// Type of the media.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Media {}


/// Represents an offer made to resellers for purchase. An offer is associated with a Sku, has a plan for payment, a price, and defines the constraints for buying.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Offer {
    /// Constraints on transacting the Offer.
    pub constraints: Option<GoogleCloudChannelV1Constraints>,
    /// Output only. End of the Offer validity time.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
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
    pub start_time: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Offer {}


/// Definition for extended entitlement parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub parameter_type: Option<String>,
}

impl client::Part for GoogleCloudChannelV1ParameterDefinition {}


/// Represents period in days/months/years.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1Period {
    /// Total duration of Period Type defined.
    pub duration: Option<i32>,
    /// Period Type.
    #[serde(rename="periodType")]
    pub period_type: Option<String>,
}

impl client::Part for GoogleCloudChannelV1Period {}


/// The payment plan for the Offer. Describes how to make a payment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub payment_plan: Option<String>,
    /// Specifies when the payment needs to happen.
    #[serde(rename="paymentType")]
    pub payment_type: Option<String>,
    /// Present for Offers with a trial period. For trial-only Offers, a paid service needs to start before the trial period ends for continued service. For Regular Offers with a trial period, the regular pricing goes into effect when trial period ends, or if paid service is started before the end of the trial period.
    #[serde(rename="trialPeriod")]
    pub trial_period: Option<GoogleCloudChannelV1Period>,
}

impl client::Part for GoogleCloudChannelV1Plan {}


/// Represents the price of the Offer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1PriceByResource {
    /// Price of the Offer. Present if there are no price phases.
    pub price: Option<GoogleCloudChannelV1Price>,
    /// Specifies the price by time range.
    #[serde(rename="pricePhases")]
    pub price_phases: Option<Vec<GoogleCloudChannelV1PricePhase>>,
    /// Resource Type. Example: SEAT
    #[serde(rename="resourceType")]
    pub resource_type: Option<String>,
}

impl client::Part for GoogleCloudChannelV1PriceByResource {}


/// Specifies the price by the duration of months. For example, a 20% discount for the first six months, then a 10% discount starting on the seventh month.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub period_type: Option<String>,
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1ProvisionedService {
    /// Output only. The product pertaining to the provisioning resource as specified in the Offer.
    #[serde(rename="productId")]
    pub product_id: Option<String>,
    /// Output only. Provisioning ID of the entitlement. For Google Workspace, this would be the underlying Subscription ID.
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
/// 
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
/// 
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
    pub payment_plan: Option<String>,
    /// If true and enable_renewal = true, the unit (for example seats or licenses) will be set to the number of active units at renewal time.
    #[serde(rename="resizeUnitCount")]
    pub resize_unit_count: Option<bool>,
}

impl client::Part for GoogleCloudChannelV1RenewalSettings {}


/// Represents a product's purchasable Stock Keeping Unit (SKU). SKUs represent the different variations of the product. For example, Google Workspace Business Standard and Google Workspace Business Plus are Google Workspace product SKUs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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


/// Request message for CloudChannelService.StartPaidService.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [customers entitlements start paid service accounts](AccountCustomerEntitlementStartPaidServiceCall) (request)
/// 
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TransferEligibility {
    /// Localized description if reseller is not eligible to transfer the SKU.
    pub description: Option<String>,
    /// Specified the reason for ineligibility.
    #[serde(rename="ineligibilityReason")]
    pub ineligibility_reason: Option<String>,
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
/// 
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
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1TrialSettings {
    /// Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example [Epoch converter](https://www.epochconverter.com).
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
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
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {
    /// Required. The channel partner link to update. Only channel_partner_link.link_state is allowed for updates.
    #[serde(rename="channelPartnerLink")]
    pub channel_partner_link: Option<GoogleCloudChannelV1ChannelPartnerLink>,
    /// Required. The update mask that applies to the resource. The only allowable value for an update mask is channel_partner_link.link_state.
    #[serde(rename="updateMask")]
    pub update_mask: Option<String>,
}

impl client::RequestValue for GoogleCloudChannelV1UpdateChannelPartnerLinkRequest {}


/// Data type and value of a parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    pub int64_value: Option<String>,
    /// Represents an 'Any' proto value.
    #[serde(rename="protoValue")]
    pub proto_value: Option<HashMap<String, String>>,
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
/// 
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
/// 
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
/// * [get operations](OperationGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunningOperation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    pub metadata: Option<HashMap<String, String>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    pub response: Option<HashMap<String, String>>,
}

impl client::ResponseResult for GoogleLongrunningOperation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channel partner links customers delete accounts](AccountChannelPartnerLinkCustomerDeleteCall) (response)
/// * [customers delete accounts](AccountCustomerDeleteCall) (response)
/// * [cancel operations](OperationCancelCall) (response)
/// * [delete operations](OperationDeleteCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeMoney {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: Option<String>,
}

impl client::Part for GoogleTypeMoney {}


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `Cloudchannel` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use cloudchannel1::Cloudchannel;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `channel_partner_links_create(...)`, `channel_partner_links_customers_create(...)`, `channel_partner_links_customers_delete(...)`, `channel_partner_links_customers_get(...)`, `channel_partner_links_customers_list(...)`, `channel_partner_links_customers_patch(...)`, `channel_partner_links_get(...)`, `channel_partner_links_list(...)`, `channel_partner_links_patch(...)`, `check_cloud_identity_accounts_exist(...)`, `customers_create(...)`, `customers_delete(...)`, `customers_entitlements_activate(...)`, `customers_entitlements_cancel(...)`, `customers_entitlements_change_offer(...)`, `customers_entitlements_change_parameters(...)`, `customers_entitlements_change_renewal_settings(...)`, `customers_entitlements_create(...)`, `customers_entitlements_get(...)`, `customers_entitlements_list(...)`, `customers_entitlements_start_paid_service(...)`, `customers_entitlements_suspend(...)`, `customers_get(...)`, `customers_list(...)`, `customers_list_purchasable_offers(...)`, `customers_list_purchasable_skus(...)`, `customers_patch(...)`, `customers_provision_cloud_identity(...)`, `customers_transfer_entitlements(...)`, `customers_transfer_entitlements_to_google(...)`, `list_subscribers(...)`, `list_transferable_offers(...)`, `list_transferable_skus(...)`, `offers_list(...)`, `register(...)` and `unregister(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
}

impl<'a, C> client::MethodsBuilder for AccountMethods<'a, C> {}

impl<'a, C> AccountMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_customers_create(&self, request: GoogleCloudChannelV1Customer, parent: &str) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C> {
        AccountChannelPartnerLinkCustomerCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the given Customer permanently and irreversibly. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to delete.
    pub fn channel_partner_links_customers_delete(&self, name: &str) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, C> {
        AccountChannelPartnerLinkCustomerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn channel_partner_links_customers_get(&self, name: &str) -> AccountChannelPartnerLinkCustomerGetCall<'a, C> {
        AccountChannelPartnerLinkCustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    pub fn channel_partner_links_customers_list(&self, parent: &str) -> AccountChannelPartnerLinkCustomerListCall<'a, C> {
        AccountChannelPartnerLinkCustomerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn channel_partner_links_customers_patch(&self, request: GoogleCloudChannelV1Customer, name: &str) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C> {
        AccountChannelPartnerLinkCustomerPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates a channel partner link between a distributor and a reseller, or between resellers in an n-tier reseller channel. Invited partners need to follow the invite_link_uri provided in the response to accept. After accepting the invitation, a link is set up between the two parties. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * ALREADY_EXISTS: The ChannelPartnerLink sent in the request already exists. * NOT_FOUND: No Cloud Identity customer exists for provided domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The new ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Create a channel partner link for the provided reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_create(&self, request: GoogleCloudChannelV1ChannelPartnerLink, parent: &str) -> AccountChannelPartnerLinkCreateCall<'a, C> {
        AccountChannelPartnerLinkCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested ChannelPartnerLink resource. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: ChannelPartnerLink resource not found because of an invalid channel partner link name. Return value: The ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the channel partner link to retrieve. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    pub fn channel_partner_links_get(&self, name: &str) -> AccountChannelPartnerLinkGetCall<'a, C> {
        AccountChannelPartnerLinkGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List ChannelPartnerLinks belonging to a distributor. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: The list of the distributor account's ChannelPartnerLink resources.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account for listing channel partner links. Parent uses the format: accounts/{account_id}
    pub fn channel_partner_links_list(&self, parent: &str) -> AccountChannelPartnerLinkListCall<'a, C> {
        AccountChannelPartnerLinkListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel partner link. Distributors call this method to change a link's status. For example, to suspend a partner link. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Link state cannot change from invited to active or suspended. * Cannot send reseller_cloud_identity_id, invite_url, or name in update mask. * NOT_FOUND: ChannelPartnerLink resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The updated ChannelPartnerLink resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the channel partner link to cancel. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    pub fn channel_partner_links_patch(&self, request: GoogleCloudChannelV1UpdateChannelPartnerLinkRequest, name: &str) -> AccountChannelPartnerLinkPatchCall<'a, C> {
        AccountChannelPartnerLinkPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a previously suspended entitlement. Entitlements suspended for pending ToS acceptance can't be activated using this method. An entitlement activation is a long-running operation and it updates the state of the customer entitlement. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * SUSPENSION_NOT_RESELLER_INITIATED: Can only activate reseller-initiated suspensions and entitlements that have accepted the TOS. * NOT_SUSPENDED: Can only activate suspended entitlements not in an ACTIVE state. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to activate. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_activate(&self, request: GoogleCloudChannelV1ActivateEntitlementRequest, name: &str) -> AccountCustomerEntitlementActivateCall<'a, C> {
        AccountCustomerEntitlementActivateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a previously fulfilled entitlement. An entitlement cancellation is a long-running operation. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * FAILED_PRECONDITION: There are Google Cloud projects linked to the Google Cloud entitlement's Cloud Billing subaccount. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * DELETION_TYPE_NOT_ALLOWED: Cancel is only allowed for Google Workspace add-ons, or entitlements for Google Cloud's development platform. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to cancel. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_cancel(&self, request: GoogleCloudChannelV1CancelEntitlementRequest, name: &str) -> AccountCustomerEntitlementCancelCall<'a, C> {
        AccountCustomerEntitlementCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Offer for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Offer or Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_offer(&self, request: GoogleCloudChannelV1ChangeOfferRequest, name: &str) -> AccountCustomerEntitlementChangeOfferCall<'a, C> {
        AccountCustomerEntitlementChangeOfferCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Change parameters of the entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. For example, the number of seats being changed is greater than the allowed number of max seats, or decreasing seats for a commitment based plan. * NOT_FOUND: Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_parameters(&self, request: GoogleCloudChannelV1ChangeParametersRequest, name: &str) -> AccountCustomerEntitlementChangeParameterCall<'a, C> {
        AccountCustomerEntitlementChangeParameterCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the renewal settings for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_COMMITMENT_PLAN: Renewal Settings are only applicable for a commitment plan. Can't enable or disable renewals for non-commitment plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_change_renewal_settings(&self, request: GoogleCloudChannelV1ChangeRenewalSettingsRequest, name: &str) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> {
        AccountCustomerEntitlementChangeRenewalSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an entitlement for a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * There is already a customer entitlement for a SKU from the same product family. * INVALID_VALUE: Make sure the OfferId is valid. If it is, contact Google Channel support for further troubleshooting. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: * The SKU was already purchased for the customer. * The customer's primary email already exists. Retry after changing the customer's primary contact email. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The domain required for purchasing a SKU has not been verified. * A pre-requisite SKU required to purchase an Add-On SKU is missing. For example, Google Workspace Business Starter is required to purchase Vault or Drive. * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account in which to create the entitlement. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_entitlements_create(&self, request: GoogleCloudChannelV1CreateEntitlementRequest, parent: &str) -> AccountCustomerEntitlementCreateCall<'a, C> {
        AccountCustomerEntitlementCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested Entitlement resource. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer entitlement was not found. Return value: The requested Entitlement resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the entitlement to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_get(&self, name: &str) -> AccountCustomerEntitlementGetCall<'a, C> {
        AccountCustomerEntitlementGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Entitlements belonging to a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's Entitlements.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller's customer account to list entitlements for. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_entitlements_list(&self, parent: &str) -> AccountCustomerEntitlementListCall<'a, C> {
        AccountCustomerEntitlementListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts paid service for a trial entitlement. Starts paid service for a trial entitlement immediately. This method is only applicable if a plan is set up for a trial entitlement but has some trial days remaining. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * FAILED_PRECONDITION/NOT_IN_TRIAL: This method only works for entitlement on trial plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the entitlement to start a paid service for. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_start_paid_service(&self, request: GoogleCloudChannelV1StartPaidServiceRequest, name: &str) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C> {
        AccountCustomerEntitlementStartPaidServiceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suspends a previously fulfilled entitlement. An entitlement suspension is a long-running operation. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_ACTIVE: Entitlement is not active. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the entitlement to suspend. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    pub fn customers_entitlements_suspend(&self, request: GoogleCloudChannelV1SuspendEntitlementRequest, name: &str) -> AccountCustomerEntitlementSuspendCall<'a, C> {
        AccountCustomerEntitlementSuspendCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    pub fn customers_create(&self, request: GoogleCloudChannelV1Customer, parent: &str) -> AccountCustomerCreateCall<'a, C> {
        AccountCustomerCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the given Customer permanently and irreversibly. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to delete.
    pub fn customers_delete(&self, name: &str) -> AccountCustomerDeleteCall<'a, C> {
        AccountCustomerDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_get(&self, name: &str) -> AccountCustomerGetCall<'a, C> {
        AccountCustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    pub fn customers_list(&self, parent: &str) -> AccountCustomerListCall<'a, C> {
        AccountCustomerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the following: * Offers that you can purchase for a customer. * Offers that you can change for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The resource name of the customer to list Offers for. Format: accounts/{account_id}/customers/{customer_id}.
    pub fn customers_list_purchasable_offers(&self, customer: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        AccountCustomerListPurchasableOfferCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _create_entitlement_purchase_sku: Default::default(),
            _change_offer_purchase_new_sku: Default::default(),
            _change_offer_purchase_entitlement: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the following: * SKUs that you can purchase for a customer * SKUs that you can upgrade or downgrade for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The resource name of the customer to list SKUs for. Format: accounts/{account_id}/customers/{customer_id}.
    pub fn customers_list_purchasable_skus(&self, customer: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        AccountCustomerListPurchasableSkuCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _create_entitlement_purchase_product: Default::default(),
            _change_offer_purchase_entitlement: Default::default(),
            _change_offer_purchase_change_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_patch(&self, request: GoogleCloudChannelV1Customer, name: &str) -> AccountCustomerPatchCall<'a, C> {
        AccountCustomerPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Cloud Identity for the given customer using the customer's information, or the information provided here. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer was not found. * ALREADY_EXISTS: The customer's primary email already exists. Retry after changing the customer's primary contact email. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata contains an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customer` - Required. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_provision_cloud_identity(&self, request: GoogleCloudChannelV1ProvisionCloudIdentityRequest, customer: &str) -> AccountCustomerProvisionCloudIdentityCall<'a, C> {
        AccountCustomerProvisionCloudIdentityCall {
            hub: self.hub,
            _request: request,
            _customer: customer.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Transfers customer entitlements to new reseller. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * Specify all transferring entitlements. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account that will receive transferred entitlements. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_transfer_entitlements(&self, request: GoogleCloudChannelV1TransferEntitlementsRequest, parent: &str) -> AccountCustomerTransferEntitlementCall<'a, C> {
        AccountCustomerTransferEntitlementCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Transfers customer entitlements from their current reseller to Google. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's customer account where the entitlements transfer from. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    pub fn customers_transfer_entitlements_to_google(&self, request: GoogleCloudChannelV1TransferEntitlementsToGoogleRequest, parent: &str) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C> {
        AccountCustomerTransferEntitlementsToGoogleCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Offers the reseller can sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the reseller account from which to list Offers. Parent uses the format: accounts/{account_id}.
    pub fn offers_list(&self, parent: &str) -> AccountOfferListCall<'a, C> {
        AccountOfferListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Confirms the existence of Cloud Identity accounts based on the domain and if the Cloud Identity accounts are owned by the reseller. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INVALID_VALUE: Invalid domain value in the request. Return value: A list of CloudIdentityCustomerAccount resources for the domain (may be empty) Note: in the v1alpha1 version of the API, a NOT_FOUND error returns if no CloudIdentityCustomerAccount resources match the domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn check_cloud_identity_accounts_exist(&self, request: GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest, parent: &str) -> AccountCheckCloudIdentityAccountsExistCall<'a, C> {
        AccountCheckCloudIdentityAccountsExistCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists service accounts with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: A list of service email addresses.
    /// 
    /// # Arguments
    ///
    /// * `account` - Required. Resource name of the account.
    pub fn list_subscribers(&self, account: &str) -> AccountListSubscriberCall<'a, C> {
        AccountListSubscriberCall {
            hub: self.hub,
            _account: account.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List TransferableOffers of a customer based on Cloud Identity ID or Customer Name in the request. Use this method when a reseller gets the entitlement information of an unowned customer. The reseller should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The supplied auth token is invalid. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of TransferableOffer for the given customer and SKU.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the reseller's account.
    pub fn list_transferable_offers(&self, request: GoogleCloudChannelV1ListTransferableOffersRequest, parent: &str) -> AccountListTransferableOfferCall<'a, C> {
        AccountListTransferableOfferCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List TransferableSkus of a customer based on the Cloud Identity ID or Customer Name in the request. Use this method to list the entitlements information of an unowned customer. You should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The supplied auth token is invalid. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's TransferableSku.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    pub fn list_transferable_skus(&self, request: GoogleCloudChannelV1ListTransferableSkusRequest, parent: &str) -> AccountListTransferableSkuCall<'a, C> {
        AccountListTransferableSkuCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers a service account with subscriber privileges on the Cloud Pub/Sub topic for this Channel Services account. After you create a subscriber, you get the events through SubscriberEvent Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name with the registered service email address.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Required. Resource name of the account.
    pub fn register(&self, request: GoogleCloudChannelV1RegisterSubscriberRequest, account: &str) -> AccountRegisterCall<'a, C> {
        AccountRegisterCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unregisters a service account with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. If there are no service accounts left with subscriber privileges, this deletes the topic. You can call ListSubscribers to check for these accounts. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name that unregistered the service email address. Returns a success response if the service email address wasn't registered with the topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Required. Resource name of the account.
    pub fn unregister(&self, request: GoogleCloudChannelV1UnregisterSubscriberRequest, account: &str) -> AccountUnregisterCall<'a, C> {
        AccountUnregisterCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the `Cloudchannel` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use cloudchannel1::Cloudchannel;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
}

impl<'a, C> client::MethodsBuilder for OperationMethods<'a, C> {}

impl<'a, C> OperationMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> OperationCancelCall<'a, C> {
        OperationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, C> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, C> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn list(&self, name: &str) -> OperationListCall<'a, C> {
        OperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *product* resources.
/// It is not used directly, but through the `Cloudchannel` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_cloudchannel1 as cloudchannel1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use cloudchannel1::Cloudchannel;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `skus_list(...)`
/// // to build up your call.
/// let rb = hub.products();
/// # }
/// ```
pub struct ProductMethods<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
}

impl<'a, C> client::MethodsBuilder for ProductMethods<'a, C> {}

impl<'a, C> ProductMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the SKUs for a product the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the Product to list SKUs for. Parent uses the format: products/{product_id}. Supports products/- to retrieve SKUs for all products.
    pub fn skus_list(&self, parent: &str) -> ProductSkuListCall<'a, C> {
        ProductSkuListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _account: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Products the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
    pub fn list(&self) -> ProductListCall<'a, C> {
        ProductListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _account: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
///
/// A builder for the *channelPartnerLinks.customers.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1Customer;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1Customer::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_customers_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCustomerCreateCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1Customer,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCustomerCreateCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCustomerCreateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.customers.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/customers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1Customer) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCustomerCreateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the given Customer permanently and irreversibly. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
///
/// A builder for the *channelPartnerLinks.customers.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_customers_delete("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCustomerDeleteCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCustomerDeleteCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCustomerDeleteCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.customers.delete",
                               http_method: hyper::Method::DELETE });
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to delete.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCustomerDeleteCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
///
/// A builder for the *channelPartnerLinks.customers.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_customers_get("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCustomerGetCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCustomerGetCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCustomerGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.customers.get",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCustomerGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCustomerGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCustomerGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
///
/// A builder for the *channelPartnerLinks.customers.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_customers_list("parent")
///              .page_token("sed")
///              .page_size(-2)
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCustomerListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCustomerListCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCustomerListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListCustomersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.customers.list",
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

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/customers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A token identifying a page of results other than the first page. Obtained through ListCustomersResponse.next_page_token of the previous CloudChannelService.ListCustomers call.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of customers to return. The service may return fewer than this value. If unspecified, returns at most 10 customers. The maximum value is 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountChannelPartnerLinkCustomerListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCustomerListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCustomerListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCustomerListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
///
/// A builder for the *channelPartnerLinks.customers.patch* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1Customer;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1Customer::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_customers_patch(req, "name")
///              .update_mask("amet.")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCustomerPatchCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1Customer,
    _name: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCustomerPatchCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCustomerPatchCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.customers.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "name", "updateMask"].iter() {
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1Customer) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The update mask that applies to the resource. Optional.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCustomerPatchCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Initiates a channel partner link between a distributor and a reseller, or between resellers in an n-tier reseller channel. Invited partners need to follow the invite_link_uri provided in the response to accept. After accepting the invitation, a link is set up between the two parties. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * ALREADY_EXISTS: The ChannelPartnerLink sent in the request already exists. * NOT_FOUND: No Cloud Identity customer exists for provided domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The new ChannelPartnerLink resource.
///
/// A builder for the *channelPartnerLinks.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ChannelPartnerLink;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ChannelPartnerLink::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkCreateCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ChannelPartnerLink,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkCreateCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkCreateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ChannelPartnerLink)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/channelPartnerLinks";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ChannelPartnerLink) -> AccountChannelPartnerLinkCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Create a channel partner link for the provided reseller account's resource name. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountChannelPartnerLinkCreateCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkCreateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkCreateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a requested ChannelPartnerLink resource. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: ChannelPartnerLink resource not found because of an invalid channel partner link name. Return value: The ChannelPartnerLink resource.
///
/// A builder for the *channelPartnerLinks.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_get("name")
///              .view("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkGetCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _view: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkGetCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ChannelPartnerLink)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        for &field in ["alt", "name", "view"].iter() {
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the channel partner link to retrieve. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountChannelPartnerLinkGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> AccountChannelPartnerLinkGetCall<'a, C> {
        self._view = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List ChannelPartnerLinks belonging to a distributor. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: The list of the distributor account's ChannelPartnerLink resources.
///
/// A builder for the *channelPartnerLinks.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_list("parent")
///              .view("gubergren")
///              .page_token("eos")
///              .page_size(-4)
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _view: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkListCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListChannelPartnerLinksResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._view {
            params.push(("view", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "view", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/channelPartnerLinks";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the reseller account for listing channel partner links. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountChannelPartnerLinkListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. The level of granularity the ChannelPartnerLink will display.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> AccountChannelPartnerLinkListCall<'a, C> {
        self._view = Some(new_value.to_string());
        self
    }
    /// Optional. A token for a page of results other than the first page. Obtained using ListChannelPartnerLinksResponse.next_page_token of the previous CloudChannelService.ListChannelPartnerLinks call.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountChannelPartnerLinkListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, server will pick a default size (25). The maximum value is 200; the server will coerce values above 200.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountChannelPartnerLinkListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a channel partner link. Distributors call this method to change a link's status. For example, to suspend a partner link. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Link state cannot change from invited to active or suspended. * Cannot send reseller_cloud_identity_id, invite_url, or name in update mask. * NOT_FOUND: ChannelPartnerLink resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The updated ChannelPartnerLink resource.
///
/// A builder for the *channelPartnerLinks.patch* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1UpdateChannelPartnerLinkRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1UpdateChannelPartnerLinkRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().channel_partner_links_patch(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountChannelPartnerLinkPatchCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1UpdateChannelPartnerLinkRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountChannelPartnerLinkPatchCall<'a, C> {}

impl<'a, C> AccountChannelPartnerLinkPatchCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ChannelPartnerLink)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.channelPartnerLinks.patch",
                               http_method: hyper::Method::PATCH });
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1UpdateChannelPartnerLinkRequest) -> AccountChannelPartnerLinkPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the channel partner link to cancel. Name uses the format: accounts/{account_id}/channelPartnerLinks/{id} where {id} is the Cloud Identity ID of the partner.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountChannelPartnerLinkPatchCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountChannelPartnerLinkPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountChannelPartnerLinkPatchCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountChannelPartnerLinkPatchCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Activates a previously suspended entitlement. Entitlements suspended for pending ToS acceptance can't be activated using this method. An entitlement activation is a long-running operation and it updates the state of the customer entitlement. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * SUSPENSION_NOT_RESELLER_INITIATED: Can only activate reseller-initiated suspensions and entitlements that have accepted the TOS. * NOT_SUSPENDED: Can only activate suspended entitlements not in an ACTIVE state. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.activate* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ActivateEntitlementRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ActivateEntitlementRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_activate(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementActivateCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ActivateEntitlementRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementActivateCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementActivateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.activate",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:activate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ActivateEntitlementRequest) -> AccountCustomerEntitlementActivateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the entitlement to activate. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementActivateCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementActivateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementActivateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementActivateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Cancels a previously fulfilled entitlement. An entitlement cancellation is a long-running operation. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * FAILED_PRECONDITION: There are Google Cloud projects linked to the Google Cloud entitlement's Cloud Billing subaccount. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * DELETION_TYPE_NOT_ALLOWED: Cancel is only allowed for Google Workspace add-ons, or entitlements for Google Cloud's development platform. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.cancel* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1CancelEntitlementRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1CancelEntitlementRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_cancel(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementCancelCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1CancelEntitlementRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementCancelCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementCancelCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.cancel",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1CancelEntitlementRequest) -> AccountCustomerEntitlementCancelCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the entitlement to cancel. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementCancelCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementCancelCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementCancelCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementCancelCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the Offer for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Offer or Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.changeOffer* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ChangeOfferRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ChangeOfferRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_change_offer(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementChangeOfferCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ChangeOfferRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementChangeOfferCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementChangeOfferCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.changeOffer",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:changeOffer";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ChangeOfferRequest) -> AccountCustomerEntitlementChangeOfferCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementChangeOfferCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementChangeOfferCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementChangeOfferCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementChangeOfferCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Change parameters of the entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. For example, the number of seats being changed is greater than the allowed number of max seats, or decreasing seats for a commitment based plan. * NOT_FOUND: Entitlement resource not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.changeParameters* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ChangeParametersRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ChangeParametersRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_change_parameters(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementChangeParameterCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ChangeParametersRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementChangeParameterCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementChangeParameterCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.changeParameters",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:changeParameters";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ChangeParametersRequest) -> AccountCustomerEntitlementChangeParameterCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementChangeParameterCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementChangeParameterCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementChangeParameterCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementChangeParameterCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the renewal settings for an existing customer entitlement. An entitlement update is a long-running operation and it updates the entitlement as a result of fulfillment. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_COMMITMENT_PLAN: Renewal Settings are only applicable for a commitment plan. Can't enable or disable renewals for non-commitment plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.changeRenewalSettings* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ChangeRenewalSettingsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ChangeRenewalSettingsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_change_renewal_settings(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementChangeRenewalSettingCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ChangeRenewalSettingsRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.changeRenewalSettings",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:changeRenewalSettings";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ChangeRenewalSettingsRequest) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The name of the entitlement to update. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementChangeRenewalSettingCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates an entitlement for a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * There is already a customer entitlement for a SKU from the same product family. * INVALID_VALUE: Make sure the OfferId is valid. If it is, contact Google Channel support for further troubleshooting. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: * The SKU was already purchased for the customer. * The customer's primary email already exists. Retry after changing the customer's primary contact email. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The domain required for purchasing a SKU has not been verified. * A pre-requisite SKU required to purchase an Add-On SKU is missing. For example, Google Workspace Business Starter is required to purchase Vault or Drive. * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1CreateEntitlementRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1CreateEntitlementRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementCreateCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1CreateEntitlementRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementCreateCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementCreateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/entitlements";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1CreateEntitlementRequest) -> AccountCustomerEntitlementCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the reseller's customer account in which to create the entitlement. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerEntitlementCreateCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementCreateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementCreateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a requested Entitlement resource. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer entitlement was not found. Return value: The requested Entitlement resource.
///
/// A builder for the *customers.entitlements.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_get("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementGetCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementGetCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Entitlement)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.get",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the entitlement to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists Entitlements belonging to a customer. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's Entitlements.
///
/// A builder for the *customers.entitlements.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_list("parent")
///              .page_token("rebum.")
///              .page_size(-57)
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementListCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListEntitlementsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.list",
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

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/entitlements";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the reseller's customer account to list entitlements for. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerEntitlementListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A token for a page of results other than the first page. Obtained using ListEntitlementsResponse.next_page_token of the previous CloudChannelService.ListEntitlements call.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountCustomerEntitlementListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, return at most 50 entitlements. The maximum value is 100; the server will coerce values above 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountCustomerEntitlementListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Starts paid service for a trial entitlement. Starts paid service for a trial entitlement immediately. This method is only applicable if a plan is set up for a trial entitlement but has some trial days remaining. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * FAILED_PRECONDITION/NOT_IN_TRIAL: This method only works for entitlement on trial plans. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.startPaidService* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1StartPaidServiceRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1StartPaidServiceRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_start_paid_service(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementStartPaidServiceCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1StartPaidServiceRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementStartPaidServiceCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementStartPaidServiceCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.startPaidService",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:startPaidService";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1StartPaidServiceRequest) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The name of the entitlement to start a paid service for. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementStartPaidServiceCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Suspends a previously fulfilled entitlement. An entitlement suspension is a long-running operation. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: Entitlement resource not found. * NOT_ACTIVE: Entitlement is not active. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.entitlements.suspend* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1SuspendEntitlementRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1SuspendEntitlementRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_entitlements_suspend(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerEntitlementSuspendCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1SuspendEntitlementRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerEntitlementSuspendCall<'a, C> {}

impl<'a, C> AccountCustomerEntitlementSuspendCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.entitlements.suspend",
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

        let mut url = self.hub._base_url.clone() + "v1/{+name}:suspend";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1SuspendEntitlementRequest) -> AccountCustomerEntitlementSuspendCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the entitlement to suspend. Name uses the format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerEntitlementSuspendCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerEntitlementSuspendCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerEntitlementSuspendCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerEntitlementSuspendCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.
///
/// A builder for the *customers.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1Customer;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1Customer::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerCreateCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1Customer,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerCreateCall<'a, C> {}

impl<'a, C> AccountCustomerCreateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/customers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1Customer) -> AccountCustomerCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerCreateCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerCreateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerCreateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the given Customer permanently and irreversibly. Possible error codes: * PERMISSION_DENIED: The account making the request does not own this customer. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * FAILED_PRECONDITION: The customer has existing entitlements. * NOT_FOUND: No Customer resource found for the name in the request.
///
/// A builder for the *customers.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_delete("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerDeleteCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerDeleteCall<'a, C> {}

impl<'a, C> AccountCustomerDeleteCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.delete",
                               http_method: hyper::Method::DELETE });
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to delete.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerDeleteCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerDeleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerDeleteCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerDeleteCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a requested Customer resource. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer resource doesn't exist. Usually the result of an invalid name parameter. Return value: The Customer resource.
///
/// A builder for the *customers.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_get("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerGetCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerGetCall<'a, C> {}

impl<'a, C> AccountCustomerGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.get",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to retrieve. Name uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List Customers. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of Customers, or an empty list if there are no customers.
///
/// A builder for the *customers.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_list("parent")
///              .page_token("Lorem")
///              .page_size(-25)
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerListCall<'a, C> {}

impl<'a, C> AccountCustomerListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListCustomersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.list",
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

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/customers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the reseller account to list customers from. Parent uses the format: accounts/{account_id}.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A token identifying a page of results other than the first page. Obtained through ListCustomersResponse.next_page_token of the previous CloudChannelService.ListCustomers call.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountCustomerListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of customers to return. The service may return fewer than this value. If unspecified, returns at most 10 customers. The maximum value is 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountCustomerListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the following: * Offers that you can purchase for a customer. * Offers that you can change for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller * INVALID_ARGUMENT: Required request parameters are missing or invalid.
///
/// A builder for the *customers.listPurchasableOffers* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_list_purchasable_offers("customer")
///              .page_token("sed")
///              .page_size(-70)
///              .language_code("sed")
///              .create_entitlement_purchase_sku("no")
///              .change_offer_purchase_new_sku("Stet")
///              .change_offer_purchase_entitlement("kasd")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerListPurchasableOfferCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _language_code: Option<String>,
    _create_entitlement_purchase_sku: Option<String>,
    _change_offer_purchase_new_sku: Option<String>,
    _change_offer_purchase_entitlement: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerListPurchasableOfferCall<'a, C> {}

impl<'a, C> AccountCustomerListPurchasableOfferCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListPurchasableOffersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.listPurchasableOffers",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._create_entitlement_purchase_sku {
            params.push(("createEntitlementPurchase.sku", value.to_string()));
        }
        if let Some(value) = self._change_offer_purchase_new_sku {
            params.push(("changeOfferPurchase.newSku", value.to_string()));
        }
        if let Some(value) = self._change_offer_purchase_entitlement {
            params.push(("changeOfferPurchase.entitlement", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "languageCode", "createEntitlementPurchase.sku", "changeOfferPurchase.newSku", "changeOfferPurchase.entitlement"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}:listPurchasableOffers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
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
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to list Offers for. Format: accounts/{account_id}/customers/{customer_id}.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// Optional. A token for a page of results other than the first page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 Offers. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Required. SKU that the result should be restricted to. Format: products/{product_id}/skus/{sku_id}.
    ///
    /// Sets the *create entitlement purchase.sku* query property to the given value.
    pub fn create_entitlement_purchase_sku(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._create_entitlement_purchase_sku = Some(new_value.to_string());
        self
    }
    /// Optional. Resource name of the new target SKU. Provide this SKU when upgrading or downgrading an entitlement. Format: products/{product_id}/skus/{sku_id}
    ///
    /// Sets the *change offer purchase.new sku* query property to the given value.
    pub fn change_offer_purchase_new_sku(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._change_offer_purchase_new_sku = Some(new_value.to_string());
        self
    }
    /// Required. Resource name of the entitlement. Format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *change offer purchase.entitlement* query property to the given value.
    pub fn change_offer_purchase_entitlement(mut self, new_value: &str) -> AccountCustomerListPurchasableOfferCall<'a, C> {
        self._change_offer_purchase_entitlement = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerListPurchasableOfferCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerListPurchasableOfferCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerListPurchasableOfferCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the following: * SKUs that you can purchase for a customer * SKUs that you can upgrade or downgrade for an entitlement. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid.
///
/// A builder for the *customers.listPurchasableSkus* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_list_purchasable_skus("customer")
///              .page_token("sed")
///              .page_size(-24)
///              .language_code("et")
///              .create_entitlement_purchase_product("vero")
///              .change_offer_purchase_entitlement("erat")
///              .change_offer_purchase_change_type("sed")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerListPurchasableSkuCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _language_code: Option<String>,
    _create_entitlement_purchase_product: Option<String>,
    _change_offer_purchase_entitlement: Option<String>,
    _change_offer_purchase_change_type: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerListPurchasableSkuCall<'a, C> {}

impl<'a, C> AccountCustomerListPurchasableSkuCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListPurchasableSkusResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.listPurchasableSkus",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._create_entitlement_purchase_product {
            params.push(("createEntitlementPurchase.product", value.to_string()));
        }
        if let Some(value) = self._change_offer_purchase_entitlement {
            params.push(("changeOfferPurchase.entitlement", value.to_string()));
        }
        if let Some(value) = self._change_offer_purchase_change_type {
            params.push(("changeOfferPurchase.changeType", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "languageCode", "createEntitlementPurchase.product", "changeOfferPurchase.entitlement", "changeOfferPurchase.changeType"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}:listPurchasableSkus";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
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
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the customer to list SKUs for. Format: accounts/{account_id}/customers/{customer_id}.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// Optional. A token for a page of results other than the first page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 SKUs. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Required. List SKUs belonging to this Product. Format: products/{product_id}. Supports products/- to retrieve SKUs for all products.
    ///
    /// Sets the *create entitlement purchase.product* query property to the given value.
    pub fn create_entitlement_purchase_product(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._create_entitlement_purchase_product = Some(new_value.to_string());
        self
    }
    /// Required. Resource name of the entitlement. Format: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}
    ///
    /// Sets the *change offer purchase.entitlement* query property to the given value.
    pub fn change_offer_purchase_entitlement(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._change_offer_purchase_entitlement = Some(new_value.to_string());
        self
    }
    /// Required. Change Type for the entitlement.
    ///
    /// Sets the *change offer purchase.change type* query property to the given value.
    pub fn change_offer_purchase_change_type(mut self, new_value: &str) -> AccountCustomerListPurchasableSkuCall<'a, C> {
        self._change_offer_purchase_change_type = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerListPurchasableSkuCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerListPurchasableSkuCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerListPurchasableSkuCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an existing Customer resource for the reseller or distributor. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: No Customer resource found for the name in the request. Return value: The updated Customer resource.
///
/// A builder for the *customers.patch* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1Customer;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1Customer::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_patch(req, "name")
///              .update_mask("dolore")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerPatchCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1Customer,
    _name: String,
    _update_mask: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerPatchCall<'a, C> {}

impl<'a, C> AccountCustomerPatchCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1Customer)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._update_mask {
            params.push(("updateMask", value.to_string()));
        }
        for &field in ["alt", "name", "updateMask"].iter() {
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1Customer) -> AccountCustomerPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountCustomerPatchCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The update mask that applies to the resource. Optional.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: &str) -> AccountCustomerPatchCall<'a, C> {
        self._update_mask = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerPatchCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerPatchCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a Cloud Identity for the given customer using the customer's information, or the information provided here. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer was not found. * ALREADY_EXISTS: The customer's primary email already exists. Retry after changing the customer's primary contact email. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata contains an instance of OperationMetadata.
///
/// A builder for the *customers.provisionCloudIdentity* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ProvisionCloudIdentityRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ProvisionCloudIdentityRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_provision_cloud_identity(req, "customer")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerProvisionCloudIdentityCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ProvisionCloudIdentityRequest,
    _customer: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerProvisionCloudIdentityCall<'a, C> {}

impl<'a, C> AccountCustomerProvisionCloudIdentityCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.provisionCloudIdentity",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        for &field in ["alt", "customer"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}:provisionCloudIdentity";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
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
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ProvisionCloudIdentityRequest) -> AccountCustomerProvisionCloudIdentityCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> AccountCustomerProvisionCloudIdentityCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerProvisionCloudIdentityCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerProvisionCloudIdentityCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerProvisionCloudIdentityCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Transfers customer entitlements to new reseller. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * Specify all transferring entitlements. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.transferEntitlements* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1TransferEntitlementsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1TransferEntitlementsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_transfer_entitlements(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerTransferEntitlementCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1TransferEntitlementsRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerTransferEntitlementCall<'a, C> {}

impl<'a, C> AccountCustomerTransferEntitlementCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.transferEntitlements",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:transferEntitlements";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1TransferEntitlementsRequest) -> AccountCustomerTransferEntitlementCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the reseller's customer account that will receive transferred entitlements. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerTransferEntitlementCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerTransferEntitlementCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerTransferEntitlementCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerTransferEntitlementCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Transfers customer entitlements from their current reseller to Google. Possible error codes: * PERMISSION_DENIED: The customer doesn't belong to the reseller. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: The SKU was already transferred for the customer. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The SKU requires domain verification to transfer, but the domain is not verified. * An Add-On SKU (example, Vault or Drive) is missing the pre-requisite SKU (example, G Suite Basic). * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The response will contain google.protobuf.Empty on success. The Operation metadata will contain an instance of OperationMetadata.
///
/// A builder for the *customers.transferEntitlementsToGoogle* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1TransferEntitlementsToGoogleRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1TransferEntitlementsToGoogleRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().customers_transfer_entitlements_to_google(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountCustomerTransferEntitlementsToGoogleCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1TransferEntitlementsToGoogleRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomerTransferEntitlementsToGoogleCall<'a, C> {}

impl<'a, C> AccountCustomerTransferEntitlementsToGoogleCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.customers.transferEntitlementsToGoogle",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:transferEntitlementsToGoogle";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1TransferEntitlementsToGoogleRequest) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the reseller's customer account where the entitlements transfer from. Parent uses the format: accounts/{account_id}/customers/{customer_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomerTransferEntitlementsToGoogleCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the Offers the reseller can sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
///
/// A builder for the *offers.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().offers_list("parent")
///              .page_token("diam")
///              .page_size(-49)
///              .language_code("et")
///              .filter("et")
///              .doit().await;
/// # }
/// ```
pub struct AccountOfferListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _language_code: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountOfferListCall<'a, C> {}

impl<'a, C> AccountOfferListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListOffersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.offers.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize", "languageCode", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/offers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the reseller account from which to list Offers. Parent uses the format: accounts/{account_id}.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountOfferListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A token for a page of results other than the first page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountOfferListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, returns at most 500 Offers. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountOfferListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> AccountOfferListCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Optional. The expression to filter results by name (name of the Offer), sku.name (name of the SKU), or sku.product.name (name of the Product). Example 1: sku.product.name=products/p1 AND sku.name!=products/p1/skus/s1 Example 2: name=accounts/a1/offers/o1
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> AccountOfferListCall<'a, C> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountOfferListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountOfferListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountOfferListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Confirms the existence of Cloud Identity accounts based on the domain and if the Cloud Identity accounts are owned by the reseller. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INVALID_VALUE: Invalid domain value in the request. Return value: A list of CloudIdentityCustomerAccount resources for the domain (may be empty) Note: in the v1alpha1 version of the API, a NOT_FOUND error returns if no CloudIdentityCustomerAccount resources match the domain.
///
/// A builder for the *checkCloudIdentityAccountsExist* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().check_cloud_identity_accounts_exist(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountCheckCloudIdentityAccountsExistCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCheckCloudIdentityAccountsExistCall<'a, C> {}

impl<'a, C> AccountCheckCloudIdentityAccountsExistCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1CheckCloudIdentityAccountsExistResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.checkCloudIdentityAccountsExist",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:checkCloudIdentityAccountsExist";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1CheckCloudIdentityAccountsExistRequest) -> AccountCheckCloudIdentityAccountsExistCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountCheckCloudIdentityAccountsExistCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCheckCloudIdentityAccountsExistCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCheckCloudIdentityAccountsExistCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCheckCloudIdentityAccountsExistCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists service accounts with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: A list of service email addresses.
///
/// A builder for the *listSubscribers* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().list_subscribers("account")
///              .page_token("dolor")
///              .page_size(-20)
///              .doit().await;
/// # }
/// ```
pub struct AccountListSubscriberCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _account: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountListSubscriberCall<'a, C> {}

impl<'a, C> AccountListSubscriberCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListSubscribersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.listSubscribers",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("account", self._account.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "account", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+account}:listSubscribers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+account}", "account")].iter() {
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
            for param_name in ["account"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. Resource name of the account.
    ///
    /// Sets the *account* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account(mut self, new_value: &str) -> AccountListSubscriberCall<'a, C> {
        self._account = new_value.to_string();
        self
    }
    /// Optional. A page token, received from a previous `ListSubscribers` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSubscribers` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountListSubscriberCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of service accounts to return. The service may return fewer than this value. If unspecified, returns at most 100 service accounts. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountListSubscriberCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountListSubscriberCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountListSubscriberCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountListSubscriberCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List TransferableOffers of a customer based on Cloud Identity ID or Customer Name in the request. Use this method when a reseller gets the entitlement information of an unowned customer. The reseller should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The supplied auth token is invalid. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of TransferableOffer for the given customer and SKU.
///
/// A builder for the *listTransferableOffers* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ListTransferableOffersRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ListTransferableOffersRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().list_transferable_offers(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountListTransferableOfferCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ListTransferableOffersRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountListTransferableOfferCall<'a, C> {}

impl<'a, C> AccountListTransferableOfferCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListTransferableOffersResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.listTransferableOffers",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:listTransferableOffers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ListTransferableOffersRequest) -> AccountListTransferableOfferCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name of the reseller's account.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountListTransferableOfferCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountListTransferableOfferCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountListTransferableOfferCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountListTransferableOfferCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// List TransferableSkus of a customer based on the Cloud Identity ID or Customer Name in the request. Use this method to list the entitlements information of an unowned customer. You should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The supplied auth token is invalid. * The reseller account making the request is different from the reseller account in the query. * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: A list of the customer's TransferableSku.
///
/// A builder for the *listTransferableSkus* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1ListTransferableSkusRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1ListTransferableSkusRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().list_transferable_skus(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct AccountListTransferableSkuCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1ListTransferableSkusRequest,
    _parent: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountListTransferableSkuCall<'a, C> {}

impl<'a, C> AccountListTransferableSkuCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListTransferableSkusResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.listTransferableSkus",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}:listTransferableSkus";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1ListTransferableSkusRequest) -> AccountListTransferableSkuCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The reseller account's resource name. Parent uses the format: accounts/{account_id}
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountListTransferableSkuCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountListTransferableSkuCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountListTransferableSkuCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountListTransferableSkuCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Registers a service account with subscriber privileges on the Cloud Pub/Sub topic for this Channel Services account. After you create a subscriber, you get the events through SubscriberEvent Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name with the registered service email address.
///
/// A builder for the *register* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1RegisterSubscriberRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1RegisterSubscriberRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().register(req, "account")
///              .doit().await;
/// # }
/// ```
pub struct AccountRegisterCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1RegisterSubscriberRequest,
    _account: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountRegisterCall<'a, C> {}

impl<'a, C> AccountRegisterCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1RegisterSubscriberResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.register",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("account", self._account.to_string()));
        for &field in ["alt", "account"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+account}:register";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+account}", "account")].iter() {
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
            for param_name in ["account"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1RegisterSubscriberRequest) -> AccountRegisterCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Resource name of the account.
    ///
    /// Sets the *account* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account(mut self, new_value: &str) -> AccountRegisterCall<'a, C> {
        self._account = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountRegisterCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountRegisterCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountRegisterCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Unregisters a service account with subscriber privileges on the Cloud Pub/Sub topic created for this Channel Services account. If there are no service accounts left with subscriber privileges, this deletes the topic. You can call ListSubscribers to check for these accounts. Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The topic resource doesn't exist. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name that unregistered the service email address. Returns a success response if the service email address wasn't registered with the topic.
///
/// A builder for the *unregister* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleCloudChannelV1UnregisterSubscriberRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleCloudChannelV1UnregisterSubscriberRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().unregister(req, "account")
///              .doit().await;
/// # }
/// ```
pub struct AccountUnregisterCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleCloudChannelV1UnregisterSubscriberRequest,
    _account: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountUnregisterCall<'a, C> {}

impl<'a, C> AccountUnregisterCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1UnregisterSubscriberResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.accounts.unregister",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("account", self._account.to_string()));
        for &field in ["alt", "account"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+account}:unregister";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+account}", "account")].iter() {
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
            for param_name in ["account"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleCloudChannelV1UnregisterSubscriberRequest) -> AccountUnregisterCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Resource name of the account.
    ///
    /// Sets the *account* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account(mut self, new_value: &str) -> AccountUnregisterCall<'a, C> {
        self._account = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountUnregisterCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountUnregisterCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountUnregisterCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
///
/// A builder for the *cancel* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// use cloudchannel1::api::GoogleLongrunningCancelOperationRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GoogleLongrunningCancelOperationRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().cancel(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct OperationCancelCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _request: GoogleLongrunningCancelOperationRequest,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for OperationCancelCall<'a, C> {}

impl<'a, C> OperationCancelCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.operations.cancel",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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
    pub fn request(mut self, new_value: GoogleLongrunningCancelOperationRequest) -> OperationCancelCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The name of the operation resource to be cancelled.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationCancelCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OperationCancelCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationCancelCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationCancelCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
///
/// A builder for the *delete* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().delete("name")
///              .doit().await;
/// # }
/// ```
pub struct OperationDeleteCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for OperationDeleteCall<'a, C> {}

impl<'a, C> OperationDeleteCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleProtobufEmpty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.operations.delete",
                               http_method: hyper::Method::DELETE });
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The name of the operation resource to be deleted.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationDeleteCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OperationDeleteCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationDeleteCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationDeleteCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
///
/// A builder for the *get* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().get("name")
///              .doit().await;
/// # }
/// ```
pub struct OperationGetCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for OperationGetCall<'a, C> {}

impl<'a, C> OperationGetCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningOperation)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.operations.get",
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The name of the operation resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationGetCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OperationGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationGetCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationGetCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
///
/// A builder for the *list* method supported by a *operation* resource.
/// It is not used directly, but through a `OperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.operations().list("name")
///              .page_token("no")
///              .page_size(-100)
///              .filter("accusam")
///              .doit().await;
/// # }
/// ```
pub struct OperationListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for OperationListCall<'a, C> {}

impl<'a, C> OperationListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleLongrunningListOperationsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.operations.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "filter"].iter() {
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
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// The name of the operation's parent resource.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> OperationListCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// The standard list page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OperationListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The standard list page size.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OperationListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The standard list filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> OperationListCall<'a, C> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OperationListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> OperationListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> OperationListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the SKUs for a product the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
///
/// A builder for the *skus.list* method supported by a *product* resource.
/// It is not used directly, but through a `ProductMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.products().skus_list("parent")
///              .page_token("consetetur")
///              .page_size(-28)
///              .language_code("et")
///              .account("erat")
///              .doit().await;
/// # }
/// ```
pub struct ProductSkuListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _language_code: Option<String>,
    _account: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for ProductSkuListCall<'a, C> {}

impl<'a, C> ProductSkuListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListSkusResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.products.skus.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._account {
            params.push(("account", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize", "languageCode", "account"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/skus";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Required. The resource name of the Product to list SKUs for. Parent uses the format: products/{product_id}. Supports products/- to retrieve SKUs for all products.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProductSkuListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A token for a page of results other than the first page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProductSkuListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 SKUs. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProductSkuListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> ProductSkuListCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Required. Resource name of the reseller. Format: accounts/{account_id}.
    ///
    /// Sets the *account* query property to the given value.
    pub fn account(mut self, new_value: &str) -> ProductSkuListCall<'a, C> {
        self._account = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProductSkuListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProductSkuListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProductSkuListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Lists the Products the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.
///
/// A builder for the *list* method supported by a *product* resource.
/// It is not used directly, but through a `ProductMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_cloudchannel1 as cloudchannel1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use cloudchannel1::Cloudchannel;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Cloudchannel::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.products().list()
///              .page_token("consetetur")
///              .page_size(-2)
///              .language_code("sed")
///              .account("takimata")
///              .doit().await;
/// # }
/// ```
pub struct ProductListCall<'a, C>
    where C: 'a {

    hub: &'a Cloudchannel<C>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _language_code: Option<String>,
    _account: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for ProductListCall<'a, C> {}

impl<'a, C> ProductListCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleCloudChannelV1ListProductsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "cloudchannel.products.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._language_code {
            params.push(("languageCode", value.to_string()));
        }
        if let Some(value) = self._account {
            params.push(("account", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize", "languageCode", "account"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/products";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AppOrder.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
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


    /// Optional. A token for a page of results other than the first page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProductListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 Products. The maximum value is 1000; the server will coerce values above 1000.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProductListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US".
    ///
    /// Sets the *language code* query property to the given value.
    pub fn language_code(mut self, new_value: &str) -> ProductListCall<'a, C> {
        self._language_code = Some(new_value.to_string());
        self
    }
    /// Required. The resource name of the reseller account. Format: accounts/{account_id}.
    ///
    /// Sets the *account* query property to the given value.
    pub fn account(mut self, new_value: &str) -> ProductListCall<'a, C> {
        self._account = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProductListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProductListCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AppOrder`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProductListCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


