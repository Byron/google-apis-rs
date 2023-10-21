use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`ManufacturerCenter`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_manufacturers1 as manufacturers1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use manufacturers1::{ManufacturerCenter, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ManufacturerCenter::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_delete(...)`, `products_get(...)`, `products_list(...)` and `products_update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ManufacturerCenter<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the product from a Manufacturer Center account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_delete(&self, parent: &str, name: &str) -> AccountProductDeleteCall<'a, S> {
        AccountProductDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the product from a Manufacturer Center account, including product issues. A recently updated product takes around 15 minutes to process. Changes are only visible after it has been processed. While some issues may be available once the product has been processed, other issues may take days to appear.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_get(&self, parent: &str, name: &str) -> AccountProductGetCall<'a, S> {
        AccountProductGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _include: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the products in a Manufacturer Center account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account.
    pub fn products_list(&self, parent: &str) -> AccountProductListCall<'a, S> {
        AccountProductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _include: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or updates the attributes of the product in a Manufacturer Center account. Creates a product with the provided attributes. If the product already exists, then all attributes are replaced with the new ones. The checks at upload time are minimal. All required attributes need to be present for a product to be valid. Issues may show up later after the API has accepted a new upload for a product and it is possible to overwrite an existing valid product with an invalid product. To detect this, you should retrieve the product and check it for issues once the new version is available. Uploaded attributes first need to be processed before they can be retrieved. Until then, new products will be unavailable, and retrieval of previously uploaded products will return the original state of the product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_update(&self, request: Attributes, parent: &str, name: &str) -> AccountProductUpdateCall<'a, S> {
        AccountProductUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



