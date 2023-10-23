use super::*;
/// Contains metadata about how much money a recommendation can save or incur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1CostProjection {
    /// An approximate projection on amount saved or amount incurred. Negative cost units indicate cost savings and positive cost units indicate increase. See google.type.Money documentation for positive/negative units. A user's permissions may affect whether the cost is computed using list prices or custom contract prices.
    
    pub cost: Option<GoogleTypeMoney>,
    /// Duration for which this cost applies.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
}

impl client::Part for GoogleCloudRecommenderV1CostProjection {}


/// Contains the impact a recommendation can have for a given category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1Impact {
    /// Category that is being targeted.
    
    pub category: Option<GoogleCloudRecommenderV1ImpactCategoryEnum>,
    /// Use with CategoryType.COST
    #[serde(rename="costProjection")]
    
    pub cost_projection: Option<GoogleCloudRecommenderV1CostProjection>,
    /// Use with CategoryType.RELAIBILITY
    #[serde(rename="reliabilityProjection")]
    
    pub reliability_projection: Option<GoogleCloudRecommenderV1ReliabilityProjection>,
    /// Use with CategoryType.SECURITY
    #[serde(rename="securityProjection")]
    
    pub security_projection: Option<GoogleCloudRecommenderV1SecurityProjection>,
    /// Use with CategoryType.SUSTAINABILITY
    #[serde(rename="sustainabilityProjection")]
    
    pub sustainability_projection: Option<GoogleCloudRecommenderV1SustainabilityProjection>,
}

impl client::Part for GoogleCloudRecommenderV1Impact {}


/// An insight along with the information used to derive the insight. The insight may have associated recommendations as well.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations insight types insights get billing accounts](BillingAccountLocationInsightTypeInsightGetCall) (response)
/// * [locations insight types insights mark accepted billing accounts](BillingAccountLocationInsightTypeInsightMarkAcceptedCall) (response)
/// * [locations insight types insights get folders](FolderLocationInsightTypeInsightGetCall) (response)
/// * [locations insight types insights mark accepted folders](FolderLocationInsightTypeInsightMarkAcceptedCall) (response)
/// * [locations insight types insights get organizations](OrganizationLocationInsightTypeInsightGetCall) (response)
/// * [locations insight types insights mark accepted organizations](OrganizationLocationInsightTypeInsightMarkAcceptedCall) (response)
/// * [locations insight types insights get projects](ProjectLocationInsightTypeInsightGetCall) (response)
/// * [locations insight types insights mark accepted projects](ProjectLocationInsightTypeInsightMarkAcceptedCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1Insight {
    /// Recommendations derived from this insight.
    #[serde(rename="associatedRecommendations")]
    
    pub associated_recommendations: Option<Vec<GoogleCloudRecommenderV1InsightRecommendationReference>>,
    /// Category being targeted by the insight.
    
    pub category: Option<GoogleCloudRecommenderV1InsightCategoryEnum>,
    /// A struct of custom fields to explain the insight. Example: "grantedPermissionsCount": "1000"
    
    pub content: Option<HashMap<String, json::Value>>,
    /// Free-form human readable summary in English. The maximum length is 500 characters.
    
    pub description: Option<String>,
    /// Fingerprint of the Insight. Provides optimistic locking when updating states.
    
    pub etag: Option<String>,
    /// Insight subtype. Insight content schema will be stable for a given subtype.
    #[serde(rename="insightSubtype")]
    
    pub insight_subtype: Option<String>,
    /// Timestamp of the latest data used to generate the insight.
    #[serde(rename="lastRefreshTime")]
    
    pub last_refresh_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of the insight.
    
    pub name: Option<String>,
    /// Observation period that led to the insight. The source data used to generate the insight ends at last_refresh_time and begins at (last_refresh_time - observation_period).
    #[serde(rename="observationPeriod")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub observation_period: Option<client::chrono::Duration>,
    /// Insight's severity.
    
    pub severity: Option<GoogleCloudRecommenderV1InsightSeverityEnum>,
    /// Information state and metadata.
    #[serde(rename="stateInfo")]
    
    pub state_info: Option<GoogleCloudRecommenderV1InsightStateInfo>,
    /// Fully qualified resource names that this insight is targeting.
    #[serde(rename="targetResources")]
    
    pub target_resources: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleCloudRecommenderV1Insight {}


/// Reference to an associated recommendation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1InsightRecommendationReference {
    /// Recommendation resource name, e.g. projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/recommendations/[RECOMMENDATION_ID]
    
    pub recommendation: Option<String>,
}

impl client::Part for GoogleCloudRecommenderV1InsightRecommendationReference {}


/// Information related to insight state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1InsightStateInfo {
    /// Insight state.
    
    pub state: Option<GoogleCloudRecommenderV1InsightStateInfoStateEnum>,
    /// A map of metadata for the state, provided by user or automations systems.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudRecommenderV1InsightStateInfo {}


/// Configuration for an InsightType.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations insight types get config billing accounts](BillingAccountLocationInsightTypeGetConfigCall) (response)
/// * [locations insight types update config billing accounts](BillingAccountLocationInsightTypeUpdateConfigCall) (request|response)
/// * [locations insight types get config organizations](OrganizationLocationInsightTypeGetConfigCall) (response)
/// * [locations insight types update config organizations](OrganizationLocationInsightTypeUpdateConfigCall) (request|response)
/// * [locations insight types get config projects](ProjectLocationInsightTypeGetConfigCall) (response)
/// * [locations insight types update config projects](ProjectLocationInsightTypeUpdateConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1InsightTypeConfig {
    /// Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.
    
    pub annotations: Option<HashMap<String, String>>,
    /// A user-settable field to provide a human-readable name to be used in user interfaces.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Fingerprint of the InsightTypeConfig. Provides optimistic locking when updating.
    
    pub etag: Option<String>,
    /// InsightTypeGenerationConfig which configures the generation of insights for this insight type.
    #[serde(rename="insightTypeGenerationConfig")]
    
    pub insight_type_generation_config: Option<GoogleCloudRecommenderV1InsightTypeGenerationConfig>,
    /// Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config
    
    pub name: Option<String>,
    /// Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Last time when the config was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1InsightTypeConfig {}
impl client::ResponseResult for GoogleCloudRecommenderV1InsightTypeConfig {}


/// A configuration to customize the generation of insights. Eg, customizing the lookback period considered when generating a insight.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1InsightTypeGenerationConfig {
    /// Parameters for this InsightTypeGenerationConfig. These configs can be used by or are applied to all subtypes.
    
    pub params: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudRecommenderV1InsightTypeGenerationConfig {}


/// Response to the `ListInsights` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations insight types insights list billing accounts](BillingAccountLocationInsightTypeInsightListCall) (response)
/// * [locations insight types insights list folders](FolderLocationInsightTypeInsightListCall) (response)
/// * [locations insight types insights list organizations](OrganizationLocationInsightTypeInsightListCall) (response)
/// * [locations insight types insights list projects](ProjectLocationInsightTypeInsightListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1ListInsightsResponse {
    /// The set of insights for the `parent` resource.
    
    pub insights: Option<Vec<GoogleCloudRecommenderV1Insight>>,
    /// A token that can be used to request the next page of results. This field is empty if there are no additional results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecommenderV1ListInsightsResponse {}


/// Response to the `ListRecommendations` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations list billing accounts](BillingAccountLocationRecommenderRecommendationListCall) (response)
/// * [locations recommenders recommendations list folders](FolderLocationRecommenderRecommendationListCall) (response)
/// * [locations recommenders recommendations list organizations](OrganizationLocationRecommenderRecommendationListCall) (response)
/// * [locations recommenders recommendations list projects](ProjectLocationRecommenderRecommendationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1ListRecommendationsResponse {
    /// A token that can be used to request the next page of results. This field is empty if there are no additional results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The set of recommendations for the `parent` resource.
    
    pub recommendations: Option<Vec<GoogleCloudRecommenderV1Recommendation>>,
}

impl client::ResponseResult for GoogleCloudRecommenderV1ListRecommendationsResponse {}


/// Request for the `MarkInsightAccepted` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations insight types insights mark accepted billing accounts](BillingAccountLocationInsightTypeInsightMarkAcceptedCall) (request)
/// * [locations insight types insights mark accepted folders](FolderLocationInsightTypeInsightMarkAcceptedCall) (request)
/// * [locations insight types insights mark accepted organizations](OrganizationLocationInsightTypeInsightMarkAcceptedCall) (request)
/// * [locations insight types insights mark accepted projects](ProjectLocationInsightTypeInsightMarkAcceptedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1MarkInsightAcceptedRequest {
    /// Required. Fingerprint of the Insight. Provides optimistic locking.
    
    pub etag: Option<String>,
    /// Optional. State properties user wish to include with this state. Full replace of the current state_metadata.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1MarkInsightAcceptedRequest {}


/// Request for the `MarkRecommendationClaimed` Method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations mark claimed billing accounts](BillingAccountLocationRecommenderRecommendationMarkClaimedCall) (request)
/// * [locations recommenders recommendations mark claimed folders](FolderLocationRecommenderRecommendationMarkClaimedCall) (request)
/// * [locations recommenders recommendations mark claimed organizations](OrganizationLocationRecommenderRecommendationMarkClaimedCall) (request)
/// * [locations recommenders recommendations mark claimed projects](ProjectLocationRecommenderRecommendationMarkClaimedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1MarkRecommendationClaimedRequest {
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    
    pub etag: Option<String>,
    /// State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex `/^a-z0-9{0,62}$/`. Values must match the regex `/^[a-zA-Z0-9_./-]{0,255}$/`.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1MarkRecommendationClaimedRequest {}


/// Request for the `MarkRecommendationDismissed` Method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations mark dismissed billing accounts](BillingAccountLocationRecommenderRecommendationMarkDismissedCall) (request)
/// * [locations recommenders recommendations mark dismissed folders](FolderLocationRecommenderRecommendationMarkDismissedCall) (request)
/// * [locations recommenders recommendations mark dismissed organizations](OrganizationLocationRecommenderRecommendationMarkDismissedCall) (request)
/// * [locations recommenders recommendations mark dismissed projects](ProjectLocationRecommenderRecommendationMarkDismissedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1MarkRecommendationDismissedRequest {
    /// Fingerprint of the Recommendation. Provides optimistic locking.
    
    pub etag: Option<String>,
}

impl client::RequestValue for GoogleCloudRecommenderV1MarkRecommendationDismissedRequest {}


/// Request for the `MarkRecommendationFailed` Method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations mark failed billing accounts](BillingAccountLocationRecommenderRecommendationMarkFailedCall) (request)
/// * [locations recommenders recommendations mark failed folders](FolderLocationRecommenderRecommendationMarkFailedCall) (request)
/// * [locations recommenders recommendations mark failed organizations](OrganizationLocationRecommenderRecommendationMarkFailedCall) (request)
/// * [locations recommenders recommendations mark failed projects](ProjectLocationRecommenderRecommendationMarkFailedCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1MarkRecommendationFailedRequest {
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    
    pub etag: Option<String>,
    /// State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex `/^a-z0-9{0,62}$/`. Values must match the regex `/^[a-zA-Z0-9_./-]{0,255}$/`.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1MarkRecommendationFailedRequest {}


/// Request for the `MarkRecommendationSucceeded` Method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations mark succeeded billing accounts](BillingAccountLocationRecommenderRecommendationMarkSucceededCall) (request)
/// * [locations recommenders recommendations mark succeeded folders](FolderLocationRecommenderRecommendationMarkSucceededCall) (request)
/// * [locations recommenders recommendations mark succeeded organizations](OrganizationLocationRecommenderRecommendationMarkSucceededCall) (request)
/// * [locations recommenders recommendations mark succeeded projects](ProjectLocationRecommenderRecommendationMarkSucceededCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1MarkRecommendationSucceededRequest {
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    
    pub etag: Option<String>,
    /// State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex `/^a-z0-9{0,62}$/`. Values must match the regex `/^[a-zA-Z0-9_./-]{0,255}$/`.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1MarkRecommendationSucceededRequest {}


/// Contains an operation for a resource loosely based on the JSON-PATCH format with support for: * Custom filters for describing partial array patch. * Extended path values for describing nested arrays. * Custom fields for describing the resource for which the operation is being described. * Allows extension to custom operations not natively supported by RFC6902. See https://tools.ietf.org/html/rfc6902 for details on the original RFC.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1Operation {
    /// Type of this operation. Contains one of 'add', 'remove', 'replace', 'move', 'copy', 'test' and custom operations. This field is case-insensitive and always populated.
    
    pub action: Option<String>,
    /// Path to the target field being operated on. If the operation is at the resource level, then path should be "/". This field is always populated.
    
    pub path: Option<String>,
    /// Set of filters to apply if `path` refers to array elements or nested array elements in order to narrow down to a single unique element that is being tested/modified. This is intended to be an exact match per filter. To perform advanced matching, use path_value_matchers. * Example: ``` { "/versions/*/name" : "it-123" "/versions/*/targetSize/percent": 20 } ``` * Example: ``` { "/bindings/*/role": "roles/owner" "/bindings/*/condition" : null } ``` * Example: ``` { "/bindings/*/role": "roles/owner" "/bindings/*/members/*" : ["x@example.com", "y@example.com"] } ``` When both path_filters and path_value_matchers are set, an implicit AND must be performed.
    #[serde(rename="pathFilters")]
    
    pub path_filters: Option<HashMap<String, json::Value>>,
    /// Similar to path_filters, this contains set of filters to apply if `path` field refers to array elements. This is meant to support value matching beyond exact match. To perform exact match, use path_filters. When both path_filters and path_value_matchers are set, an implicit AND must be performed.
    #[serde(rename="pathValueMatchers")]
    
    pub path_value_matchers: Option<HashMap<String, GoogleCloudRecommenderV1ValueMatcher>>,
    /// Contains the fully qualified resource name. This field is always populated. ex: //cloudresourcemanager.googleapis.com/projects/foo.
    
    pub resource: Option<String>,
    /// Type of GCP resource being modified/tested. This field is always populated. Example: cloudresourcemanager.googleapis.com/Project, compute.googleapis.com/Instance
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
    /// Can be set with action 'copy' or 'move' to indicate the source field within resource or source_resource, ignored if provided for other operation types.
    #[serde(rename="sourcePath")]
    
    pub source_path: Option<String>,
    /// Can be set with action 'copy' to copy resource configuration across different resources of the same type. Example: A resource clone can be done via action = 'copy', path = "/", from = "/", source_resource = and resource_name = . This field is empty for all other values of `action`.
    #[serde(rename="sourceResource")]
    
    pub source_resource: Option<String>,
    /// Value for the `path` field. Will be set for actions:'add'/'replace'. Maybe set for action: 'test'. Either this or `value_matcher` will be set for 'test' operation. An exact match must be performed.
    
    pub value: Option<json::Value>,
    /// Can be set for action 'test' for advanced matching for the value of 'path' field. Either this or `value` will be set for 'test' operation.
    #[serde(rename="valueMatcher")]
    
    pub value_matcher: Option<GoogleCloudRecommenderV1ValueMatcher>,
}

impl client::Part for GoogleCloudRecommenderV1Operation {}


/// Group of operations that need to be performed atomically.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1OperationGroup {
    /// List of operations across one or more resources that belong to this group. Loosely based on RFC6902 and should be performed in the order they appear.
    
    pub operations: Option<Vec<GoogleCloudRecommenderV1Operation>>,
}

impl client::Part for GoogleCloudRecommenderV1OperationGroup {}


/// A recommendation along with a suggested action. E.g., a rightsizing recommendation for an underutilized VM, IAM role recommendations, etc
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders recommendations get billing accounts](BillingAccountLocationRecommenderRecommendationGetCall) (response)
/// * [locations recommenders recommendations mark claimed billing accounts](BillingAccountLocationRecommenderRecommendationMarkClaimedCall) (response)
/// * [locations recommenders recommendations mark dismissed billing accounts](BillingAccountLocationRecommenderRecommendationMarkDismissedCall) (response)
/// * [locations recommenders recommendations mark failed billing accounts](BillingAccountLocationRecommenderRecommendationMarkFailedCall) (response)
/// * [locations recommenders recommendations mark succeeded billing accounts](BillingAccountLocationRecommenderRecommendationMarkSucceededCall) (response)
/// * [locations recommenders recommendations get folders](FolderLocationRecommenderRecommendationGetCall) (response)
/// * [locations recommenders recommendations mark claimed folders](FolderLocationRecommenderRecommendationMarkClaimedCall) (response)
/// * [locations recommenders recommendations mark dismissed folders](FolderLocationRecommenderRecommendationMarkDismissedCall) (response)
/// * [locations recommenders recommendations mark failed folders](FolderLocationRecommenderRecommendationMarkFailedCall) (response)
/// * [locations recommenders recommendations mark succeeded folders](FolderLocationRecommenderRecommendationMarkSucceededCall) (response)
/// * [locations recommenders recommendations get organizations](OrganizationLocationRecommenderRecommendationGetCall) (response)
/// * [locations recommenders recommendations mark claimed organizations](OrganizationLocationRecommenderRecommendationMarkClaimedCall) (response)
/// * [locations recommenders recommendations mark dismissed organizations](OrganizationLocationRecommenderRecommendationMarkDismissedCall) (response)
/// * [locations recommenders recommendations mark failed organizations](OrganizationLocationRecommenderRecommendationMarkFailedCall) (response)
/// * [locations recommenders recommendations mark succeeded organizations](OrganizationLocationRecommenderRecommendationMarkSucceededCall) (response)
/// * [locations recommenders recommendations get projects](ProjectLocationRecommenderRecommendationGetCall) (response)
/// * [locations recommenders recommendations mark claimed projects](ProjectLocationRecommenderRecommendationMarkClaimedCall) (response)
/// * [locations recommenders recommendations mark dismissed projects](ProjectLocationRecommenderRecommendationMarkDismissedCall) (response)
/// * [locations recommenders recommendations mark failed projects](ProjectLocationRecommenderRecommendationMarkFailedCall) (response)
/// * [locations recommenders recommendations mark succeeded projects](ProjectLocationRecommenderRecommendationMarkSucceededCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1Recommendation {
    /// Optional set of additional impact that this recommendation may have when trying to optimize for the primary category. These may be positive or negative.
    #[serde(rename="additionalImpact")]
    
    pub additional_impact: Option<Vec<GoogleCloudRecommenderV1Impact>>,
    /// Insights that led to this recommendation.
    #[serde(rename="associatedInsights")]
    
    pub associated_insights: Option<Vec<GoogleCloudRecommenderV1RecommendationInsightReference>>,
    /// Content of the recommendation describing recommended changes to resources.
    
    pub content: Option<GoogleCloudRecommenderV1RecommendationContent>,
    /// Free-form human readable summary in English. The maximum length is 500 characters.
    
    pub description: Option<String>,
    /// Fingerprint of the Recommendation. Provides optimistic locking when updating states.
    
    pub etag: Option<String>,
    /// Last time this recommendation was refreshed by the system that created it in the first place.
    #[serde(rename="lastRefreshTime")]
    
    pub last_refresh_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name of recommendation.
    
    pub name: Option<String>,
    /// The primary impact that this recommendation can have while trying to optimize for one category.
    #[serde(rename="primaryImpact")]
    
    pub primary_impact: Option<GoogleCloudRecommenderV1Impact>,
    /// Recommendation's priority.
    
    pub priority: Option<GoogleCloudRecommenderV1RecommendationPriorityEnum>,
    /// Contains an identifier for a subtype of recommendations produced for the same recommender. Subtype is a function of content and impact, meaning a new subtype might be added when significant changes to `content` or `primary_impact.category` are introduced. See the Recommenders section to see a list of subtypes for a given Recommender. Examples: For recommender = "google.iam.policy.Recommender", recommender_subtype can be one of "REMOVE_ROLE"/"REPLACE_ROLE"
    #[serde(rename="recommenderSubtype")]
    
    pub recommender_subtype: Option<String>,
    /// Information for state. Contains state and metadata.
    #[serde(rename="stateInfo")]
    
    pub state_info: Option<GoogleCloudRecommenderV1RecommendationStateInfo>,
    /// Corresponds to a mutually exclusive group ID within a recommender. A non-empty ID indicates that the recommendation belongs to a mutually exclusive group. This means that only one recommendation within the group is suggested to be applied.
    #[serde(rename="xorGroupId")]
    
    pub xor_group_id: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecommenderV1Recommendation {}


/// Contains what resources are changing and how they are changing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1RecommendationContent {
    /// Operations to one or more Google Cloud resources grouped in such a way that, all operations within one group are expected to be performed atomically and in an order.
    #[serde(rename="operationGroups")]
    
    pub operation_groups: Option<Vec<GoogleCloudRecommenderV1OperationGroup>>,
    /// Condensed overview information about the recommendation.
    
    pub overview: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudRecommenderV1RecommendationContent {}


/// Reference to an associated insight.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1RecommendationInsightReference {
    /// Insight resource name, e.g. projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/insights/[INSIGHT_ID]
    
    pub insight: Option<String>,
}

impl client::Part for GoogleCloudRecommenderV1RecommendationInsightReference {}


/// Information for state. Contains state and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1RecommendationStateInfo {
    /// The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED.
    
    pub state: Option<GoogleCloudRecommenderV1RecommendationStateInfoStateEnum>,
    /// A map of metadata for the state, provided by user or automations systems.
    #[serde(rename="stateMetadata")]
    
    pub state_metadata: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudRecommenderV1RecommendationStateInfo {}


/// Configuration for a Recommender.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations recommenders get config billing accounts](BillingAccountLocationRecommenderGetConfigCall) (response)
/// * [locations recommenders update config billing accounts](BillingAccountLocationRecommenderUpdateConfigCall) (request|response)
/// * [locations recommenders get config organizations](OrganizationLocationRecommenderGetConfigCall) (response)
/// * [locations recommenders update config organizations](OrganizationLocationRecommenderUpdateConfigCall) (request|response)
/// * [locations recommenders get config projects](ProjectLocationRecommenderGetConfigCall) (response)
/// * [locations recommenders update config projects](ProjectLocationRecommenderUpdateConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1RecommenderConfig {
    /// Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between.
    
    pub annotations: Option<HashMap<String, String>>,
    /// A user-settable field to provide a human-readable name to be used in user interfaces.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Fingerprint of the RecommenderConfig. Provides optimistic locking when updating.
    
    pub etag: Option<String>,
    /// Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config
    
    pub name: Option<String>,
    /// RecommenderGenerationConfig which configures the Generation of recommendations for this recommender.
    #[serde(rename="recommenderGenerationConfig")]
    
    pub recommender_generation_config: Option<GoogleCloudRecommenderV1RecommenderGenerationConfig>,
    /// Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Last time when the config was updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for GoogleCloudRecommenderV1RecommenderConfig {}
impl client::ResponseResult for GoogleCloudRecommenderV1RecommenderConfig {}


/// A Configuration to customize the generation of recommendations. Eg, customizing the lookback period considered when generating a recommendation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1RecommenderGenerationConfig {
    /// Parameters for this RecommenderGenerationConfig. These configs can be used by or are applied to all subtypes.
    
    pub params: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudRecommenderV1RecommenderGenerationConfig {}


/// Contains information on the impact of a reliability recommendation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1ReliabilityProjection {
    /// Per-recommender projection.
    
    pub details: Option<HashMap<String, json::Value>>,
    /// Reliability risks mitigated by this recommendation.
    
    pub risks: Option<Vec<GoogleCloudRecommenderV1ReliabilityProjectionRisksEnum>>,
}

impl client::Part for GoogleCloudRecommenderV1ReliabilityProjection {}


/// Contains various ways of describing the impact on Security.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1SecurityProjection {
    /// Additional security impact details that is provided by the recommender.
    
    pub details: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudRecommenderV1SecurityProjection {}


/// Contains metadata about how much sustainability a recommendation can save or incur.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1SustainabilityProjection {
    /// Duration for which this sustainability applies.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Carbon Footprint generated in kg of CO2 equivalent. Chose kg_c_o2e so that the name renders correctly in camelCase (kgCO2e).
    #[serde(rename="kgCO2e")]
    
    pub kg_co2e: Option<f64>,
}

impl client::Part for GoogleCloudRecommenderV1SustainabilityProjection {}


/// Contains various matching options for values for a GCP resource field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommenderV1ValueMatcher {
    /// To be used for full regex matching. The regular expression is using the Google RE2 syntax (https://github.com/google/re2/wiki/Syntax), so to be used with RE2::FullMatch
    #[serde(rename="matchesPattern")]
    
    pub matches_pattern: Option<String>,
}

impl client::Part for GoogleCloudRecommenderV1ValueMatcher {}


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


