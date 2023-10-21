use super::*;
/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete license assignments](LicenseAssignmentDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Representation of a license assignment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete license assignments](LicenseAssignmentDeleteCall) (none)
/// * [get license assignments](LicenseAssignmentGetCall) (response)
/// * [insert license assignments](LicenseAssignmentInsertCall) (response)
/// * [list for product license assignments](LicenseAssignmentListForProductCall) (none)
/// * [list for product and sku license assignments](LicenseAssignmentListForProductAndSkuCall) (none)
/// * [patch license assignments](LicenseAssignmentPatchCall) (request|response)
/// * [update license assignments](LicenseAssignmentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LicenseAssignment {
    /// ETag of the resource.
    
    pub etags: Option<String>,
    /// Identifies the resource as a LicenseAssignment, which is `licensing#licenseAssignment`.
    
    pub kind: Option<String>,
    /// A product's unique identifier. For more information about products in this version of the API, see Product and SKU IDs.
    #[serde(rename="productId")]
    
    pub product_id: Option<String>,
    /// Display Name of the product.
    #[serde(rename="productName")]
    
    pub product_name: Option<String>,
    /// Link to this page.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    #[serde(rename="skuId")]
    
    pub sku_id: Option<String>,
    /// Display Name of the sku of the product.
    #[serde(rename="skuName")]
    
    pub sku_name: Option<String>,
    /// The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for LicenseAssignment {}
impl client::Resource for LicenseAssignment {}
impl client::ResponseResult for LicenseAssignment {}


/// Representation of a license assignment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert license assignments](LicenseAssignmentInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LicenseAssignmentInsert {
    /// Email id of the user
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for LicenseAssignmentInsert {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list for product license assignments](LicenseAssignmentListForProductCall) (response)
/// * [list for product and sku license assignments](LicenseAssignmentListForProductAndSkuCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LicenseAssignmentList {
    /// ETag of the resource.
    
    pub etag: Option<String>,
    /// The LicenseAssignments in this page of results.
    
    pub items: Option<Vec<LicenseAssignment>>,
    /// Identifies the resource as a collection of LicenseAssignments.
    
    pub kind: Option<String>,
    /// The token that you must submit in a subsequent request to retrieve additional license results matching your query parameters. The `maxResults` query string is related to the `nextPageToken` since `maxResults` determines how many entries are returned on each next page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for LicenseAssignmentList {}


