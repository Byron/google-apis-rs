use super::*;
/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events collect projects](ProjectLocationCatalogEventStoreUserEventCollectCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleApiHttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::ResponseResult for GoogleApiHttpBody {}


/// BigQuery source import data from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1BigQuerySource {
    /// Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: "catalog_recommendations_ai" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: "catalog_merchant_center" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user event imports: 1: "user_events_recommendations_ai" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. "user_events_ga360" using https://support.google.com/analytics/answer/3437719?hl=en
    #[serde(rename="dataSchema")]
    
    pub data_schema: Option<String>,
    /// Required. The BigQuery data set to copy the data from.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Optional. Intermediate Cloud Storage directory used for the import. Can be specified if one wants to have the BigQuery export to a specific Cloud Storage directory.
    #[serde(rename="gcsStagingDir")]
    
    pub gcs_staging_dir: Option<String>,
    /// Optional. The project id (can be project # or id) that the BigQuery source is in. If not specified, inherits the project id from the parent request.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The BigQuery table to copy the data from.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1BigQuerySource {}


/// The catalog configuration. Next ID: 5.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs patch projects](ProjectLocationCatalogPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1Catalog {
    /// Required. The catalog item level configuration.
    #[serde(rename="catalogItemLevelConfig")]
    
    pub catalog_item_level_config: Option<GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfig>,
    /// Required. The ID of the default event store.
    #[serde(rename="defaultEventStoreId")]
    
    pub default_event_store_id: Option<String>,
    /// Required. The catalog display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The fully qualified resource name of the catalog.
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1Catalog {}
impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1Catalog {}


/// The inline source for the input config for ImportCatalogItems method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1CatalogInlineSource {
    /// Optional. A list of catalog items to update/create. Recommended max of 10k items.
    #[serde(rename="catalogItems")]
    
    pub catalog_items: Option<Vec<GoogleCloudRecommendationengineV1beta1CatalogItem>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1CatalogInlineSource {}


/// CatalogItem captures all metadata information of items to be recommended.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs catalog items create projects](ProjectLocationCatalogCatalogItemCreateCall) (request|response)
/// * [locations catalogs catalog items get projects](ProjectLocationCatalogCatalogItemGetCall) (response)
/// * [locations catalogs catalog items patch projects](ProjectLocationCatalogCatalogItemPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItem {
    /// Required. Catalog item categories. This field is repeated for supporting one catalog item belonging to several parallel category hierarchies. For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categoryHierarchies": [ { "categories": ["Shoes & Accessories", "Shoes"]}, { "categories": ["Sports & Fitness", "Athletic Clothing", "Shoes"] } ]
    #[serde(rename="categoryHierarchies")]
    
    pub category_hierarchies: Option<Vec<GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy>>,
    /// Optional. Catalog item description. UTF-8 encoded string with a length limit of 5 KiB.
    
    pub description: Option<String>,
    /// Required. Catalog item identifier. UTF-8 encoded string with a length limit of 128 bytes. This id must be unique among all catalog items within the same catalog. It should also be used when logging user events in order for the user events to be joined with the Catalog.
    
    pub id: Option<String>,
    /// Optional. Highly encouraged. Extra catalog item attributes to be included in the recommendation model. For example, for retail products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the item attributes here.
    #[serde(rename="itemAttributes")]
    
    pub item_attributes: Option<GoogleCloudRecommendationengineV1beta1FeatureMap>,
    /// Optional. Variant group identifier for prediction results. UTF-8 encoded string with a length limit of 128 bytes. This field must be enabled before it can be used. [Learn more](https://cloud.google.com/recommendations-ai/docs/catalog#item-group-id).
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// Optional. Deprecated. The model automatically detects the text language. Your catalog can include text in different languages, but duplicating catalog items to provide text in multiple languages can result in degraded model performance.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Metadata specific to retail products.
    #[serde(rename="productMetadata")]
    
    pub product_metadata: Option<GoogleCloudRecommendationengineV1beta1ProductCatalogItem>,
    /// Optional. Filtering tags associated with the catalog item. Each tag should be a UTF-8 encoded string with a length limit of 1 KiB. This tag can be used for filtering recommendation results by passing the tag as part of the predict request filter.
    
    pub tags: Option<Vec<String>>,
    /// Required. Catalog item title. UTF-8 encoded string with a length limit of 1 KiB.
    
    pub title: Option<String>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1CatalogItem {}
impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1CatalogItem {}


/// Category represents catalog item category hierarchy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy {
    /// Required. Catalog item categories. Each category should be a UTF-8 encoded string with a length limit of 2 KiB. Note that the order in the list denotes the specificity (from least to most specific).
    
    pub categories: Option<Vec<String>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy {}


/// Configures the catalog level that users send events to, and the level at which predictions are made.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfig {
    /// Optional. Level of the catalog at which events are uploaded. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details.
    #[serde(rename="eventItemLevel")]
    
    pub event_item_level: Option<String>,
    /// Optional. Level of the catalog at which predictions are made. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details.
    #[serde(rename="predictItemLevel")]
    
    pub predict_item_level: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1CatalogItemLevelConfig {}


/// Request message for the `CreatePredictionApiKeyRegistration` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores prediction api key registrations create projects](ProjectLocationCatalogEventStorePredictionApiKeyRegistrationCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1CreatePredictionApiKeyRegistrationRequest {
    /// Required. The prediction API key registration.
    #[serde(rename="predictionApiKeyRegistration")]
    
    pub prediction_api_key_registration: Option<GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1CreatePredictionApiKeyRegistrationRequest {}


/// User event details shared by all recommendation types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1EventDetail {
    /// Optional. Extra user event features to include in the recommendation model. For product recommendation, an example of extra user information is traffic_channel, i.e. how user arrives at the site. Users can arrive at the site by coming to the site directly, or coming through Google search, and etc.
    #[serde(rename="eventAttributes")]
    
    pub event_attributes: Option<GoogleCloudRecommendationengineV1beta1FeatureMap>,
    /// Optional. A list of identifiers for the independent experiment groups this user event belongs to. This is used to distinguish between user events associated with different experiment setups (e.g. using Recommendation Engine system, using different recommendation models).
    #[serde(rename="experimentIds")]
    
    pub experiment_ids: Option<Vec<String>>,
    /// Optional. A unique id of a web page view. This should be kept the same for all user events triggered from the same pageview. For example, an item detail page view could trigger multiple events as the user is browsing the page. The `pageViewId` property should be kept the same for all these events so that they can be grouped together properly. This `pageViewId` will be automatically generated if using the JavaScript pixel.
    #[serde(rename="pageViewId")]
    
    pub page_view_id: Option<String>,
    /// Optional. Recommendation token included in the recommendation prediction response. This field enables accurate attribution of recommendation model performance. This token enables us to accurately attribute page view or purchase back to the event and the particular predict response containing this clicked/purchased item. If user clicks on product K in the recommendation results, pass the `PredictResponse.recommendationToken` property as a url parameter to product K's page. When recording events on product K's page, log the PredictResponse.recommendation_token to this field. Optional, but highly encouraged for user events that are the result of a recommendation prediction query.
    #[serde(rename="recommendationToken")]
    
    pub recommendation_token: Option<String>,
    /// Optional. The referrer url of the current page. When using the JavaScript pixel, this value is filled in automatically.
    #[serde(rename="referrerUri")]
    
    pub referrer_uri: Option<String>,
    /// Optional. Complete url (window.location.href) of the user's current page. When using the JavaScript pixel, this value is filled in automatically. Maximum length 5KB.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1EventDetail {}


/// FeatureMap represents extra features that customers want to include in the recommendation model for catalogs/user events as categorical/numerical features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMap {
    /// Categorical features that can take on one of a limited number of possible values. Some examples would be the brand/maker of a product, or country of a customer. Feature names and values must be UTF-8 encoded strings. For example: `{ "colors": {"value": ["yellow", "green"]}, "sizes": {"value":["S", "M"]}`
    #[serde(rename="categoricalFeatures")]
    
    pub categorical_features: Option<HashMap<String, GoogleCloudRecommendationengineV1beta1FeatureMapStringList>>,
    /// Numerical features. Some examples would be the height/weight of a product, or age of a customer. Feature names must be UTF-8 encoded strings. For example: `{ "lengths_cm": {"value":[2.3, 15.4]}, "heights_cm": {"value":[8.1, 6.4]} }`
    #[serde(rename="numericalFeatures")]
    
    pub numerical_features: Option<HashMap<String, GoogleCloudRecommendationengineV1beta1FeatureMapFloatList>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1FeatureMap {}


/// A list of float features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMapFloatList {
    /// Float feature value.
    
    pub value: Option<Vec<f32>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1FeatureMapFloatList {}


/// A list of string features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1FeatureMapStringList {
    /// String feature value with a length limit of 128 bytes.
    
    pub value: Option<Vec<String>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1FeatureMapStringList {}


/// Google Cloud Storage location for input content. format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1GcsSource {
    /// Required. Google Cloud Storage URIs to input files. URI can be up to 2000 characters long. URIs can match the full object path (for example, `gs://bucket/directory/object.json`) or a pattern matching one or more files, such as `gs://bucket/directory/*.json`. A request can contain at most 100 files, and each file can be up to 2 GB. See [Importing catalog information](https://cloud.google.com/recommendations-ai/docs/upload-catalog) for the expected file format and setup instructions.
    #[serde(rename="inputUris")]
    
    pub input_uris: Option<Vec<String>>,
    /// Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: "catalog_recommendations_ai" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: "catalog_merchant_center" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user events imports: 1: "user_events_recommendations_ai" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. "user_events_ga360" using https://support.google.com/analytics/answer/3437719?hl=en
    #[serde(rename="jsonSchema")]
    
    pub json_schema: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1GcsSource {}


/// Catalog item thumbnail/detail image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1Image {
    /// Optional. Height of the image in number of pixels.
    
    pub height: Option<i32>,
    /// Required. URL of the image with a length limit of 5 KiB.
    
    pub uri: Option<String>,
    /// Optional. Width of the image in number of pixels.
    
    pub width: Option<i32>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1Image {}


/// Request message for Import methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs catalog items import projects](ProjectLocationCatalogCatalogItemImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ImportCatalogItemsRequest {
    /// Optional. The desired location of errors incurred during the Import.
    #[serde(rename="errorsConfig")]
    
    pub errors_config: Option<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    /// Required. The desired input location of the data.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudRecommendationengineV1beta1InputConfig>,
    /// Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency and used for request deduplication. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Optional. Indicates which fields in the provided imported 'items' to update. If not set, will by default update all fields.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1ImportCatalogItemsRequest {}


/// Configuration of destination for Import related errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ImportErrorsConfig {
    /// Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message.
    #[serde(rename="gcsPrefix")]
    
    pub gcs_prefix: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ImportErrorsConfig {}


/// Request message for the ImportUserEvents request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events import projects](ProjectLocationCatalogEventStoreUserEventImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ImportUserEventsRequest {
    /// Optional. The desired location of errors incurred during the Import.
    #[serde(rename="errorsConfig")]
    
    pub errors_config: Option<GoogleCloudRecommendationengineV1beta1ImportErrorsConfig>,
    /// Required. The desired input location of the data.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<GoogleCloudRecommendationengineV1beta1InputConfig>,
    /// Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency for expensive long running operations. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response. Note that this field must not be set if the desired input config is catalog_inline_source.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1ImportUserEventsRequest {}


/// The input config source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1InputConfig {
    /// BigQuery input source.
    #[serde(rename="bigQuerySource")]
    
    pub big_query_source: Option<GoogleCloudRecommendationengineV1beta1BigQuerySource>,
    /// The Inline source for the input content for Catalog items.
    #[serde(rename="catalogInlineSource")]
    
    pub catalog_inline_source: Option<GoogleCloudRecommendationengineV1beta1CatalogInlineSource>,
    /// Google Cloud Storage location for the input content.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudRecommendationengineV1beta1GcsSource>,
    /// The Inline source for the input content for UserEvents.
    #[serde(rename="userEventInlineSource")]
    
    pub user_event_inline_source: Option<GoogleCloudRecommendationengineV1beta1UserEventInlineSource>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1InputConfig {}


/// Response message for ListCatalogItems method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs catalog items list projects](ProjectLocationCatalogCatalogItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ListCatalogItemsResponse {
    /// The catalog items.
    #[serde(rename="catalogItems")]
    
    pub catalog_items: Option<Vec<GoogleCloudRecommendationengineV1beta1CatalogItem>>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next request's ListCatalogItemRequest.page_token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1ListCatalogItemsResponse {}


/// Response for ListCatalogs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs list projects](ProjectLocationCatalogListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ListCatalogsResponse {
    /// Output only. All the customer's catalogs.
    
    pub catalogs: Option<Vec<GoogleCloudRecommendationengineV1beta1Catalog>>,
    /// Pagination token, if not returned indicates the last page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1ListCatalogsResponse {}


/// Response message for the `ListPredictionApiKeyRegistrations`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores prediction api key registrations list projects](ProjectLocationCatalogEventStorePredictionApiKeyRegistrationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ListPredictionApiKeyRegistrationsResponse {
    /// If empty, the list is complete. If nonempty, pass the token to the next request's `ListPredictionApiKeysRegistrationsRequest.pageToken`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of registered API keys.
    #[serde(rename="predictionApiKeyRegistrations")]
    
    pub prediction_api_key_registrations: Option<Vec<GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration>>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1ListPredictionApiKeyRegistrationsResponse {}


/// Response message for ListUserEvents method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events list projects](ProjectLocationCatalogEventStoreUserEventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ListUserEventsResponse {
    /// If empty, the list is complete. If nonempty, the token to pass to the next request's ListUserEvents.page_token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The user events.
    #[serde(rename="userEvents")]
    
    pub user_events: Option<Vec<GoogleCloudRecommendationengineV1beta1UserEvent>>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1ListUserEventsResponse {}


/// Request message for Predict method. Full resource name of the format: `{name=projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/placements/*}` The id of the recommendation engine placement. This id is used to identify the set of models that will be used to make the prediction. We currently support three placements with the following IDs by default: // * `shopping_cart`: Predicts items frequently bought together with one or more catalog items in the same shopping session. Commonly displayed after `add-to-cart` event, on product detail pages, or on the shopping cart page. * `home_page`: Predicts the next product that a user will most likely engage with or purchase based on the shopping or viewing history of the specified `userId` or `visitorId`. For example - Recommendations for you. * `product_detail`: Predicts the next product that a user will most likely engage with or purchase. The prediction is based on the shopping or viewing history of the specified `userId` or `visitorId` and its relevance to a specified `CatalogItem`. Typically used on product detail pages. For example - More items like this. * `recently_viewed_default`: Returns up to 75 items recently viewed by the specified `userId` or `visitorId`, most recent ones first. Returns nothing if neither of them has viewed any items yet. For example - Recently viewed. The full list of available placements can be seen at https://console.cloud.google.com/recommendation/catalogs/default_catalog/placements
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores placements predict projects](ProjectLocationCatalogEventStorePlacementPredictCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PredictRequest {
    /// Optional. Use dryRun mode for this prediction query. If set to true, a fake model will be used that returns arbitrary catalog items. Note that the dryRun mode should only be used for testing the API, or if the model is not ready.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Optional. Filter for restricting prediction results. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to items that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-"tagA"` is also supported and is equivalent to `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1 KiB. * filterOutOfStockItems. Restricts predictions to items that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional") * filterOutOfStockItems tag=(-"promotional") * filterOutOfStockItems If your filter blocks all prediction results, nothing will be returned. If you want generic (unfiltered) popular items to be returned instead, set `strictFiltering` to false in `PredictRequest.params`.
    
    pub filter: Option<String>,
    /// Optional. The labels for the predict request. * Label keys can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * Non-zero label values can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * No more than 64 labels can be associated with a given request. See https://goo.gl/xmQnxf for more information on and examples of labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Maximum number of results to return per page. Set this property to the number of prediction results required. If zero, the service will choose a reasonable default.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. The previous PredictResponse.next_page_token.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Optional. Additional domain specific parameters for the predictions. Allowed values: * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem object will be returned in the `PredictResponse.PredictionResult.itemMetadata` object in the method response. * `returnItemScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned item will be set in the `metadata` field in the prediction response. The given 'score' indicates the probability of an item being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular items instead of empty if your filter blocks all prediction results. * `priceRerankLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-price-reranking', 'low-price-reranking', 'medium-price-reranking', 'high-price-reranking'}. This gives request level control and adjust prediction results based on product price. * `diversityLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-diversity', 'low-diversity', 'medium-diversity', 'high-diversity', 'auto-diversity'}. This gives request level control and adjust prediction results based on product category.
    
    pub params: Option<HashMap<String, json::Value>>,
    /// Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging. Don't set UserInfo.visitor_id or UserInfo.user_id to the same fixed ID for different users. If you are trying to receive non-personalized recommendations (not recommended; this can negatively impact model performance), instead set UserInfo.visitor_id to a random unique ID and leave UserInfo.user_id unset.
    #[serde(rename="userEvent")]
    
    pub user_event: Option<GoogleCloudRecommendationengineV1beta1UserEvent>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1PredictRequest {}


/// Response message for predict method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores placements predict projects](ProjectLocationCatalogEventStorePlacementPredictCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PredictResponse {
    /// True if the dryRun property was set in the request.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// IDs of items in the request that were missing from the catalog.
    #[serde(rename="itemsMissingInCatalog")]
    
    pub items_missing_in_catalog: Option<Vec<String>>,
    /// Additional domain specific prediction response metadata.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next request's PredictRequest.page_token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A unique recommendation token. This should be included in the user event logs resulting from this recommendation, which enables accurate attribution of recommendation model performance.
    #[serde(rename="recommendationToken")]
    
    pub recommendation_token: Option<String>,
    /// A list of recommended items. The order represents the ranking (from the most relevant item to the least).
    
    pub results: Option<Vec<GoogleCloudRecommendationengineV1beta1PredictResponsePredictionResult>>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1PredictResponse {}


/// PredictionResult represents the recommendation prediction results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PredictResponsePredictionResult {
    /// ID of the recommended catalog item
    
    pub id: Option<String>,
    /// Additional item metadata / annotations. Possible values: * `catalogItem`: JSON representation of the catalogItem. Will be set if `returnCatalogItem` is set to true in `PredictRequest.params`. * `score`: Prediction score in double value. Will be set if `returnItemScore` is set to true in `PredictRequest.params`.
    #[serde(rename="itemMetadata")]
    
    pub item_metadata: Option<HashMap<String, json::Value>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1PredictResponsePredictionResult {}


/// Registered Api Key.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores prediction api key registrations create projects](ProjectLocationCatalogEventStorePredictionApiKeyRegistrationCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration {
    /// The API key.
    #[serde(rename="apiKey")]
    
    pub api_key: Option<String>,
}

impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1PredictionApiKeyRegistration {}


/// ProductCatalogItem captures item metadata specific to retail products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItem {
    /// Optional. The available quantity of the item.
    #[serde(rename="availableQuantity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub available_quantity: Option<i64>,
    /// Optional. Canonical URL directly linking to the item detail page with a length limit of 5 KiB..
    #[serde(rename="canonicalProductUri")]
    
    pub canonical_product_uri: Option<String>,
    /// Optional. A map to pass the costs associated with the product. For example: {"manufacturing": 45.5} The profit of selling this item is computed like so: * If 'exactPrice' is provided, profit = displayPrice - sum(costs) * If 'priceRange' is provided, profit = minPrice - sum(costs)
    
    pub costs: Option<HashMap<String, f32>>,
    /// Optional. Only required if the price is set. Currency code for price/costs. Use three-character ISO-4217 code.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Optional. The exact product price.
    #[serde(rename="exactPrice")]
    
    pub exact_price: Option<GoogleCloudRecommendationengineV1beta1ProductCatalogItemExactPrice>,
    /// Optional. Product images for the catalog item.
    
    pub images: Option<Vec<GoogleCloudRecommendationengineV1beta1Image>>,
    /// Optional. The product price range.
    #[serde(rename="priceRange")]
    
    pub price_range: Option<GoogleCloudRecommendationengineV1beta1ProductCatalogItemPriceRange>,
    /// Optional. Online stock state of the catalog item. Default is `IN_STOCK`.
    #[serde(rename="stockState")]
    
    pub stock_state: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ProductCatalogItem {}


/// Exact product price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItemExactPrice {
    /// Optional. Display price of the product.
    #[serde(rename="displayPrice")]
    
    pub display_price: Option<f32>,
    /// Optional. Price of the product without any discount. If zero, by default set to be the 'displayPrice'.
    #[serde(rename="originalPrice")]
    
    pub original_price: Option<f32>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ProductCatalogItemExactPrice {}


/// Product price range when there are a range of prices for different variations of the same product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ProductCatalogItemPriceRange {
    /// Required. The maximum product price.
    
    pub max: Option<f32>,
    /// Required. The minimum product price.
    
    pub min: Option<f32>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ProductCatalogItemPriceRange {}


/// Detailed product information associated with a user event.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ProductDetail {
    /// Optional. Quantity of the products in stock when a user event happens. Optional. If provided, this overrides the available quantity in Catalog for this event. and can only be set if `stock_status` is set to `IN_STOCK`. Note that if an item is out of stock, you must set the `stock_state` field to be `OUT_OF_STOCK`. Leaving this field unspecified / as zero is not sufficient to mark the item out of stock.
    #[serde(rename="availableQuantity")]
    
    pub available_quantity: Option<i32>,
    /// Optional. Currency code for price/costs. Use three-character ISO-4217 code. Required only if originalPrice or displayPrice is set.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Optional. Display price of the product (e.g. discounted price). If provided, this will override the display price in Catalog for this product.
    #[serde(rename="displayPrice")]
    
    pub display_price: Option<f32>,
    /// Required. Catalog item ID. UTF-8 encoded string with a length limit of 128 characters.
    
    pub id: Option<String>,
    /// Optional. Extra features associated with a product in the user event.
    #[serde(rename="itemAttributes")]
    
    pub item_attributes: Option<GoogleCloudRecommendationengineV1beta1FeatureMap>,
    /// Optional. Original price of the product. If provided, this will override the original price in Catalog for this product.
    #[serde(rename="originalPrice")]
    
    pub original_price: Option<f32>,
    /// Optional. Quantity of the product associated with the user event. For example, this field will be 2 if two products are added to the shopping cart for `add-to-cart` event. Required for `add-to-cart`, `add-to-list`, `remove-from-cart`, `checkout-start`, `purchase-complete`, `refund` event types.
    
    pub quantity: Option<i32>,
    /// Optional. Item stock state. If provided, this overrides the stock state in Catalog for items in this event.
    #[serde(rename="stockState")]
    
    pub stock_state: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ProductDetail {}


/// ProductEventDetail captures user event information specific to retail products.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1ProductEventDetail {
    /// Optional. The id or name of the associated shopping cart. This id is used to associate multiple items added or present in the cart before purchase. This can only be set for `add-to-cart`, `remove-from-cart`, `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events.
    #[serde(rename="cartId")]
    
    pub cart_id: Option<String>,
    /// Required for `add-to-list` and `remove-from-list` events. The id or name of the list that the item is being added to or removed from. Other event types should not set this field.
    #[serde(rename="listId")]
    
    pub list_id: Option<String>,
    /// Required for `category-page-view` events. At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The categories associated with a category page. Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: categories : ["Sales", "2017 Black Friday Deals"].
    #[serde(rename="pageCategories")]
    
    pub page_categories: Option<Vec<GoogleCloudRecommendationengineV1beta1CatalogItemCategoryHierarchy>>,
    /// The main product details related to the event. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_details' should be set for this unless the shopping cart is empty. * `search` (highly encouraged) In a `search` event, this field represents the products returned to the end user on the current page (the end user may have not finished broswing the whole page yet). When a new page is returned to the end user, after pagination/filtering/ordering even for the same query, a new SEARCH event with different product_details is desired. The end user may have not finished broswing the whole page yet. This field is not allowed for the following event types: * `category-page-view` * `home-page-view`
    #[serde(rename="productDetails")]
    
    pub product_details: Option<Vec<GoogleCloudRecommendationengineV1beta1ProductDetail>>,
    /// Optional. A transaction represents the entire purchase transaction. Required for `purchase-complete` events. Optional for `checkout-start` events. Other event types should not set this field.
    #[serde(rename="purchaseTransaction")]
    
    pub purchase_transaction: Option<GoogleCloudRecommendationengineV1beta1PurchaseTransaction>,
    /// At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The user's search query as UTF-8 encoded text with a length limit of 5 KiB.
    #[serde(rename="searchQuery")]
    
    pub search_query: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1ProductEventDetail {}


/// A transaction represents the entire purchase transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PurchaseTransaction {
    /// Optional. All the costs associated with the product. These can be manufacturing costs, shipping expenses not borne by the end user, or any other costs. Total product cost such that profit = revenue - (sum(taxes) + sum(costs)) If product_cost is not set, then profit = revenue - tax - shipping - sum(CatalogItem.costs). If CatalogItem.cost is not specified for one of the items, CatalogItem.cost based profit *cannot* be calculated for this Transaction.
    
    pub costs: Option<HashMap<String, f32>>,
    /// Required. Currency code. Use three-character ISO-4217 code. This field is not required if the event type is `refund`.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Optional. The transaction ID with a length limit of 128 bytes.
    
    pub id: Option<String>,
    /// Required. Total revenue or grand total associated with the transaction. This value include shipping, tax, or other adjustments to total revenue that you want to include as part of your revenue calculations. This field is not required if the event type is `refund`.
    
    pub revenue: Option<f32>,
    /// Optional. All the taxes associated with the transaction.
    
    pub taxes: Option<HashMap<String, f32>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1PurchaseTransaction {}


/// Request message for PurgeUserEvents method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events purge projects](ProjectLocationCatalogEventStoreUserEventPurgeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1PurgeUserEventsRequest {
    /// Required. The filter string to specify the events to be deleted. Empty string filter is not allowed. The eligible fields for filtering are: * `eventType`: UserEvent.eventType field of type string. * `eventTime`: in ISO 8601 "zulu" format. * `visitorId`: field of type string. Specifying this will delete all events associated with a visitor. * `userId`: field of type string. Specifying this will delete all events associated with a user. Examples: * Deleting all events in a time range: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z"` * Deleting specific eventType in time range: `eventTime > "2012-04-23T18:25:43.511Z" eventType = "detail-page-view"` * Deleting all events for a specific visitor: `visitorId = "visitor1024"` The filtering fields are assumed to have an implicit AND.
    
    pub filter: Option<String>,
    /// Optional. The default value is false. Override this flag to true to actually perform the purge. If the field is not set to true, a sampling of events to be deleted will be returned.
    
    pub force: Option<bool>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1PurgeUserEventsRequest {}


/// Request message for CatalogRejoin method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events rejoin projects](ProjectLocationCatalogEventStoreUserEventRejoinCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequest {
    /// Required. The type of the catalog rejoin to define the scope and range of the user events to be rejoined with catalog items.
    #[serde(rename="userEventRejoinScope")]
    
    pub user_event_rejoin_scope: Option<String>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequest {}


/// UserEvent captures all metadata information recommendation engine needs to know about how end users interact with customers’ website.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores user events write projects](ProjectLocationCatalogEventStoreUserEventWriteCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1UserEvent {
    /// Optional. User event detailed information common across different recommendation types.
    #[serde(rename="eventDetail")]
    
    pub event_detail: Option<GoogleCloudRecommendationengineV1beta1EventDetail>,
    /// Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`.
    #[serde(rename="eventSource")]
    
    pub event_source: Option<String>,
    /// Optional. Only required for ImportUserEvents method. Timestamp of user event created.
    #[serde(rename="eventTime")]
    
    pub event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. User event type. Allowed values are: * `add-to-cart` Products being added to cart. * `add-to-list` Items being added to a list (shopping list, favorites etc). * `category-page-view` Special pages such as sale or promotion pages viewed. * `checkout-start` User starting a checkout process. * `detail-page-view` Products detail page viewed. * `home-page-view` Homepage viewed. * `page-visit` Generic page visits not included in the event types above. * `purchase-complete` User finishing a purchase. * `refund` Purchased items being refunded or returned. * `remove-from-cart` Products being removed from cart. * `remove-from-list` Items being removed from a list. * `search` Product search. * `shopping-cart-page-view` User viewing a shopping cart. * `impression` List of items displayed. Used by Google Tag Manager.
    #[serde(rename="eventType")]
    
    pub event_type: Option<String>,
    /// Optional. Retail product specific user event metadata. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `category-page-view` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` * `search` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_event_detail' should be set for this unless the shopping cart is empty. This field is not allowed for the following event types: * `home-page-view`
    #[serde(rename="productEventDetail")]
    
    pub product_event_detail: Option<GoogleCloudRecommendationengineV1beta1ProductEventDetail>,
    /// Required. User information.
    #[serde(rename="userInfo")]
    
    pub user_info: Option<GoogleCloudRecommendationengineV1beta1UserInfo>,
}

impl client::RequestValue for GoogleCloudRecommendationengineV1beta1UserEvent {}
impl client::ResponseResult for GoogleCloudRecommendationengineV1beta1UserEvent {}


/// The inline source for the input config for ImportUserEvents method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1UserEventInlineSource {
    /// Optional. A list of user events to import. Recommended max of 10k items.
    #[serde(rename="userEvents")]
    
    pub user_events: Option<Vec<GoogleCloudRecommendationengineV1beta1UserEvent>>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1UserEventInlineSource {}


/// Information of end users.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudRecommendationengineV1beta1UserInfo {
    /// Optional. Indicates if the request is made directly from the end user in which case the user_agent and ip_address fields can be populated from the HTTP request. This should *not* be set when using the javascript pixel. This flag should be set only if the API request is made directly from the end user such as a mobile app (and not if a gateway or a server is processing and pushing the user events).
    #[serde(rename="directUserRequest")]
    
    pub direct_user_request: Option<bool>,
    /// Optional. IP address of the user. This could be either IPv4 (e.g. 104.133.9.80) or IPv6 (e.g. 2001:0db8:85a3:0000:0000:8a2e:0370:7334). This should *not* be set when using the javascript pixel or if `direct_user_request` is set. Used to extract location information for personalization.
    #[serde(rename="ipAddress")]
    
    pub ip_address: Option<String>,
    /// Optional. User agent as included in the HTTP header. UTF-8 encoded string with a length limit of 1 KiB. This should *not* be set when using the JavaScript pixel or if `directUserRequest` is set.
    #[serde(rename="userAgent")]
    
    pub user_agent: Option<String>,
    /// Optional. Unique identifier for logged-in user with a length limit of 128 bytes. Required only for logged-in users. Don't set for anonymous users. Don't set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
    /// Required. A unique identifier for tracking visitors with a length limit of 128 bytes. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. Maximum length 128 bytes. Cannot be empty. Don't set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::Part for GoogleCloudRecommendationengineV1beta1UserInfo {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations catalogs event stores operations list projects](ProjectLocationCatalogEventStoreOperationListCall) (response)
/// * [locations catalogs operations list projects](ProjectLocationCatalogOperationListCall) (response)
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
/// * [locations catalogs catalog items import projects](ProjectLocationCatalogCatalogItemImportCall) (response)
/// * [locations catalogs event stores operations get projects](ProjectLocationCatalogEventStoreOperationGetCall) (response)
/// * [locations catalogs event stores user events import projects](ProjectLocationCatalogEventStoreUserEventImportCall) (response)
/// * [locations catalogs event stores user events purge projects](ProjectLocationCatalogEventStoreUserEventPurgeCall) (response)
/// * [locations catalogs event stores user events rejoin projects](ProjectLocationCatalogEventStoreUserEventRejoinCall) (response)
/// * [locations catalogs operations get projects](ProjectLocationCatalogOperationGetCall) (response)
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
/// * [locations catalogs catalog items delete projects](ProjectLocationCatalogCatalogItemDeleteCall) (response)
/// * [locations catalogs event stores prediction api key registrations delete projects](ProjectLocationCatalogEventStorePredictionApiKeyRegistrationDeleteCall) (response)
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


