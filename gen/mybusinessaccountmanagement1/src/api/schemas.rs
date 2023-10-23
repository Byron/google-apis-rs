use super::*;
/// Request message for AccessControl.AcceptInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations accept accounts](AccountInvitationAcceptCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AcceptInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for AcceptInvitationRequest {}


/// An account is a container for your location. If you are the only user who manages locations for your business, you can use your personal Google Account. To share management of locations with multiple users, \[create a business account\] (https://support.google.com/business/answer/6085339?ref_topic=6085325).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins create accounts](AccountAdminCreateCall) (none)
/// * [admins delete accounts](AccountAdminDeleteCall) (none)
/// * [admins list accounts](AccountAdminListCall) (none)
/// * [admins patch accounts](AccountAdminPatchCall) (none)
/// * [invitations accept accounts](AccountInvitationAcceptCall) (none)
/// * [invitations decline accounts](AccountInvitationDeclineCall) (none)
/// * [invitations list accounts](AccountInvitationListCall) (none)
/// * [create accounts](AccountCreateCall) (request|response)
/// * [get accounts](AccountGetCall) (response)
/// * [list accounts](AccountListCall) (none)
/// * [patch accounts](AccountPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Required. The name of the account. For an account of type `PERSONAL`, this is the first and last name of the user account.
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// Output only. Account reference number if provisioned.
    #[serde(rename="accountNumber")]
    
    pub account_number: Option<String>,
    /// Immutable. The resource name, in the format `accounts/{account_id}`.
    
    pub name: Option<String>,
    /// Output only. Additional info for an organization. This is populated only for an organization account.
    #[serde(rename="organizationInfo")]
    
    pub organization_info: Option<OrganizationInfo>,
    /// Output only. Specifies the permission level the user has for this account.
    #[serde(rename="permissionLevel")]
    
    pub permission_level: Option<AccountPermissionLevelEnum>,
    /// Required. Input only. The resource name of the account which will be the primary owner of the account being created. It should be of the form `accounts/{account_id}`.
    #[serde(rename="primaryOwner")]
    
    pub primary_owner: Option<String>,
    /// Output only. Specifies the AccountRole of this account.
    
    pub role: Option<AccountRoleEnum>,
    /// Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API.
    #[serde(rename="type")]
    
    pub type_: Option<AccountTypeEnum>,
    /// Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation.
    #[serde(rename="verificationState")]
    
    pub verification_state: Option<AccountVerificationStateEnum>,
    /// Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method.
    #[serde(rename="vettedState")]
    
    pub vetted_state: Option<AccountVettedStateEnum>,
}

impl client::RequestValue for Account {}
impl client::Resource for Account {}
impl client::ResponseResult for Account {}


/// An administrator of an Account or a location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins create accounts](AccountAdminCreateCall) (request|response)
/// * [admins patch accounts](AccountAdminPatchCall) (request|response)
/// * [admins create locations](LocationAdminCreateCall) (request|response)
/// * [admins patch locations](LocationAdminPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Admin {
    /// Immutable. The name of the Account resource that this Admin refers to. Used when calling locations.admins.create to invite a LocationGroup as an admin. If both this field and `admin` are set on `CREATE` requests, this field takes precedence and the email address in `admin` will be ignored. Format: `accounts/{account}`.
    
    pub account: Option<String>,
    /// Optional. The name of the admin. When making the initial invitation, this is the invitee's email address. On `GET` calls, the user's email address is returned if the invitation is still pending. Otherwise, it contains the user's first and last names. This field is only needed to be set during admin creation.
    
    pub admin: Option<String>,
    /// Immutable. The resource name. For account admins, this is in the form: `accounts/{account_id}/admins/{admin_id}` For location admins, this is in the form: `locations/{location_id}/admins/{admin_id}` This field will be ignored if set during admin creation.
    
    pub name: Option<String>,
    /// Output only. Indicates whether this admin has a pending invitation for the specified resource.
    #[serde(rename="pendingInvitation")]
    
    pub pending_invitation: Option<bool>,
    /// Required. Specifies the role that this admin uses with the specified Account or Location.
    
    pub role: Option<AdminRoleEnum>,
}

impl client::RequestValue for Admin {}
impl client::ResponseResult for Admin {}


/// Request message for AccessControl.DeclineInvitation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations decline accounts](AccountInvitationDeclineCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeclineInvitationRequest { _never_set: Option<bool> }

impl client::RequestValue for DeclineInvitationRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins delete accounts](AccountAdminDeleteCall) (response)
/// * [invitations accept accounts](AccountInvitationAcceptCall) (response)
/// * [invitations decline accounts](AccountInvitationDeclineCall) (response)
/// * [admins delete locations](LocationAdminDeleteCall) (response)
/// * [transfer locations](LocationTransferCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Represents a pending invitation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Invitation {
    /// Required. The resource name for the invitation. `accounts/{account_id}/invitations/{invitation_id}`.
    
    pub name: Option<String>,
    /// Output only. The invited role on the account.
    
    pub role: Option<InvitationRoleEnum>,
    /// The sparsely populated account this invitation is for.
    #[serde(rename="targetAccount")]
    
    pub target_account: Option<Account>,
    /// The target location this invitation is for.
    #[serde(rename="targetLocation")]
    
    pub target_location: Option<TargetLocation>,
    /// Output only. Specifies which target types should appear in the response.
    #[serde(rename="targetType")]
    
    pub target_type: Option<InvitationTargetTypeEnum>,
}

impl client::Part for Invitation {}


/// Response message for AccessControl.ListAccountAdmins.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins list accounts](AccountAdminListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountAdminsResponse {
    /// A collection of Admin instances.
    #[serde(rename="accountAdmins")]
    
    pub account_admins: Option<Vec<Admin>>,
}

impl client::ResponseResult for ListAccountAdminsResponse {}


/// Response message for Accounts.ListAccounts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list accounts](AccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAccountsResponse {
    /// A collection of accounts to which the user has access. The personal account of the user doing the query will always be the first item of the result, unless it is filtered out.
    
    pub accounts: Option<Vec<Account>>,
    /// If the number of accounts exceeds the requested page size, this field is populated with a token to fetch the next page of accounts on a subsequent call to `accounts.list`. If there are no more accounts, this field is not present in the response.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAccountsResponse {}


/// Response message for AccessControl.ListInvitations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [invitations list accounts](AccountInvitationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListInvitationsResponse {
    /// A collection of invitations that are pending for the account. The number of invitations listed here cannot exceed 1000.
    
    pub invitations: Option<Vec<Invitation>>,
}

impl client::ResponseResult for ListInvitationsResponse {}


/// Response message for AccessControl.ListLocationAdmins.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [admins list locations](LocationAdminListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationAdminsResponse {
    /// A collection of Admins.
    
    pub admins: Option<Vec<Admin>>,
}

impl client::ResponseResult for ListLocationAdminsResponse {}


/// Additional information stored for an organization.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationInfo {
    /// Output only. The postal address for the account.
    
    pub address: Option<PostalAddress>,
    /// Output only. The contact number for the organization.
    #[serde(rename="phoneNumber")]
    
    pub phone_number: Option<String>,
    /// Output only. The registered domain for the account.
    #[serde(rename="registeredDomain")]
    
    pub registered_domain: Option<String>,
}

impl client::Part for OrganizationInfo {}


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
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

impl client::Part for PostalAddress {}


/// Represents a target location for a pending invitation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TargetLocation {
    /// The address of the location to which the user is invited.
    
    pub address: Option<String>,
    /// The name of the location to which the user is invited.
    #[serde(rename="locationName")]
    
    pub location_name: Option<String>,
}

impl client::Part for TargetLocation {}


/// Request message for AccessControl.TransferLocation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [transfer locations](LocationTransferCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransferLocationRequest {
    /// Required. Name of the account resource to transfer the location to (for example, "accounts/{account}").
    #[serde(rename="destinationAccount")]
    
    pub destination_account: Option<String>,
}

impl client::RequestValue for TransferLocationRequest {}


