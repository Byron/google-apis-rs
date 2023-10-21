use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CloudRetail`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_retail2 as retail2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use retail2::{CloudRetail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudRetail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_catalogs_attributes_config_add_catalog_attribute(...)`, `locations_catalogs_attributes_config_remove_catalog_attribute(...)`, `locations_catalogs_attributes_config_replace_catalog_attribute(...)`, `locations_catalogs_branches_operations_get(...)`, `locations_catalogs_branches_products_add_fulfillment_places(...)`, `locations_catalogs_branches_products_add_local_inventories(...)`, `locations_catalogs_branches_products_create(...)`, `locations_catalogs_branches_products_delete(...)`, `locations_catalogs_branches_products_get(...)`, `locations_catalogs_branches_products_import(...)`, `locations_catalogs_branches_products_list(...)`, `locations_catalogs_branches_products_patch(...)`, `locations_catalogs_branches_products_remove_fulfillment_places(...)`, `locations_catalogs_branches_products_remove_local_inventories(...)`, `locations_catalogs_branches_products_set_inventory(...)`, `locations_catalogs_complete_query(...)`, `locations_catalogs_completion_data_import(...)`, `locations_catalogs_controls_create(...)`, `locations_catalogs_controls_delete(...)`, `locations_catalogs_controls_get(...)`, `locations_catalogs_controls_list(...)`, `locations_catalogs_controls_patch(...)`, `locations_catalogs_get_attributes_config(...)`, `locations_catalogs_get_completion_config(...)`, `locations_catalogs_get_default_branch(...)`, `locations_catalogs_list(...)`, `locations_catalogs_operations_get(...)`, `locations_catalogs_operations_list(...)`, `locations_catalogs_patch(...)`, `locations_catalogs_placements_predict(...)`, `locations_catalogs_placements_search(...)`, `locations_catalogs_serving_configs_add_control(...)`, `locations_catalogs_serving_configs_create(...)`, `locations_catalogs_serving_configs_delete(...)`, `locations_catalogs_serving_configs_get(...)`, `locations_catalogs_serving_configs_list(...)`, `locations_catalogs_serving_configs_patch(...)`, `locations_catalogs_serving_configs_predict(...)`, `locations_catalogs_serving_configs_remove_control(...)`, `locations_catalogs_serving_configs_search(...)`, `locations_catalogs_set_default_branch(...)`, `locations_catalogs_update_attributes_config(...)`, `locations_catalogs_update_completion_config(...)`, `locations_catalogs_user_events_collect(...)`, `locations_catalogs_user_events_import(...)`, `locations_catalogs_user_events_purge(...)`, `locations_catalogs_user_events_rejoin(...)`, `locations_catalogs_user_events_write(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `operations_get(...)` and `operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudRetail<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `attributesConfig` - Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
    pub fn locations_catalogs_attributes_config_add_catalog_attribute(&self, request: GoogleCloudRetailV2AddCatalogAttributeRequest, attributes_config: &str) -> ProjectLocationCatalogAttributesConfigAddCatalogAttributeCall<'a, S> {
        ProjectLocationCatalogAttributesConfigAddCatalogAttributeCall {
            hub: self.hub,
            _request: request,
            _attributes_config: attributes_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified CatalogAttribute from the AttributesConfig. If the CatalogAttribute to remove does not exist, a NOT_FOUND error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `attributesConfig` - Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
    pub fn locations_catalogs_attributes_config_remove_catalog_attribute(&self, request: GoogleCloudRetailV2RemoveCatalogAttributeRequest, attributes_config: &str) -> ProjectLocationCatalogAttributesConfigRemoveCatalogAttributeCall<'a, S> {
        ProjectLocationCatalogAttributesConfigRemoveCatalogAttributeCall {
            hub: self.hub,
            _request: request,
            _attributes_config: attributes_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces the specified CatalogAttribute in the AttributesConfig by updating the catalog attribute with the same CatalogAttribute.key. If the CatalogAttribute to replace does not exist, a NOT_FOUND error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `attributesConfig` - Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
    pub fn locations_catalogs_attributes_config_replace_catalog_attribute(&self, request: GoogleCloudRetailV2ReplaceCatalogAttributeRequest, attributes_config: &str) -> ProjectLocationCatalogAttributesConfigReplaceCatalogAttributeCall<'a, S> {
        ProjectLocationCatalogAttributesConfigReplaceCatalogAttributeCall {
            hub: self.hub,
            _request: request,
            _attributes_config: attributes_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_catalogs_branches_operations_get(&self, name: &str) -> ProjectLocationCatalogBranchOperationGetCall<'a, S> {
        ProjectLocationCatalogBranchOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Incrementally adds place IDs to Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the added place IDs are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `product` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_branches_products_add_fulfillment_places(&self, request: GoogleCloudRetailV2AddFulfillmentPlacesRequest, product: &str) -> ProjectLocationCatalogBranchProductAddFulfillmentPlaceCall<'a, S> {
        ProjectLocationCatalogBranchProductAddFulfillmentPlaceCall {
            hub: self.hub,
            _request: request,
            _product: product.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates local inventory information for a Product at a list of places, while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating inventory information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. Local inventory information can only be modified using this method. ProductService.CreateProduct and ProductService.UpdateProduct has no effect on local inventories. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `product` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_branches_products_add_local_inventories(&self, request: GoogleCloudRetailV2AddLocalInventoriesRequest, product: &str) -> ProjectLocationCatalogBranchProductAddLocalInventoryCall<'a, S> {
        ProjectLocationCatalogBranchProductAddLocalInventoryCall {
            hub: self.hub,
            _request: request,
            _product: product.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`.
    pub fn locations_catalogs_branches_products_create(&self, request: GoogleCloudRetailV2Product, parent: &str) -> ProjectLocationCatalogBranchProductCreateCall<'a, S> {
        ProjectLocationCatalogBranchProductCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _product_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Product.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to delete the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned. If the Product to delete does not exist, a NOT_FOUND error is returned. The Product to delete can neither be a Product.Type.COLLECTION Product member nor a Product.Type.PRIMARY Product with more than one variants. Otherwise, an INVALID_ARGUMENT error is returned. All inventory information for the named Product will be deleted.
    pub fn locations_catalogs_branches_products_delete(&self, name: &str) -> ProjectLocationCatalogBranchProductDeleteCall<'a, S> {
        ProjectLocationCatalogBranchProductDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Product.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned. If the requested Product does not exist, a NOT_FOUND error is returned.
    pub fn locations_catalogs_branches_products_get(&self, name: &str) -> ProjectLocationCatalogBranchProductGetCall<'a, S> {
        ProjectLocationCatalogBranchProductGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk import of multiple Products. Request processing may be synchronous. Non-existing items are created. Note that it is possible for a subset of the Products to be successfully updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. `projects/1234/locations/global/catalogs/default_catalog/branches/default_branch` If no updateMask is specified, requires products.create permission. If updateMask is specified, requires products.update permission.
    pub fn locations_catalogs_branches_products_import(&self, request: GoogleCloudRetailV2ImportProductsRequest, parent: &str) -> ProjectLocationCatalogBranchProductImportCall<'a, S> {
        ProjectLocationCatalogBranchProductImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of Products.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent branch resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/0`. Use `default_branch` as the branch ID, to list products under the default branch. If the caller does not have permission to list Products under this branch, regardless of whether or not this branch exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_branches_products_list(&self, parent: &str) -> ProjectLocationCatalogBranchProductListCall<'a, S> {
        ProjectLocationCatalogBranchProductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`.
    pub fn locations_catalogs_branches_products_patch(&self, request: GoogleCloudRetailV2Product, name: &str) -> ProjectLocationCatalogBranchProductPatchCall<'a, S> {
        ProjectLocationCatalogBranchProductPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Incrementally removes place IDs from a Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the removed place IDs are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `product` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_branches_products_remove_fulfillment_places(&self, request: GoogleCloudRetailV2RemoveFulfillmentPlacesRequest, product: &str) -> ProjectLocationCatalogBranchProductRemoveFulfillmentPlaceCall<'a, S> {
        ProjectLocationCatalogBranchProductRemoveFulfillmentPlaceCall {
            hub: self.hub,
            _request: request,
            _product: product.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove local inventory information for a Product at a list of places at a removal timestamp. This process is asynchronous. If the request is valid, the removal will be enqueued and processed downstream. As a consequence, when a response is returned, removals are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. Local inventory information can only be removed using this method. ProductService.CreateProduct and ProductService.UpdateProduct has no effect on local inventories. The returned Operations will be obsolete after 1 day, and GetOperation API will return NOT_FOUND afterwards. If conflicting updates are issued, the Operations associated with the stale updates will not be marked as done until being obsolete. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `product` - Required. Full resource name of Product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`. If the caller does not have permission to access the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_branches_products_remove_local_inventories(&self, request: GoogleCloudRetailV2RemoveLocalInventoriesRequest, product: &str) -> ProjectLocationCatalogBranchProductRemoveLocalInventoryCall<'a, S> {
        ProjectLocationCatalogBranchProductRemoveLocalInventoryCall {
            hub: self.hub,
            _request: request,
            _product: product.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates inventory information for a Product while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update is enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by ProductService.GetProduct or ProductService.ListProducts. When inventory is updated with ProductService.CreateProduct and ProductService.UpdateProduct, the specified inventory field value(s) overwrite any existing value(s) while ignoring the last update time for this field. Furthermore, the last update times for the specified inventory fields are overwritten by the times of the ProductService.CreateProduct or ProductService.UpdateProduct request. If no inventory fields are set in CreateProductRequest.product, then any pre-existing inventory information for this product is used. If no inventory fields are set in SetInventoryRequest.set_mask, then any existing inventory information is preserved. Pre-existing inventory information can only be updated with ProductService.SetInventory, ProductService.AddFulfillmentPlaces, and ProductService.RemoveFulfillmentPlaces. The returned Operations is obsolete after one day, and the GetOperation API returns `NOT_FOUND` afterwards. If conflicting updates are issued, the Operations associated with the stale updates are not marked as done until they are obsolete. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`.
    pub fn locations_catalogs_branches_products_set_inventory(&self, request: GoogleCloudRetailV2SetInventoryRequest, name: &str) -> ProjectLocationCatalogBranchProductSetInventoryCall<'a, S> {
        ProjectLocationCatalogBranchProductSetInventoryCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_completion_data_import(&self, request: GoogleCloudRetailV2ImportCompletionDataRequest, parent: &str) -> ProjectLocationCatalogCompletionDataImportCall<'a, S> {
        ProjectLocationCatalogCompletionDataImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
    pub fn locations_catalogs_controls_create(&self, request: GoogleCloudRetailV2Control, parent: &str) -> ProjectLocationCatalogControlCreateCall<'a, S> {
        ProjectLocationCatalogControlCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _control_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a Control. If the Control to delete does not exist, a NOT_FOUND error is returned.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Control to delete. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/controls/{control_id}`
    pub fn locations_catalogs_controls_delete(&self, name: &str) -> ProjectLocationCatalogControlDeleteCall<'a, S> {
        ProjectLocationCatalogControlDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Control.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Control to get. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/controls/{control_id}`
    pub fn locations_catalogs_controls_get(&self, name: &str) -> ProjectLocationCatalogControlGetCall<'a, S> {
        ProjectLocationCatalogControlGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Controls by their parent Catalog.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The catalog resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
    pub fn locations_catalogs_controls_list(&self, parent: &str) -> ProjectLocationCatalogControlListCall<'a, S> {
        ProjectLocationCatalogControlListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Control. Control cannot be set to a different oneof field, if so an INVALID_ARGUMENT is returned. If the Control to update does not exist, a NOT_FOUND error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*`
    pub fn locations_catalogs_controls_patch(&self, request: GoogleCloudRetailV2Control, name: &str) -> ProjectLocationCatalogControlPatchCall<'a, S> {
        ProjectLocationCatalogControlPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_catalogs_operations_get(&self, name: &str) -> ProjectLocationCatalogOperationGetCall<'a, S> {
        ProjectLocationCatalogOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_catalogs_operations_list(&self, name: &str) -> ProjectLocationCatalogOperationListCall<'a, S> {
        ProjectLocationCatalogOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Makes a recommendation prediction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `placement` - Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs
    pub fn locations_catalogs_placements_predict(&self, request: GoogleCloudRetailV2PredictRequest, placement: &str) -> ProjectLocationCatalogPlacementPredictCall<'a, S> {
        ProjectLocationCatalogPlacementPredictCall {
            hub: self.hub,
            _request: request,
            _placement: placement.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a search. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `placement` - Required. The resource name of the Retail Search serving config, such as `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` or the name of the legacy placement resource, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search`. This field is used to identify the serving config name and the set of models that will be used to make the search.
    pub fn locations_catalogs_placements_search(&self, request: GoogleCloudRetailV2SearchRequest, placement: &str) -> ProjectLocationCatalogPlacementSearchCall<'a, S> {
        ProjectLocationCatalogPlacementSearchCall {
            hub: self.hub,
            _request: request,
            _placement: placement.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enables a Control on the specified ServingConfig. The control is added in the last position of the list of controls it belongs to (e.g. if it's a facet spec control it will be applied in the last position of servingConfig.facetSpecIds) Returns a ALREADY_EXISTS error if the control has already been applied. Returns a FAILED_PRECONDITION error if the addition could exceed maximum number of control allowed for that type of control.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `servingConfig` - Required. The source ServingConfig resource name . Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`
    pub fn locations_catalogs_serving_configs_add_control(&self, request: GoogleCloudRetailV2AddControlRequest, serving_config: &str) -> ProjectLocationCatalogServingConfigAddControlCall<'a, S> {
        ProjectLocationCatalogServingConfigAddControlCall {
            hub: self.hub,
            _request: request,
            _serving_config: serving_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
    pub fn locations_catalogs_serving_configs_create(&self, request: GoogleCloudRetailV2ServingConfig, parent: &str) -> ProjectLocationCatalogServingConfigCreateCall<'a, S> {
        ProjectLocationCatalogServingConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _serving_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a ServingConfig. Returns a NotFound error if the ServingConfig does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the ServingConfig to delete. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`
    pub fn locations_catalogs_serving_configs_delete(&self, name: &str) -> ProjectLocationCatalogServingConfigDeleteCall<'a, S> {
        ProjectLocationCatalogServingConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a ServingConfig. Returns a NotFound error if the ServingConfig does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the ServingConfig to get. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`
    pub fn locations_catalogs_serving_configs_get(&self, name: &str) -> ProjectLocationCatalogServingConfigGetCall<'a, S> {
        ProjectLocationCatalogServingConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all ServingConfigs linked to this catalog.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The catalog resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
    pub fn locations_catalogs_serving_configs_list(&self, parent: &str) -> ProjectLocationCatalogServingConfigListCall<'a, S> {
        ProjectLocationCatalogServingConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a ServingConfig.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*`
    pub fn locations_catalogs_serving_configs_patch(&self, request: GoogleCloudRetailV2ServingConfig, name: &str) -> ProjectLocationCatalogServingConfigPatchCall<'a, S> {
        ProjectLocationCatalogServingConfigPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Makes a recommendation prediction.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `placement` - Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs
    pub fn locations_catalogs_serving_configs_predict(&self, request: GoogleCloudRetailV2PredictRequest, placement: &str) -> ProjectLocationCatalogServingConfigPredictCall<'a, S> {
        ProjectLocationCatalogServingConfigPredictCall {
            hub: self.hub,
            _request: request,
            _placement: placement.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disables a Control on the specified ServingConfig. The control is removed from the ServingConfig. Returns a NOT_FOUND error if the Control is not enabled for the ServingConfig.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `servingConfig` - Required. The source ServingConfig resource name . Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/servingConfigs/{serving_config_id}`
    pub fn locations_catalogs_serving_configs_remove_control(&self, request: GoogleCloudRetailV2RemoveControlRequest, serving_config: &str) -> ProjectLocationCatalogServingConfigRemoveControlCall<'a, S> {
        ProjectLocationCatalogServingConfigRemoveControlCall {
            hub: self.hub,
            _request: request,
            _serving_config: serving_config.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a search. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `placement` - Required. The resource name of the Retail Search serving config, such as `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` or the name of the legacy placement resource, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search`. This field is used to identify the serving config name and the set of models that will be used to make the search.
    pub fn locations_catalogs_serving_configs_search(&self, request: GoogleCloudRetailV2SearchRequest, placement: &str) -> ProjectLocationCatalogServingConfigSearchCall<'a, S> {
        ProjectLocationCatalogServingConfigSearchCall {
            hub: self.hub,
            _request: request,
            _placement: placement.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Retail API JavaScript pixel and Google Tag Manager. Users should not call this method directly.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent catalog name, such as `projects/1234/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_user_events_collect(&self, parent: &str) -> ProjectLocationCatalogUserEventCollectCall<'a, S> {
        ProjectLocationCatalogUserEventCollectCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _user_event: Default::default(),
            _uri: Default::default(),
            _raw_json: Default::default(),
            _prebuilt_rule: Default::default(),
            _ets: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. `Operation.response` is of type `ImportResponse`. Note that it is possible for a subset of the items to be successfully inserted. `Operation.metadata` is of type `ImportMetadata`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. `projects/1234/locations/global/catalogs/default_catalog`
    pub fn locations_catalogs_user_events_import(&self, request: GoogleCloudRetailV2ImportUserEventsRequest, parent: &str) -> ProjectLocationCatalogUserEventImportCall<'a, S> {
        ProjectLocationCatalogUserEventImportCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the catalog under which the events are created. The format is `projects/${projectId}/locations/global/catalogs/${catalogId}`
    pub fn locations_catalogs_user_events_purge(&self, request: GoogleCloudRetailV2PurgeUserEventsRequest, parent: &str) -> ProjectLocationCatalogUserEventPurgeCall<'a, S> {
        ProjectLocationCatalogUserEventPurgeCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a user-event rejoin operation with latest product catalog. Events are not annotated with detailed product information for products that are missing from the catalog when the user event is ingested. These events are stored as unjoined events with limited usage on training and serving. You can use this method to start a join operation on specified events with the latest version of product catalog. You can also use this method to correct events joined with the wrong product catalog. A rejoin operation can take hours or days to complete.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent catalog resource name, such as `projects/1234/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_user_events_rejoin(&self, request: GoogleCloudRetailV2RejoinUserEventsRequest, parent: &str) -> ProjectLocationCatalogUserEventRejoinCall<'a, S> {
        ProjectLocationCatalogUserEventRejoinCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Writes a single user event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent catalog resource name, such as `projects/1234/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_user_events_write(&self, request: GoogleCloudRetailV2UserEvent, parent: &str) -> ProjectLocationCatalogUserEventWriteCall<'a, S> {
        ProjectLocationCatalogUserEventWriteCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _write_async: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes the specified prefix with keyword suggestions. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.
    /// 
    /// # Arguments
    ///
    /// * `catalog` - Required. Catalog for which the completion is performed. Full resource name of catalog, such as `projects/*/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_complete_query(&self, catalog: &str) -> ProjectLocationCatalogCompleteQueryCall<'a, S> {
        ProjectLocationCatalogCompleteQueryCall {
            hub: self.hub,
            _catalog: catalog.to_string(),
            _visitor_id: Default::default(),
            _query: Default::default(),
            _max_suggestions: Default::default(),
            _language_codes: Default::default(),
            _device_type: Default::default(),
            _dataset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an AttributesConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
    pub fn locations_catalogs_get_attributes_config(&self, name: &str) -> ProjectLocationCatalogGetAttributesConfigCall<'a, S> {
        ProjectLocationCatalogGetAttributesConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a CompletionConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full CompletionConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/completionConfig`
    pub fn locations_catalogs_get_completion_config(&self, name: &str) -> ProjectLocationCatalogGetCompletionConfigCall<'a, S> {
        ProjectLocationCatalogGetCompletionConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get which branch is currently default branch set by CatalogService.SetDefaultBranch method under a specified parent catalog.
    /// 
    /// # Arguments
    ///
    /// * `catalog` - The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_get_default_branch(&self, catalog: &str) -> ProjectLocationCatalogGetDefaultBranchCall<'a, S> {
        ProjectLocationCatalogGetDefaultBranchCall {
            hub: self.hub,
            _catalog: catalog.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the Catalogs associated with the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account resource name with an associated location. If the caller does not have permission to list Catalogs under this location, regardless of whether or not this location exists, a PERMISSION_DENIED error is returned.
    pub fn locations_catalogs_list(&self, parent: &str) -> ProjectLocationCatalogListCall<'a, S> {
        ProjectLocationCatalogListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Catalogs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Immutable. The fully qualified resource name of the catalog.
    pub fn locations_catalogs_patch(&self, request: GoogleCloudRetailV2Catalog, name: &str) -> ProjectLocationCatalogPatchCall<'a, S> {
        ProjectLocationCatalogPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set a specified branch id as default branch. API methods such as SearchService.Search, ProductService.GetProduct, ProductService.ListProducts will treat requests using "default_branch" to the actual branch id set as default. For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as default, setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent to setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/1`. Using multiple branches can be useful when developers would like to have a staging branch to test and verify for future usage. When it becomes ready, developers switch on the staging branch using this API while keeping using `projects/*/locations/*/catalogs/*/branches/default_branch` as SearchRequest.branch to route the traffic to this staging branch. CAUTION: If you have live predict/search traffic, switching the default branch could potentially cause outages if the ID space of the new branch is very different from the old one. More specifically: * PredictionService will only return product IDs from branch {newBranch}. * SearchService will only return product IDs from branch {newBranch} (if branch is not explicitly set). * UserEventService will only join events with products from branch {newBranch}.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `catalog` - Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_set_default_branch(&self, request: GoogleCloudRetailV2SetDefaultBranchRequest, catalog: &str) -> ProjectLocationCatalogSetDefaultBranchCall<'a, S> {
        ProjectLocationCatalogSetDefaultBranchCall {
            hub: self.hub,
            _request: request,
            _catalog: catalog.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the AttributesConfig. The catalog attributes in the request will be updated in the catalog, or inserted if they do not exist. Existing catalog attributes not included in the request will remain unchanged. Attributes that are assigned to products, but do not exist at the catalog level, are always included in the response. The product attribute is assigned default values for missing catalog attribute fields, e.g., searchable and dynamic facetable options.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Immutable. The fully qualified resource name of the attribute config. Format: `projects/*/locations/*/catalogs/*/attributesConfig`
    pub fn locations_catalogs_update_attributes_config(&self, request: GoogleCloudRetailV2AttributesConfig, name: &str) -> ProjectLocationCatalogUpdateAttributesConfigCall<'a, S> {
        ProjectLocationCatalogUpdateAttributesConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the CompletionConfigs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Immutable. Fully qualified name `projects/*/locations/*/catalogs/*/completionConfig`
    pub fn locations_catalogs_update_completion_config(&self, request: GoogleCloudRetailV2CompletionConfig, name: &str) -> ProjectLocationCatalogUpdateCompletionConfigCall<'a, S> {
        ProjectLocationCatalogUpdateCompletionConfigCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> ProjectLocationOperationGetCall<'a, S> {
        ProjectLocationOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn locations_operations_list(&self, name: &str) -> ProjectLocationOperationListCall<'a, S> {
        ProjectLocationOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn operations_get(&self, name: &str) -> ProjectOperationGetCall<'a, S> {
        ProjectOperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `"/v1/{name=users/*}/operations"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation's parent resource.
    pub fn operations_list(&self, name: &str) -> ProjectOperationListCall<'a, S> {
        ProjectOperationListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



