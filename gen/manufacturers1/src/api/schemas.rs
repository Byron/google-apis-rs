use super::*;
/// Attributes of the product. For more information, see https://support.google.com/manufacturers/answer/6124116.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products update accounts](AccountProductUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// The additional images of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#addlimage.
    #[serde(rename="additionalImageLink")]
    
    pub additional_image_link: Option<Vec<Image>>,
    /// The target age group of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#agegroup.
    #[serde(rename="ageGroup")]
    
    pub age_group: Option<String>,
    /// The brand name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#brand.
    
    pub brand: Option<String>,
    /// The capacity of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity.
    
    pub capacity: Option<Capacity>,
    /// The color of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#color.
    
    pub color: Option<String>,
    /// The count of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#count.
    
    pub count: Option<Count>,
    /// The description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#description.
    
    pub description: Option<String>,
    /// The disclosure date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#disclosure.
    #[serde(rename="disclosureDate")]
    
    pub disclosure_date: Option<String>,
    /// A list of excluded destinations such as "ClientExport", "ClientShoppingCatalog" or "PartnerShoppingCatalog". For more information, see https://support.google.com/manufacturers/answer/7443550
    #[serde(rename="excludedDestination")]
    
    pub excluded_destination: Option<Vec<String>>,
    /// The rich format description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc.
    #[serde(rename="featureDescription")]
    
    pub feature_description: Option<Vec<FeatureDescription>>,
    /// The flavor of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#flavor.
    
    pub flavor: Option<String>,
    /// The format of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#format.
    
    pub format: Option<String>,
    /// The target gender of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gender.
    
    pub gender: Option<String>,
    /// Grocery Attributes. See more at https://support.google.com/manufacturers/answer/12098458#grocery.
    
    pub grocery: Option<Grocery>,
    /// The Global Trade Item Number (GTIN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gtin.
    
    pub gtin: Option<Vec<String>>,
    /// The image of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#image.
    #[serde(rename="imageLink")]
    
    pub image_link: Option<Image>,
    /// A list of included destinations such as "ClientExport", "ClientShoppingCatalog" or "PartnerShoppingCatalog". For more information, see https://support.google.com/manufacturers/answer/7443550
    #[serde(rename="includedDestination")]
    
    pub included_destination: Option<Vec<String>>,
    /// The item group id of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#itemgroupid.
    #[serde(rename="itemGroupId")]
    
    pub item_group_id: Option<String>,
    /// The material of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#material.
    
    pub material: Option<String>,
    /// The Manufacturer Part Number (MPN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#mpn.
    
    pub mpn: Option<String>,
    /// Nutrition Attributes. See more at https://support.google.com/manufacturers/answer/12098458#food-servings.
    
    pub nutrition: Option<Nutrition>,
    /// The pattern of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#pattern.
    
    pub pattern: Option<String>,
    /// The details of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail.
    #[serde(rename="productDetail")]
    
    pub product_detail: Option<Vec<ProductDetail>>,
    /// The product highlights. For more information, see https://support.google.com/manufacturers/answer/10066942
    #[serde(rename="productHighlight")]
    
    pub product_highlight: Option<Vec<String>>,
    /// The name of the group of products related to the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productline.
    #[serde(rename="productLine")]
    
    pub product_line: Option<String>,
    /// The canonical name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productname.
    #[serde(rename="productName")]
    
    pub product_name: Option<String>,
    /// The URL of the detail page of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productpage.
    #[serde(rename="productPageUrl")]
    
    pub product_page_url: Option<String>,
    /// The type or category of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#producttype.
    #[serde(rename="productType")]
    
    pub product_type: Option<Vec<String>>,
    /// The release date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#release.
    #[serde(rename="releaseDate")]
    
    pub release_date: Option<String>,
    /// Rich product content. For more information, see https://support.google.com/manufacturers/answer/9389865
    #[serde(rename="richProductContent")]
    
    pub rich_product_content: Option<Vec<String>>,
    /// The scent of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#scent.
    
    pub scent: Option<String>,
    /// The size of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#size.
    
    pub size: Option<String>,
    /// The size system of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizesystem.
    #[serde(rename="sizeSystem")]
    
    pub size_system: Option<String>,
    /// The size type of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizetype.
    #[serde(rename="sizeType")]
    
    pub size_type: Option<Vec<String>>,
    /// The suggested retail price (MSRP) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#price.
    #[serde(rename="suggestedRetailPrice")]
    
    pub suggested_retail_price: Option<Price>,
    /// The target client id. Should only be used in the accounts of the data partners. For more information, see https://support.google.com/manufacturers/answer/10857344
    #[serde(rename="targetClientId")]
    
    pub target_client_id: Option<String>,
    /// The theme of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#theme.
    
    pub theme: Option<String>,
    /// The title of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#title.
    
    pub title: Option<String>,
    /// The videos of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#video.
    #[serde(rename="videoLink")]
    
    pub video_link: Option<Vec<String>>,
}

impl client::RequestValue for Attributes {}


/// The capacity of a product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Capacity {
    /// The unit of the capacity, i.e., MB, GB, or TB.
    
    pub unit: Option<String>,
    /// The numeric value of the capacity.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for Capacity {}


/// The number of products in a single package. For more information, see https://support.google.com/manufacturers/answer/6124116#count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Count {
    /// The unit in which these products are counted.
    
    pub unit: Option<String>,
    /// The numeric value of the number of products in a package.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for Count {}


/// The destination status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationStatus {
    /// The name of the destination.
    
    pub destination: Option<String>,
    /// The status of the destination.
    
    pub status: Option<String>,
}

impl client::Part for DestinationStatus {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products delete accounts](AccountProductDeleteCall) (response)
/// * [products update accounts](AccountProductUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A feature description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureDescription {
    /// A short description of the feature.
    
    pub headline: Option<String>,
    /// An optional image describing the feature.
    
    pub image: Option<Image>,
    /// A detailed description of the feature.
    
    pub text: Option<String>,
}

impl client::Part for FeatureDescription {}


/// Combination of float amount and unit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FloatUnit {
    /// amount.
    
    pub amount: Option<f64>,
    /// unit.
    
    pub unit: Option<String>,
}

impl client::Part for FloatUnit {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Grocery {
    /// Active ingredients.
    #[serde(rename="activeIngredients")]
    
    pub active_ingredients: Option<String>,
    /// Alcohol by volume.
    #[serde(rename="alcoholByVolume")]
    
    pub alcohol_by_volume: Option<f64>,
    /// Allergens.
    
    pub allergens: Option<String>,
    /// Derived nutrition claim.
    #[serde(rename="derivedNutritionClaim")]
    
    pub derived_nutrition_claim: Option<Vec<String>>,
    /// Directions.
    
    pub directions: Option<String>,
    /// Indications.
    
    pub indications: Option<String>,
    /// Ingredients.
    
    pub ingredients: Option<String>,
    /// Nutrition claim.
    #[serde(rename="nutritionClaim")]
    
    pub nutrition_claim: Option<Vec<String>>,
    /// Storage instructions.
    #[serde(rename="storageInstructions")]
    
    pub storage_instructions: Option<String>,
}

impl client::Part for Grocery {}


/// An image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// The URL of the image. For crawled images, this is the provided URL. For uploaded images, this is a serving URL from Google if the image has been processed successfully.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// The status of the image. @OutputOnly
    
    pub status: Option<String>,
    /// The type of the image, i.e., crawled or uploaded. @OutputOnly
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Image {}


/// Product issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Issue {
    /// If present, the attribute that triggered the issue. For more information about attributes, see https://support.google.com/manufacturers/answer/6124116.
    
    pub attribute: Option<String>,
    /// Longer description of the issue focused on how to resolve it.
    
    pub description: Option<String>,
    /// The destination this issue applies to.
    
    pub destination: Option<String>,
    /// What needs to happen to resolve the issue.
    
    pub resolution: Option<String>,
    /// The severity of the issue.
    
    pub severity: Option<String>,
    /// The timestamp when this issue appeared.
    
    pub timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Short title describing the nature of the issue.
    
    pub title: Option<String>,
    /// The server-generated type of the issue, for example, “INCORRECT_TEXT_FORMATTING”, “IMAGE_NOT_SERVEABLE”, etc.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Issue {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products list accounts](AccountProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    /// The token for the retrieval of the next page of product statuses.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of the products.
    
    pub products: Option<Vec<Product>>,
}

impl client::ResponseResult for ListProductsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Nutrition {
    /// Added sugars.
    #[serde(rename="addedSugars")]
    
    pub added_sugars: Option<FloatUnit>,
    /// Added sugars daily percentage.
    #[serde(rename="addedSugarsDailyPercentage")]
    
    pub added_sugars_daily_percentage: Option<f64>,
    /// Calcium.
    
    pub calcium: Option<FloatUnit>,
    /// Calcium daily percentage.
    #[serde(rename="calciumDailyPercentage")]
    
    pub calcium_daily_percentage: Option<f64>,
    /// Cholesterol.
    
    pub cholesterol: Option<FloatUnit>,
    /// Cholesterol daily percentage.
    #[serde(rename="cholesterolDailyPercentage")]
    
    pub cholesterol_daily_percentage: Option<f64>,
    /// Dietary fiber.
    #[serde(rename="dietaryFiber")]
    
    pub dietary_fiber: Option<FloatUnit>,
    /// Dietary fiber daily percentage.
    #[serde(rename="dietaryFiberDailyPercentage")]
    
    pub dietary_fiber_daily_percentage: Option<f64>,
    /// Mandatory Nutrition Facts. Energy.
    
    pub energy: Option<FloatUnit>,
    /// Energy from fat.
    #[serde(rename="energyFromFat")]
    
    pub energy_from_fat: Option<FloatUnit>,
    /// Folate daily percentage.
    #[serde(rename="folateDailyPercentage")]
    
    pub folate_daily_percentage: Option<f64>,
    /// Folate folic acid.
    #[serde(rename="folateFolicAcid")]
    
    pub folate_folic_acid: Option<FloatUnit>,
    /// Folate mcg DFE.
    #[serde(rename="folateMcgDfe")]
    
    pub folate_mcg_dfe: Option<f64>,
    /// Iron.
    
    pub iron: Option<FloatUnit>,
    /// Iron daily percentage.
    #[serde(rename="ironDailyPercentage")]
    
    pub iron_daily_percentage: Option<f64>,
    /// Monounsaturated fat.
    #[serde(rename="monounsaturatedFat")]
    
    pub monounsaturated_fat: Option<FloatUnit>,
    /// Nutrition fact measure.
    #[serde(rename="nutritionFactMeasure")]
    
    pub nutrition_fact_measure: Option<String>,
    /// Polyols.
    
    pub polyols: Option<FloatUnit>,
    /// Polyunsaturated fat.
    #[serde(rename="polyunsaturatedFat")]
    
    pub polyunsaturated_fat: Option<FloatUnit>,
    /// Potassium.
    
    pub potassium: Option<FloatUnit>,
    /// Potassium daily percentage.
    #[serde(rename="potassiumDailyPercentage")]
    
    pub potassium_daily_percentage: Option<f64>,
    /// Prepared size description.
    #[serde(rename="preparedSizeDescription")]
    
    pub prepared_size_description: Option<String>,
    /// Protein.
    
    pub protein: Option<FloatUnit>,
    /// Protein daily percentage.
    #[serde(rename="proteinDailyPercentage")]
    
    pub protein_daily_percentage: Option<f64>,
    /// Saturated fat.
    #[serde(rename="saturatedFat")]
    
    pub saturated_fat: Option<FloatUnit>,
    /// Saturated fat daily percentage.
    #[serde(rename="saturatedFatDailyPercentage")]
    
    pub saturated_fat_daily_percentage: Option<f64>,
    /// Food Serving Size. Serving size description.
    #[serde(rename="servingSizeDescription")]
    
    pub serving_size_description: Option<String>,
    /// Serving size measure.
    #[serde(rename="servingSizeMeasure")]
    
    pub serving_size_measure: Option<FloatUnit>,
    /// Servings per container.
    #[serde(rename="servingsPerContainer")]
    
    pub servings_per_container: Option<String>,
    /// Sodium.
    
    pub sodium: Option<FloatUnit>,
    /// Sodium daily percentage.
    #[serde(rename="sodiumDailyPercentage")]
    
    pub sodium_daily_percentage: Option<f64>,
    /// Starch.
    
    pub starch: Option<FloatUnit>,
    /// Total carbohydrate.
    #[serde(rename="totalCarbohydrate")]
    
    pub total_carbohydrate: Option<FloatUnit>,
    /// Total carbohydrate daily percentage.
    #[serde(rename="totalCarbohydrateDailyPercentage")]
    
    pub total_carbohydrate_daily_percentage: Option<f64>,
    /// Total fat.
    #[serde(rename="totalFat")]
    
    pub total_fat: Option<FloatUnit>,
    /// Total fat daily percentage.
    #[serde(rename="totalFatDailyPercentage")]
    
    pub total_fat_daily_percentage: Option<f64>,
    /// Total sugars.
    #[serde(rename="totalSugars")]
    
    pub total_sugars: Option<FloatUnit>,
    /// Total sugars daily percentage.
    #[serde(rename="totalSugarsDailyPercentage")]
    
    pub total_sugars_daily_percentage: Option<f64>,
    /// Trans fat.
    #[serde(rename="transFat")]
    
    pub trans_fat: Option<FloatUnit>,
    /// Trans fat daily percentage.
    #[serde(rename="transFatDailyPercentage")]
    
    pub trans_fat_daily_percentage: Option<f64>,
    /// Vitamin D.
    #[serde(rename="vitaminD")]
    
    pub vitamin_d: Option<FloatUnit>,
    /// Vitamin D daily percentage.
    #[serde(rename="vitaminDDailyPercentage")]
    
    pub vitamin_d_daily_percentage: Option<f64>,
    /// Voluntary nutrition fact.
    #[serde(rename="voluntaryNutritionFact")]
    
    pub voluntary_nutrition_fact: Option<Vec<VoluntaryNutritionFact>>,
}

impl client::Part for Nutrition {}


/// A price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The numeric value of the price.
    
    pub amount: Option<String>,
    /// The currency in which the price is denoted.
    
    pub currency: Option<String>,
}

impl client::Part for Price {}


/// Product data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products get accounts](AccountProductGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// Attributes of the product uploaded to the Manufacturer Center. Manually edited attributes are taken into account.
    
    pub attributes: Option<Attributes>,
    /// The content language of the product as a two-letter ISO 639-1 language code (for example, en).
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// The status of the destinations.
    #[serde(rename="destinationStatuses")]
    
    pub destination_statuses: Option<Vec<DestinationStatus>>,
    /// A server-generated list of issues associated with the product.
    
    pub issues: Option<Vec<Issue>>,
    /// Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id.
    
    pub name: Option<String>,
    /// Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account.
    
    pub parent: Option<String>,
    /// The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// The target country of the product as a CLDR territory code (for example, US).
    #[serde(rename="targetCountry")]
    
    pub target_country: Option<String>,
}

impl client::ResponseResult for Product {}


/// A product detail of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductDetail {
    /// The name of the attribute.
    #[serde(rename="attributeName")]
    
    pub attribute_name: Option<String>,
    /// The value of the attribute.
    #[serde(rename="attributeValue")]
    
    pub attribute_value: Option<String>,
    /// A short section name that can be reused between multiple product details.
    #[serde(rename="sectionName")]
    
    pub section_name: Option<String>,
}

impl client::Part for ProductDetail {}


/// Voluntary Nutrition Facts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VoluntaryNutritionFact {
    /// Daily percentage.
    #[serde(rename="dailyPercentage")]
    
    pub daily_percentage: Option<f64>,
    /// Name.
    
    pub name: Option<String>,
    /// Value.
    
    pub value: Option<FloatUnit>,
}

impl client::Part for VoluntaryNutritionFact {}


