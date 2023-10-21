use super::*;
/// A builder providing access to all methods supported on *catalog* resources.
/// It is not used directly, but through the [`CloudPrivateCatalogProducer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudprivatecatalogproducer1_beta1 as cloudprivatecatalogproducer1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudprivatecatalogproducer1_beta1::{CloudPrivateCatalogProducer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudPrivateCatalogProducer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associations_create(...)`, `associations_delete(...)`, `associations_get(...)`, `associations_list(...)`, `create(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `list(...)`, `patch(...)`, `products_copy(...)`, `products_create(...)`, `products_delete(...)`, `products_get(...)`, `products_icons_upload(...)`, `products_list(...)`, `products_patch(...)`, `products_versions_create(...)`, `products_versions_delete(...)`, `products_versions_get(...)`, `products_versions_list(...)`, `products_versions_patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `undelete(...)`
/// // to build up your call.
/// let rb = hub.catalogs();
/// # }
/// ```
pub struct CatalogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudPrivateCatalogProducer<S>,
}

impl<'a, S> client::MethodsBuilder for CatalogMethods<'a, S> {}

impl<'a, S> CatalogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an Association instance under a given Catalog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The `Catalog` resource's name.
    pub fn associations_create(&self, request: GoogleCloudPrivatecatalogproducerV1beta1CreateAssociationRequest, parent: &str) -> CatalogAssociationCreateCall<'a, S> {
        CatalogAssociationCreateCall {
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
    /// Deletes the given Association.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the `Association` to delete.
    pub fn associations_delete(&self, name: &str) -> CatalogAssociationDeleteCall<'a, S> {
        CatalogAssociationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Association resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the `Association` to retrieve.
    pub fn associations_get(&self, name: &str) -> CatalogAssociationGetCall<'a, S> {
        CatalogAssociationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Association resources under a catalog.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the `Catalog` whose `Associations` are
    ///              being retrieved. In the format `catalogs/<catalog>`.
    pub fn associations_list(&self, parent: &str) -> CatalogAssociationListCall<'a, S> {
        CatalogAssociationListCall {
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
    /// Creates an Icon instance under a given Product.
    /// If Product only has a default icon, a new Icon
    /// instance is created and associated with the given Product.
    /// If Product already has a non-default icon, the action creates
    /// a new Icon instance, associates the newly created
    /// Icon with the given Product and deletes the old icon.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `product` - The resource name of the product.
    pub fn products_icons_upload(&self, request: GoogleCloudPrivatecatalogproducerV1beta1UploadIconRequest, product: &str) -> CatalogProductIconUploadCall<'a, S> {
        CatalogProductIconUploadCall {
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
    /// Creates a Version instance under a given Product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The product name of the new version's parent.
    pub fn products_versions_create(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Version, parent: &str) -> CatalogProductVersionCreateCall<'a, S> {
        CatalogProductVersionCreateCall {
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
    /// Hard deletes a Version.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the version.
    pub fn products_versions_delete(&self, name: &str) -> CatalogProductVersionDeleteCall<'a, S> {
        CatalogProductVersionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Version resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the version.
    pub fn products_versions_get(&self, name: &str) -> CatalogProductVersionGetCall<'a, S> {
        CatalogProductVersionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Version resources that the producer has access to, within the
    /// scope of the parent Product.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent resource.
    pub fn products_versions_list(&self, parent: &str) -> CatalogProductVersionListCall<'a, S> {
        CatalogProductVersionListCall {
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
    /// Updates a specific Version resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the version, in the format
    ///            `catalogs/{catalog_id}/products/{product_id}/versions/a-z*[a-z0-9]'.
    ///            A unique identifier for the version under a product, which can't
    ///            be changed after the version is created. The final segment of the name must
    ///            between 1 and 63 characters in length.
    pub fn products_versions_patch(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Version, name: &str) -> CatalogProductVersionPatchCall<'a, S> {
        CatalogProductVersionPatchCall {
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
    /// Copies a Product under another Catalog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the current product that is copied from.
    pub fn products_copy(&self, request: GoogleCloudPrivatecatalogproducerV1beta1CopyProductRequest, name: &str) -> CatalogProductCopyCall<'a, S> {
        CatalogProductCopyCall {
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
    /// Creates a Product instance under a given Catalog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The catalog name of the new product's parent.
    pub fn products_create(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Product, parent: &str) -> CatalogProductCreateCall<'a, S> {
        CatalogProductCreateCall {
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
    /// Hard deletes a Product.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the product.
    pub fn products_delete(&self, name: &str) -> CatalogProductDeleteCall<'a, S> {
        CatalogProductDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Product resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the product.
    pub fn products_get(&self, name: &str) -> CatalogProductGetCall<'a, S> {
        CatalogProductGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Product resources that the producer has access to, within the
    /// scope of the parent catalog.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the parent resource.
    pub fn products_list(&self, parent: &str) -> CatalogProductListCall<'a, S> {
        CatalogProductListCall {
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
    /// Updates a specific Product resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the product in the format
    ///            `catalogs/{catalog_id}/products/a-z*[a-z0-9]'.
    ///            A unique identifier for the product under a catalog, which cannot
    ///            be changed after the product is created. The final
    ///            segment of the name must between 1 and 256 characters in length.
    pub fn products_patch(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Product, name: &str) -> CatalogProductPatchCall<'a, S> {
        CatalogProductPatchCall {
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
    /// Creates a new Catalog resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Catalog) -> CatalogCreateCall<'a, S> {
        CatalogCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Soft deletes an existing Catalog and all resources under it.
    /// The catalog can only be deleted if there is no associations under it or
    /// DeleteCatalogRequest.force is true. The delete operation
    /// can be recovered by the PrivateCatalogProducer.UndeleteCatalog
    /// method.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the catalog.
    pub fn delete(&self, name: &str) -> CatalogDeleteCall<'a, S> {
        CatalogDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the requested Catalog resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the catalog.
    pub fn get(&self, name: &str) -> CatalogGetCall<'a, S> {
        CatalogGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets IAM policy for the specified Catalog.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn get_iam_policy(&self, resource: &str) -> CatalogGetIamPolicyCall<'a, S> {
        CatalogGetIamPolicyCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Catalog resources that the producer has access to, within the
    /// scope of the parent resource.
    pub fn list(&self) -> CatalogListCall<'a, S> {
        CatalogListCall {
            hub: self.hub,
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a specific Catalog resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the catalog, in the format
    ///            `catalogs/{catalog_id}'.
    ///            A unique identifier for the catalog, which is generated
    ///            by catalog service.
    pub fn patch(&self, request: GoogleCloudPrivatecatalogproducerV1beta1Catalog, name: &str) -> CatalogPatchCall<'a, S> {
        CatalogPatchCall {
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
    /// Sets the IAM policy for the specified Catalog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> CatalogSetIamPolicyCall<'a, S> {
        CatalogSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests the IAM permissions for the specified Catalog.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested.
    ///                See the operation documentation for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> CatalogTestIamPermissionCall<'a, S> {
        CatalogTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a deleted Catalog and all resources under it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the catalog.
    pub fn undelete(&self, request: GoogleCloudPrivatecatalogproducerV1beta1UndeleteCatalogRequest, name: &str) -> CatalogUndeleteCall<'a, S> {
        CatalogUndeleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`CloudPrivateCatalogProducer`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudprivatecatalogproducer1_beta1 as cloudprivatecatalogproducer1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudprivatecatalogproducer1_beta1::{CloudPrivateCatalogProducer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudPrivateCatalogProducer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudPrivateCatalogProducer<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation.  The server
    /// makes a best effort to cancel the operation, but success is not
    /// guaranteed.  If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.  Clients can use
    /// Operations.GetOperation or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an Operation.error value with a google.rpc.Status.code of 1,
    /// corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn cancel(&self, request: GoogleLongrunningCancelOperationRequest, name: &str) -> OperationCancelCall<'a, S> {
        OperationCancelCall {
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
    /// Deletes a long-running operation. This method indicates that the client is
    /// no longer interested in the operation result. It does not cancel the
    /// operation. If the server doesn't support this method, it returns
    /// `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn delete(&self, name: &str) -> OperationDeleteCall<'a, S> {
        OperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation.  Clients can use this
    /// method to poll the operation result at intervals as recommended by the API
    /// service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the
    /// server doesn't support this method, it returns `UNIMPLEMENTED`.
    /// 
    /// NOTE: the `name` binding allows API services to override the binding
    /// to use different resource name schemes, such as `users/*/operations`. To
    /// override the binding, API services can add a binding such as
    /// `"/v1/{name=users/*}/operations"` to their service configuration.
    /// For backwards compatibility, the default name includes the operations
    /// collection id, however overriding users must ensure the name binding
    /// is the parent resource, without the operations collection id.
    pub fn list(&self) -> OperationListCall<'a, S> {
        OperationListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



