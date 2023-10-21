use super::*;
/// A builder providing access to all methods supported on *licenseAssignment* resources.
/// It is not used directly, but through the [`Licensing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_licensing1 as licensing1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use licensing1::{Licensing, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Licensing::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list_for_product(...)`, `list_for_product_and_sku(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.license_assignments();
/// # }
/// ```
pub struct LicenseAssignmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Licensing<S>,
}

impl<'a, S> client::MethodsBuilder for LicenseAssignmentMethods<'a, S> {}

impl<'a, S> LicenseAssignmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Revoke a license.
    /// 
    /// # Arguments
    ///
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    /// * `userId` - The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes.
    pub fn delete(&self, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentDeleteCall<'a, S> {
        LicenseAssignmentDeleteCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific user's license by product SKU.
    /// 
    /// # Arguments
    ///
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    /// * `userId` - The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes.
    pub fn get(&self, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentGetCall<'a, S> {
        LicenseAssignmentGetCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Assign a license.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    pub fn insert(&self, request: LicenseAssignmentInsert, product_id: &str, sku_id: &str) -> LicenseAssignmentInsertCall<'a, S> {
        LicenseAssignmentInsertCall {
            hub: self.hub,
            _request: request,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all users assigned licenses for a specific product SKU.
    /// 
    /// # Arguments
    ///
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `customerId` - The customer's unique ID as defined in the Admin console, such as `C00000000`. If the customer is suspended, the server returns an error.
    pub fn list_for_product(&self, product_id: &str, customer_id: &str) -> LicenseAssignmentListForProductCall<'a, S> {
        LicenseAssignmentListForProductCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _customer_id: customer_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all users assigned licenses for a specific product SKU.
    /// 
    /// # Arguments
    ///
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    /// * `customerId` - The customer's unique ID as defined in the Admin console, such as `C00000000`. If the customer is suspended, the server returns an error.
    pub fn list_for_product_and_sku(&self, product_id: &str, sku_id: &str, customer_id: &str) -> LicenseAssignmentListForProductAndSkuCall<'a, S> {
        LicenseAssignmentListForProductAndSkuCall {
            hub: self.hub,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _customer_id: customer_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reassign a user's product SKU with a different SKU in the same product. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    /// * `userId` - The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes.
    pub fn patch(&self, request: LicenseAssignment, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentPatchCall<'a, S> {
        LicenseAssignmentPatchCall {
            hub: self.hub,
            _request: request,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reassign a user's product SKU with a different SKU in the same product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `productId` - A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    /// * `skuId` - A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
    /// * `userId` - The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes.
    pub fn update(&self, request: LicenseAssignment, product_id: &str, sku_id: &str, user_id: &str) -> LicenseAssignmentUpdateCall<'a, S> {
        LicenseAssignmentUpdateCall {
            hub: self.hub,
            _request: request,
            _product_id: product_id.to_string(),
            _sku_id: sku_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



