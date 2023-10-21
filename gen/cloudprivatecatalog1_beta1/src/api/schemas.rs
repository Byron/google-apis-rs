use super::*;
/// The readonly representation of a catalog computed with a given resource
/// context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1Catalog {
    /// Output only. The time when the catalog was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The description of the catalog.
    
    pub description: Option<String>,
    /// Output only. The descriptive name of the catalog as it appears in UIs.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The resource name of the target catalog, in the format of
    /// `catalogs/{catalog_id}'.
    
    pub name: Option<String>,
    /// Output only. The time when the catalog was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudPrivatecatalogV1beta1Catalog {}


/// The readonly representation of a product computed with a given resource
/// context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1Product {
    /// Output only. The type of the product asset. It can be one of the
    /// following values:
    /// 
    /// * `google.deploymentmanager.Template`
    /// * `google.cloudprivatecatalog.ListingOnly`
    #[serde(rename="assetType")]
    
    pub asset_type: Option<String>,
    /// Output only. The time when the product was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The display metadata to describe the product.
    /// The JSON schema of the metadata differs by Product.asset_type.
    /// When the type is `google.deploymentmanager.Template`, the schema is as
    /// follows:
    /// 
    /// ````
    /// "$schema": http://json-schema.org/draft-04/schema#
    /// type: object
    /// properties:
    ///   name:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 64
    ///   description:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 2048
    ///   tagline:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 100
    ///   support_info:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 2048
    ///   creator:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 100
    ///   documentation:
    ///     type: array
    ///     items:
    ///       type: object
    ///       properties:
    ///         url:
    ///           type: string
    ///           pattern:
    ///           "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
    ///         title:
    ///           type: string
    ///           minLength: 1
    ///           maxLength: 64
    ///         description:
    ///           type: string
    ///           minLength: 1
    ///           maxLength: 2048
    /// required:
    /// - name
    /// - description
    /// additionalProperties: false
    /// 
    /// ````
    /// 
    /// When the asset type is `google.cloudprivatecatalog.ListingOnly`, the schema
    /// is as follows:
    /// 
    /// ````
    /// "$schema": http://json-schema.org/draft-04/schema#
    /// type: object
    /// properties:
    ///   name:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 64
    ///   description:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 2048
    ///   tagline:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 100
    ///   support_info:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 2048
    ///   creator:
    ///     type: string
    ///     minLength: 1
    ///     maxLength: 100
    ///   documentation:
    ///     type: array
    ///     items:
    ///       type: object
    ///       properties:
    ///         url:
    ///           type: string
    ///           pattern:
    ///           "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
    ///         title:
    ///           type: string
    ///           minLength: 1
    ///           maxLength: 64
    ///         description:
    ///           type: string
    ///           minLength: 1
    ///           maxLength: 2048
    ///   signup_url:
    ///     type: string
    ///     pattern:
    ///     "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
    /// required:
    /// - name
    /// - description
    /// - signup_url
    /// additionalProperties: false
    /// ````
    #[serde(rename="displayMetadata")]
    
    pub display_metadata: Option<HashMap<String, json::Value>>,
    /// Output only. The icon URI of the product.
    #[serde(rename="iconUri")]
    
    pub icon_uri: Option<String>,
    /// Output only. The resource name of the target product, in the format of
    /// `products/a-z*[a-z0-9]'.
    /// 
    /// A unique identifier for the product under a catalog.
    
    pub name: Option<String>,
    /// Output only. The time when the product was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudPrivatecatalogV1beta1Product {}


/// Response message for PrivateCatalog.SearchCatalogs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [catalogs search folders](FolderCatalogSearchCall) (response)
/// * [catalogs search organizations](OrganizationCatalogSearchCall) (response)
/// * [catalogs search projects](ProjectCatalogSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1SearchCatalogsResponse {
    /// The `Catalog`s computed from the resource context.
    
    pub catalogs: Option<Vec<GoogleCloudPrivatecatalogV1beta1Catalog>>,
    /// A pagination token returned from a previous call to SearchCatalogs that
    /// indicates from where listing should continue.
    /// This field is optional.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudPrivatecatalogV1beta1SearchCatalogsResponse {}


/// Response message for PrivateCatalog.SearchProducts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products search folders](FolderProductSearchCall) (response)
/// * [products search organizations](OrganizationProductSearchCall) (response)
/// * [products search projects](ProjectProductSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1SearchProductsResponse {
    /// A pagination token returned from a previous call to SearchProducts that
    /// indicates from where listing should continue.
    /// This field is optional.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Product` resources computed from the resource context.
    
    pub products: Option<Vec<GoogleCloudPrivatecatalogV1beta1Product>>,
}

impl client::ResponseResult for GoogleCloudPrivatecatalogV1beta1SearchProductsResponse {}


/// Response message for PrivateCatalog.SearchVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [versions search folders](FolderVersionSearchCall) (response)
/// * [versions search organizations](OrganizationVersionSearchCall) (response)
/// * [versions search projects](ProjectVersionSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1SearchVersionsResponse {
    /// A pagination token returned from a previous call to SearchVersions that
    /// indicates from where the listing should continue.
    /// This field is optional.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The `Version` resources computed from the resource context.
    
    pub versions: Option<Vec<GoogleCloudPrivatecatalogV1beta1Version>>,
}

impl client::ResponseResult for GoogleCloudPrivatecatalogV1beta1SearchVersionsResponse {}


/// The consumer representation of a version which is a child resource under a
/// `Product` with asset data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudPrivatecatalogV1beta1Version {
    /// Output only. The asset which has been validated and is ready to be
    /// provisioned. See
    /// google.cloud.privatecatalogproducer.v1beta.Version.asset for details.
    
    pub asset: Option<HashMap<String, json::Value>>,
    /// Output only. The time when the version was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The user-supplied description of the version. Maximum of 256
    /// characters.
    
    pub description: Option<String>,
    /// Output only. The resource name of the version, in the format
    /// `catalogs/{catalog_id}/products/{product_id}/versions/a-z*[a-z0-9]'.
    /// 
    /// A unique identifier for the version under a product.
    
    pub name: Option<String>,
    /// Output only. The time when the version was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GoogleCloudPrivatecatalogV1beta1Version {}


