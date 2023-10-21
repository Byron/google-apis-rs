use super::*;
/// Represents the aggregation level and interval for pricing of a single SKU.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationInfo {
    /// The number of intervals to aggregate over. Example: If aggregation_level is "DAILY" and aggregation_count is 14, aggregation will be over 14 days.
    #[serde(rename="aggregationCount")]
    
    pub aggregation_count: Option<i32>,
    /// no description provided
    #[serde(rename="aggregationInterval")]
    
    pub aggregation_interval: Option<String>,
    /// no description provided
    #[serde(rename="aggregationLevel")]
    
    pub aggregation_level: Option<String>,
}

impl client::Part for AggregationInfo {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for AuditLogConfig {}


/// A billing account in the [Google Cloud Console](https://console.cloud.google.com/). You can assign a billing account to one or more projects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects list billing accounts](BillingAccountProjectListCall) (none)
/// * [create billing accounts](BillingAccountCreateCall) (request|response)
/// * [get billing accounts](BillingAccountGetCall) (response)
/// * [get iam policy billing accounts](BillingAccountGetIamPolicyCall) (none)
/// * [list billing accounts](BillingAccountListCall) (none)
/// * [patch billing accounts](BillingAccountPatchCall) (request|response)
/// * [set iam policy billing accounts](BillingAccountSetIamPolicyCall) (none)
/// * [test iam permissions billing accounts](BillingAccountTestIamPermissionCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BillingAccount {
    /// The display name given to the billing account, such as `My Billing Account`. This name is displayed in the Google Cloud Console.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// If this account is a [subaccount](https://cloud.google.com/billing/docs/concepts), then this will be the resource name of the parent billing account that it is being resold through. Otherwise this will be empty.
    #[serde(rename="masterBillingAccount")]
    
    pub master_billing_account: Option<String>,
    /// Output only. The resource name of the billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF` would be the resource name for billing account `012345-567890-ABCDEF`.
    
    pub name: Option<String>,
    /// Output only. True if the billing account is open, and will therefore be charged for any usage on associated projects. False if the billing account is closed, and therefore projects associated with it will be unable to use paid services.
    
    pub open: Option<bool>,
}

impl client::RequestValue for BillingAccount {}
impl client::Resource for BillingAccount {}
impl client::ResponseResult for BillingAccount {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// Represents the category hierarchy of a SKU.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// The type of product the SKU refers to. Example: "Compute", "Storage", "Network", "ApplicationServices" etc.
    #[serde(rename="resourceFamily")]
    
    pub resource_family: Option<String>,
    /// A group classification for related SKUs. Example: "RAM", "GPU", "Prediction", "Ops", "GoogleEgress" etc.
    #[serde(rename="resourceGroup")]
    
    pub resource_group: Option<String>,
    /// The display name of the service this SKU belongs to.
    #[serde(rename="serviceDisplayName")]
    
    pub service_display_name: Option<String>,
    /// Represents how the SKU is consumed. Example: "OnDemand", "Preemptible", "Commit1Mo", "Commit1Yr" etc.
    #[serde(rename="usageType")]
    
    pub usage_type: Option<String>,
}

impl client::Part for Category {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Encapsulates the geographic taxonomy data for a sku.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoTaxonomy {
    /// The list of regions associated with a sku. Empty for Global skus, which are associated with all Google Cloud regions.
    
    pub regions: Option<Vec<String>>,
    /// The type of Geo Taxonomy: GLOBAL, REGIONAL, or MULTI_REGIONAL.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GeoTaxonomy {}


/// Response message for `ListBillingAccounts`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list billing accounts](BillingAccountListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBillingAccountsResponse {
    /// A list of billing accounts.
    #[serde(rename="billingAccounts")]
    
    pub billing_accounts: Option<Vec<BillingAccount>>,
    /// A token to retrieve the next page of results. To retrieve the next page, call `ListBillingAccounts` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBillingAccountsResponse {}


/// Request message for `ListProjectBillingInfoResponse`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [projects list billing accounts](BillingAccountProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProjectBillingInfoResponse {
    /// A token to retrieve the next page of results. To retrieve the next page, call `ListProjectBillingInfo` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of `ProjectBillingInfo` resources representing the projects associated with the billing account.
    #[serde(rename="projectBillingInfo")]
    
    pub project_billing_info: Option<Vec<ProjectBillingInfo>>,
}

impl client::ResponseResult for ListProjectBillingInfoResponse {}


/// Response message for `ListServices`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list services](ServiceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListServicesResponse {
    /// A token to retrieve the next page of results. To retrieve the next page, call `ListServices` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of services.
    
    pub services: Option<Vec<Service>>,
}

impl client::ResponseResult for ListServicesResponse {}


/// Response message for `ListSkus`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [skus list services](ServiceSkuListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSkusResponse {
    /// A token to retrieve the next page of results. To retrieve the next page, call `ListSkus` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of public SKUs of the given service.
    
    pub skus: Option<Vec<Sku>>,
}

impl client::ResponseResult for ListSkusResponse {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy billing accounts](BillingAccountGetIamPolicyCall) (response)
/// * [set iam policy billing accounts](BillingAccountSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Expresses a mathematical pricing formula. For Example:- `usage_unit: GBy` `tiered_rates:` `[start_usage_amount: 20, unit_price: $10]` `[start_usage_amount: 100, unit_price: $5]` The above expresses a pricing formula where the first 20GB is free, the next 80GB is priced at $10 per GB followed by $5 per GB for additional usage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricingExpression {
    /// The base unit for the SKU which is the unit used in usage exports. Example: "By"
    #[serde(rename="baseUnit")]
    
    pub base_unit: Option<String>,
    /// Conversion factor for converting from price per usage_unit to price per base_unit, and start_usage_amount to start_usage_amount in base_unit. unit_price / base_unit_conversion_factor = price per base_unit. start_usage_amount * base_unit_conversion_factor = start_usage_amount in base_unit.
    #[serde(rename="baseUnitConversionFactor")]
    
    pub base_unit_conversion_factor: Option<f64>,
    /// The base unit in human readable form. Example: "byte".
    #[serde(rename="baseUnitDescription")]
    
    pub base_unit_description: Option<String>,
    /// The recommended quantity of units for displaying pricing info. When displaying pricing info it is recommended to display: (unit_price * display_quantity) per display_quantity usage_unit. This field does not affect the pricing formula and is for display purposes only. Example: If the unit_price is "0.0001 USD", the usage_unit is "GB" and the display_quantity is "1000" then the recommended way of displaying the pricing info is "0.10 USD per 1000 GB"
    #[serde(rename="displayQuantity")]
    
    pub display_quantity: Option<f64>,
    /// The list of tiered rates for this pricing. The total cost is computed by applying each of the tiered rates on usage. This repeated list is sorted by ascending order of start_usage_amount.
    #[serde(rename="tieredRates")]
    
    pub tiered_rates: Option<Vec<TierRate>>,
    /// The short hand for unit of usage this pricing is specified in. Example: usage_unit of "GiBy" means that usage is specified in "Gibi Byte".
    #[serde(rename="usageUnit")]
    
    pub usage_unit: Option<String>,
    /// The unit of usage in human readable form. Example: "gibi byte".
    #[serde(rename="usageUnitDescription")]
    
    pub usage_unit_description: Option<String>,
}

impl client::Part for PricingExpression {}


/// Represents the pricing information for a SKU at a single point of time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PricingInfo {
    /// Aggregation Info. This can be left unspecified if the pricing expression doesn't require aggregation.
    #[serde(rename="aggregationInfo")]
    
    pub aggregation_info: Option<AggregationInfo>,
    /// Conversion rate used for currency conversion, from USD to the currency specified in the request. This includes any surcharge collected for billing in non USD currency. If a currency is not specified in the request this defaults to 1.0. Example: USD * currency_conversion_rate = JPY
    #[serde(rename="currencyConversionRate")]
    
    pub currency_conversion_rate: Option<f64>,
    /// The timestamp from which this pricing was effective within the requested time range. This is guaranteed to be greater than or equal to the start_time field in the request and less than the end_time field in the request. If a time range was not specified in the request this field will be equivalent to a time within the last 12 hours, indicating the latest pricing info.
    #[serde(rename="effectiveTime")]
    
    pub effective_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Expresses the pricing formula. See `PricingExpression` for an example.
    #[serde(rename="pricingExpression")]
    
    pub pricing_expression: Option<PricingExpression>,
    /// An optional human readable summary of the pricing information, has a maximum length of 256 characters.
    
    pub summary: Option<String>,
}

impl client::Part for PricingInfo {}


/// Encapsulation of billing information for a Google Cloud Console project. A project has at most one associated billing account at a time (but a billing account can be assigned to multiple projects).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get billing info projects](ProjectGetBillingInfoCall) (response)
/// * [update billing info projects](ProjectUpdateBillingInfoCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBillingInfo {
    /// The resource name of the billing account associated with the project, if any. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[serde(rename="billingAccountName")]
    
    pub billing_account_name: Option<String>,
    /// True if the project is associated with an open billing account, to which usage on the project is charged. False if the project is associated with a closed billing account, or no billing account at all, and therefore cannot use paid services. This field is read-only.
    #[serde(rename="billingEnabled")]
    
    pub billing_enabled: Option<bool>,
    /// The resource name for the `ProjectBillingInfo`; has the form `projects/{project_id}/billingInfo`. For example, the resource name for the billing information for project `tokyo-rain-123` would be `projects/tokyo-rain-123/billingInfo`. This field is read-only.
    
    pub name: Option<String>,
    /// The ID of the project that this `ProjectBillingInfo` represents, such as `tokyo-rain-123`. This is a convenience field so that you don't need to parse the `name` field to obtain a project ID. This field is read-only.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::RequestValue for ProjectBillingInfo {}
impl client::ResponseResult for ProjectBillingInfo {}


/// Encapsulates a single service in Google Cloud Platform.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [skus list services](ServiceSkuListCall) (none)
/// * [list services](ServiceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    /// The business under which the service is offered. Ex. "businessEntities/GCP", "businessEntities/Maps"
    #[serde(rename="businessEntityName")]
    
    pub business_entity_name: Option<String>,
    /// A human readable display name for this service.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name for the service. Example: "services/DA34-426B-A397"
    
    pub name: Option<String>,
    /// The identifier for the service. Example: "DA34-426B-A397"
    #[serde(rename="serviceId")]
    
    pub service_id: Option<String>,
}

impl client::Resource for Service {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy billing accounts](BillingAccountSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Encapsulates a single SKU in Google Cloud Platform
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sku {
    /// The category hierarchy of this SKU, purely for organizational purpose.
    
    pub category: Option<Category>,
    /// A human readable description of the SKU, has a maximum length of 256 characters.
    
    pub description: Option<String>,
    /// The geographic taxonomy for this sku.
    #[serde(rename="geoTaxonomy")]
    
    pub geo_taxonomy: Option<GeoTaxonomy>,
    /// The resource name for the SKU. Example: "services/DA34-426B-A397/skus/AA95-CD31-42FE"
    
    pub name: Option<String>,
    /// A timeline of pricing info for this SKU in chronological order.
    #[serde(rename="pricingInfo")]
    
    pub pricing_info: Option<Vec<PricingInfo>>,
    /// Identifies the service provider. This is 'Google' for first party services in Google Cloud Platform.
    #[serde(rename="serviceProviderName")]
    
    pub service_provider_name: Option<String>,
    /// List of service regions this SKU is offered at. Example: "asia-east1" Service regions can be found at https://cloud.google.com/about/locations/
    #[serde(rename="serviceRegions")]
    
    pub service_regions: Option<Vec<String>>,
    /// The identifier for the SKU. Example: "AA95-CD31-42FE"
    #[serde(rename="skuId")]
    
    pub sku_id: Option<String>,
}

impl client::Part for Sku {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions billing accounts](BillingAccountTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions billing accounts](BillingAccountTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// The price rate indicating starting usage and its corresponding price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TierRate {
    /// Usage is priced at this rate only after this amount. Example: start_usage_amount of 10 indicates that the usage will be priced at the unit_price after the first 10 usage_units.
    #[serde(rename="startUsageAmount")]
    
    pub start_usage_amount: Option<f64>,
    /// The price per unit of usage. Example: unit_price of amount $10 indicates that each unit will cost $10.
    #[serde(rename="unitPrice")]
    
    pub unit_price: Option<Money>,
}

impl client::Part for TierRate {}


