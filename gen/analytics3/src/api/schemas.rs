use super::*;
/// JSON template for Analytics account entry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Child link for an account entry. Points to the list of web properties for this account.
    #[serde(rename="childLink")]
    
    pub child_link: Option<AccountChildLink>,
    /// Time the account was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Account ID.
    
    pub id: Option<String>,
    /// Resource type for Analytics account.
    
    pub kind: Option<String>,
    /// Account name.
    
    pub name: Option<String>,
    /// Permissions the user has for this account.
    
    pub permissions: Option<AccountPermissions>,
    /// Link for this account.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Indicates whether this account is starred or not.
    
    pub starred: Option<bool>,
    /// Time the account was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Account {}


/// JSON template for a linked account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountRef {
    /// Link for this account.
    
    pub href: Option<String>,
    /// Account ID.
    
    pub id: Option<String>,
    /// Analytics account reference.
    
    pub kind: Option<String>,
    /// Account name.
    
    pub name: Option<String>,
}

impl client::Part for AccountRef {}


/// An AccountSummary collection lists a summary of accounts, properties and views (profiles) to which the user has access. Each resource in the collection corresponds to a single AccountSummary.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [account summaries list management](ManagementAccountSummaryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountSummaries {
    /// A list of AccountSummaries.
    
    pub items: Option<Vec<AccountSummary>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this AccountSummary collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this AccountSummary collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for AccountSummaries {}


/// JSON template for an Analytics AccountSummary. An AccountSummary is a lightweight tree comprised of properties/profiles.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountSummary {
    /// Account ID.
    
    pub id: Option<String>,
    /// Resource type for Analytics AccountSummary.
    
    pub kind: Option<String>,
    /// Account name.
    
    pub name: Option<String>,
    /// Indicates whether this account is starred or not.
    
    pub starred: Option<bool>,
    /// List of web properties under this account.
    #[serde(rename="webProperties")]
    
    pub web_properties: Option<Vec<WebPropertySummary>>,
}

impl client::Part for AccountSummary {}


/// JSON template for an Analytics account ticket. The account ticket consists of the ticket ID and the basic information for the account, property and profile.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create account ticket provisioning](ProvisioningCreateAccountTicketCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountTicket {
    /// Account for this ticket.
    
    pub account: Option<Account>,
    /// Account ticket ID used to access the account ticket.
    
    pub id: Option<String>,
    /// Resource type for account ticket.
    
    pub kind: Option<String>,
    /// View (Profile) for the account.
    
    pub profile: Option<Profile>,
    /// Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in APIs console as a callback URL.
    #[serde(rename="redirectUri")]
    
    pub redirect_uri: Option<String>,
    /// Web property for the account.
    
    pub webproperty: Option<Webproperty>,
}

impl client::RequestValue for AccountTicket {}
impl client::ResponseResult for AccountTicket {}


/// JSON template for an Analytics account tree requests. The account tree request is used in the provisioning api to create an account, property, and view (profile). It contains the basic information required to make these fields.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create account tree provisioning](ProvisioningCreateAccountTreeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountTreeRequest {
    /// no description provided
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// Resource type for account ticket.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// no description provided
    
    pub timezone: Option<String>,
    /// no description provided
    #[serde(rename="webpropertyName")]
    
    pub webproperty_name: Option<String>,
    /// no description provided
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::RequestValue for AccountTreeRequest {}


/// JSON template for an Analytics account tree response. The account tree response is used in the provisioning api to return the result of creating an account, property, and view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create account tree provisioning](ProvisioningCreateAccountTreeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountTreeResponse {
    /// The account created.
    
    pub account: Option<Account>,
    /// Resource type for account ticket.
    
    pub kind: Option<String>,
    /// View (Profile) for the account.
    
    pub profile: Option<Profile>,
    /// Web property for the account.
    
    pub webproperty: Option<Webproperty>,
}

impl client::ResponseResult for AccountTreeResponse {}


/// An account collection provides a list of Analytics accounts to which a user has access. The account collection is the entry point to all management information. Each resource in the collection corresponds to a single Analytics account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [accounts list management](ManagementAccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Accounts {
    /// A list of accounts.
    
    pub items: Option<Vec<Account>>,
    /// The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Next link for this account collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Previous link for this account collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Accounts {}


/// JSON template for an Google Ads account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdWordsAccount {
    /// True if auto-tagging is enabled on the Google Ads account. Read-only after the insert operation.
    #[serde(rename="autoTaggingEnabled")]
    
    pub auto_tagging_enabled: Option<bool>,
    /// Customer ID. This field is required when creating a Google Ads link.
    #[serde(rename="customerId")]
    
    pub customer_id: Option<String>,
    /// Resource type for Google Ads account.
    
    pub kind: Option<String>,
}

impl client::Part for AdWordsAccount {}


/// Request template for the delete upload data request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploads delete upload data management](ManagementUploadDeleteUploadDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsDataimportDeleteUploadDataRequest {
    /// A list of upload UIDs.
    #[serde(rename="customDataImportUids")]
    
    pub custom_data_import_uids: Option<Vec<String>>,
}

impl client::RequestValue for AnalyticsDataimportDeleteUploadDataRequest {}


/// JSON template for a metadata column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    /// Map of attribute name and value for this column.
    
    pub attributes: Option<HashMap<String, String>>,
    /// Column id.
    
    pub id: Option<String>,
    /// Resource type for Analytics column.
    
    pub kind: Option<String>,
}

impl client::Part for Column {}


/// Lists columns (dimensions and metrics) for a particular report type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [columns list metadata](MetadataColumnListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Columns {
    /// List of attributes names returned by columns.
    #[serde(rename="attributeNames")]
    
    pub attribute_names: Option<Vec<String>>,
    /// Etag of collection. This etag can be compared with the last response etag to check if response has changed.
    
    pub etag: Option<String>,
    /// List of columns for a report type.
    
    pub items: Option<Vec<Column>>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Total number of columns returned in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::ResponseResult for Columns {}


/// JSON template for an Analytics custom data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDataSource {
    /// Account ID to which this custom data source belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// no description provided
    #[serde(rename="childLink")]
    
    pub child_link: Option<CustomDataSourceChildLink>,
    /// Time this custom data source was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Description of custom data source.
    
    pub description: Option<String>,
    /// Custom data source ID.
    
    pub id: Option<String>,
    /// no description provided
    #[serde(rename="importBehavior")]
    
    pub import_behavior: Option<String>,
    /// Resource type for Analytics custom data source.
    
    pub kind: Option<String>,
    /// Name of this custom data source.
    
    pub name: Option<String>,
    /// Parent link for this custom data source. Points to the web property to which this custom data source belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<CustomDataSourceParentLink>,
    /// IDs of views (profiles) linked to the custom data source.
    #[serde(rename="profilesLinked")]
    
    pub profiles_linked: Option<Vec<String>>,
    /// Collection of schema headers of the custom data source.
    
    pub schema: Option<Vec<String>>,
    /// Link for this Analytics custom data source.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Type of the custom data source.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time this custom data source was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Upload type of the custom data source.
    #[serde(rename="uploadType")]
    
    pub upload_type: Option<String>,
    /// Web property ID of the form UA-XXXXX-YY to which this custom data source belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::Part for CustomDataSource {}


/// Lists Analytics custom data sources to which the user has access. Each resource in the collection corresponds to a single Analytics custom data source.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom data sources list management](ManagementCustomDataSourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDataSources {
    /// Collection of custom data sources.
    
    pub items: Option<Vec<CustomDataSource>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this custom data source collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this custom data source collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for CustomDataSources {}


/// JSON template for Analytics Custom Dimension.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom dimensions get management](ManagementCustomDimensionGetCall) (response)
/// * [custom dimensions insert management](ManagementCustomDimensionInsertCall) (request|response)
/// * [custom dimensions patch management](ManagementCustomDimensionPatchCall) (request|response)
/// * [custom dimensions update management](ManagementCustomDimensionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDimension {
    /// Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Boolean indicating whether the custom dimension is active.
    
    pub active: Option<bool>,
    /// Time the custom dimension was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Custom dimension ID.
    
    pub id: Option<String>,
    /// Index of the custom dimension.
    
    pub index: Option<i32>,
    /// Kind value for a custom dimension. Set to "analytics#customDimension". It is a read-only field.
    
    pub kind: Option<String>,
    /// Name of the custom dimension.
    
    pub name: Option<String>,
    /// Parent link for the custom dimension. Points to the property to which the custom dimension belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<CustomDimensionParentLink>,
    /// Scope of the custom dimension: HIT, SESSION, USER or PRODUCT.
    
    pub scope: Option<String>,
    /// Link for the custom dimension
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Time the custom dimension was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Property ID.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for CustomDimension {}
impl client::ResponseResult for CustomDimension {}


/// A custom dimension collection lists Analytics custom dimensions to which the user has access. Each resource in the collection corresponds to a single Analytics custom dimension.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom dimensions list management](ManagementCustomDimensionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDimensions {
    /// Collection of custom dimensions.
    
    pub items: Option<Vec<CustomDimension>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this custom dimension collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this custom dimension collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for CustomDimensions {}


/// JSON template for Analytics Custom Metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom metrics get management](ManagementCustomMetricGetCall) (response)
/// * [custom metrics insert management](ManagementCustomMetricInsertCall) (request|response)
/// * [custom metrics patch management](ManagementCustomMetricPatchCall) (request|response)
/// * [custom metrics update management](ManagementCustomMetricUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Account ID.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Boolean indicating whether the custom metric is active.
    
    pub active: Option<bool>,
    /// Time the custom metric was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Custom metric ID.
    
    pub id: Option<String>,
    /// Index of the custom metric.
    
    pub index: Option<i32>,
    /// Kind value for a custom metric. Set to "analytics#customMetric". It is a read-only field.
    
    pub kind: Option<String>,
    /// Max value of custom metric.
    
    pub max_value: Option<String>,
    /// Min value of custom metric.
    
    pub min_value: Option<String>,
    /// Name of the custom metric.
    
    pub name: Option<String>,
    /// Parent link for the custom metric. Points to the property to which the custom metric belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<CustomMetricParentLink>,
    /// Scope of the custom metric: HIT or PRODUCT.
    
    pub scope: Option<String>,
    /// Link for the custom metric
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Data type of custom metric.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time the custom metric was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Property ID.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for CustomMetric {}
impl client::ResponseResult for CustomMetric {}


/// A custom metric collection lists Analytics custom metrics to which the user has access. Each resource in the collection corresponds to a single Analytics custom metric.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom metrics list management](ManagementCustomMetricListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetrics {
    /// Collection of custom metrics.
    
    pub items: Option<Vec<CustomMetric>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this custom metric collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this custom metric collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for CustomMetrics {}


/// JSON template for Analytics Entity Google Ads Link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web property ad words links get management](ManagementWebPropertyAdWordsLinkGetCall) (response)
/// * [web property ad words links insert management](ManagementWebPropertyAdWordsLinkInsertCall) (request|response)
/// * [web property ad words links patch management](ManagementWebPropertyAdWordsLinkPatchCall) (request|response)
/// * [web property ad words links update management](ManagementWebPropertyAdWordsLinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityAdWordsLink {
    /// A list of Google Ads client accounts. These cannot be MCC accounts. This field is required when creating a Google Ads link. It cannot be empty.
    #[serde(rename="adWordsAccounts")]
    
    pub ad_words_accounts: Option<Vec<AdWordsAccount>>,
    /// Web property being linked.
    
    pub entity: Option<EntityAdWordsLinkEntity>,
    /// Entity Google Ads link ID
    
    pub id: Option<String>,
    /// Resource type for entity Google Ads link.
    
    pub kind: Option<String>,
    /// Name of the link. This field is required when creating a Google Ads link.
    
    pub name: Option<String>,
    /// IDs of linked Views (Profiles) represented as strings.
    #[serde(rename="profileIds")]
    
    pub profile_ids: Option<Vec<String>>,
    /// URL link for this Google Analytics - Google Ads link.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for EntityAdWordsLink {}
impl client::ResponseResult for EntityAdWordsLink {}


/// An entity Google Ads link collection provides a list of GA-Google Ads links Each resource in this collection corresponds to a single link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web property ad words links list management](ManagementWebPropertyAdWordsLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityAdWordsLinks {
    /// A list of entity Google Ads links.
    
    pub items: Option<Vec<EntityAdWordsLink>>,
    /// The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Next link for this Google Ads link collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Previous link for this Google Ads link collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::ResponseResult for EntityAdWordsLinks {}


/// JSON template for an Analytics Entity-User Link. Returns permissions that a user has for an entity.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [account user links insert management](ManagementAccountUserLinkInsertCall) (request|response)
/// * [account user links update management](ManagementAccountUserLinkUpdateCall) (request|response)
/// * [profile user links insert management](ManagementProfileUserLinkInsertCall) (request|response)
/// * [profile user links update management](ManagementProfileUserLinkUpdateCall) (request|response)
/// * [webproperty user links insert management](ManagementWebpropertyUserLinkInsertCall) (request|response)
/// * [webproperty user links update management](ManagementWebpropertyUserLinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityUserLink {
    /// Entity for this link. It can be an account, a web property, or a view (profile).
    
    pub entity: Option<EntityUserLinkEntity>,
    /// Entity user link ID
    
    pub id: Option<String>,
    /// Resource type for entity user link.
    
    pub kind: Option<String>,
    /// Permissions the user has for this entity.
    
    pub permissions: Option<EntityUserLinkPermissions>,
    /// Self link for this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// User reference.
    #[serde(rename="userRef")]
    
    pub user_ref: Option<UserRef>,
}

impl client::RequestValue for EntityUserLink {}
impl client::ResponseResult for EntityUserLink {}


/// An entity user link collection provides a list of Analytics ACL links Each resource in this collection corresponds to a single link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [account user links list management](ManagementAccountUserLinkListCall) (response)
/// * [profile user links list management](ManagementProfileUserLinkListCall) (response)
/// * [webproperty user links list management](ManagementWebpropertyUserLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityUserLinks {
    /// A list of entity user links.
    
    pub items: Option<Vec<EntityUserLink>>,
    /// The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Next link for this account collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Previous link for this account collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::ResponseResult for EntityUserLinks {}


/// JSON template for Analytics experiment resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [experiments get management](ManagementExperimentGetCall) (response)
/// * [experiments insert management](ManagementExperimentInsertCall) (request|response)
/// * [experiments patch management](ManagementExperimentPatchCall) (request|response)
/// * [experiments update management](ManagementExperimentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Experiment {
    /// Account ID to which this experiment belongs. This field is read-only.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Time the experiment was created. This field is read-only.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Notes about this experiment.
    
    pub description: Option<String>,
    /// If true, the end user will be able to edit the experiment via the Google Analytics user interface.
    #[serde(rename="editableInGaUi")]
    
    pub editable_in_ga_ui: Option<bool>,
    /// The ending time of the experiment (the time the status changed from RUNNING to ENDED). This field is present only if the experiment has ended. This field is read-only.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Boolean specifying whether to distribute traffic evenly across all variations. If the value is False, content experiments follows the default behavior of adjusting traffic dynamically based on variation performance. Optional -- defaults to False. This field may not be changed for an experiment whose status is ENDED.
    #[serde(rename="equalWeighting")]
    
    pub equal_weighting: Option<bool>,
    /// Experiment ID. Required for patch and update. Disallowed for create.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this experiment belongs. This field is read-only.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for an Analytics experiment. This field is read-only.
    
    pub kind: Option<String>,
    /// An integer number in [3, 90]. Specifies the minimum length of the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED.
    #[serde(rename="minimumExperimentLengthInDays")]
    
    pub minimum_experiment_length_in_days: Option<i32>,
    /// Experiment name. This field may not be changed for an experiment whose status is ENDED. This field is required when creating an experiment.
    
    pub name: Option<String>,
    /// The metric that the experiment is optimizing. Valid values: "ga:goal(n)Completions", "ga:adsenseAdsClicks", "ga:adsenseAdsViewed", "ga:adsenseRevenue", "ga:bounces", "ga:pageviews", "ga:sessionDuration", "ga:transactions", "ga:transactionRevenue". This field is required if status is "RUNNING" and servingFramework is one of "REDIRECT" or "API".
    #[serde(rename="objectiveMetric")]
    
    pub objective_metric: Option<String>,
    /// Whether the objectiveMetric should be minimized or maximized. Possible values: "MAXIMUM", "MINIMUM". Optional--defaults to "MAXIMUM". Cannot be specified without objectiveMetric. Cannot be modified when status is "RUNNING" or "ENDED".
    #[serde(rename="optimizationType")]
    
    pub optimization_type: Option<String>,
    /// Parent link for an experiment. Points to the view (profile) to which this experiment belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<ExperimentParentLink>,
    /// View (Profile) ID to which this experiment belongs. This field is read-only.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// Why the experiment ended. Possible values: "STOPPED_BY_USER", "WINNER_FOUND", "EXPERIMENT_EXPIRED", "ENDED_WITH_NO_WINNER", "GOAL_OBJECTIVE_CHANGED". "ENDED_WITH_NO_WINNER" means that the experiment didn't expire but no winner was projected to be found. If the experiment status is changed via the API to ENDED this field is set to STOPPED_BY_USER. This field is read-only.
    #[serde(rename="reasonExperimentEnded")]
    
    pub reason_experiment_ended: Option<String>,
    /// Boolean specifying whether variations URLS are rewritten to match those of the original. This field may not be changed for an experiments whose status is ENDED.
    #[serde(rename="rewriteVariationUrlsAsOriginal")]
    
    pub rewrite_variation_urls_as_original: Option<bool>,
    /// Link for this experiment. This field is read-only.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The framework used to serve the experiment variations and evaluate the results. One of:  
    /// - REDIRECT: Google Analytics redirects traffic to different variation pages, reports the chosen variation and evaluates the results.
    /// - API: Google Analytics chooses and reports the variation to serve and evaluates the results; the caller is responsible for serving the selected variation.
    /// - EXTERNAL: The variations will be served externally and the chosen variation reported to Google Analytics. The caller is responsible for serving the selected variation and evaluating the results.
    #[serde(rename="servingFramework")]
    
    pub serving_framework: Option<String>,
    /// The snippet of code to include on the control page(s). This field is read-only.
    
    pub snippet: Option<String>,
    /// The starting time of the experiment (the time the status changed from READY_TO_RUN to RUNNING). This field is present only if the experiment has started. This field is read-only.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Experiment status. Possible values: "DRAFT", "READY_TO_RUN", "RUNNING", "ENDED". Experiments can be created in the "DRAFT", "READY_TO_RUN" or "RUNNING" state. This field is required when creating an experiment.
    
    pub status: Option<String>,
    /// A floating-point number in (0, 1]. Specifies the fraction of the traffic that participates in the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED.
    #[serde(rename="trafficCoverage")]
    
    pub traffic_coverage: Option<f64>,
    /// Time the experiment was last modified. This field is read-only.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Array of variations. The first variation in the array is the original. The number of variations may not change once an experiment is in the RUNNING state. At least two variations are required before status can be set to RUNNING.
    
    pub variations: Option<Vec<ExperimentVariations>>,
    /// Web property ID to which this experiment belongs. The web property ID is of the form UA-XXXXX-YY. This field is read-only.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
    /// A floating-point number in (0, 1). Specifies the necessary confidence level to choose a winner. This field may not be changed for an experiments whose status is ENDED.
    #[serde(rename="winnerConfidenceLevel")]
    
    pub winner_confidence_level: Option<f64>,
    /// Boolean specifying whether a winner has been found for this experiment. This field is read-only.
    #[serde(rename="winnerFound")]
    
    pub winner_found: Option<bool>,
}

impl client::RequestValue for Experiment {}
impl client::ResponseResult for Experiment {}


/// An experiment collection lists Analytics experiments to which the user has access. Each view (profile) can have a set of experiments. Each resource in the Experiment collection corresponds to a single Analytics experiment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [experiments list management](ManagementExperimentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Experiments {
    /// A list of experiments.
    
    pub items: Option<Vec<Experiment>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this experiment collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this experiment collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of resources in the result.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Experiments {}


/// JSON template for an Analytics account filter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [filters delete management](ManagementFilterDeleteCall) (response)
/// * [filters get management](ManagementFilterGetCall) (response)
/// * [filters insert management](ManagementFilterInsertCall) (request|response)
/// * [filters patch management](ManagementFilterPatchCall) (request|response)
/// * [filters update management](ManagementFilterUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// Account ID to which this filter belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Details for the filter of the type ADVANCED.
    #[serde(rename="advancedDetails")]
    
    pub advanced_details: Option<FilterAdvancedDetails>,
    /// Time this filter was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details for the filter of the type EXCLUDE.
    #[serde(rename="excludeDetails")]
    
    pub exclude_details: Option<FilterExpression>,
    /// Filter ID.
    
    pub id: Option<String>,
    /// Details for the filter of the type INCLUDE.
    #[serde(rename="includeDetails")]
    
    pub include_details: Option<FilterExpression>,
    /// Resource type for Analytics filter.
    
    pub kind: Option<String>,
    /// Details for the filter of the type LOWER.
    #[serde(rename="lowercaseDetails")]
    
    pub lowercase_details: Option<FilterLowercaseDetails>,
    /// Name of this filter.
    
    pub name: Option<String>,
    /// Parent link for this filter. Points to the account to which this filter belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<FilterParentLink>,
    /// Details for the filter of the type SEARCH_AND_REPLACE.
    #[serde(rename="searchAndReplaceDetails")]
    
    pub search_and_replace_details: Option<FilterSearchAndReplaceDetails>,
    /// Link for this filter.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Type of this filter. Possible values are INCLUDE, EXCLUDE, LOWERCASE, UPPERCASE, SEARCH_AND_REPLACE and ADVANCED.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time this filter was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details for the filter of the type UPPER.
    #[serde(rename="uppercaseDetails")]
    
    pub uppercase_details: Option<FilterUppercaseDetails>,
}

impl client::RequestValue for Filter {}
impl client::ResponseResult for Filter {}


/// JSON template for an Analytics filter expression.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterExpression {
    /// Determines if the filter is case sensitive.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// Filter expression value
    #[serde(rename="expressionValue")]
    
    pub expression_value: Option<String>,
    /// Field to filter. Possible values:  
    /// - Content and Traffic  
    /// - PAGE_REQUEST_URI, 
    /// - PAGE_HOSTNAME, 
    /// - PAGE_TITLE, 
    /// - REFERRAL, 
    /// - COST_DATA_URI (Campaign target URL), 
    /// - HIT_TYPE, 
    /// - INTERNAL_SEARCH_TERM, 
    /// - INTERNAL_SEARCH_TYPE, 
    /// - SOURCE_PROPERTY_TRACKING_ID,   
    /// - Campaign or AdGroup  
    /// - CAMPAIGN_SOURCE, 
    /// - CAMPAIGN_MEDIUM, 
    /// - CAMPAIGN_NAME, 
    /// - CAMPAIGN_AD_GROUP, 
    /// - CAMPAIGN_TERM, 
    /// - CAMPAIGN_CONTENT, 
    /// - CAMPAIGN_CODE, 
    /// - CAMPAIGN_REFERRAL_PATH,   
    /// - E-Commerce  
    /// - TRANSACTION_COUNTRY, 
    /// - TRANSACTION_REGION, 
    /// - TRANSACTION_CITY, 
    /// - TRANSACTION_AFFILIATION (Store or order location), 
    /// - ITEM_NAME, 
    /// - ITEM_CODE, 
    /// - ITEM_VARIATION, 
    /// - TRANSACTION_ID, 
    /// - TRANSACTION_CURRENCY_CODE, 
    /// - PRODUCT_ACTION_TYPE,   
    /// - Audience/Users  
    /// - BROWSER, 
    /// - BROWSER_VERSION, 
    /// - BROWSER_SIZE, 
    /// - PLATFORM, 
    /// - PLATFORM_VERSION, 
    /// - LANGUAGE, 
    /// - SCREEN_RESOLUTION, 
    /// - SCREEN_COLORS, 
    /// - JAVA_ENABLED (Boolean Field), 
    /// - FLASH_VERSION, 
    /// - GEO_SPEED (Connection speed), 
    /// - VISITOR_TYPE, 
    /// - GEO_ORGANIZATION (ISP organization), 
    /// - GEO_DOMAIN, 
    /// - GEO_IP_ADDRESS, 
    /// - GEO_IP_VERSION,   
    /// - Location  
    /// - GEO_COUNTRY, 
    /// - GEO_REGION, 
    /// - GEO_CITY,   
    /// - Event  
    /// - EVENT_CATEGORY, 
    /// - EVENT_ACTION, 
    /// - EVENT_LABEL,   
    /// - Other  
    /// - CUSTOM_FIELD_1, 
    /// - CUSTOM_FIELD_2, 
    /// - USER_DEFINED_VALUE,   
    /// - Application  
    /// - APP_ID, 
    /// - APP_INSTALLER_ID, 
    /// - APP_NAME, 
    /// - APP_VERSION, 
    /// - SCREEN, 
    /// - IS_APP (Boolean Field), 
    /// - IS_FATAL_EXCEPTION (Boolean Field), 
    /// - EXCEPTION_DESCRIPTION,   
    /// - Mobile device  
    /// - IS_MOBILE (Boolean Field, Deprecated. Use DEVICE_CATEGORY=mobile), 
    /// - IS_TABLET (Boolean Field, Deprecated. Use DEVICE_CATEGORY=tablet), 
    /// - DEVICE_CATEGORY, 
    /// - MOBILE_HAS_QWERTY_KEYBOARD (Boolean Field), 
    /// - MOBILE_HAS_NFC_SUPPORT (Boolean Field), 
    /// - MOBILE_HAS_CELLULAR_RADIO (Boolean Field), 
    /// - MOBILE_HAS_WIFI_SUPPORT (Boolean Field), 
    /// - MOBILE_BRAND_NAME, 
    /// - MOBILE_MODEL_NAME, 
    /// - MOBILE_MARKETING_NAME, 
    /// - MOBILE_POINTING_METHOD,   
    /// - Social  
    /// - SOCIAL_NETWORK, 
    /// - SOCIAL_ACTION, 
    /// - SOCIAL_ACTION_TARGET,   
    /// - Custom dimension  
    /// - CUSTOM_DIMENSION (See accompanying field index),
    
    pub field: Option<String>,
    /// The Index of the custom dimension. Set only if the field is a is CUSTOM_DIMENSION.
    #[serde(rename="fieldIndex")]
    
    pub field_index: Option<i32>,
    /// Kind value for filter expression
    
    pub kind: Option<String>,
    /// Match type for this filter. Possible values are BEGINS_WITH, EQUAL, ENDS_WITH, CONTAINS, or MATCHES. GEO_DOMAIN, GEO_IP_ADDRESS, PAGE_REQUEST_URI, or PAGE_HOSTNAME filters can use any match type; all other filters must use MATCHES.
    #[serde(rename="matchType")]
    
    pub match_type: Option<String>,
}

impl client::Part for FilterExpression {}


/// JSON template for a profile filter link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterRef {
    /// Account ID to which this filter belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Link for this filter.
    
    pub href: Option<String>,
    /// Filter ID.
    
    pub id: Option<String>,
    /// Kind value for filter reference.
    
    pub kind: Option<String>,
    /// Name of this filter.
    
    pub name: Option<String>,
}

impl client::Part for FilterRef {}


/// A filter collection lists filters created by users in an Analytics account. Each resource in the collection corresponds to a filter.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [filters list management](ManagementFilterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filters {
    /// A list of filters.
    
    pub items: Option<Vec<Filter>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this filter collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this filter collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Filters {}


/// Analytics data for a given view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ga get data](DataGaGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaData {
    /// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
    #[serde(rename="columnHeaders")]
    
    pub column_headers: Option<Vec<GaDataColumnHeaders>>,
    /// Determines if Analytics data contains samples.
    #[serde(rename="containsSampledData")]
    
    pub contains_sampled_data: Option<bool>,
    /// The last refreshed time in seconds for Analytics data.
    #[serde(rename="dataLastRefreshed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub data_last_refreshed: Option<i64>,
    /// no description provided
    #[serde(rename="dataTable")]
    
    pub data_table: Option<GaDataDataTable>,
    /// Unique ID for this data response.
    
    pub id: Option<String>,
    /// The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Link to next page for this Analytics data query.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this Analytics data query.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// Information for the view (profile), for which the Analytics data was requested.
    #[serde(rename="profileInfo")]
    
    pub profile_info: Option<GaDataProfileInfo>,
    /// Analytics data request query parameters.
    
    pub query: Option<GaDataQuery>,
    /// Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request.
    
    pub rows: Option<Vec<Vec<String>>>,
    /// The number of samples used to calculate the result.
    #[serde(rename="sampleSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sample_size: Option<i64>,
    /// Total size of the sample space from which the samples were selected.
    #[serde(rename="sampleSpace")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sample_space: Option<i64>,
    /// Link to this page.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The total number of rows for the query, regardless of the number of rows in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request.
    #[serde(rename="totalsForAllResults")]
    
    pub totals_for_all_results: Option<HashMap<String, String>>,
}

impl client::ResponseResult for GaData {}


/// JSON template for Analytics goal resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [goals get management](ManagementGoalGetCall) (response)
/// * [goals insert management](ManagementGoalInsertCall) (request|response)
/// * [goals patch management](ManagementGoalPatchCall) (request|response)
/// * [goals update management](ManagementGoalUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Goal {
    /// Account ID to which this goal belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Determines whether this goal is active.
    
    pub active: Option<bool>,
    /// Time this goal was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details for the goal of the type EVENT.
    #[serde(rename="eventDetails")]
    
    pub event_details: Option<GoalEventDetails>,
    /// Goal ID.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this goal belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for an Analytics goal.
    
    pub kind: Option<String>,
    /// Goal name.
    
    pub name: Option<String>,
    /// Parent link for a goal. Points to the view (profile) to which this goal belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<GoalParentLink>,
    /// View (Profile) ID to which this goal belongs.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// Link for this goal.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Goal type. Possible values are URL_DESTINATION, VISIT_TIME_ON_SITE, VISIT_NUM_PAGES, AND EVENT.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time this goal was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details for the goal of the type URL_DESTINATION.
    #[serde(rename="urlDestinationDetails")]
    
    pub url_destination_details: Option<GoalUrlDestinationDetails>,
    /// Goal value.
    
    pub value: Option<f32>,
    /// Details for the goal of the type VISIT_NUM_PAGES.
    #[serde(rename="visitNumPagesDetails")]
    
    pub visit_num_pages_details: Option<GoalVisitNumPagesDetails>,
    /// Details for the goal of the type VISIT_TIME_ON_SITE.
    #[serde(rename="visitTimeOnSiteDetails")]
    
    pub visit_time_on_site_details: Option<GoalVisitTimeOnSiteDetails>,
    /// Web property ID to which this goal belongs. The web property ID is of the form UA-XXXXX-YY.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for Goal {}
impl client::ResponseResult for Goal {}


/// A goal collection lists Analytics goals to which the user has access. Each view (profile) can have a set of goals. Each resource in the Goal collection corresponds to a single Analytics goal.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [goals list management](ManagementGoalListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Goals {
    /// A list of goals.
    
    pub items: Option<Vec<Goal>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this goal collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this goal collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of resources in the result.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Goals {}


/// JSON template for a hash Client Id request resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client id hash client id management](ManagementClientIdHashClientIdCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HashClientIdRequest {
    /// no description provided
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for HashClientIdRequest {}


/// JSON template for a hash Client Id response resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [client id hash client id management](ManagementClientIdHashClientIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HashClientIdResponse {
    /// no description provided
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// no description provided
    #[serde(rename="hashedClientId")]
    
    pub hashed_client_id: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::ResponseResult for HashClientIdResponse {}


/// JSON template for an Analytics Remarketing Include Conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IncludeConditions {
    /// The look-back window lets you specify a time frame for evaluating the behavior that qualifies users for your audience. For example, if your filters include users from Central Asia, and Transactions Greater than 2, and you set the look-back window to 14 days, then any user from Central Asia whose cumulative transactions exceed 2 during the last 14 days is added to the audience.
    #[serde(rename="daysToLookBack")]
    
    pub days_to_look_back: Option<i32>,
    /// Boolean indicating whether this segment is a smart list. https://support.google.com/analytics/answer/4628577
    #[serde(rename="isSmartList")]
    
    pub is_smart_list: Option<bool>,
    /// Resource type for include conditions.
    
    pub kind: Option<String>,
    /// Number of days (in the range 1 to 540) a user remains in the audience.
    #[serde(rename="membershipDurationDays")]
    
    pub membership_duration_days: Option<i32>,
    /// The segment condition that will cause a user to be added to an audience.
    
    pub segment: Option<String>,
}

impl client::Part for IncludeConditions {}


/// JSON template for an Analytics Remarketing Audience Foreign Link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedForeignAccount {
    /// Account ID to which this linked foreign account belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Boolean indicating whether this is eligible for search.
    #[serde(rename="eligibleForSearch")]
    
    pub eligible_for_search: Option<bool>,
    /// Entity ad account link ID.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this linked foreign account belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for linked foreign account.
    
    pub kind: Option<String>,
    /// The foreign account ID. For example the an Google Ads `linkedAccountId` has the following format XXX-XXX-XXXX.
    #[serde(rename="linkedAccountId")]
    
    pub linked_account_id: Option<String>,
    /// Remarketing audience ID to which this linked foreign account belongs.
    #[serde(rename="remarketingAudienceId")]
    
    pub remarketing_audience_id: Option<String>,
    /// The status of this foreign account link.
    
    pub status: Option<String>,
    /// The type of the foreign account. For example, `ADWORDS_LINKS`, `DBM_LINKS`, `MCC_LINKS` or `OPTIMIZE`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Web property ID of the form UA-XXXXX-YY to which this linked foreign account belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::Part for LinkedForeignAccount {}


/// Multi-Channel Funnels data for a given view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [mcf get data](DataMcfGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfData {
    /// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
    #[serde(rename="columnHeaders")]
    
    pub column_headers: Option<Vec<McfDataColumnHeaders>>,
    /// Determines if the Analytics data contains sampled data.
    #[serde(rename="containsSampledData")]
    
    pub contains_sampled_data: Option<bool>,
    /// Unique ID for this data response.
    
    pub id: Option<String>,
    /// The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Link to next page for this Analytics data query.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this Analytics data query.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// Information for the view (profile), for which the Analytics data was requested.
    #[serde(rename="profileInfo")]
    
    pub profile_info: Option<McfDataProfileInfo>,
    /// Analytics data request query parameters.
    
    pub query: Option<McfDataQuery>,
    /// Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request.
    
    pub rows: Option<Vec<Vec<McfDataRows>>>,
    /// The number of samples used to calculate the result.
    #[serde(rename="sampleSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sample_size: Option<i64>,
    /// Total size of the sample space from which the samples were selected.
    #[serde(rename="sampleSpace")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sample_space: Option<i64>,
    /// Link to this page.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The total number of rows for the query, regardless of the number of rows in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request.
    #[serde(rename="totalsForAllResults")]
    
    pub totals_for_all_results: Option<HashMap<String, String>>,
}

impl client::ResponseResult for McfData {}


/// JSON template for an Analytics view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profiles get management](ManagementProfileGetCall) (response)
/// * [profiles insert management](ManagementProfileInsertCall) (request|response)
/// * [profiles patch management](ManagementProfilePatchCall) (request|response)
/// * [profiles update management](ManagementProfileUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Account ID to which this view (profile) belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Indicates whether bot filtering is enabled for this view (profile).
    #[serde(rename="botFilteringEnabled")]
    
    pub bot_filtering_enabled: Option<bool>,
    /// Child link for this view (profile). Points to the list of goals for this view (profile).
    #[serde(rename="childLink")]
    
    pub child_link: Option<ProfileChildLink>,
    /// Time this view (profile) was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The currency type associated with this view (profile), defaults to USD. The supported values are:
    /// USD, JPY, EUR, GBP, AUD, KRW, BRL, CNY, DKK, RUB, SEK, NOK, PLN, TRY, TWD, HKD, THB, IDR, ARS, MXN, VND, PHP, INR, CHF, CAD, CZK, NZD, HUF, BGN, LTL, ZAR, UAH, AED, BOB, CLP, COP, EGP, HRK, ILS, MAD, MYR, PEN, PKR, RON, RSD, SAR, SGD, VEF, LVL
    
    pub currency: Option<String>,
    /// Default page for this view (profile).
    #[serde(rename="defaultPage")]
    
    pub default_page: Option<String>,
    /// Indicates whether ecommerce tracking is enabled for this view (profile).
    #[serde(rename="eCommerceTracking")]
    
    pub e_commerce_tracking: Option<bool>,
    /// Indicates whether enhanced ecommerce tracking is enabled for this view (profile). This property can only be enabled if ecommerce tracking is enabled.
    #[serde(rename="enhancedECommerceTracking")]
    
    pub enhanced_e_commerce_tracking: Option<bool>,
    /// The query parameters that are excluded from this view (profile).
    #[serde(rename="excludeQueryParameters")]
    
    pub exclude_query_parameters: Option<String>,
    /// View (Profile) ID.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this view (profile) belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for Analytics view (profile).
    
    pub kind: Option<String>,
    /// Name of this view (profile).
    
    pub name: Option<String>,
    /// Parent link for this view (profile). Points to the web property to which this view (profile) belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<ProfileParentLink>,
    /// Permissions the user has for this view (profile).
    
    pub permissions: Option<ProfilePermissions>,
    /// Link for this view (profile).
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Site search category parameters for this view (profile).
    #[serde(rename="siteSearchCategoryParameters")]
    
    pub site_search_category_parameters: Option<String>,
    /// The site search query parameters for this view (profile).
    #[serde(rename="siteSearchQueryParameters")]
    
    pub site_search_query_parameters: Option<String>,
    /// Indicates whether this view (profile) is starred or not.
    
    pub starred: Option<bool>,
    /// Whether or not Analytics will strip search category parameters from the URLs in your reports.
    #[serde(rename="stripSiteSearchCategoryParameters")]
    
    pub strip_site_search_category_parameters: Option<bool>,
    /// Whether or not Analytics will strip search query parameters from the URLs in your reports.
    #[serde(rename="stripSiteSearchQueryParameters")]
    
    pub strip_site_search_query_parameters: Option<bool>,
    /// Time zone for which this view (profile) has been configured. Time zones are identified by strings from the TZ database.
    
    pub timezone: Option<String>,
    /// View (Profile) type. Supported types: WEB or APP.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time this view (profile) was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
    /// Website URL for this view (profile).
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::RequestValue for Profile {}
impl client::ResponseResult for Profile {}


/// JSON template for an Analytics profile filter link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profile filter links get management](ManagementProfileFilterLinkGetCall) (response)
/// * [profile filter links insert management](ManagementProfileFilterLinkInsertCall) (request|response)
/// * [profile filter links patch management](ManagementProfileFilterLinkPatchCall) (request|response)
/// * [profile filter links update management](ManagementProfileFilterLinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileFilterLink {
    /// Filter for this link.
    #[serde(rename="filterRef")]
    
    pub filter_ref: Option<FilterRef>,
    /// Profile filter link ID.
    
    pub id: Option<String>,
    /// Resource type for Analytics filter.
    
    pub kind: Option<String>,
    /// View (Profile) for this link.
    #[serde(rename="profileRef")]
    
    pub profile_ref: Option<ProfileRef>,
    /// The rank of this profile filter link relative to the other filters linked to the same profile.
    /// For readonly (i.e., list and get) operations, the rank always starts at 1.
    /// For write (i.e., create, update, or delete) operations, you may specify a value between 0 and 255 inclusively, [0, 255]. In order to insert a link at the end of the list, either don't specify a rank or set a rank to a number greater than the largest rank in the list. In order to insert a link to the beginning of the list specify a rank that is less than or equal to 1. The new link will move all existing filters with the same or lower rank down the list. After the link is inserted/updated/deleted all profile filter links will be renumbered starting at 1.
    
    pub rank: Option<i32>,
    /// Link for this profile filter link.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for ProfileFilterLink {}
impl client::ResponseResult for ProfileFilterLink {}


/// A profile filter link collection lists profile filter links between profiles and filters. Each resource in the collection corresponds to a profile filter link.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profile filter links list management](ManagementProfileFilterLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileFilterLinks {
    /// A list of profile filter links.
    
    pub items: Option<Vec<ProfileFilterLink>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this profile filter link collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this profile filter link collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for ProfileFilterLinks {}


/// JSON template for a linked view (profile).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileRef {
    /// Account ID to which this view (profile) belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Link for this view (profile).
    
    pub href: Option<String>,
    /// View (Profile) ID.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this view (profile) belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Analytics view (profile) reference.
    
    pub kind: Option<String>,
    /// Name of this view (profile).
    
    pub name: Option<String>,
    /// Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::Part for ProfileRef {}


/// JSON template for an Analytics ProfileSummary. ProfileSummary returns basic information (i.e., summary) for a profile.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileSummary {
    /// View (profile) ID.
    
    pub id: Option<String>,
    /// Resource type for Analytics ProfileSummary.
    
    pub kind: Option<String>,
    /// View (profile) name.
    
    pub name: Option<String>,
    /// Indicates whether this view (profile) is starred or not.
    
    pub starred: Option<bool>,
    /// View (Profile) type. Supported types: WEB or APP.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ProfileSummary {}


/// A view (profile) collection lists Analytics views (profiles) to which the user has access. Each resource in the collection corresponds to a single Analytics view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profiles list management](ManagementProfileListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profiles {
    /// A list of views (profiles).
    
    pub items: Option<Vec<Profile>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this view (profile) collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this view (profile) collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Profiles {}


/// Real time data for a given view (profile).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [realtime get data](DataRealtimeGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeData {
    /// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
    #[serde(rename="columnHeaders")]
    
    pub column_headers: Option<Vec<RealtimeDataColumnHeaders>>,
    /// Unique ID for this data response.
    
    pub id: Option<String>,
    /// Resource type.
    
    pub kind: Option<String>,
    /// Information for the view (profile), for which the real time data was requested.
    #[serde(rename="profileInfo")]
    
    pub profile_info: Option<RealtimeDataProfileInfo>,
    /// Real time data request query parameters.
    
    pub query: Option<RealtimeDataQuery>,
    /// Real time data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request.
    
    pub rows: Option<Vec<Vec<String>>>,
    /// Link to this page.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The total number of rows for the query, regardless of the number of rows in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request.
    #[serde(rename="totalsForAllResults")]
    
    pub totals_for_all_results: Option<HashMap<String, String>>,
}

impl client::ResponseResult for RealtimeData {}


/// JSON template for an Analytics remarketing audience.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remarketing audience get management](ManagementRemarketingAudienceGetCall) (response)
/// * [remarketing audience insert management](ManagementRemarketingAudienceInsertCall) (request|response)
/// * [remarketing audience patch management](ManagementRemarketingAudiencePatchCall) (request|response)
/// * [remarketing audience update management](ManagementRemarketingAudienceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingAudience {
    /// Account ID to which this remarketing audience belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The simple audience definition that will cause a user to be added to an audience.
    #[serde(rename="audienceDefinition")]
    
    pub audience_definition: Option<RemarketingAudienceAudienceDefinition>,
    /// The type of audience, either SIMPLE or STATE_BASED.
    #[serde(rename="audienceType")]
    
    pub audience_type: Option<String>,
    /// Time this remarketing audience was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The description of this remarketing audience.
    
    pub description: Option<String>,
    /// Remarketing Audience ID.
    
    pub id: Option<String>,
    /// Internal ID for the web property to which this remarketing audience belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// The linked ad accounts associated with this remarketing audience. A remarketing audience can have only one linkedAdAccount currently.
    #[serde(rename="linkedAdAccounts")]
    
    pub linked_ad_accounts: Option<Vec<LinkedForeignAccount>>,
    /// The views (profiles) that this remarketing audience is linked to.
    #[serde(rename="linkedViews")]
    
    pub linked_views: Option<Vec<String>>,
    /// The name of this remarketing audience.
    
    pub name: Option<String>,
    /// A state based audience definition that will cause a user to be added or removed from an audience.
    #[serde(rename="stateBasedAudienceDefinition")]
    
    pub state_based_audience_definition: Option<RemarketingAudienceStateBasedAudienceDefinition>,
    /// Time this remarketing audience was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Web property ID of the form UA-XXXXX-YY to which this remarketing audience belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for RemarketingAudience {}
impl client::ResponseResult for RemarketingAudience {}


/// A remarketing audience collection lists Analytics remarketing audiences to which the user has access. Each resource in the collection corresponds to a single Analytics remarketing audience.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remarketing audience list management](ManagementRemarketingAudienceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingAudiences {
    /// A list of remarketing audiences.
    
    pub items: Option<Vec<RemarketingAudience>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this remarketing audience collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this view (profile) collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for RemarketingAudiences {}


/// JSON template for an Analytics segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    /// Time the segment was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Segment definition.
    
    pub definition: Option<String>,
    /// Segment ID.
    
    pub id: Option<String>,
    /// Resource type for Analytics segment.
    
    pub kind: Option<String>,
    /// Segment name.
    
    pub name: Option<String>,
    /// Segment ID. Can be used with the 'segment' parameter in Core Reporting API.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
    /// Link for this segment.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Type for a segment. Possible values are "BUILT_IN" or "CUSTOM".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Time the segment was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Segment {}


/// An segment collection lists Analytics segments that the user has access to. Each resource in the collection corresponds to a single Analytics segment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [segments list management](ManagementSegmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Segments {
    /// A list of segments.
    
    pub items: Option<Vec<Segment>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type for segments.
    
    pub kind: Option<String>,
    /// Link to next page for this segment collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this segment collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Segments {}


/// JSON template for Analytics unsampled report resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unsampled reports get management](ManagementUnsampledReportGetCall) (response)
/// * [unsampled reports insert management](ManagementUnsampledReportInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsampledReport {
    /// Account ID to which this unsampled report belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Download details for a file stored in Google Cloud Storage.
    #[serde(rename="cloudStorageDownloadDetails")]
    
    pub cloud_storage_download_details: Option<UnsampledReportCloudStorageDownloadDetails>,
    /// Time this unsampled report was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The dimensions for the unsampled report.
    
    pub dimensions: Option<String>,
    /// The type of download you need to use for the report data file. Possible values include `GOOGLE_DRIVE` and `GOOGLE_CLOUD_STORAGE`. If the value is `GOOGLE_DRIVE`, see the `driveDownloadDetails` field. If the value is `GOOGLE_CLOUD_STORAGE`, see the `cloudStorageDownloadDetails` field.
    #[serde(rename="downloadType")]
    
    pub download_type: Option<String>,
    /// Download details for a file stored in Google Drive.
    #[serde(rename="driveDownloadDetails")]
    
    pub drive_download_details: Option<UnsampledReportDriveDownloadDetails>,
    /// The end date for the unsampled report.
    #[serde(rename="end-date")]
    
    pub end_date: Option<String>,
    /// The filters for the unsampled report.
    
    pub filters: Option<String>,
    /// Unsampled report ID.
    
    pub id: Option<String>,
    /// Resource type for an Analytics unsampled report.
    
    pub kind: Option<String>,
    /// The metrics for the unsampled report.
    
    pub metrics: Option<String>,
    /// View (Profile) ID to which this unsampled report belongs.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// The segment for the unsampled report.
    
    pub segment: Option<String>,
    /// Link for this unsampled report.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The start date for the unsampled report.
    #[serde(rename="start-date")]
    
    pub start_date: Option<String>,
    /// Status of this unsampled report. Possible values are PENDING, COMPLETED, or FAILED.
    
    pub status: Option<String>,
    /// Title of the unsampled report.
    
    pub title: Option<String>,
    /// Time this unsampled report was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Web property ID to which this unsampled report belongs. The web property ID is of the form UA-XXXXX-YY.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for UnsampledReport {}
impl client::ResponseResult for UnsampledReport {}


/// An unsampled report collection lists Analytics unsampled reports to which the user has access. Each view (profile) can have a set of unsampled reports. Each resource in the unsampled report collection corresponds to a single Analytics unsampled report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unsampled reports list management](ManagementUnsampledReportListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsampledReports {
    /// A list of unsampled reports.
    
    pub items: Option<Vec<UnsampledReport>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this unsampled report collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this unsampled report collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of resources in the result.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for UnsampledReports {}


/// Metadata returned for an upload operation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploads get management](ManagementUploadGetCall) (response)
/// * [uploads upload data management](ManagementUploadUploadDataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Upload {
    /// Account Id to which this upload belongs.
    #[serde(rename="accountId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub account_id: Option<i64>,
    /// Custom data source Id to which this data import belongs.
    #[serde(rename="customDataSourceId")]
    
    pub custom_data_source_id: Option<String>,
    /// Data import errors collection.
    
    pub errors: Option<Vec<String>>,
    /// A unique ID for this upload.
    
    pub id: Option<String>,
    /// Resource type for Analytics upload.
    
    pub kind: Option<String>,
    /// Upload status. Possible values: PENDING, COMPLETED, FAILED, DELETING, DELETED.
    
    pub status: Option<String>,
    /// Time this file is uploaded.
    #[serde(rename="uploadTime")]
    
    pub upload_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for Upload {}


/// Upload collection lists Analytics uploads to which the user has access. Each custom data source can have a set of uploads. Each resource in the upload collection corresponds to a single Analytics data upload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [uploads list management](ManagementUploadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Uploads {
    /// A list of uploads.
    
    pub items: Option<Vec<Upload>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this upload collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this upload collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of resources in the result.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::ResponseResult for Uploads {}


/// JSON template for a user deletion request resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [user deletion request upsert user deletion](UserDeletionUserDeletionRequestUpsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDeletionRequest {
    /// This marks the point in time for which all user data before should be deleted
    #[serde(rename="deletionRequestTime")]
    
    pub deletion_request_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Firebase Project Id
    #[serde(rename="firebaseProjectId")]
    
    pub firebase_project_id: Option<String>,
    /// User ID.
    
    pub id: Option<UserDeletionRequestId>,
    /// Value is "analytics#userDeletionRequest".
    
    pub kind: Option<String>,
    /// Property ID
    #[serde(rename="propertyId")]
    
    pub property_id: Option<String>,
    /// Web property ID of the form UA-XXXXX-YY.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::RequestValue for UserDeletionRequest {}
impl client::ResponseResult for UserDeletionRequest {}


/// JSON template for a user reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserRef {
    /// Email ID of this user.
    
    pub email: Option<String>,
    /// User ID.
    
    pub id: Option<String>,
    /// no description provided
    
    pub kind: Option<String>,
}

impl client::Part for UserRef {}


/// JSON template for a web property reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebPropertyRef {
    /// Account ID to which this web property belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Link for this web property.
    
    pub href: Option<String>,
    /// Web property ID of the form UA-XXXXX-YY.
    
    pub id: Option<String>,
    /// Internal ID for this web property.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Analytics web property reference.
    
    pub kind: Option<String>,
    /// Name of this web property.
    
    pub name: Option<String>,
}

impl client::Part for WebPropertyRef {}


/// JSON template for an Analytics WebPropertySummary. WebPropertySummary returns basic information (i.e., summary) for a web property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebPropertySummary {
    /// Web property ID of the form UA-XXXXX-YY.
    
    pub id: Option<String>,
    /// Internal ID for this web property.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for Analytics WebPropertySummary.
    
    pub kind: Option<String>,
    /// Level for this web property. Possible values are STANDARD or PREMIUM.
    
    pub level: Option<String>,
    /// Web property name.
    
    pub name: Option<String>,
    /// List of profiles under this web property.
    
    pub profiles: Option<Vec<ProfileSummary>>,
    /// Indicates whether this web property is starred or not.
    
    pub starred: Option<bool>,
    /// Website url for this web property.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::Part for WebPropertySummary {}


/// A web property collection lists Analytics web properties to which the user has access. Each resource in the collection corresponds to a single Analytics web property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [webproperties list management](ManagementWebpropertyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Webproperties {
    /// A list of web properties.
    
    pub items: Option<Vec<Webproperty>>,
    /// The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter.
    #[serde(rename="itemsPerPage")]
    
    pub items_per_page: Option<i32>,
    /// Collection type.
    
    pub kind: Option<String>,
    /// Link to next page for this web property collection.
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// Link to previous page for this web property collection.
    #[serde(rename="previousLink")]
    
    pub previous_link: Option<String>,
    /// The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The total number of results for the query, regardless of the number of results in the response.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
    /// Email ID of the authenticated user
    
    pub username: Option<String>,
}

impl client::ResponseResult for Webproperties {}


/// JSON template for an Analytics web property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [webproperties get management](ManagementWebpropertyGetCall) (response)
/// * [webproperties insert management](ManagementWebpropertyInsertCall) (request|response)
/// * [webproperties patch management](ManagementWebpropertyPatchCall) (request|response)
/// * [webproperties update management](ManagementWebpropertyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Webproperty {
    /// Account ID to which this web property belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Child link for this web property. Points to the list of views (profiles) for this web property.
    #[serde(rename="childLink")]
    
    pub child_link: Option<WebpropertyChildLink>,
    /// Time this web property was created.
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).
    /// Set to false to delete data associated with the user identifier automatically after the rentention period.
    /// This property cannot be set on insert.
    #[serde(rename="dataRetentionResetOnNewActivity")]
    
    pub data_retention_reset_on_new_activity: Option<bool>,
    /// The length of time for which user and event data is retained.
    /// This property cannot be set on insert.
    #[serde(rename="dataRetentionTtl")]
    
    pub data_retention_ttl: Option<String>,
    /// Default view (profile) ID.
    #[serde(rename="defaultProfileId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_profile_id: Option<i64>,
    /// Web property ID of the form UA-XXXXX-YY.
    
    pub id: Option<String>,
    /// The industry vertical/category selected for this web property.
    #[serde(rename="industryVertical")]
    
    pub industry_vertical: Option<String>,
    /// Internal ID for this web property.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// Resource type for Analytics WebProperty.
    
    pub kind: Option<String>,
    /// Level for this web property. Possible values are STANDARD or PREMIUM.
    
    pub level: Option<String>,
    /// Name of this web property.
    
    pub name: Option<String>,
    /// Parent link for this web property. Points to the account to which this web property belongs.
    #[serde(rename="parentLink")]
    
    pub parent_link: Option<WebpropertyParentLink>,
    /// Permissions the user has for this web property.
    
    pub permissions: Option<WebpropertyPermissions>,
    /// View (Profile) count for this web property.
    #[serde(rename="profileCount")]
    
    pub profile_count: Option<i32>,
    /// Link for this web property.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Indicates whether this web property is starred or not.
    
    pub starred: Option<bool>,
    /// Time this web property was last modified.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Website url for this web property.
    #[serde(rename="websiteUrl")]
    
    pub website_url: Option<String>,
}

impl client::RequestValue for Webproperty {}
impl client::ResponseResult for Webproperty {}


/// Child link for an account entry. Points to the list of web properties for this account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountChildLink {
    /// Link to the list of web properties for this account.
    
    pub href: Option<String>,
    /// Type of the child link. Its value is "analytics#webproperties".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for AccountChildLink {}
impl client::Part for AccountChildLink {}


/// Permissions the user has for this account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountPermissions {
    /// All the permissions that the user has for this account. These include any implied permissions (e.g., EDIT implies VIEW).
    
    pub effective: Option<Vec<String>>,
}

impl client::NestedType for AccountPermissions {}
impl client::Part for AccountPermissions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDataSourceChildLink {
    /// Link to the list of daily uploads for this custom data source. Link to the list of uploads for this custom data source.
    
    pub href: Option<String>,
    /// Value is "analytics#dailyUploads". Value is "analytics#uploads".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for CustomDataSourceChildLink {}
impl client::Part for CustomDataSourceChildLink {}


/// Parent link for this custom data source. Points to the web property to which this custom data source belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDataSourceParentLink {
    /// Link to the web property to which this custom data source belongs.
    
    pub href: Option<String>,
    /// Value is "analytics#webproperty".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for CustomDataSourceParentLink {}
impl client::Part for CustomDataSourceParentLink {}


/// Parent link for the custom dimension. Points to the property to which the custom dimension belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomDimensionParentLink {
    /// Link to the property to which the custom dimension belongs.
    
    pub href: Option<String>,
    /// Type of the parent link. Set to "analytics#webproperty".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for CustomDimensionParentLink {}
impl client::Part for CustomDimensionParentLink {}


/// Parent link for the custom metric. Points to the property to which the custom metric belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomMetricParentLink {
    /// Link to the property to which the custom metric belongs.
    
    pub href: Option<String>,
    /// Type of the parent link. Set to "analytics#webproperty".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for CustomMetricParentLink {}
impl client::Part for CustomMetricParentLink {}


/// Web property being linked.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityAdWordsLinkEntity {
    /// no description provided
    #[serde(rename="webPropertyRef")]
    
    pub web_property_ref: Option<WebPropertyRef>,
}

impl client::NestedType for EntityAdWordsLinkEntity {}
impl client::Part for EntityAdWordsLinkEntity {}


/// Entity for this link. It can be an account, a web property, or a view (profile).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityUserLinkEntity {
    /// Account for this link.
    #[serde(rename="accountRef")]
    
    pub account_ref: Option<AccountRef>,
    /// View (Profile) for this link.
    #[serde(rename="profileRef")]
    
    pub profile_ref: Option<ProfileRef>,
    /// Web property for this link.
    #[serde(rename="webPropertyRef")]
    
    pub web_property_ref: Option<WebPropertyRef>,
}

impl client::NestedType for EntityUserLinkEntity {}
impl client::Part for EntityUserLinkEntity {}


/// Permissions the user has for this entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityUserLinkPermissions {
    /// Effective permissions represent all the permissions that a user has for this entity. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent entity. Effective permissions are read-only.
    
    pub effective: Option<Vec<String>>,
    /// Permissions that a user has been assigned at this very level. Does not include any implied or inherited permissions. Local permissions are modifiable.
    
    pub local: Option<Vec<String>>,
}

impl client::NestedType for EntityUserLinkPermissions {}
impl client::Part for EntityUserLinkPermissions {}


/// Parent link for an experiment. Points to the view (profile) to which this experiment belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExperimentParentLink {
    /// Link to the view (profile) to which this experiment belongs. This field is read-only.
    
    pub href: Option<String>,
    /// Value is "analytics#profile". This field is read-only.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for ExperimentParentLink {}
impl client::Part for ExperimentParentLink {}


/// Array of variations. The first variation in the array is the original. The number of variations may not change once an experiment is in the RUNNING state. At least two variations are required before status can be set to RUNNING.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExperimentVariations {
    /// The name of the variation. This field is required when creating an experiment. This field may not be changed for an experiment whose status is ENDED.
    
    pub name: Option<String>,
    /// Status of the variation. Possible values: "ACTIVE", "INACTIVE". INACTIVE variations are not served. This field may not be changed for an experiment whose status is ENDED.
    
    pub status: Option<String>,
    /// The URL of the variation. This field may not be changed for an experiment whose status is RUNNING or ENDED.
    
    pub url: Option<String>,
    /// Weight that this variation should receive. Only present if the experiment is running. This field is read-only.
    
    pub weight: Option<f64>,
    /// True if the experiment has ended and this variation performed (statistically) significantly better than the original. This field is read-only.
    
    pub won: Option<bool>,
}

impl client::NestedType for ExperimentVariations {}
impl client::Part for ExperimentVariations {}


/// Details for the filter of the type ADVANCED.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterAdvancedDetails {
    /// Indicates if the filter expressions are case sensitive.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// Expression to extract from field A.
    #[serde(rename="extractA")]
    
    pub extract_a: Option<String>,
    /// Expression to extract from field B.
    #[serde(rename="extractB")]
    
    pub extract_b: Option<String>,
    /// Field A.
    #[serde(rename="fieldA")]
    
    pub field_a: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="fieldAIndex")]
    
    pub field_a_index: Option<i32>,
    /// Indicates if field A is required to match.
    #[serde(rename="fieldARequired")]
    
    pub field_a_required: Option<bool>,
    /// Field B.
    #[serde(rename="fieldB")]
    
    pub field_b: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="fieldBIndex")]
    
    pub field_b_index: Option<i32>,
    /// Indicates if field B is required to match.
    #[serde(rename="fieldBRequired")]
    
    pub field_b_required: Option<bool>,
    /// Expression used to construct the output value.
    #[serde(rename="outputConstructor")]
    
    pub output_constructor: Option<String>,
    /// Output field.
    #[serde(rename="outputToField")]
    
    pub output_to_field: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="outputToFieldIndex")]
    
    pub output_to_field_index: Option<i32>,
    /// Indicates if the existing value of the output field, if any, should be overridden by the output expression.
    #[serde(rename="overrideOutputField")]
    
    pub override_output_field: Option<bool>,
}

impl client::NestedType for FilterAdvancedDetails {}
impl client::Part for FilterAdvancedDetails {}


/// Details for the filter of the type LOWER.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterLowercaseDetails {
    /// Field to use in the filter.
    
    pub field: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="fieldIndex")]
    
    pub field_index: Option<i32>,
}

impl client::NestedType for FilterLowercaseDetails {}
impl client::Part for FilterLowercaseDetails {}


/// Parent link for this filter. Points to the account to which this filter belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterParentLink {
    /// Link to the account to which this filter belongs.
    
    pub href: Option<String>,
    /// Value is "analytics#account".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for FilterParentLink {}
impl client::Part for FilterParentLink {}


/// Details for the filter of the type SEARCH_AND_REPLACE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterSearchAndReplaceDetails {
    /// Determines if the filter is case sensitive.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// Field to use in the filter.
    
    pub field: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="fieldIndex")]
    
    pub field_index: Option<i32>,
    /// Term to replace the search term with.
    #[serde(rename="replaceString")]
    
    pub replace_string: Option<String>,
    /// Term to search.
    #[serde(rename="searchString")]
    
    pub search_string: Option<String>,
}

impl client::NestedType for FilterSearchAndReplaceDetails {}
impl client::Part for FilterSearchAndReplaceDetails {}


/// Details for the filter of the type UPPER.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterUppercaseDetails {
    /// Field to use in the filter.
    
    pub field: Option<String>,
    /// The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION.
    #[serde(rename="fieldIndex")]
    
    pub field_index: Option<i32>,
}

impl client::NestedType for FilterUppercaseDetails {}
impl client::Part for FilterUppercaseDetails {}


/// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataColumnHeaders {
    /// Column Type. Either DIMENSION or METRIC.
    #[serde(rename="columnType")]
    
    pub column_type: Option<String>,
    /// Data type. Dimension column headers have only STRING as the data type. Metric column headers have data types for metric values such as INTEGER, DOUBLE, CURRENCY etc.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column name.
    
    pub name: Option<String>,
}

impl client::NestedType for GaDataColumnHeaders {}
impl client::Part for GaDataColumnHeaders {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataDataTable {
    /// no description provided
    
    pub cols: Option<Vec<GaDataDataTableCols>>,
    /// no description provided
    
    pub rows: Option<Vec<GaDataDataTableRows>>,
}

impl client::NestedType for GaDataDataTable {}
impl client::Part for GaDataDataTable {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataDataTableCols {
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub label: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for GaDataDataTableCols {}
impl client::Part for GaDataDataTableCols {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataDataTableRows {
    /// no description provided
    
    pub c: Option<Vec<GaDataDataTableRowsC>>,
}

impl client::NestedType for GaDataDataTableRows {}
impl client::Part for GaDataDataTableRows {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataDataTableRowsC {
    /// no description provided
    
    pub v: Option<String>,
}

impl client::NestedType for GaDataDataTableRowsC {}
impl client::Part for GaDataDataTableRowsC {}


/// Information for the view (profile), for which the Analytics data was requested.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataProfileInfo {
    /// Account ID to which this view (profile) belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Internal ID for the web property to which this view (profile) belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// View (Profile) ID.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// View (Profile) name.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// Table ID for view (profile).
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// Web Property ID to which this view (profile) belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::NestedType for GaDataProfileInfo {}
impl client::Part for GaDataProfileInfo {}


/// Analytics data request query parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GaDataQuery {
    /// List of analytics dimensions.
    
    pub dimensions: Option<String>,
    /// End date.
    #[serde(rename="end-date")]
    
    pub end_date: Option<String>,
    /// Comma-separated list of dimension or metric filters.
    
    pub filters: Option<String>,
    /// Unique table ID.
    
    pub ids: Option<String>,
    /// Maximum results per page.
    #[serde(rename="max-results")]
    
    pub max_results: Option<i32>,
    /// List of analytics metrics.
    
    pub metrics: Option<Vec<String>>,
    /// Desired sampling level
    #[serde(rename="samplingLevel")]
    
    pub sampling_level: Option<String>,
    /// Analytics advanced segment.
    
    pub segment: Option<String>,
    /// List of dimensions or metrics based on which Analytics data is sorted.
    
    pub sort: Option<Vec<String>>,
    /// Start date.
    #[serde(rename="start-date")]
    
    pub start_date: Option<String>,
    /// Start index.
    #[serde(rename="start-index")]
    
    pub start_index: Option<i32>,
}

impl client::NestedType for GaDataQuery {}
impl client::Part for GaDataQuery {}


/// Details for the goal of the type EVENT.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalEventDetails {
    /// List of event conditions.
    #[serde(rename="eventConditions")]
    
    pub event_conditions: Option<Vec<GoalEventDetailsEventConditions>>,
    /// Determines if the event value should be used as the value for this goal.
    #[serde(rename="useEventValue")]
    
    pub use_event_value: Option<bool>,
}

impl client::NestedType for GoalEventDetails {}
impl client::Part for GoalEventDetails {}


/// List of event conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalEventDetailsEventConditions {
    /// Type of comparison. Possible values are LESS_THAN, GREATER_THAN or EQUAL.
    #[serde(rename="comparisonType")]
    
    pub comparison_type: Option<String>,
    /// Value used for this comparison.
    #[serde(rename="comparisonValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub comparison_value: Option<i64>,
    /// Expression used for this match.
    
    pub expression: Option<String>,
    /// Type of the match to be performed. Possible values are REGEXP, BEGINS_WITH, or EXACT.
    #[serde(rename="matchType")]
    
    pub match_type: Option<String>,
    /// Type of this event condition. Possible values are CATEGORY, ACTION, LABEL, or VALUE.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for GoalEventDetailsEventConditions {}
impl client::Part for GoalEventDetailsEventConditions {}


/// Parent link for a goal. Points to the view (profile) to which this goal belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalParentLink {
    /// Link to the view (profile) to which this goal belongs.
    
    pub href: Option<String>,
    /// Value is "analytics#profile".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for GoalParentLink {}
impl client::Part for GoalParentLink {}


/// Details for the goal of the type URL_DESTINATION.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalUrlDestinationDetails {
    /// Determines if the goal URL must exactly match the capitalization of visited URLs.
    #[serde(rename="caseSensitive")]
    
    pub case_sensitive: Option<bool>,
    /// Determines if the first step in this goal is required.
    #[serde(rename="firstStepRequired")]
    
    pub first_step_required: Option<bool>,
    /// Match type for the goal URL. Possible values are HEAD, EXACT, or REGEX.
    #[serde(rename="matchType")]
    
    pub match_type: Option<String>,
    /// List of steps configured for this goal funnel.
    
    pub steps: Option<Vec<GoalUrlDestinationDetailsSteps>>,
    /// URL for this goal.
    
    pub url: Option<String>,
}

impl client::NestedType for GoalUrlDestinationDetails {}
impl client::Part for GoalUrlDestinationDetails {}


/// List of steps configured for this goal funnel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalUrlDestinationDetailsSteps {
    /// Step name.
    
    pub name: Option<String>,
    /// Step number.
    
    pub number: Option<i32>,
    /// URL for this step.
    
    pub url: Option<String>,
}

impl client::NestedType for GoalUrlDestinationDetailsSteps {}
impl client::Part for GoalUrlDestinationDetailsSteps {}


/// Details for the goal of the type VISIT_NUM_PAGES.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalVisitNumPagesDetails {
    /// Type of comparison. Possible values are LESS_THAN, GREATER_THAN, or EQUAL.
    #[serde(rename="comparisonType")]
    
    pub comparison_type: Option<String>,
    /// Value used for this comparison.
    #[serde(rename="comparisonValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub comparison_value: Option<i64>,
}

impl client::NestedType for GoalVisitNumPagesDetails {}
impl client::Part for GoalVisitNumPagesDetails {}


/// Details for the goal of the type VISIT_TIME_ON_SITE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoalVisitTimeOnSiteDetails {
    /// Type of comparison. Possible values are LESS_THAN or GREATER_THAN.
    #[serde(rename="comparisonType")]
    
    pub comparison_type: Option<String>,
    /// Value used for this comparison.
    #[serde(rename="comparisonValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub comparison_value: Option<i64>,
}

impl client::NestedType for GoalVisitTimeOnSiteDetails {}
impl client::Part for GoalVisitTimeOnSiteDetails {}


/// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfDataColumnHeaders {
    /// Column Type. Either DIMENSION or METRIC.
    #[serde(rename="columnType")]
    
    pub column_type: Option<String>,
    /// Data type. Dimension and metric values data types such as INTEGER, DOUBLE, CURRENCY, MCF_SEQUENCE etc.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column name.
    
    pub name: Option<String>,
}

impl client::NestedType for McfDataColumnHeaders {}
impl client::Part for McfDataColumnHeaders {}


/// Information for the view (profile), for which the Analytics data was requested.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfDataProfileInfo {
    /// Account ID to which this view (profile) belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Internal ID for the web property to which this view (profile) belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// View (Profile) ID.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// View (Profile) name.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// Table ID for view (profile).
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// Web Property ID to which this view (profile) belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::NestedType for McfDataProfileInfo {}
impl client::Part for McfDataProfileInfo {}


/// Analytics data request query parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfDataQuery {
    /// List of analytics dimensions.
    
    pub dimensions: Option<String>,
    /// End date.
    #[serde(rename="end-date")]
    
    pub end_date: Option<String>,
    /// Comma-separated list of dimension or metric filters.
    
    pub filters: Option<String>,
    /// Unique table ID.
    
    pub ids: Option<String>,
    /// Maximum results per page.
    #[serde(rename="max-results")]
    
    pub max_results: Option<i32>,
    /// List of analytics metrics.
    
    pub metrics: Option<Vec<String>>,
    /// Desired sampling level
    #[serde(rename="samplingLevel")]
    
    pub sampling_level: Option<String>,
    /// Analytics advanced segment.
    
    pub segment: Option<String>,
    /// List of dimensions or metrics based on which Analytics data is sorted.
    
    pub sort: Option<Vec<String>>,
    /// Start date.
    #[serde(rename="start-date")]
    
    pub start_date: Option<String>,
    /// Start index.
    #[serde(rename="start-index")]
    
    pub start_index: Option<i32>,
}

impl client::NestedType for McfDataQuery {}
impl client::Part for McfDataQuery {}


/// A union object representing a dimension or metric value. Only one of "primitiveValue" or "conversionPathValue" attribute will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfDataRows {
    /// A conversion path dimension value, containing a list of interactions with their attributes.
    #[serde(rename="conversionPathValue")]
    
    pub conversion_path_value: Option<Vec<McfDataRowsConversionPathValue>>,
    /// A primitive dimension value. A primitive metric value.
    #[serde(rename="primitiveValue")]
    
    pub primitive_value: Option<String>,
}

impl client::NestedType for McfDataRows {}
impl client::Part for McfDataRows {}


/// A conversion path dimension value, containing a list of interactions with their attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct McfDataRowsConversionPathValue {
    /// Type of an interaction on conversion path. Such as CLICK, IMPRESSION etc.
    #[serde(rename="interactionType")]
    
    pub interaction_type: Option<String>,
    /// Node value of an interaction on conversion path. Such as source, medium etc.
    #[serde(rename="nodeValue")]
    
    pub node_value: Option<String>,
}

impl client::NestedType for McfDataRowsConversionPathValue {}
impl client::Part for McfDataRowsConversionPathValue {}


/// Child link for this view (profile). Points to the list of goals for this view (profile).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileChildLink {
    /// Link to the list of goals for this view (profile).
    
    pub href: Option<String>,
    /// Value is "analytics#goals".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for ProfileChildLink {}
impl client::Part for ProfileChildLink {}


/// Parent link for this view (profile). Points to the web property to which this view (profile) belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileParentLink {
    /// Link to the web property to which this view (profile) belongs.
    
    pub href: Option<String>,
    /// Value is "analytics#webproperty".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for ProfileParentLink {}
impl client::Part for ProfileParentLink {}


/// Permissions the user has for this view (profile).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfilePermissions {
    /// All the permissions that the user has for this view (profile). These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent web property.
    
    pub effective: Option<Vec<String>>,
}

impl client::NestedType for ProfilePermissions {}
impl client::Part for ProfilePermissions {}


/// Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeDataColumnHeaders {
    /// Column Type. Either DIMENSION or METRIC.
    #[serde(rename="columnType")]
    
    pub column_type: Option<String>,
    /// Data type. Dimension column headers have only STRING as the data type. Metric column headers have data types for metric values such as INTEGER, DOUBLE, CURRENCY etc.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Column name.
    
    pub name: Option<String>,
}

impl client::NestedType for RealtimeDataColumnHeaders {}
impl client::Part for RealtimeDataColumnHeaders {}


/// Information for the view (profile), for which the real time data was requested.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeDataProfileInfo {
    /// Account ID to which this view (profile) belongs.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// Internal ID for the web property to which this view (profile) belongs.
    #[serde(rename="internalWebPropertyId")]
    
    pub internal_web_property_id: Option<String>,
    /// View (Profile) ID.
    #[serde(rename="profileId")]
    
    pub profile_id: Option<String>,
    /// View (Profile) name.
    #[serde(rename="profileName")]
    
    pub profile_name: Option<String>,
    /// Table ID for view (profile).
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// Web Property ID to which this view (profile) belongs.
    #[serde(rename="webPropertyId")]
    
    pub web_property_id: Option<String>,
}

impl client::NestedType for RealtimeDataProfileInfo {}
impl client::Part for RealtimeDataProfileInfo {}


/// Real time data request query parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RealtimeDataQuery {
    /// List of real time dimensions.
    
    pub dimensions: Option<String>,
    /// Comma-separated list of dimension or metric filters.
    
    pub filters: Option<String>,
    /// Unique table ID.
    
    pub ids: Option<String>,
    /// Maximum results per page.
    #[serde(rename="max-results")]
    
    pub max_results: Option<i32>,
    /// List of real time metrics.
    
    pub metrics: Option<Vec<String>>,
    /// List of dimensions or metrics based on which real time data is sorted.
    
    pub sort: Option<Vec<String>>,
}

impl client::NestedType for RealtimeDataQuery {}
impl client::Part for RealtimeDataQuery {}


/// The simple audience definition that will cause a user to be added to an audience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingAudienceAudienceDefinition {
    /// Defines the conditions to include users to the audience.
    #[serde(rename="includeConditions")]
    
    pub include_conditions: Option<IncludeConditions>,
}

impl client::NestedType for RemarketingAudienceAudienceDefinition {}
impl client::Part for RemarketingAudienceAudienceDefinition {}


/// A state based audience definition that will cause a user to be added or removed from an audience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingAudienceStateBasedAudienceDefinition {
    /// Defines the conditions to exclude users from the audience.
    #[serde(rename="excludeConditions")]
    
    pub exclude_conditions: Option<RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions>,
    /// Defines the conditions to include users to the audience.
    #[serde(rename="includeConditions")]
    
    pub include_conditions: Option<IncludeConditions>,
}

impl client::NestedType for RemarketingAudienceStateBasedAudienceDefinition {}
impl client::Part for RemarketingAudienceStateBasedAudienceDefinition {}


/// Defines the conditions to exclude users from the audience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions {
    /// Whether to make the exclusion TEMPORARY or PERMANENT.
    #[serde(rename="exclusionDuration")]
    
    pub exclusion_duration: Option<String>,
    /// The segment condition that will cause a user to be removed from an audience.
    
    pub segment: Option<String>,
}

impl client::NestedType for RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions {}
impl client::Part for RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions {}


/// Download details for a file stored in Google Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsampledReportCloudStorageDownloadDetails {
    /// Id of the bucket the file object is stored in.
    #[serde(rename="bucketId")]
    
    pub bucket_id: Option<String>,
    /// Id of the file object containing the report data.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
}

impl client::NestedType for UnsampledReportCloudStorageDownloadDetails {}
impl client::Part for UnsampledReportCloudStorageDownloadDetails {}


/// Download details for a file stored in Google Drive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnsampledReportDriveDownloadDetails {
    /// Id of the document/file containing the report data.
    #[serde(rename="documentId")]
    
    pub document_id: Option<String>,
}

impl client::NestedType for UnsampledReportDriveDownloadDetails {}
impl client::Part for UnsampledReportDriveDownloadDetails {}


/// User ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDeletionRequestId {
    /// Type of user
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The User's id
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::NestedType for UserDeletionRequestId {}
impl client::Part for UserDeletionRequestId {}


/// Child link for this web property. Points to the list of views (profiles) for this web property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpropertyChildLink {
    /// Link to the list of views (profiles) for this web property.
    
    pub href: Option<String>,
    /// Type of the parent link. Its value is "analytics#profiles".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for WebpropertyChildLink {}
impl client::Part for WebpropertyChildLink {}


/// Parent link for this web property. Points to the account to which this web property belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpropertyParentLink {
    /// Link to the account for this web property.
    
    pub href: Option<String>,
    /// Type of the parent link. Its value is "analytics#account".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for WebpropertyParentLink {}
impl client::Part for WebpropertyParentLink {}


/// Permissions the user has for this web property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebpropertyPermissions {
    /// All the permissions that the user has for this web property. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent account.
    
    pub effective: Option<Vec<String>>,
}

impl client::NestedType for WebpropertyPermissions {}
impl client::Part for WebpropertyPermissions {}


