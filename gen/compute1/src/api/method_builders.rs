use super::*;
/// A builder providing access to all methods supported on *acceleratorType* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.accelerator_types();
/// # }
/// ```
pub struct AcceleratorTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for AcceleratorTypeMethods<'a, S> {}

impl<'a, S> AcceleratorTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of accelerator types.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> AcceleratorTypeAggregatedListCall<'a, S> {
        AcceleratorTypeAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified accelerator type.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `acceleratorType` - Name of the accelerator type to return.
    pub fn get(&self, project: &str, zone: &str, accelerator_type: &str) -> AcceleratorTypeGetCall<'a, S> {
        AcceleratorTypeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _accelerator_type: accelerator_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of accelerator types that are available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> AcceleratorTypeListCall<'a, S> {
        AcceleratorTypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *address* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.addresses();
/// # }
/// ```
pub struct AddressMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for AddressMethods<'a, S> {}

impl<'a, S> AddressMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of addresses.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> AddressAggregatedListCall<'a, S> {
        AddressAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified address resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `address` - Name of the address resource to delete.
    pub fn delete(&self, project: &str, region: &str, address: &str) -> AddressDeleteCall<'a, S> {
        AddressDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _address: address.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified address resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `address` - Name of the address resource to return.
    pub fn get(&self, project: &str, region: &str, address: &str) -> AddressGetCall<'a, S> {
        AddressGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _address: address.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an address resource in the specified project by using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: Address, project: &str, region: &str) -> AddressInsertCall<'a, S> {
        AddressInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of addresses contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> AddressListCall<'a, S> {
        AddressListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on an Address. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> AddressSetLabelCall<'a, S> {
        AddressSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *autoscaler* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.autoscalers();
/// # }
/// ```
pub struct AutoscalerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for AutoscalerMethods<'a, S> {}

impl<'a, S> AutoscalerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of autoscalers.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> AutoscalerAggregatedListCall<'a, S> {
        AutoscalerAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified autoscaler.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `autoscaler` - Name of the autoscaler to delete.
    pub fn delete(&self, project: &str, zone: &str, autoscaler: &str) -> AutoscalerDeleteCall<'a, S> {
        AutoscalerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified autoscaler resource. Gets a list of available autoscalers by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `autoscaler` - Name of the autoscaler to return.
    pub fn get(&self, project: &str, zone: &str, autoscaler: &str) -> AutoscalerGetCall<'a, S> {
        AutoscalerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an autoscaler in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn insert(&self, request: Autoscaler, project: &str, zone: &str) -> AutoscalerInsertCall<'a, S> {
        AutoscalerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of autoscalers contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> AutoscalerListCall<'a, S> {
        AutoscalerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an autoscaler in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn patch(&self, request: Autoscaler, project: &str, zone: &str) -> AutoscalerPatchCall<'a, S> {
        AutoscalerPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _autoscaler: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an autoscaler in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn update(&self, request: Autoscaler, project: &str, zone: &str) -> AutoscalerUpdateCall<'a, S> {
        AutoscalerUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _autoscaler: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *backendBucket* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_signed_url_key(...)`, `delete(...)`, `delete_signed_url_key(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_edge_security_policy(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.backend_buckets();
/// # }
/// ```
pub struct BackendBucketMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for BackendBucketMethods<'a, S> {}

impl<'a, S> BackendBucketMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a key for validating requests with signed URLs for this backend bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to which the Signed URL Key should be added. The name should conform to RFC1035.
    pub fn add_signed_url_key(&self, request: SignedUrlKey, project: &str, backend_bucket: &str) -> BackendBucketAddSignedUrlKeyCall<'a, S> {
        BackendBucketAddSignedUrlKeyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified BackendBucket resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to delete.
    pub fn delete(&self, project: &str, backend_bucket: &str) -> BackendBucketDeleteCall<'a, S> {
        BackendBucketDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a key for validating requests with signed URLs for this backend bucket.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to which the Signed URL Key should be added. The name should conform to RFC1035.
    /// * `keyName` - The name of the Signed URL Key to delete.
    pub fn delete_signed_url_key(&self, project: &str, backend_bucket: &str, key_name: &str) -> BackendBucketDeleteSignedUrlKeyCall<'a, S> {
        BackendBucketDeleteSignedUrlKeyCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _key_name: key_name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified BackendBucket resource. Gets a list of available backend buckets by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to return.
    pub fn get(&self, project: &str, backend_bucket: &str) -> BackendBucketGetCall<'a, S> {
        BackendBucketGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BackendBucket resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: BackendBucket, project: &str) -> BackendBucketInsertCall<'a, S> {
        BackendBucketInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of BackendBucket resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> BackendBucketListCall<'a, S> {
        BackendBucketListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified BackendBucket resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to patch.
    pub fn patch(&self, request: BackendBucket, project: &str, backend_bucket: &str) -> BackendBucketPatchCall<'a, S> {
        BackendBucketPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the edge security policy for the specified backend bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendService resource to which the security policy should be set. The name should conform to RFC1035.
    pub fn set_edge_security_policy(&self, request: SecurityPolicyReference, project: &str, backend_bucket: &str) -> BackendBucketSetEdgeSecurityPolicyCall<'a, S> {
        BackendBucketSetEdgeSecurityPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified BackendBucket resource with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendBucket` - Name of the BackendBucket resource to update.
    pub fn update(&self, request: BackendBucket, project: &str, backend_bucket: &str) -> BackendBucketUpdateCall<'a, S> {
        BackendBucketUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_bucket: backend_bucket.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *backendService* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_signed_url_key(...)`, `aggregated_list(...)`, `delete(...)`, `delete_signed_url_key(...)`, `get(...)`, `get_health(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_edge_security_policy(...)`, `set_iam_policy(...)`, `set_security_policy(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.backend_services();
/// # }
/// ```
pub struct BackendServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for BackendServiceMethods<'a, S> {}

impl<'a, S> BackendServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a key for validating requests with signed URLs for this backend service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to which the Signed URL Key should be added. The name should conform to RFC1035.
    pub fn add_signed_url_key(&self, request: SignedUrlKey, project: &str, backend_service: &str) -> BackendServiceAddSignedUrlKeyCall<'a, S> {
        BackendServiceAddSignedUrlKeyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all BackendService resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> BackendServiceAggregatedListCall<'a, S> {
        BackendServiceAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified BackendService resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to delete.
    pub fn delete(&self, project: &str, backend_service: &str) -> BackendServiceDeleteCall<'a, S> {
        BackendServiceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a key for validating requests with signed URLs for this backend service.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to which the Signed URL Key should be added. The name should conform to RFC1035.
    /// * `keyName` - The name of the Signed URL Key to delete.
    pub fn delete_signed_url_key(&self, project: &str, backend_service: &str, key_name: &str) -> BackendServiceDeleteSignedUrlKeyCall<'a, S> {
        BackendServiceDeleteSignedUrlKeyCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _key_name: key_name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified BackendService resource. Gets a list of available backend services.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to return.
    pub fn get(&self, project: &str, backend_service: &str) -> BackendServiceGetCall<'a, S> {
        BackendServiceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the most recent health check results for this BackendService. Example request body: { "group": "/zones/us-east1-b/instanceGroups/lb-backend-example" }
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - No description provided.
    /// * `backendService` - Name of the BackendService resource to which the queried instance belongs.
    pub fn get_health(&self, request: ResourceGroupReference, project: &str, backend_service: &str) -> BackendServiceGetHealthCall<'a, S> {
        BackendServiceGetHealthCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> BackendServiceGetIamPolicyCall<'a, S> {
        BackendServiceGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a BackendService resource in the specified project using the data included in the request. For more information, see Backend services overview .
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: BackendService, project: &str) -> BackendServiceInsertCall<'a, S> {
        BackendServiceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of BackendService resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> BackendServiceListCall<'a, S> {
        BackendServiceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified BackendService resource with the data included in the request. For more information, see Backend services overview. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to patch.
    pub fn patch(&self, request: BackendService, project: &str, backend_service: &str) -> BackendServicePatchCall<'a, S> {
        BackendServicePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the edge security policy for the specified backend service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to which the edge security policy should be set. The name should conform to RFC1035.
    pub fn set_edge_security_policy(&self, request: SecurityPolicyReference, project: &str, backend_service: &str) -> BackendServiceSetEdgeSecurityPolicyCall<'a, S> {
        BackendServiceSetEdgeSecurityPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> BackendServiceSetIamPolicyCall<'a, S> {
        BackendServiceSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the Google Cloud Armor security policy for the specified backend service. For more information, see Google Cloud Armor Overview
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to which the security policy should be set. The name should conform to RFC1035.
    pub fn set_security_policy(&self, request: SecurityPolicyReference, project: &str, backend_service: &str) -> BackendServiceSetSecurityPolicyCall<'a, S> {
        BackendServiceSetSecurityPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified BackendService resource with the data included in the request. For more information, see Backend services overview.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `backendService` - Name of the BackendService resource to update.
    pub fn update(&self, request: BackendService, project: &str, backend_service: &str) -> BackendServiceUpdateCall<'a, S> {
        BackendServiceUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *diskType* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.disk_types();
/// # }
/// ```
pub struct DiskTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for DiskTypeMethods<'a, S> {}

impl<'a, S> DiskTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of disk types.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> DiskTypeAggregatedListCall<'a, S> {
        DiskTypeAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified disk type. Gets a list of available disk types by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `diskType` - Name of the disk type to return.
    pub fn get(&self, project: &str, zone: &str, disk_type: &str) -> DiskTypeGetCall<'a, S> {
        DiskTypeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk_type: disk_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of disk types available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> DiskTypeListCall<'a, S> {
        DiskTypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *disk* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_resource_policies(...)`, `aggregated_list(...)`, `create_snapshot(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `remove_resource_policies(...)`, `resize(...)`, `set_iam_policy(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.disks();
/// # }
/// ```
pub struct DiskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for DiskMethods<'a, S> {}

impl<'a, S> DiskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds existing resource policies to a disk. You can only add one policy which will be applied to this disk for scheduling snapshot creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - The disk name for this request.
    pub fn add_resource_policies(&self, request: DisksAddResourcePoliciesRequest, project: &str, zone: &str, disk: &str) -> DiskAddResourcePolicyCall<'a, S> {
        DiskAddResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of persistent disks.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> DiskAggregatedListCall<'a, S> {
        DiskAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a snapshot of a specified persistent disk. For regular snapshot creation, consider using snapshots.insert instead, as that method supports more features, such as creating snapshots in a project different from the source disk project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - Name of the persistent disk to snapshot.
    pub fn create_snapshot(&self, request: Snapshot, project: &str, zone: &str, disk: &str) -> DiskCreateSnapshotCall<'a, S> {
        DiskCreateSnapshotCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _guest_flush: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified persistent disk. Deleting a disk removes its data permanently and is irreversible. However, deleting a disk does not delete any snapshots previously made from the disk. You must separately delete snapshots.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - Name of the persistent disk to delete.
    pub fn delete(&self, project: &str, zone: &str, disk: &str) -> DiskDeleteCall<'a, S> {
        DiskDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specified persistent disk. Gets a list of available persistent disks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - Name of the persistent disk to return.
    pub fn get(&self, project: &str, zone: &str, disk: &str) -> DiskGetCall<'a, S> {
        DiskGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, zone: &str, resource: &str) -> DiskGetIamPolicyCall<'a, S> {
        DiskGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a persistent disk in the specified project using the data in the request. You can create a disk from a source (sourceImage, sourceSnapshot, or sourceDisk) or create an empty 500 GB data disk by omitting all properties. You can also create a disk that is larger than the default size by specifying the sizeGb property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn insert(&self, request: Disk, project: &str, zone: &str) -> DiskInsertCall<'a, S> {
        DiskInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _source_image: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of persistent disks contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> DiskListCall<'a, S> {
        DiskListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes resource policies from a disk.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - The disk name for this request.
    pub fn remove_resource_policies(&self, request: DisksRemoveResourcePoliciesRequest, project: &str, zone: &str, disk: &str) -> DiskRemoveResourcePolicyCall<'a, S> {
        DiskRemoveResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the specified persistent disk. You can only increase the size of the disk.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `disk` - The name of the persistent disk.
    pub fn resize(&self, request: DisksResizeRequest, project: &str, zone: &str, disk: &str) -> DiskResizeCall<'a, S> {
        DiskResizeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: ZoneSetPolicyRequest, project: &str, zone: &str, resource: &str) -> DiskSetIamPolicyCall<'a, S> {
        DiskSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a disk. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: ZoneSetLabelsRequest, project: &str, zone: &str, resource: &str) -> DiskSetLabelCall<'a, S> {
        DiskSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, zone: &str, resource: &str) -> DiskTestIamPermissionCall<'a, S> {
        DiskTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *externalVpnGateway* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.external_vpn_gateways();
/// # }
/// ```
pub struct ExternalVpnGatewayMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ExternalVpnGatewayMethods<'a, S> {}

impl<'a, S> ExternalVpnGatewayMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified externalVpnGateway.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `externalVpnGateway` - Name of the externalVpnGateways to delete.
    pub fn delete(&self, project: &str, external_vpn_gateway: &str) -> ExternalVpnGatewayDeleteCall<'a, S> {
        ExternalVpnGatewayDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _external_vpn_gateway: external_vpn_gateway.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified externalVpnGateway. Get a list of available externalVpnGateways by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `externalVpnGateway` - Name of the externalVpnGateway to return.
    pub fn get(&self, project: &str, external_vpn_gateway: &str) -> ExternalVpnGatewayGetCall<'a, S> {
        ExternalVpnGatewayGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _external_vpn_gateway: external_vpn_gateway.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a ExternalVpnGateway in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: ExternalVpnGateway, project: &str) -> ExternalVpnGatewayInsertCall<'a, S> {
        ExternalVpnGatewayInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of ExternalVpnGateway available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> ExternalVpnGatewayListCall<'a, S> {
        ExternalVpnGatewayListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on an ExternalVpnGateway. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> ExternalVpnGatewaySetLabelCall<'a, S> {
        ExternalVpnGatewaySetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> ExternalVpnGatewayTestIamPermissionCall<'a, S> {
        ExternalVpnGatewayTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *firewallPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_association(...)`, `add_rule(...)`, `clone_rules(...)`, `delete(...)`, `get(...)`, `get_association(...)`, `get_iam_policy(...)`, `get_rule(...)`, `insert(...)`, `list(...)`, `list_associations(...)`, `move_(...)`, `patch(...)`, `patch_rule(...)`, `remove_association(...)`, `remove_rule(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.firewall_policies();
/// # }
/// ```
pub struct FirewallPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for FirewallPolicyMethods<'a, S> {}

impl<'a, S> FirewallPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an association for the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_association(&self, request: FirewallPolicyAssociation, firewall_policy: &str) -> FirewallPolicyAddAssociationCall<'a, S> {
        FirewallPolicyAddAssociationCall {
            hub: self.hub,
            _request: request,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _replace_existing_association: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a rule into a firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_rule(&self, request: FirewallPolicyRule, firewall_policy: &str) -> FirewallPolicyAddRuleCall<'a, S> {
        FirewallPolicyAddRuleCall {
            hub: self.hub,
            _request: request,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies rules to the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn clone_rules(&self, firewall_policy: &str) -> FirewallPolicyCloneRuleCall<'a, S> {
        FirewallPolicyCloneRuleCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _source_firewall_policy: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified policy.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to delete.
    pub fn delete(&self, firewall_policy: &str) -> FirewallPolicyDeleteCall<'a, S> {
        FirewallPolicyDeleteCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to get.
    pub fn get(&self, firewall_policy: &str) -> FirewallPolicyGetCall<'a, S> {
        FirewallPolicyGetCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an association with the specified name.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to which the queried rule belongs.
    pub fn get_association(&self, firewall_policy: &str) -> FirewallPolicyGetAssociationCall<'a, S> {
        FirewallPolicyGetAssociationCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, resource: &str) -> FirewallPolicyGetIamPolicyCall<'a, S> {
        FirewallPolicyGetIamPolicyCall {
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
    /// Gets a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to which the queried rule belongs.
    pub fn get_rule(&self, firewall_policy: &str) -> FirewallPolicyGetRuleCall<'a, S> {
        FirewallPolicyGetRuleCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: FirewallPolicy) -> FirewallPolicyInsertCall<'a, S> {
        FirewallPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _request_id: Default::default(),
            _parent_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the policies that have been configured for the specified folder or organization.
    pub fn list(&self) -> FirewallPolicyListCall<'a, S> {
        FirewallPolicyListCall {
            hub: self.hub,
            _return_partial_success: Default::default(),
            _parent_id: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists associations of a specified target, i.e., organization or folder.
    pub fn list_associations(&self) -> FirewallPolicyListAssociationCall<'a, S> {
        FirewallPolicyListAssociationCall {
            hub: self.hub,
            _target_resource: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn move_(&self, firewall_policy: &str) -> FirewallPolicyMoveCall<'a, S> {
        FirewallPolicyMoveCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _parent_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified policy with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch(&self, request: FirewallPolicy, firewall_policy: &str) -> FirewallPolicyPatchCall<'a, S> {
        FirewallPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch_rule(&self, request: FirewallPolicyRule, firewall_policy: &str) -> FirewallPolicyPatchRuleCall<'a, S> {
        FirewallPolicyPatchRuleCall {
            hub: self.hub,
            _request: request,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an association for the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_association(&self, firewall_policy: &str) -> FirewallPolicyRemoveAssociationCall<'a, S> {
        FirewallPolicyRemoveAssociationCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_rule(&self, firewall_policy: &str) -> FirewallPolicyRemoveRuleCall<'a, S> {
        FirewallPolicyRemoveRuleCall {
            hub: self.hub,
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalOrganizationSetPolicyRequest, resource: &str) -> FirewallPolicySetIamPolicyCall<'a, S> {
        FirewallPolicySetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, resource: &str) -> FirewallPolicyTestIamPermissionCall<'a, S> {
        FirewallPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *firewall* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.firewalls();
/// # }
/// ```
pub struct FirewallMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for FirewallMethods<'a, S> {}

impl<'a, S> FirewallMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified firewall.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewall` - Name of the firewall rule to delete.
    pub fn delete(&self, project: &str, firewall: &str) -> FirewallDeleteCall<'a, S> {
        FirewallDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall: firewall.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified firewall.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewall` - Name of the firewall rule to return.
    pub fn get(&self, project: &str, firewall: &str) -> FirewallGetCall<'a, S> {
        FirewallGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall: firewall.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a firewall rule in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Firewall, project: &str) -> FirewallInsertCall<'a, S> {
        FirewallInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of firewall rules available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> FirewallListCall<'a, S> {
        FirewallListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified firewall rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewall` - Name of the firewall rule to patch.
    pub fn patch(&self, request: Firewall, project: &str, firewall: &str) -> FirewallPatchCall<'a, S> {
        FirewallPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall: firewall.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified firewall rule with the data included in the request. Note that all fields will be updated if using PUT, even fields that are not specified. To update individual fields, please use PATCH instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewall` - Name of the firewall rule to update.
    pub fn update(&self, request: Firewall, project: &str, firewall: &str) -> FirewallUpdateCall<'a, S> {
        FirewallUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall: firewall.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *forwardingRule* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_labels(...)` and `set_target(...)`
/// // to build up your call.
/// let rb = hub.forwarding_rules();
/// # }
/// ```
pub struct ForwardingRuleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ForwardingRuleMethods<'a, S> {}

impl<'a, S> ForwardingRuleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of forwarding rules.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> ForwardingRuleAggregatedListCall<'a, S> {
        ForwardingRuleAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified ForwardingRule resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to delete.
    pub fn delete(&self, project: &str, region: &str, forwarding_rule: &str) -> ForwardingRuleDeleteCall<'a, S> {
        ForwardingRuleDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified ForwardingRule resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to return.
    pub fn get(&self, project: &str, region: &str, forwarding_rule: &str) -> ForwardingRuleGetCall<'a, S> {
        ForwardingRuleGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a ForwardingRule resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: ForwardingRule, project: &str, region: &str) -> ForwardingRuleInsertCall<'a, S> {
        ForwardingRuleInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of ForwardingRule resources available to the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> ForwardingRuleListCall<'a, S> {
        ForwardingRuleListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified forwarding rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. Currently, you can only patch the network_tier field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to patch.
    pub fn patch(&self, request: ForwardingRule, project: &str, region: &str, forwarding_rule: &str) -> ForwardingRulePatchCall<'a, S> {
        ForwardingRulePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on the specified resource. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> ForwardingRuleSetLabelCall<'a, S> {
        ForwardingRuleSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes target URL for forwarding rule. The new target should be of the same type as the old target.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource in which target is to be set.
    pub fn set_target(&self, request: TargetReference, project: &str, region: &str, forwarding_rule: &str) -> ForwardingRuleSetTargetCall<'a, S> {
        ForwardingRuleSetTargetCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalAddress* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.global_addresses();
/// # }
/// ```
pub struct GlobalAddressMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalAddressMethods<'a, S> {}

impl<'a, S> GlobalAddressMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified address resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `address` - Name of the address resource to delete.
    pub fn delete(&self, project: &str, address: &str) -> GlobalAddressDeleteCall<'a, S> {
        GlobalAddressDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _address: address.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified address resource. Gets a list of available addresses by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `address` - Name of the address resource to return.
    pub fn get(&self, project: &str, address: &str) -> GlobalAddressGetCall<'a, S> {
        GlobalAddressGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _address: address.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an address resource in the specified project by using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Address, project: &str) -> GlobalAddressInsertCall<'a, S> {
        GlobalAddressInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of global addresses.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalAddressListCall<'a, S> {
        GlobalAddressListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a GlobalAddress. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> GlobalAddressSetLabelCall<'a, S> {
        GlobalAddressSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalForwardingRule* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_labels(...)` and `set_target(...)`
/// // to build up your call.
/// let rb = hub.global_forwarding_rules();
/// # }
/// ```
pub struct GlobalForwardingRuleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalForwardingRuleMethods<'a, S> {}

impl<'a, S> GlobalForwardingRuleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified GlobalForwardingRule resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to delete.
    pub fn delete(&self, project: &str, forwarding_rule: &str) -> GlobalForwardingRuleDeleteCall<'a, S> {
        GlobalForwardingRuleDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified GlobalForwardingRule resource. Gets a list of available forwarding rules by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to return.
    pub fn get(&self, project: &str, forwarding_rule: &str) -> GlobalForwardingRuleGetCall<'a, S> {
        GlobalForwardingRuleGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a GlobalForwardingRule resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: ForwardingRule, project: &str) -> GlobalForwardingRuleInsertCall<'a, S> {
        GlobalForwardingRuleInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of GlobalForwardingRule resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalForwardingRuleListCall<'a, S> {
        GlobalForwardingRuleListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified forwarding rule with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. Currently, you can only patch the network_tier field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource to patch.
    pub fn patch(&self, request: ForwardingRule, project: &str, forwarding_rule: &str) -> GlobalForwardingRulePatchCall<'a, S> {
        GlobalForwardingRulePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on the specified resource. To learn more about labels, read the Labeling resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> GlobalForwardingRuleSetLabelCall<'a, S> {
        GlobalForwardingRuleSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes target URL for the GlobalForwardingRule resource. The new target should be of the same type as the old target.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `forwardingRule` - Name of the ForwardingRule resource in which target is to be set.
    pub fn set_target(&self, request: TargetReference, project: &str, forwarding_rule: &str) -> GlobalForwardingRuleSetTargetCall<'a, S> {
        GlobalForwardingRuleSetTargetCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _forwarding_rule: forwarding_rule.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalNetworkEndpointGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `attach_network_endpoints(...)`, `delete(...)`, `detach_network_endpoints(...)`, `get(...)`, `insert(...)`, `list(...)` and `list_network_endpoints(...)`
/// // to build up your call.
/// let rb = hub.global_network_endpoint_groups();
/// # }
/// ```
pub struct GlobalNetworkEndpointGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalNetworkEndpointGroupMethods<'a, S> {}

impl<'a, S> GlobalNetworkEndpointGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attach a network endpoint to the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `networkEndpointGroup` - The name of the network endpoint group where you are attaching network endpoints to. It should comply with RFC1035.
    pub fn attach_network_endpoints(&self, request: GlobalNetworkEndpointGroupsAttachEndpointsRequest, project: &str, network_endpoint_group: &str) -> GlobalNetworkEndpointGroupAttachNetworkEndpointCall<'a, S> {
        GlobalNetworkEndpointGroupAttachNetworkEndpointCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified network endpoint group.Note that the NEG cannot be deleted if there are backend services referencing it.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `networkEndpointGroup` - The name of the network endpoint group to delete. It should comply with RFC1035.
    pub fn delete(&self, project: &str, network_endpoint_group: &str) -> GlobalNetworkEndpointGroupDeleteCall<'a, S> {
        GlobalNetworkEndpointGroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detach the network endpoint from the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `networkEndpointGroup` - The name of the network endpoint group where you are removing network endpoints. It should comply with RFC1035.
    pub fn detach_network_endpoints(&self, request: GlobalNetworkEndpointGroupsDetachEndpointsRequest, project: &str, network_endpoint_group: &str) -> GlobalNetworkEndpointGroupDetachNetworkEndpointCall<'a, S> {
        GlobalNetworkEndpointGroupDetachNetworkEndpointCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network endpoint group. Gets a list of available network endpoint groups by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `networkEndpointGroup` - The name of the network endpoint group. It should comply with RFC1035.
    pub fn get(&self, project: &str, network_endpoint_group: &str) -> GlobalNetworkEndpointGroupGetCall<'a, S> {
        GlobalNetworkEndpointGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a network endpoint group in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: NetworkEndpointGroup, project: &str) -> GlobalNetworkEndpointGroupInsertCall<'a, S> {
        GlobalNetworkEndpointGroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of network endpoint groups that are located in the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalNetworkEndpointGroupListCall<'a, S> {
        GlobalNetworkEndpointGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the network endpoints in the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `networkEndpointGroup` - The name of the network endpoint group from which you want to generate a list of included network endpoints. It should comply with RFC1035.
    pub fn list_network_endpoints(&self, project: &str, network_endpoint_group: &str) -> GlobalNetworkEndpointGroupListNetworkEndpointCall<'a, S> {
        GlobalNetworkEndpointGroupListNetworkEndpointCall {
            hub: self.hub,
            _project: project.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalOperation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `list(...)` and `wait(...)`
/// // to build up your call.
/// let rb = hub.global_operations();
/// # }
/// ```
pub struct GlobalOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalOperationMethods<'a, S> {}

impl<'a, S> GlobalOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of all operations.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> GlobalOperationAggregatedListCall<'a, S> {
        GlobalOperationAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `operation` - Name of the Operations resource to delete.
    pub fn delete(&self, project: &str, operation: &str) -> GlobalOperationDeleteCall<'a, S> {
        GlobalOperationDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn get(&self, project: &str, operation: &str) -> GlobalOperationGetCall<'a, S> {
        GlobalOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of Operation resources contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalOperationListCall<'a, S> {
        GlobalOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress. This method is called on a best-effort basis. Specifically: - In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. - If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn wait(&self, project: &str, operation: &str) -> GlobalOperationWaitCall<'a, S> {
        GlobalOperationWaitCall {
            hub: self.hub,
            _project: project.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalOrganizationOperation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.global_organization_operations();
/// # }
/// ```
pub struct GlobalOrganizationOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalOrganizationOperationMethods<'a, S> {}

impl<'a, S> GlobalOrganizationOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `operation` - Name of the Operations resource to delete.
    pub fn delete(&self, operation: &str) -> GlobalOrganizationOperationDeleteCall<'a, S> {
        GlobalOrganizationOperationDeleteCall {
            hub: self.hub,
            _operation: operation.to_string(),
            _parent_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified Operations resource. Gets a list of operations by making a `list()` request.
    /// 
    /// # Arguments
    ///
    /// * `operation` - Name of the Operations resource to return.
    pub fn get(&self, operation: &str) -> GlobalOrganizationOperationGetCall<'a, S> {
        GlobalOrganizationOperationGetCall {
            hub: self.hub,
            _operation: operation.to_string(),
            _parent_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of Operation resources contained within the specified organization.
    pub fn list(&self) -> GlobalOrganizationOperationListCall<'a, S> {
        GlobalOrganizationOperationListCall {
            hub: self.hub,
            _return_partial_success: Default::default(),
            _parent_id: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *globalPublicDelegatedPrefix* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.global_public_delegated_prefixes();
/// # }
/// ```
pub struct GlobalPublicDelegatedPrefixMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for GlobalPublicDelegatedPrefixMethods<'a, S> {}

impl<'a, S> GlobalPublicDelegatedPrefixMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified global PublicDelegatedPrefix.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to delete.
    pub fn delete(&self, project: &str, public_delegated_prefix: &str) -> GlobalPublicDelegatedPrefixDeleteCall<'a, S> {
        GlobalPublicDelegatedPrefixDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified global PublicDelegatedPrefix resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to return.
    pub fn get(&self, project: &str, public_delegated_prefix: &str) -> GlobalPublicDelegatedPrefixGetCall<'a, S> {
        GlobalPublicDelegatedPrefixGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a global PublicDelegatedPrefix in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: PublicDelegatedPrefix, project: &str) -> GlobalPublicDelegatedPrefixInsertCall<'a, S> {
        GlobalPublicDelegatedPrefixInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the global PublicDelegatedPrefixes for a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> GlobalPublicDelegatedPrefixListCall<'a, S> {
        GlobalPublicDelegatedPrefixListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified global PublicDelegatedPrefix resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to patch.
    pub fn patch(&self, request: PublicDelegatedPrefix, project: &str, public_delegated_prefix: &str) -> GlobalPublicDelegatedPrefixPatchCall<'a, S> {
        GlobalPublicDelegatedPrefixPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *healthCheck* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.health_checks();
/// # }
/// ```
pub struct HealthCheckMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for HealthCheckMethods<'a, S> {}

impl<'a, S> HealthCheckMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all HealthCheck resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> HealthCheckAggregatedListCall<'a, S> {
        HealthCheckAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified HealthCheck resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `healthCheck` - Name of the HealthCheck resource to delete.
    pub fn delete(&self, project: &str, health_check: &str) -> HealthCheckDeleteCall<'a, S> {
        HealthCheckDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified HealthCheck resource. Gets a list of available health checks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `healthCheck` - Name of the HealthCheck resource to return.
    pub fn get(&self, project: &str, health_check: &str) -> HealthCheckGetCall<'a, S> {
        HealthCheckGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _health_check: health_check.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a HealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: HealthCheck, project: &str) -> HealthCheckInsertCall<'a, S> {
        HealthCheckInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of HealthCheck resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> HealthCheckListCall<'a, S> {
        HealthCheckListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `healthCheck` - Name of the HealthCheck resource to patch.
    pub fn patch(&self, request: HealthCheck, project: &str, health_check: &str) -> HealthCheckPatchCall<'a, S> {
        HealthCheckPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `healthCheck` - Name of the HealthCheck resource to update.
    pub fn update(&self, request: HealthCheck, project: &str, health_check: &str) -> HealthCheckUpdateCall<'a, S> {
        HealthCheckUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *httpHealthCheck* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.http_health_checks();
/// # }
/// ```
pub struct HttpHealthCheckMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for HttpHealthCheckMethods<'a, S> {}

impl<'a, S> HttpHealthCheckMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified HttpHealthCheck resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `httpHealthCheck` - Name of the HttpHealthCheck resource to delete.
    pub fn delete(&self, project: &str, http_health_check: &str) -> HttpHealthCheckDeleteCall<'a, S> {
        HttpHealthCheckDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _http_health_check: http_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified HttpHealthCheck resource. Gets a list of available HTTP health checks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `httpHealthCheck` - Name of the HttpHealthCheck resource to return.
    pub fn get(&self, project: &str, http_health_check: &str) -> HttpHealthCheckGetCall<'a, S> {
        HttpHealthCheckGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _http_health_check: http_health_check.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a HttpHealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: HttpHealthCheck, project: &str) -> HttpHealthCheckInsertCall<'a, S> {
        HttpHealthCheckInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of HttpHealthCheck resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> HttpHealthCheckListCall<'a, S> {
        HttpHealthCheckListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HttpHealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `httpHealthCheck` - Name of the HttpHealthCheck resource to patch.
    pub fn patch(&self, request: HttpHealthCheck, project: &str, http_health_check: &str) -> HttpHealthCheckPatchCall<'a, S> {
        HttpHealthCheckPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _http_health_check: http_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HttpHealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `httpHealthCheck` - Name of the HttpHealthCheck resource to update.
    pub fn update(&self, request: HttpHealthCheck, project: &str, http_health_check: &str) -> HttpHealthCheckUpdateCall<'a, S> {
        HttpHealthCheckUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _http_health_check: http_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *httpsHealthCheck* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.https_health_checks();
/// # }
/// ```
pub struct HttpsHealthCheckMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for HttpsHealthCheckMethods<'a, S> {}

impl<'a, S> HttpsHealthCheckMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified HttpsHealthCheck resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `httpsHealthCheck` - Name of the HttpsHealthCheck resource to delete.
    pub fn delete(&self, project: &str, https_health_check: &str) -> HttpsHealthCheckDeleteCall<'a, S> {
        HttpsHealthCheckDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _https_health_check: https_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified HttpsHealthCheck resource. Gets a list of available HTTPS health checks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `httpsHealthCheck` - Name of the HttpsHealthCheck resource to return.
    pub fn get(&self, project: &str, https_health_check: &str) -> HttpsHealthCheckGetCall<'a, S> {
        HttpsHealthCheckGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _https_health_check: https_health_check.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a HttpsHealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: HttpsHealthCheck, project: &str) -> HttpsHealthCheckInsertCall<'a, S> {
        HttpsHealthCheckInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of HttpsHealthCheck resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> HttpsHealthCheckListCall<'a, S> {
        HttpsHealthCheckListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HttpsHealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `httpsHealthCheck` - Name of the HttpsHealthCheck resource to patch.
    pub fn patch(&self, request: HttpsHealthCheck, project: &str, https_health_check: &str) -> HttpsHealthCheckPatchCall<'a, S> {
        HttpsHealthCheckPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _https_health_check: https_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HttpsHealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `httpsHealthCheck` - Name of the HttpsHealthCheck resource to update.
    pub fn update(&self, request: HttpsHealthCheck, project: &str, https_health_check: &str) -> HttpsHealthCheckUpdateCall<'a, S> {
        HttpsHealthCheckUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _https_health_check: https_health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *imageFamilyView* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.image_family_views();
/// # }
/// ```
pub struct ImageFamilyViewMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ImageFamilyViewMethods<'a, S> {}

impl<'a, S> ImageFamilyViewMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the latest image that is part of an image family, is not deprecated and is rolled out in the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `family` - Name of the image family to search for.
    pub fn get(&self, project: &str, zone: &str, family: &str) -> ImageFamilyViewGetCall<'a, S> {
        ImageFamilyViewGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _family: family.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *image* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `deprecate(...)`, `get(...)`, `get_from_family(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.images();
/// # }
/// ```
pub struct ImageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ImageMethods<'a, S> {}

impl<'a, S> ImageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified image.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `image` - Name of the image resource to delete.
    pub fn delete(&self, project: &str, image: &str) -> ImageDeleteCall<'a, S> {
        ImageDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _image: image.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the deprecation status of an image. If an empty request body is given, clears the deprecation status instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `image` - Image name.
    pub fn deprecate(&self, request: DeprecationStatus, project: &str, image: &str) -> ImageDeprecateCall<'a, S> {
        ImageDeprecateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _image: image.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified image. Gets a list of available images by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `image` - Name of the image resource to return.
    pub fn get(&self, project: &str, image: &str) -> ImageGetCall<'a, S> {
        ImageGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _image: image.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the latest image that is part of an image family and is not deprecated.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `family` - Name of the image family to search for.
    pub fn get_from_family(&self, project: &str, family: &str) -> ImageGetFromFamilyCall<'a, S> {
        ImageGetFromFamilyCall {
            hub: self.hub,
            _project: project.to_string(),
            _family: family.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> ImageGetIamPolicyCall<'a, S> {
        ImageGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an image in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Image, project: &str) -> ImageInsertCall<'a, S> {
        ImageInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _force_create: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of custom images available to the specified project. Custom images are images you create that belong to your project. This method does not get any images that belong to other projects, including publicly-available images, like Debian 8. If you want to get a list of publicly-available images, use this method to make a request to the respective image project, such as debian-cloud or windows-cloud.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> ImageListCall<'a, S> {
        ImageListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified image with the data included in the request. Only the following fields can be modified: family, description, deprecation status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `image` - Name of the image resource to patch.
    pub fn patch(&self, request: Image, project: &str, image: &str) -> ImagePatchCall<'a, S> {
        ImagePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _image: image.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> ImageSetIamPolicyCall<'a, S> {
        ImageSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on an image. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> ImageSetLabelCall<'a, S> {
        ImageSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> ImageTestIamPermissionCall<'a, S> {
        ImageTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *instanceGroupManager* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `abandon_instances(...)`, `aggregated_list(...)`, `apply_updates_to_instances(...)`, `create_instances(...)`, `delete(...)`, `delete_instances(...)`, `delete_per_instance_configs(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_errors(...)`, `list_managed_instances(...)`, `list_per_instance_configs(...)`, `patch(...)`, `patch_per_instance_configs(...)`, `recreate_instances(...)`, `resize(...)`, `set_instance_template(...)`, `set_target_pools(...)` and `update_per_instance_configs(...)`
/// // to build up your call.
/// let rb = hub.instance_group_managers();
/// # }
/// ```
pub struct InstanceGroupManagerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InstanceGroupManagerMethods<'a, S> {}

impl<'a, S> InstanceGroupManagerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified instances to be removed from the managed instance group. Abandoning an instance does not delete the instance, but it does remove the instance from any target pools that are applied by the managed instance group. This method reduces the targetSize of the managed instance group by the number of instances that you abandon. This operation is marked as DONE when the action is scheduled even if the instances have not yet been removed from the group. You must separately verify the status of the abandoning action with the listmanagedinstances method. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn abandon_instances(&self, request: InstanceGroupManagersAbandonInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerAbandonInstanceCall<'a, S> {
        InstanceGroupManagerAbandonInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of managed instance groups and groups them by zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> InstanceGroupManagerAggregatedListCall<'a, S> {
        InstanceGroupManagerAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies changes to selected instances on the managed instance group. This method can be used to apply new overrides and/or new versions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. Should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group, should conform to RFC1035.
    pub fn apply_updates_to_instances(&self, request: InstanceGroupManagersApplyUpdatesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerApplyUpdatesToInstanceCall<'a, S> {
        InstanceGroupManagerApplyUpdatesToInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates instances with per-instance configurations in this managed instance group. Instances are created using the current instance template. The create instances operation is marked DONE if the createInstances request is successful. The underlying actions take additional time. You must separately verify the status of the creating or actions with the listmanagedinstances method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn create_instances(&self, request: InstanceGroupManagersCreateInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerCreateInstanceCall<'a, S> {
        InstanceGroupManagerCreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified managed instance group and all of the instances in that group. Note that the instance group must not belong to a backend service. Read Deleting an instance group for more information.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group to delete.
    pub fn delete(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteCall<'a, S> {
        InstanceGroupManagerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified instances in the managed instance group for immediate deletion. The instances are also removed from any target pools of which they were a member. This method reduces the targetSize of the managed instance group by the number of instances that you delete. This operation is marked as DONE when the action is scheduled even if the instances are still being deleted. You must separately verify the status of the deleting action with the listmanagedinstances method. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn delete_instances(&self, request: InstanceGroupManagersDeleteInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteInstanceCall<'a, S> {
        InstanceGroupManagerDeleteInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes selected per-instance configurations for the managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn delete_per_instance_configs(&self, request: InstanceGroupManagersDeletePerInstanceConfigsReq, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeletePerInstanceConfigCall<'a, S> {
        InstanceGroupManagerDeletePerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all of the details about the specified managed instance group. Gets a list of available managed instance groups by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn get(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerGetCall<'a, S> {
        InstanceGroupManagerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a managed instance group using the information that you specify in the request. After the group is created, instances in the group are created using the specified instance template. This operation is marked as DONE when the group is created even if the instances in the group have not yet been created. You must separately verify the status of the individual instances with the listmanagedinstances method. A managed instance group can have up to 1000 VM instances per group. Please contact Cloud Support if you need an increase in this limit.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where you want to create the managed instance group.
    pub fn insert(&self, request: InstanceGroupManager, project: &str, zone: &str) -> InstanceGroupManagerInsertCall<'a, S> {
        InstanceGroupManagerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of managed instance groups that are contained within the specified project and zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    pub fn list(&self, project: &str, zone: &str) -> InstanceGroupManagerListCall<'a, S> {
        InstanceGroupManagerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all errors thrown by actions on instances for a given managed instance group. The filter and orderBy query parameters are not supported.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It must be a string that meets the requirements in RFC1035, or an unsigned long integer: must match regexp pattern: (?:[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?)|1-9{0,19}.
    pub fn list_errors(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerListErrorCall<'a, S> {
        InstanceGroupManagerListErrorCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the instances in the managed instance group. Each instance in the list has a currentAction, which indicates the action that the managed instance group is performing on the instance. For example, if the group is still creating an instance, the currentAction is CREATING. If a previous action failed, the list displays the errors for that failed action. The orderBy query parameter is not supported. The `pageToken` query parameter is supported only in the alpha and beta API and only if the group's `listManagedInstancesResults` field is set to `PAGINATED`.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn list_managed_instances(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerListManagedInstanceCall<'a, S> {
        InstanceGroupManagerListManagedInstanceCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the per-instance configurations defined for the managed instance group. The orderBy query parameter is not supported.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn list_per_instance_configs(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerListPerInstanceConfigCall<'a, S> {
        InstanceGroupManagerListPerInstanceConfigCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a managed instance group using the information that you specify in the request. This operation is marked as DONE when the group is patched even if the instances in the group are still in the process of being patched. You must separately verify the status of the individual instances with the listManagedInstances method. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. If you update your group to specify a new template or instance configuration, it's possible that your intended specification for each VM in the group is different from the current state of that VM. To learn how to apply an updated configuration to the VMs in a MIG, see Updating instances in a MIG.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where you want to create the managed instance group.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn patch(&self, request: InstanceGroupManager, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerPatchCall<'a, S> {
        InstanceGroupManagerPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or patches per-instance configurations for the managed instance group. perInstanceConfig.name serves as a key used to distinguish whether to perform insert or patch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn patch_per_instance_configs(&self, request: InstanceGroupManagersPatchPerInstanceConfigsReq, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerPatchPerInstanceConfigCall<'a, S> {
        InstanceGroupManagerPatchPerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified VM instances in the managed instance group to be immediately recreated. Each instance is recreated using the group's current configuration. This operation is marked as DONE when the flag is set even if the instances have not yet been recreated. You must separately verify the status of each instance by checking its currentAction field; for more information, see Checking the status of managed instances. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn recreate_instances(&self, request: InstanceGroupManagersRecreateInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerRecreateInstanceCall<'a, S> {
        InstanceGroupManagerRecreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the managed instance group. If you increase the size, the group creates new instances using the current instance template. If you decrease the size, the group deletes instances. The resize operation is marked DONE when the resize actions are scheduled even if the group has not yet added or deleted any instances. You must separately verify the status of the creating or deleting actions with the listmanagedinstances method. When resizing down, the instance group arbitrarily chooses the order in which VMs are deleted. The group takes into account some VM attributes when making the selection including: + The status of the VM instance. + The health of the VM instance. + The instance template version the VM is based on. + For regional managed instance groups, the location of the VM instance. This list is subject to change. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    /// * `size` - The number of running instances that the managed instance group should maintain at any given time. The group automatically adds or removes instances to maintain the number of instances specified by this parameter.
    pub fn resize(&self, project: &str, zone: &str, instance_group_manager: &str, size: i32) -> InstanceGroupManagerResizeCall<'a, S> {
        InstanceGroupManagerResizeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _size: size,
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Specifies the instance template to use when creating new instances in this group. The templates for existing instances in the group do not change unless you run recreateInstances, run applyUpdatesToInstances, or set the group's updatePolicy.type to PROACTIVE.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn set_instance_template(&self, request: InstanceGroupManagersSetInstanceTemplateRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a, S> {
        InstanceGroupManagerSetInstanceTemplateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the target pools to which all instances in this managed instance group are assigned. The target pools automatically apply to all of the instances in the managed instance group. This operation is marked DONE when you make the request even if the instances have not yet been added to their target pools. The change might take some time to apply to all of the instances in the group depending on the size of the group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn set_target_pools(&self, request: InstanceGroupManagersSetTargetPoolsRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetTargetPoolCall<'a, S> {
        InstanceGroupManagerSetTargetPoolCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or updates per-instance configurations for the managed instance group. perInstanceConfig.name serves as a key used to distinguish whether to perform insert or patch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn update_per_instance_configs(&self, request: InstanceGroupManagersUpdatePerInstanceConfigsReq, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerUpdatePerInstanceConfigCall<'a, S> {
        InstanceGroupManagerUpdatePerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *instanceGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_instances(...)`, `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_instances(...)`, `remove_instances(...)` and `set_named_ports(...)`
/// // to build up your call.
/// let rb = hub.instance_groups();
/// # }
/// ```
pub struct InstanceGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InstanceGroupMethods<'a, S> {}

impl<'a, S> InstanceGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a list of instances to the specified instance group. All of the instances in the instance group must be in the same network/subnetwork. Read Adding instances for more information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group where you are adding instances.
    pub fn add_instances(&self, request: InstanceGroupsAddInstancesRequest, project: &str, zone: &str, instance_group: &str) -> InstanceGroupAddInstanceCall<'a, S> {
        InstanceGroupAddInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of instance groups and sorts them by zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> InstanceGroupAggregatedListCall<'a, S> {
        InstanceGroupAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified instance group. The instances in the group are not deleted. Note that instance group must not belong to a backend service. Read Deleting an instance group for more information.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group to delete.
    pub fn delete(&self, project: &str, zone: &str, instance_group: &str) -> InstanceGroupDeleteCall<'a, S> {
        InstanceGroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified zonal instance group. Get a list of available zonal instance groups by making a list() request. For managed instance groups, use the instanceGroupManagers or regionInstanceGroupManagers methods instead.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group.
    pub fn get(&self, project: &str, zone: &str, instance_group: &str) -> InstanceGroupGetCall<'a, S> {
        InstanceGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an instance group in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where you want to create the instance group.
    pub fn insert(&self, request: InstanceGroup, project: &str, zone: &str) -> InstanceGroupInsertCall<'a, S> {
        InstanceGroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of zonal instance group resources contained within the specified zone. For managed instance groups, use the instanceGroupManagers or regionInstanceGroupManagers methods instead.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    pub fn list(&self, project: &str, zone: &str) -> InstanceGroupListCall<'a, S> {
        InstanceGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the instances in the specified instance group. The orderBy query parameter is not supported. The filter query parameter is supported, but only for expressions that use `eq` (equal) or `ne` (not equal) operators.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group from which you want to generate a list of included instances.
    pub fn list_instances(&self, request: InstanceGroupsListInstancesRequest, project: &str, zone: &str, instance_group: &str) -> InstanceGroupListInstanceCall<'a, S> {
        InstanceGroupListInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes one or more instances from the specified instance group, but does not delete those instances. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration before the VM instance is removed or deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group where the specified instances will be removed.
    pub fn remove_instances(&self, request: InstanceGroupsRemoveInstancesRequest, project: &str, zone: &str, instance_group: &str) -> InstanceGroupRemoveInstanceCall<'a, S> {
        InstanceGroupRemoveInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the named ports for the specified instance group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the instance group is located.
    /// * `instanceGroup` - The name of the instance group where the named ports are updated.
    pub fn set_named_ports(&self, request: InstanceGroupsSetNamedPortsRequest, project: &str, zone: &str, instance_group: &str) -> InstanceGroupSetNamedPortCall<'a, S> {
        InstanceGroupSetNamedPortCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group: instance_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *instanceTemplate* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.instance_templates();
/// # }
/// ```
pub struct InstanceTemplateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InstanceTemplateMethods<'a, S> {}

impl<'a, S> InstanceTemplateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified instance template. Deleting an instance template is permanent and cannot be undone. It is not possible to delete templates that are already in use by a managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `instanceTemplate` - The name of the instance template to delete.
    pub fn delete(&self, project: &str, instance_template: &str) -> InstanceTemplateDeleteCall<'a, S> {
        InstanceTemplateDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _instance_template: instance_template.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified instance template. Gets a list of available instance templates by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `instanceTemplate` - The name of the instance template.
    pub fn get(&self, project: &str, instance_template: &str) -> InstanceTemplateGetCall<'a, S> {
        InstanceTemplateGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _instance_template: instance_template.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> InstanceTemplateGetIamPolicyCall<'a, S> {
        InstanceTemplateGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an instance template in the specified project using the data that is included in the request. If you are creating a new template to update an existing instance group, your new instance template must use the same network or, if applicable, the same subnetwork as the original template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: InstanceTemplate, project: &str) -> InstanceTemplateInsertCall<'a, S> {
        InstanceTemplateInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of instance templates that are contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> InstanceTemplateListCall<'a, S> {
        InstanceTemplateListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> InstanceTemplateSetIamPolicyCall<'a, S> {
        InstanceTemplateSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> InstanceTemplateTestIamPermissionCall<'a, S> {
        InstanceTemplateTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *instance* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_access_config(...)`, `add_resource_policies(...)`, `aggregated_list(...)`, `attach_disk(...)`, `bulk_insert(...)`, `delete(...)`, `delete_access_config(...)`, `detach_disk(...)`, `get(...)`, `get_effective_firewalls(...)`, `get_guest_attributes(...)`, `get_iam_policy(...)`, `get_screenshot(...)`, `get_serial_port_output(...)`, `get_shielded_instance_identity(...)`, `insert(...)`, `list(...)`, `list_referrers(...)`, `remove_resource_policies(...)`, `reset(...)`, `resume(...)`, `send_diagnostic_interrupt(...)`, `set_deletion_protection(...)`, `set_disk_auto_delete(...)`, `set_iam_policy(...)`, `set_labels(...)`, `set_machine_resources(...)`, `set_machine_type(...)`, `set_metadata(...)`, `set_min_cpu_platform(...)`, `set_scheduling(...)`, `set_service_account(...)`, `set_shielded_instance_integrity_policy(...)`, `set_tags(...)`, `simulate_maintenance_event(...)`, `start(...)`, `start_with_encryption_key(...)`, `stop(...)`, `suspend(...)`, `test_iam_permissions(...)`, `update(...)`, `update_access_config(...)`, `update_display_device(...)`, `update_network_interface(...)` and `update_shielded_instance_config(...)`
/// // to build up your call.
/// let rb = hub.instances();
/// # }
/// ```
pub struct InstanceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InstanceMethods<'a, S> {}

impl<'a, S> InstanceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an access config to an instance's network interface.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    /// * `networkInterface` - The name of the network interface to add to this instance.
    pub fn add_access_config(&self, request: AccessConfig, project: &str, zone: &str, instance: &str, network_interface: &str) -> InstanceAddAccessConfigCall<'a, S> {
        InstanceAddAccessConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _network_interface: network_interface.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds existing resource policies to an instance. You can only add one policy right now which will be applied to this instance for scheduling live migrations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    pub fn add_resource_policies(&self, request: InstancesAddResourcePoliciesRequest, project: &str, zone: &str, instance: &str) -> InstanceAddResourcePolicyCall<'a, S> {
        InstanceAddResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of all of the instances in your project across all regions and zones. The performance of this method degrades when a filter is specified on a project that has a very large number of instances.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> InstanceAggregatedListCall<'a, S> {
        InstanceAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attaches an existing Disk resource to an instance. You must first create the disk before you can attach it. It is not possible to create and attach a disk at the same time. For more information, read Adding a persistent disk to your instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    pub fn attach_disk(&self, request: AttachedDisk, project: &str, zone: &str, instance: &str) -> InstanceAttachDiskCall<'a, S> {
        InstanceAttachDiskCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _force_attach: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates multiple instances. Count specifies the number of instances to create. For more information, see About bulk creation of VMs.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn bulk_insert(&self, request: BulkInsertInstanceResource, project: &str, zone: &str) -> InstanceBulkInsertCall<'a, S> {
        InstanceBulkInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Instance resource. For more information, see Deleting an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to delete.
    pub fn delete(&self, project: &str, zone: &str, instance: &str) -> InstanceDeleteCall<'a, S> {
        InstanceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an access config from an instance's network interface.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    /// * `accessConfig` - The name of the access config to delete.
    /// * `networkInterface` - The name of the network interface.
    pub fn delete_access_config(&self, project: &str, zone: &str, instance: &str, access_config: &str, network_interface: &str) -> InstanceDeleteAccessConfigCall<'a, S> {
        InstanceDeleteAccessConfigCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _access_config: access_config.to_string(),
            _network_interface: network_interface.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detaches a disk from an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Instance name for this request.
    /// * `deviceName` - The device name of the disk to detach. Make a get() request on the instance to view currently attached disks and device names.
    pub fn detach_disk(&self, project: &str, zone: &str, instance: &str, device_name: &str) -> InstanceDetachDiskCall<'a, S> {
        InstanceDetachDiskCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _device_name: device_name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Instance resource. Gets a list of available instances by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to return.
    pub fn get(&self, project: &str, zone: &str, instance: &str) -> InstanceGetCall<'a, S> {
        InstanceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns effective firewalls applied to an interface of the instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    /// * `networkInterface` - The name of the network interface to get the effective firewalls.
    pub fn get_effective_firewalls(&self, project: &str, zone: &str, instance: &str, network_interface: &str) -> InstanceGetEffectiveFirewallCall<'a, S> {
        InstanceGetEffectiveFirewallCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _network_interface: network_interface.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified guest attributes entry.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn get_guest_attributes(&self, project: &str, zone: &str, instance: &str) -> InstanceGetGuestAttributeCall<'a, S> {
        InstanceGetGuestAttributeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _variable_key: Default::default(),
            _query_path: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, zone: &str, resource: &str) -> InstanceGetIamPolicyCall<'a, S> {
        InstanceGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the screenshot from the specified instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn get_screenshot(&self, project: &str, zone: &str, instance: &str) -> InstanceGetScreenshotCall<'a, S> {
        InstanceGetScreenshotCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the last 1 MB of serial port output from the specified instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance for this request.
    pub fn get_serial_port_output(&self, project: &str, zone: &str, instance: &str) -> InstanceGetSerialPortOutputCall<'a, S> {
        InstanceGetSerialPortOutputCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _start: Default::default(),
            _port: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the Shielded Instance Identity of an instance
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name or id of the instance scoping this request.
    pub fn get_shielded_instance_identity(&self, project: &str, zone: &str, instance: &str) -> InstanceGetShieldedInstanceIdentityCall<'a, S> {
        InstanceGetShieldedInstanceIdentityCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an instance resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn insert(&self, request: Instance, project: &str, zone: &str) -> InstanceInsertCall<'a, S> {
        InstanceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _source_machine_image: Default::default(),
            _source_instance_template: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of instances contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> InstanceListCall<'a, S> {
        InstanceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources that refer to the VM instance specified in the request. For example, if the VM instance is part of a managed or unmanaged instance group, the referrers list includes the instance group. For more information, read Viewing referrers to VM instances.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the target instance scoping this request, or '-' if the request should span over all instances in the container.
    pub fn list_referrers(&self, project: &str, zone: &str, instance: &str) -> InstanceListReferrerCall<'a, S> {
        InstanceListReferrerCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes resource policies from an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    pub fn remove_resource_policies(&self, request: InstancesRemoveResourcePoliciesRequest, project: &str, zone: &str, instance: &str) -> InstanceRemoveResourcePolicyCall<'a, S> {
        InstanceRemoveResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a reset on the instance. This is a hard reset. The VM does not do a graceful shutdown. For more information, see Resetting an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn reset(&self, project: &str, zone: &str, instance: &str) -> InstanceResetCall<'a, S> {
        InstanceResetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resumes an instance that was suspended using the instances().suspend method.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to resume.
    pub fn resume(&self, project: &str, zone: &str, instance: &str) -> InstanceResumeCall<'a, S> {
        InstanceResumeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends diagnostic interrupt to the instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn send_diagnostic_interrupt(&self, project: &str, zone: &str, instance: &str) -> InstanceSendDiagnosticInterruptCall<'a, S> {
        InstanceSendDiagnosticInterruptCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets deletion protection on the instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_deletion_protection(&self, project: &str, zone: &str, resource: &str) -> InstanceSetDeletionProtectionCall<'a, S> {
        InstanceSetDeletionProtectionCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _deletion_protection: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the auto-delete flag for a disk attached to an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    /// * `autoDelete` - Whether to auto-delete the disk when the instance is deleted.
    /// * `deviceName` - The device name of the disk to modify. Make a get() request on the instance to view currently attached disks and device names.
    pub fn set_disk_auto_delete(&self, project: &str, zone: &str, instance: &str, auto_delete: bool, device_name: &str) -> InstanceSetDiskAutoDeleteCall<'a, S> {
        InstanceSetDiskAutoDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _auto_delete: auto_delete,
            _device_name: device_name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: ZoneSetPolicyRequest, project: &str, zone: &str, resource: &str) -> InstanceSetIamPolicyCall<'a, S> {
        InstanceSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets labels on an instance. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_labels(&self, request: InstancesSetLabelsRequest, project: &str, zone: &str, instance: &str) -> InstanceSetLabelCall<'a, S> {
        InstanceSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the number and/or type of accelerator for a stopped instance to the values specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_machine_resources(&self, request: InstancesSetMachineResourcesRequest, project: &str, zone: &str, instance: &str) -> InstanceSetMachineResourceCall<'a, S> {
        InstanceSetMachineResourceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the machine type for a stopped instance to the machine type specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_machine_type(&self, request: InstancesSetMachineTypeRequest, project: &str, zone: &str, instance: &str) -> InstanceSetMachineTypeCall<'a, S> {
        InstanceSetMachineTypeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets metadata for the specified instance to the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_metadata(&self, request: Metadata, project: &str, zone: &str, instance: &str) -> InstanceSetMetadataCall<'a, S> {
        InstanceSetMetadataCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the minimum CPU platform that this instance should use. This method can only be called on a stopped instance. For more information, read Specifying a Minimum CPU Platform.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_min_cpu_platform(&self, request: InstancesSetMinCpuPlatformRequest, project: &str, zone: &str, instance: &str) -> InstanceSetMinCpuPlatformCall<'a, S> {
        InstanceSetMinCpuPlatformCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets an instance's scheduling options. You can only call this method on a stopped instance, that is, a VM instance that is in a `TERMINATED` state. See Instance Life Cycle for more information on the possible instance states. For more information about setting scheduling options for a VM, see Set VM host maintenance policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Instance name for this request.
    pub fn set_scheduling(&self, request: Scheduling, project: &str, zone: &str, instance: &str) -> InstanceSetSchedulingCall<'a, S> {
        InstanceSetSchedulingCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the service account on the instance. For more information, read Changing the service account and access scopes for an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to start.
    pub fn set_service_account(&self, request: InstancesSetServiceAccountRequest, project: &str, zone: &str, instance: &str) -> InstanceSetServiceAccountCall<'a, S> {
        InstanceSetServiceAccountCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the Shielded Instance integrity policy for an instance. You can only use this method on a running instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name or id of the instance scoping this request.
    pub fn set_shielded_instance_integrity_policy(&self, request: ShieldedInstanceIntegrityPolicy, project: &str, zone: &str, instance: &str) -> InstanceSetShieldedInstanceIntegrityPolicyCall<'a, S> {
        InstanceSetShieldedInstanceIntegrityPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets network tags for the specified instance to the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn set_tags(&self, request: Tags, project: &str, zone: &str, instance: &str) -> InstanceSetTagCall<'a, S> {
        InstanceSetTagCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Simulates a host maintenance event on a VM. For more information, see Simulate a host maintenance event.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn simulate_maintenance_event(&self, project: &str, zone: &str, instance: &str) -> InstanceSimulateMaintenanceEventCall<'a, S> {
        InstanceSimulateMaintenanceEventCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts an instance that was stopped using the instances().stop method. For more information, see Restart an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to start.
    pub fn start(&self, project: &str, zone: &str, instance: &str) -> InstanceStartCall<'a, S> {
        InstanceStartCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts an instance that was stopped using the instances().stop method. For more information, see Restart an instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to start.
    pub fn start_with_encryption_key(&self, request: InstancesStartWithEncryptionKeyRequest, project: &str, zone: &str, instance: &str) -> InstanceStartWithEncryptionKeyCall<'a, S> {
        InstanceStartWithEncryptionKeyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops a running instance, shutting it down cleanly, and allows you to restart the instance at a later time. Stopped instances do not incur VM usage charges while they are stopped. However, resources that the VM is using, such as persistent disks and static IP addresses, will continue to be charged until they are deleted. For more information, see Stopping an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to stop.
    pub fn stop(&self, project: &str, zone: &str, instance: &str) -> InstanceStopCall<'a, S> {
        InstanceStopCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _discard_local_ssd: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// This method suspends a running instance, saving its state to persistent storage, and allows you to resume the instance at a later time. Suspended instances have no compute costs (cores or RAM), and incur only storage charges for the saved VM memory and localSSD data. Any charged resources the virtual machine was using, such as persistent disks and static IP addresses, will continue to be charged while the instance is suspended. For more information, see Suspending and resuming an instance.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to suspend.
    pub fn suspend(&self, project: &str, zone: &str, instance: &str) -> InstanceSuspendCall<'a, S> {
        InstanceSuspendCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _discard_local_ssd: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, zone: &str, resource: &str) -> InstanceTestIamPermissionCall<'a, S> {
        InstanceTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an instance only if the necessary resources are available. This method can update only a specific set of instance properties. See Updating a running instance for a list of updatable instance properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance resource to update.
    pub fn update(&self, request: Instance, project: &str, zone: &str, instance: &str) -> InstanceUpdateCall<'a, S> {
        InstanceUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _most_disruptive_allowed_action: Default::default(),
            _minimal_action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified access config from an instance's network interface with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    /// * `networkInterface` - The name of the network interface where the access config is attached.
    pub fn update_access_config(&self, request: AccessConfig, project: &str, zone: &str, instance: &str, network_interface: &str) -> InstanceUpdateAccessConfigCall<'a, S> {
        InstanceUpdateAccessConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _network_interface: network_interface.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Display config for a VM instance. You can only use this method on a stopped VM instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name of the instance scoping this request.
    pub fn update_display_device(&self, request: DisplayDevice, project: &str, zone: &str, instance: &str) -> InstanceUpdateDisplayDeviceCall<'a, S> {
        InstanceUpdateDisplayDeviceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an instance's network interface. This method can only update an interface's alias IP range and attached network. See Modifying alias IP ranges for an existing instance for instructions on changing alias IP ranges. See Migrating a VM between networks for instructions on migrating an interface. This method follows PATCH semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - The instance name for this request.
    /// * `networkInterface` - The name of the network interface to update.
    pub fn update_network_interface(&self, request: NetworkInterface, project: &str, zone: &str, instance: &str, network_interface: &str) -> InstanceUpdateNetworkInterfaceCall<'a, S> {
        InstanceUpdateNetworkInterfaceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _network_interface: network_interface.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Shielded Instance config for an instance. You can only use this method on a stopped instance. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `instance` - Name or id of the instance scoping this request.
    pub fn update_shielded_instance_config(&self, request: ShieldedInstanceConfig, project: &str, zone: &str, instance: &str) -> InstanceUpdateShieldedInstanceConfigCall<'a, S> {
        InstanceUpdateShieldedInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance: instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *interconnectAttachment* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.interconnect_attachments();
/// # }
/// ```
pub struct InterconnectAttachmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InterconnectAttachmentMethods<'a, S> {}

impl<'a, S> InterconnectAttachmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of interconnect attachments.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> InterconnectAttachmentAggregatedListCall<'a, S> {
        InterconnectAttachmentAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified interconnect attachment.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `interconnectAttachment` - Name of the interconnect attachment to delete.
    pub fn delete(&self, project: &str, region: &str, interconnect_attachment: &str) -> InterconnectAttachmentDeleteCall<'a, S> {
        InterconnectAttachmentDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _interconnect_attachment: interconnect_attachment.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified interconnect attachment.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `interconnectAttachment` - Name of the interconnect attachment to return.
    pub fn get(&self, project: &str, region: &str, interconnect_attachment: &str) -> InterconnectAttachmentGetCall<'a, S> {
        InterconnectAttachmentGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _interconnect_attachment: interconnect_attachment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an InterconnectAttachment in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: InterconnectAttachment, project: &str, region: &str) -> InterconnectAttachmentInsertCall<'a, S> {
        InterconnectAttachmentInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of interconnect attachments contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> InterconnectAttachmentListCall<'a, S> {
        InterconnectAttachmentListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified interconnect attachment with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `interconnectAttachment` - Name of the interconnect attachment to patch.
    pub fn patch(&self, request: InterconnectAttachment, project: &str, region: &str, interconnect_attachment: &str) -> InterconnectAttachmentPatchCall<'a, S> {
        InterconnectAttachmentPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _interconnect_attachment: interconnect_attachment.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on an InterconnectAttachment. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> InterconnectAttachmentSetLabelCall<'a, S> {
        InterconnectAttachmentSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *interconnectLocation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.interconnect_locations();
/// # }
/// ```
pub struct InterconnectLocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InterconnectLocationMethods<'a, S> {}

impl<'a, S> InterconnectLocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the details for the specified interconnect location. Gets a list of available interconnect locations by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `interconnectLocation` - Name of the interconnect location to return.
    pub fn get(&self, project: &str, interconnect_location: &str) -> InterconnectLocationGetCall<'a, S> {
        InterconnectLocationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _interconnect_location: interconnect_location.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of interconnect locations available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> InterconnectLocationListCall<'a, S> {
        InterconnectLocationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *interconnect* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_diagnostics(...)`, `insert(...)`, `list(...)`, `patch(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.interconnects();
/// # }
/// ```
pub struct InterconnectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for InterconnectMethods<'a, S> {}

impl<'a, S> InterconnectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Interconnect.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `interconnect` - Name of the interconnect to delete.
    pub fn delete(&self, project: &str, interconnect: &str) -> InterconnectDeleteCall<'a, S> {
        InterconnectDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _interconnect: interconnect.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Interconnect. Get a list of available Interconnects by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `interconnect` - Name of the interconnect to return.
    pub fn get(&self, project: &str, interconnect: &str) -> InterconnectGetCall<'a, S> {
        InterconnectGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _interconnect: interconnect.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the interconnectDiagnostics for the specified Interconnect.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `interconnect` - Name of the interconnect resource to query.
    pub fn get_diagnostics(&self, project: &str, interconnect: &str) -> InterconnectGetDiagnosticCall<'a, S> {
        InterconnectGetDiagnosticCall {
            hub: self.hub,
            _project: project.to_string(),
            _interconnect: interconnect.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an Interconnect in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Interconnect, project: &str) -> InterconnectInsertCall<'a, S> {
        InterconnectInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Interconnects available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> InterconnectListCall<'a, S> {
        InterconnectListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Interconnect with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `interconnect` - Name of the interconnect to update.
    pub fn patch(&self, request: Interconnect, project: &str, interconnect: &str) -> InterconnectPatchCall<'a, S> {
        InterconnectPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _interconnect: interconnect.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on an Interconnect. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> InterconnectSetLabelCall<'a, S> {
        InterconnectSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *licenseCode* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.license_codes();
/// # }
/// ```
pub struct LicenseCodeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for LicenseCodeMethods<'a, S> {}

impl<'a, S> LicenseCodeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a specified license code. License codes are mirrored across all projects that have permissions to read the License Code. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `licenseCode` - Number corresponding to the License code resource to return.
    pub fn get(&self, project: &str, license_code: &str) -> LicenseCodeGetCall<'a, S> {
        LicenseCodeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _license_code: license_code.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> LicenseCodeTestIamPermissionCall<'a, S> {
        LicenseCodeTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *license* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.licenses();
/// # }
/// ```
pub struct LicenseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for LicenseMethods<'a, S> {}

impl<'a, S> LicenseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified license. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `license` - Name of the license resource to delete.
    pub fn delete(&self, project: &str, license: &str) -> LicenseDeleteCall<'a, S> {
        LicenseDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _license: license.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified License resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `license` - Name of the License resource to return.
    pub fn get(&self, project: &str, license: &str) -> LicenseGetCall<'a, S> {
        LicenseGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _license: license.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> LicenseGetIamPolicyCall<'a, S> {
        LicenseGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a License resource in the specified project. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: License, project: &str) -> LicenseInsertCall<'a, S> {
        LicenseInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of licenses available in the specified project. This method does not get any licenses that belong to other projects, including licenses attached to publicly-available images, like Debian 9. If you want to get a list of publicly-available licenses, use this method to make a request to the respective image project, such as debian-cloud or windows-cloud. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> LicenseListCall<'a, S> {
        LicenseListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> LicenseSetIamPolicyCall<'a, S> {
        LicenseSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> LicenseTestIamPermissionCall<'a, S> {
        LicenseTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *machineImage* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.machine_images();
/// # }
/// ```
pub struct MachineImageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for MachineImageMethods<'a, S> {}

impl<'a, S> MachineImageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified machine image. Deleting a machine image is permanent and cannot be undone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `machineImage` - The name of the machine image to delete.
    pub fn delete(&self, project: &str, machine_image: &str) -> MachineImageDeleteCall<'a, S> {
        MachineImageDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _machine_image: machine_image.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified machine image. Gets a list of available machine images by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `machineImage` - The name of the machine image.
    pub fn get(&self, project: &str, machine_image: &str) -> MachineImageGetCall<'a, S> {
        MachineImageGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _machine_image: machine_image.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> MachineImageGetIamPolicyCall<'a, S> {
        MachineImageGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a machine image in the specified project using the data that is included in the request. If you are creating a new machine image to update an existing instance, your new machine image should use the same network or, if applicable, the same subnetwork as the original instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: MachineImage, project: &str) -> MachineImageInsertCall<'a, S> {
        MachineImageInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _source_instance: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of machine images that are contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> MachineImageListCall<'a, S> {
        MachineImageListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> MachineImageSetIamPolicyCall<'a, S> {
        MachineImageSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> MachineImageTestIamPermissionCall<'a, S> {
        MachineImageTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *machineType* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.machine_types();
/// # }
/// ```
pub struct MachineTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for MachineTypeMethods<'a, S> {}

impl<'a, S> MachineTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of machine types.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> MachineTypeAggregatedListCall<'a, S> {
        MachineTypeAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified machine type. Gets a list of available machine types by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `machineType` - Name of the machine type to return.
    pub fn get(&self, project: &str, zone: &str, machine_type: &str) -> MachineTypeGetCall<'a, S> {
        MachineTypeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _machine_type: machine_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of machine types available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> MachineTypeListCall<'a, S> {
        MachineTypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *networkAttachment* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.network_attachments();
/// # }
/// ```
pub struct NetworkAttachmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NetworkAttachmentMethods<'a, S> {}

impl<'a, S> NetworkAttachmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all NetworkAttachment resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> NetworkAttachmentAggregatedListCall<'a, S> {
        NetworkAttachmentAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified NetworkAttachment in the given scope
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `networkAttachment` - Name of the NetworkAttachment resource to delete.
    pub fn delete(&self, project: &str, region: &str, network_attachment: &str) -> NetworkAttachmentDeleteCall<'a, S> {
        NetworkAttachmentDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_attachment: network_attachment.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified NetworkAttachment resource in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `networkAttachment` - Name of the NetworkAttachment resource to return.
    pub fn get(&self, project: &str, region: &str, network_attachment: &str) -> NetworkAttachmentGetCall<'a, S> {
        NetworkAttachmentGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_attachment: network_attachment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> NetworkAttachmentGetIamPolicyCall<'a, S> {
        NetworkAttachmentGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a NetworkAttachment in the specified project in the given scope using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn insert(&self, request: NetworkAttachment, project: &str, region: &str) -> NetworkAttachmentInsertCall<'a, S> {
        NetworkAttachmentInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the NetworkAttachments for a project in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn list(&self, project: &str, region: &str) -> NetworkAttachmentListCall<'a, S> {
        NetworkAttachmentListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> NetworkAttachmentSetIamPolicyCall<'a, S> {
        NetworkAttachmentSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> NetworkAttachmentTestIamPermissionCall<'a, S> {
        NetworkAttachmentTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *networkEdgeSecurityService* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.network_edge_security_services();
/// # }
/// ```
pub struct NetworkEdgeSecurityServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NetworkEdgeSecurityServiceMethods<'a, S> {}

impl<'a, S> NetworkEdgeSecurityServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all NetworkEdgeSecurityService resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> NetworkEdgeSecurityServiceAggregatedListCall<'a, S> {
        NetworkEdgeSecurityServiceAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified service.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `networkEdgeSecurityService` - Name of the network edge security service to delete.
    pub fn delete(&self, project: &str, region: &str, network_edge_security_service: &str) -> NetworkEdgeSecurityServiceDeleteCall<'a, S> {
        NetworkEdgeSecurityServiceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_edge_security_service: network_edge_security_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specified NetworkEdgeSecurityService.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `networkEdgeSecurityService` - Name of the network edge security service to get.
    pub fn get(&self, project: &str, region: &str, network_edge_security_service: &str) -> NetworkEdgeSecurityServiceGetCall<'a, S> {
        NetworkEdgeSecurityServiceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_edge_security_service: network_edge_security_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new service in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: NetworkEdgeSecurityService, project: &str, region: &str) -> NetworkEdgeSecurityServiceInsertCall<'a, S> {
        NetworkEdgeSecurityServiceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified policy with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `networkEdgeSecurityService` - Name of the network edge security service to update.
    pub fn patch(&self, request: NetworkEdgeSecurityService, project: &str, region: &str, network_edge_security_service: &str) -> NetworkEdgeSecurityServicePatchCall<'a, S> {
        NetworkEdgeSecurityServicePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_edge_security_service: network_edge_security_service.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *networkEndpointGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `attach_network_endpoints(...)`, `delete(...)`, `detach_network_endpoints(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_network_endpoints(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.network_endpoint_groups();
/// # }
/// ```
pub struct NetworkEndpointGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NetworkEndpointGroupMethods<'a, S> {}

impl<'a, S> NetworkEndpointGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of network endpoint groups and sorts them by zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> NetworkEndpointGroupAggregatedListCall<'a, S> {
        NetworkEndpointGroupAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attach a list of network endpoints to the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group where you are attaching network endpoints to. It should comply with RFC1035.
    pub fn attach_network_endpoints(&self, request: NetworkEndpointGroupsAttachEndpointsRequest, project: &str, zone: &str, network_endpoint_group: &str) -> NetworkEndpointGroupAttachNetworkEndpointCall<'a, S> {
        NetworkEndpointGroupAttachNetworkEndpointCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified network endpoint group. The network endpoints in the NEG and the VM instances they belong to are not terminated when the NEG is deleted. Note that the NEG cannot be deleted if there are backend services referencing it.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group to delete. It should comply with RFC1035.
    pub fn delete(&self, project: &str, zone: &str, network_endpoint_group: &str) -> NetworkEndpointGroupDeleteCall<'a, S> {
        NetworkEndpointGroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detach a list of network endpoints from the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group where you are removing network endpoints. It should comply with RFC1035.
    pub fn detach_network_endpoints(&self, request: NetworkEndpointGroupsDetachEndpointsRequest, project: &str, zone: &str, network_endpoint_group: &str) -> NetworkEndpointGroupDetachNetworkEndpointCall<'a, S> {
        NetworkEndpointGroupDetachNetworkEndpointCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network endpoint group. Gets a list of available network endpoint groups by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group. It should comply with RFC1035.
    pub fn get(&self, project: &str, zone: &str, network_endpoint_group: &str) -> NetworkEndpointGroupGetCall<'a, S> {
        NetworkEndpointGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a network endpoint group in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where you want to create the network endpoint group. It should comply with RFC1035.
    pub fn insert(&self, request: NetworkEndpointGroup, project: &str, zone: &str) -> NetworkEndpointGroupInsertCall<'a, S> {
        NetworkEndpointGroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of network endpoint groups that are located in the specified project and zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    pub fn list(&self, project: &str, zone: &str) -> NetworkEndpointGroupListCall<'a, S> {
        NetworkEndpointGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the network endpoints in the specified network endpoint group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group from which you want to generate a list of included network endpoints. It should comply with RFC1035.
    pub fn list_network_endpoints(&self, request: NetworkEndpointGroupsListEndpointsRequest, project: &str, zone: &str, network_endpoint_group: &str) -> NetworkEndpointGroupListNetworkEndpointCall<'a, S> {
        NetworkEndpointGroupListNetworkEndpointCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, zone: &str, resource: &str) -> NetworkEndpointGroupTestIamPermissionCall<'a, S> {
        NetworkEndpointGroupTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *networkFirewallPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_association(...)`, `add_rule(...)`, `clone_rules(...)`, `delete(...)`, `get(...)`, `get_association(...)`, `get_iam_policy(...)`, `get_rule(...)`, `insert(...)`, `list(...)`, `patch(...)`, `patch_rule(...)`, `remove_association(...)`, `remove_rule(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.network_firewall_policies();
/// # }
/// ```
pub struct NetworkFirewallPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NetworkFirewallPolicyMethods<'a, S> {}

impl<'a, S> NetworkFirewallPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an association for the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_association(&self, request: FirewallPolicyAssociation, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyAddAssociationCall<'a, S> {
        NetworkFirewallPolicyAddAssociationCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _replace_existing_association: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a rule into a firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_rule(&self, request: FirewallPolicyRule, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyAddRuleCall<'a, S> {
        NetworkFirewallPolicyAddRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _min_priority: Default::default(),
            _max_priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies rules to the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn clone_rules(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyCloneRuleCall<'a, S> {
        NetworkFirewallPolicyCloneRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _source_firewall_policy: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to delete.
    pub fn delete(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyDeleteCall<'a, S> {
        NetworkFirewallPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to get.
    pub fn get(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyGetCall<'a, S> {
        NetworkFirewallPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an association with the specified name.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to which the queried association belongs.
    pub fn get_association(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyGetAssociationCall<'a, S> {
        NetworkFirewallPolicyGetAssociationCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> NetworkFirewallPolicyGetIamPolicyCall<'a, S> {
        NetworkFirewallPolicyGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to which the queried rule belongs.
    pub fn get_rule(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyGetRuleCall<'a, S> {
        NetworkFirewallPolicyGetRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: FirewallPolicy, project: &str) -> NetworkFirewallPolicyInsertCall<'a, S> {
        NetworkFirewallPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the policies that have been configured for the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> NetworkFirewallPolicyListCall<'a, S> {
        NetworkFirewallPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified policy with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch(&self, request: FirewallPolicy, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyPatchCall<'a, S> {
        NetworkFirewallPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch_rule(&self, request: FirewallPolicyRule, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyPatchRuleCall<'a, S> {
        NetworkFirewallPolicyPatchRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an association for the specified firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_association(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyRemoveAssociationCall<'a, S> {
        NetworkFirewallPolicyRemoveAssociationCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_rule(&self, project: &str, firewall_policy: &str) -> NetworkFirewallPolicyRemoveRuleCall<'a, S> {
        NetworkFirewallPolicyRemoveRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> NetworkFirewallPolicySetIamPolicyCall<'a, S> {
        NetworkFirewallPolicySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> NetworkFirewallPolicyTestIamPermissionCall<'a, S> {
        NetworkFirewallPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *network* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_peering(...)`, `delete(...)`, `get(...)`, `get_effective_firewalls(...)`, `insert(...)`, `list(...)`, `list_peering_routes(...)`, `patch(...)`, `remove_peering(...)`, `switch_to_custom_mode(...)` and `update_peering(...)`
/// // to build up your call.
/// let rb = hub.networks();
/// # }
/// ```
pub struct NetworkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NetworkMethods<'a, S> {}

impl<'a, S> NetworkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a peering to the specified network.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network resource to add peering to.
    pub fn add_peering(&self, request: NetworksAddPeeringRequest, project: &str, network: &str) -> NetworkAddPeeringCall<'a, S> {
        NetworkAddPeeringCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified network.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network to delete.
    pub fn delete(&self, project: &str, network: &str) -> NetworkDeleteCall<'a, S> {
        NetworkDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network. Gets a list of available networks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network to return.
    pub fn get(&self, project: &str, network: &str) -> NetworkGetCall<'a, S> {
        NetworkGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _network: network.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the effective firewalls on a given network.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network for this request.
    pub fn get_effective_firewalls(&self, project: &str, network: &str) -> NetworkGetEffectiveFirewallCall<'a, S> {
        NetworkGetEffectiveFirewallCall {
            hub: self.hub,
            _project: project.to_string(),
            _network: network.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a network in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Network, project: &str) -> NetworkInsertCall<'a, S> {
        NetworkInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of networks available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> NetworkListCall<'a, S> {
        NetworkListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the peering routes exchanged over peering connection.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network for this request.
    pub fn list_peering_routes(&self, project: &str, network: &str) -> NetworkListPeeringRouteCall<'a, S> {
        NetworkListPeeringRouteCall {
            hub: self.hub,
            _project: project.to_string(),
            _network: network.to_string(),
            _return_partial_success: Default::default(),
            _region: Default::default(),
            _peering_name: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _direction: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified network with the data included in the request. Only the following fields can be modified: routingConfig.routingMode.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network to update.
    pub fn patch(&self, request: Network, project: &str, network: &str) -> NetworkPatchCall<'a, S> {
        NetworkPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a peering from the specified network.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network resource to remove peering from.
    pub fn remove_peering(&self, request: NetworksRemovePeeringRequest, project: &str, network: &str) -> NetworkRemovePeeringCall<'a, S> {
        NetworkRemovePeeringCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Switches the network mode from auto subnet mode to custom subnet mode.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network to be updated.
    pub fn switch_to_custom_mode(&self, project: &str, network: &str) -> NetworkSwitchToCustomModeCall<'a, S> {
        NetworkSwitchToCustomModeCall {
            hub: self.hub,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified network peering with the data included in the request. You can only modify the NetworkPeering.export_custom_routes field and the NetworkPeering.import_custom_routes field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `network` - Name of the network resource which the updated peering is belonging to.
    pub fn update_peering(&self, request: NetworksUpdatePeeringRequest, project: &str, network: &str) -> NetworkUpdatePeeringCall<'a, S> {
        NetworkUpdatePeeringCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _network: network.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *nodeGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_nodes(...)`, `aggregated_list(...)`, `delete(...)`, `delete_nodes(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `list_nodes(...)`, `patch(...)`, `set_iam_policy(...)`, `set_node_template(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.node_groups();
/// # }
/// ```
pub struct NodeGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NodeGroupMethods<'a, S> {}

impl<'a, S> NodeGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds specified number of nodes to the node group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource.
    pub fn add_nodes(&self, request: NodeGroupsAddNodesRequest, project: &str, zone: &str, node_group: &str) -> NodeGroupAddNodeCall<'a, S> {
        NodeGroupAddNodeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of node groups. Note: use nodeGroups.listNodes for more details about each group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> NodeGroupAggregatedListCall<'a, S> {
        NodeGroupAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified NodeGroup resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource to delete.
    pub fn delete(&self, project: &str, zone: &str, node_group: &str) -> NodeGroupDeleteCall<'a, S> {
        NodeGroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes specified nodes from the node group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource whose nodes will be deleted.
    pub fn delete_nodes(&self, request: NodeGroupsDeleteNodesRequest, project: &str, zone: &str, node_group: &str) -> NodeGroupDeleteNodeCall<'a, S> {
        NodeGroupDeleteNodeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified NodeGroup. Get a list of available NodeGroups by making a list() request. Note: the "nodes" field should not be used. Use nodeGroups.listNodes instead.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the node group to return.
    pub fn get(&self, project: &str, zone: &str, node_group: &str) -> NodeGroupGetCall<'a, S> {
        NodeGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, zone: &str, resource: &str) -> NodeGroupGetIamPolicyCall<'a, S> {
        NodeGroupGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a NodeGroup resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `initialNodeCount` - Initial count of nodes in the node group.
    pub fn insert(&self, request: NodeGroup, project: &str, zone: &str, initial_node_count: i32) -> NodeGroupInsertCall<'a, S> {
        NodeGroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _initial_node_count: initial_node_count,
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of node groups available to the specified project. Note: use nodeGroups.listNodes for more details about each group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> NodeGroupListCall<'a, S> {
        NodeGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists nodes in the node group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource whose nodes you want to list.
    pub fn list_nodes(&self, project: &str, zone: &str, node_group: &str) -> NodeGroupListNodeCall<'a, S> {
        NodeGroupListNodeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified node group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource to update.
    pub fn patch(&self, request: NodeGroup, project: &str, zone: &str, node_group: &str) -> NodeGroupPatchCall<'a, S> {
        NodeGroupPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: ZoneSetPolicyRequest, project: &str, zone: &str, resource: &str) -> NodeGroupSetIamPolicyCall<'a, S> {
        NodeGroupSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the node template of the node group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeGroup` - Name of the NodeGroup resource to update.
    pub fn set_node_template(&self, request: NodeGroupsSetNodeTemplateRequest, project: &str, zone: &str, node_group: &str) -> NodeGroupSetNodeTemplateCall<'a, S> {
        NodeGroupSetNodeTemplateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_group: node_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, zone: &str, resource: &str) -> NodeGroupTestIamPermissionCall<'a, S> {
        NodeGroupTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *nodeTemplate* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.node_templates();
/// # }
/// ```
pub struct NodeTemplateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NodeTemplateMethods<'a, S> {}

impl<'a, S> NodeTemplateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of node templates.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> NodeTemplateAggregatedListCall<'a, S> {
        NodeTemplateAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified NodeTemplate resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `nodeTemplate` - Name of the NodeTemplate resource to delete.
    pub fn delete(&self, project: &str, region: &str, node_template: &str) -> NodeTemplateDeleteCall<'a, S> {
        NodeTemplateDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _node_template: node_template.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified node template. Gets a list of available node templates by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `nodeTemplate` - Name of the node template to return.
    pub fn get(&self, project: &str, region: &str, node_template: &str) -> NodeTemplateGetCall<'a, S> {
        NodeTemplateGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _node_template: node_template.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> NodeTemplateGetIamPolicyCall<'a, S> {
        NodeTemplateGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a NodeTemplate resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    pub fn insert(&self, request: NodeTemplate, project: &str, region: &str) -> NodeTemplateInsertCall<'a, S> {
        NodeTemplateInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of node templates available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> NodeTemplateListCall<'a, S> {
        NodeTemplateListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> NodeTemplateSetIamPolicyCall<'a, S> {
        NodeTemplateSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> NodeTemplateTestIamPermissionCall<'a, S> {
        NodeTemplateTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *nodeType* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.node_types();
/// # }
/// ```
pub struct NodeTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for NodeTypeMethods<'a, S> {}

impl<'a, S> NodeTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of node types.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> NodeTypeAggregatedListCall<'a, S> {
        NodeTypeAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified node type. Gets a list of available node types by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `nodeType` - Name of the node type to return.
    pub fn get(&self, project: &str, zone: &str, node_type: &str) -> NodeTypeGetCall<'a, S> {
        NodeTypeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _node_type: node_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of node types available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> NodeTypeListCall<'a, S> {
        NodeTypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *packetMirroring* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.packet_mirrorings();
/// # }
/// ```
pub struct PacketMirroringMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for PacketMirroringMethods<'a, S> {}

impl<'a, S> PacketMirroringMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of packetMirrorings.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> PacketMirroringAggregatedListCall<'a, S> {
        PacketMirroringAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified PacketMirroring resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `packetMirroring` - Name of the PacketMirroring resource to delete.
    pub fn delete(&self, project: &str, region: &str, packet_mirroring: &str) -> PacketMirroringDeleteCall<'a, S> {
        PacketMirroringDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _packet_mirroring: packet_mirroring.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified PacketMirroring resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `packetMirroring` - Name of the PacketMirroring resource to return.
    pub fn get(&self, project: &str, region: &str, packet_mirroring: &str) -> PacketMirroringGetCall<'a, S> {
        PacketMirroringGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _packet_mirroring: packet_mirroring.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a PacketMirroring resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: PacketMirroring, project: &str, region: &str) -> PacketMirroringInsertCall<'a, S> {
        PacketMirroringInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of PacketMirroring resources available to the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> PacketMirroringListCall<'a, S> {
        PacketMirroringListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified PacketMirroring resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `packetMirroring` - Name of the PacketMirroring resource to patch.
    pub fn patch(&self, request: PacketMirroring, project: &str, region: &str, packet_mirroring: &str) -> PacketMirroringPatchCall<'a, S> {
        PacketMirroringPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _packet_mirroring: packet_mirroring.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> PacketMirroringTestIamPermissionCall<'a, S> {
        PacketMirroringTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `disable_xpn_host(...)`, `disable_xpn_resource(...)`, `enable_xpn_host(...)`, `enable_xpn_resource(...)`, `get(...)`, `get_xpn_host(...)`, `get_xpn_resources(...)`, `list_xpn_hosts(...)`, `move_disk(...)`, `move_instance(...)`, `set_common_instance_metadata(...)`, `set_default_network_tier(...)` and `set_usage_export_bucket(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disable this project as a shared VPC host project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn disable_xpn_host(&self, project: &str) -> ProjectDisableXpnHostCall<'a, S> {
        ProjectDisableXpnHostCall {
            hub: self.hub,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Disable a service resource (also known as service project) associated with this host project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn disable_xpn_resource(&self, request: ProjectsDisableXpnResourceRequest, project: &str) -> ProjectDisableXpnResourceCall<'a, S> {
        ProjectDisableXpnResourceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enable this project as a shared VPC host project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn enable_xpn_host(&self, project: &str) -> ProjectEnableXpnHostCall<'a, S> {
        ProjectEnableXpnHostCall {
            hub: self.hub,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enable service resource (a.k.a service project) for a host project, so that subnets in the host project can be used by instances in the service project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn enable_xpn_resource(&self, request: ProjectsEnableXpnResourceRequest, project: &str) -> ProjectEnableXpnResourceCall<'a, S> {
        ProjectEnableXpnResourceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Project resource. To decrease latency for this method, you can optionally omit any unneeded information from the response by using a field mask. This practice is especially recommended for unused quota information (the `quotas` field). To exclude one or more fields, set your request's `fields` query parameter to only include the fields you need. For example, to only include the `id` and `selfLink` fields, add the query parameter `?fields=id,selfLink` to your request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn get(&self, project: &str) -> ProjectGetCall<'a, S> {
        ProjectGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the shared VPC host project that this project links to. May be empty if no link exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn get_xpn_host(&self, project: &str) -> ProjectGetXpnHostCall<'a, S> {
        ProjectGetXpnHostCall {
            hub: self.hub,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets service resources (a.k.a service project) associated with this host project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn get_xpn_resources(&self, project: &str) -> ProjectGetXpnResourceCall<'a, S> {
        ProjectGetXpnResourceCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all shared VPC host projects visible to the user in an organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn list_xpn_hosts(&self, request: ProjectsListXpnHostsRequest, project: &str) -> ProjectListXpnHostCall<'a, S> {
        ProjectListXpnHostCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a persistent disk from one zone to another.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn move_disk(&self, request: DiskMoveRequest, project: &str) -> ProjectMoveDiskCall<'a, S> {
        ProjectMoveDiskCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves an instance and its attached persistent disks from one zone to another. *Note*: Moving VMs or disks by using this method might cause unexpected behavior. For more information, see the [known issue](https://cloud.google.com/compute/docs/troubleshooting/known-issues#moving_vms_or_disks_using_the_moveinstance_api_or_the_causes_unexpected_behavior).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn move_instance(&self, request: InstanceMoveRequest, project: &str) -> ProjectMoveInstanceCall<'a, S> {
        ProjectMoveInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets metadata common to all instances within the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn set_common_instance_metadata(&self, request: Metadata, project: &str) -> ProjectSetCommonInstanceMetadataCall<'a, S> {
        ProjectSetCommonInstanceMetadataCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the default network tier of the project. The default network tier is used when an address/forwardingRule/instance is created without specifying the network tier field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn set_default_network_tier(&self, request: ProjectsSetDefaultNetworkTierRequest, project: &str) -> ProjectSetDefaultNetworkTierCall<'a, S> {
        ProjectSetDefaultNetworkTierCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enables the usage export feature and sets the usage export bucket where reports are stored. If you provide an empty request body using this method, the usage export feature will be disabled.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn set_usage_export_bucket(&self, request: UsageExportLocation, project: &str) -> ProjectSetUsageExportBucketCall<'a, S> {
        ProjectSetUsageExportBucketCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *publicAdvertisedPrefix* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.public_advertised_prefixes();
/// # }
/// ```
pub struct PublicAdvertisedPrefixMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for PublicAdvertisedPrefixMethods<'a, S> {}

impl<'a, S> PublicAdvertisedPrefixMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified PublicAdvertisedPrefix
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `publicAdvertisedPrefix` - Name of the PublicAdvertisedPrefix resource to delete.
    pub fn delete(&self, project: &str, public_advertised_prefix: &str) -> PublicAdvertisedPrefixDeleteCall<'a, S> {
        PublicAdvertisedPrefixDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _public_advertised_prefix: public_advertised_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified PublicAdvertisedPrefix resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `publicAdvertisedPrefix` - Name of the PublicAdvertisedPrefix resource to return.
    pub fn get(&self, project: &str, public_advertised_prefix: &str) -> PublicAdvertisedPrefixGetCall<'a, S> {
        PublicAdvertisedPrefixGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _public_advertised_prefix: public_advertised_prefix.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a PublicAdvertisedPrefix in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: PublicAdvertisedPrefix, project: &str) -> PublicAdvertisedPrefixInsertCall<'a, S> {
        PublicAdvertisedPrefixInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the PublicAdvertisedPrefixes for a project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> PublicAdvertisedPrefixListCall<'a, S> {
        PublicAdvertisedPrefixListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified Router resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `publicAdvertisedPrefix` - Name of the PublicAdvertisedPrefix resource to patch.
    pub fn patch(&self, request: PublicAdvertisedPrefix, project: &str, public_advertised_prefix: &str) -> PublicAdvertisedPrefixPatchCall<'a, S> {
        PublicAdvertisedPrefixPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _public_advertised_prefix: public_advertised_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *publicDelegatedPrefix* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.public_delegated_prefixes();
/// # }
/// ```
pub struct PublicDelegatedPrefixMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for PublicDelegatedPrefixMethods<'a, S> {}

impl<'a, S> PublicDelegatedPrefixMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all PublicDelegatedPrefix resources owned by the specific project across all scopes.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> PublicDelegatedPrefixAggregatedListCall<'a, S> {
        PublicDelegatedPrefixAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified PublicDelegatedPrefix in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to delete.
    pub fn delete(&self, project: &str, region: &str, public_delegated_prefix: &str) -> PublicDelegatedPrefixDeleteCall<'a, S> {
        PublicDelegatedPrefixDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified PublicDelegatedPrefix resource in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to return.
    pub fn get(&self, project: &str, region: &str, public_delegated_prefix: &str) -> PublicDelegatedPrefixGetCall<'a, S> {
        PublicDelegatedPrefixGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a PublicDelegatedPrefix in the specified project in the given region using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn insert(&self, request: PublicDelegatedPrefix, project: &str, region: &str) -> PublicDelegatedPrefixInsertCall<'a, S> {
        PublicDelegatedPrefixInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the PublicDelegatedPrefixes for a project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn list(&self, project: &str, region: &str) -> PublicDelegatedPrefixListCall<'a, S> {
        PublicDelegatedPrefixListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified PublicDelegatedPrefix resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `publicDelegatedPrefix` - Name of the PublicDelegatedPrefix resource to patch.
    pub fn patch(&self, request: PublicDelegatedPrefix, project: &str, region: &str, public_delegated_prefix: &str) -> PublicDelegatedPrefixPatchCall<'a, S> {
        PublicDelegatedPrefixPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _public_delegated_prefix: public_delegated_prefix.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionAutoscaler* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.region_autoscalers();
/// # }
/// ```
pub struct RegionAutoscalerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionAutoscalerMethods<'a, S> {}

impl<'a, S> RegionAutoscalerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified autoscaler.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `autoscaler` - Name of the autoscaler to delete.
    pub fn delete(&self, project: &str, region: &str, autoscaler: &str) -> RegionAutoscalerDeleteCall<'a, S> {
        RegionAutoscalerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _autoscaler: autoscaler.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified autoscaler.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `autoscaler` - Name of the autoscaler to return.
    pub fn get(&self, project: &str, region: &str, autoscaler: &str) -> RegionAutoscalerGetCall<'a, S> {
        RegionAutoscalerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _autoscaler: autoscaler.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an autoscaler in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: Autoscaler, project: &str, region: &str) -> RegionAutoscalerInsertCall<'a, S> {
        RegionAutoscalerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of autoscalers contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionAutoscalerListCall<'a, S> {
        RegionAutoscalerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an autoscaler in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn patch(&self, request: Autoscaler, project: &str, region: &str) -> RegionAutoscalerPatchCall<'a, S> {
        RegionAutoscalerPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _autoscaler: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an autoscaler in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn update(&self, request: Autoscaler, project: &str, region: &str) -> RegionAutoscalerUpdateCall<'a, S> {
        RegionAutoscalerUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _autoscaler: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionBackendService* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_health(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.region_backend_services();
/// # }
/// ```
pub struct RegionBackendServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionBackendServiceMethods<'a, S> {}

impl<'a, S> RegionBackendServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified regional BackendService resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `backendService` - Name of the BackendService resource to delete.
    pub fn delete(&self, project: &str, region: &str, backend_service: &str) -> RegionBackendServiceDeleteCall<'a, S> {
        RegionBackendServiceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified regional BackendService resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `backendService` - Name of the BackendService resource to return.
    pub fn get(&self, project: &str, region: &str, backend_service: &str) -> RegionBackendServiceGetCall<'a, S> {
        RegionBackendServiceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _backend_service: backend_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the most recent health check results for this regional BackendService.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - No description provided.
    /// * `region` - Name of the region scoping this request.
    /// * `backendService` - Name of the BackendService resource for which to get health.
    pub fn get_health(&self, request: ResourceGroupReference, project: &str, region: &str, backend_service: &str) -> RegionBackendServiceGetHealthCall<'a, S> {
        RegionBackendServiceGetHealthCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _backend_service: backend_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> RegionBackendServiceGetIamPolicyCall<'a, S> {
        RegionBackendServiceGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a regional BackendService resource in the specified project using the data included in the request. For more information, see Backend services overview.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: BackendService, project: &str, region: &str) -> RegionBackendServiceInsertCall<'a, S> {
        RegionBackendServiceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of regional BackendService resources available to the specified project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionBackendServiceListCall<'a, S> {
        RegionBackendServiceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified regional BackendService resource with the data included in the request. For more information, see Understanding backend services This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `backendService` - Name of the BackendService resource to patch.
    pub fn patch(&self, request: BackendService, project: &str, region: &str, backend_service: &str) -> RegionBackendServicePatchCall<'a, S> {
        RegionBackendServicePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> RegionBackendServiceSetIamPolicyCall<'a, S> {
        RegionBackendServiceSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified regional BackendService resource with the data included in the request. For more information, see Backend services overview .
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `backendService` - Name of the BackendService resource to update.
    pub fn update(&self, request: BackendService, project: &str, region: &str, backend_service: &str) -> RegionBackendServiceUpdateCall<'a, S> {
        RegionBackendServiceUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _backend_service: backend_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionCommitment* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.region_commitments();
/// # }
/// ```
pub struct RegionCommitmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionCommitmentMethods<'a, S> {}

impl<'a, S> RegionCommitmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of commitments by region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> RegionCommitmentAggregatedListCall<'a, S> {
        RegionCommitmentAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified commitment resource. Gets a list of available commitments by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `commitment` - Name of the commitment to return.
    pub fn get(&self, project: &str, region: &str, commitment: &str) -> RegionCommitmentGetCall<'a, S> {
        RegionCommitmentGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _commitment: commitment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a commitment in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: Commitment, project: &str, region: &str) -> RegionCommitmentInsertCall<'a, S> {
        RegionCommitmentInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of commitments contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> RegionCommitmentListCall<'a, S> {
        RegionCommitmentListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified commitment with the data included in the request. Update is performed only on selected fields included as part of update-mask. Only the following fields can be modified: auto_renew.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `commitment` - Name of the commitment for which auto renew is being updated.
    pub fn update(&self, request: Commitment, project: &str, region: &str, commitment: &str) -> RegionCommitmentUpdateCall<'a, S> {
        RegionCommitmentUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _commitment: commitment.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionDiskType* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.region_disk_types();
/// # }
/// ```
pub struct RegionDiskTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionDiskTypeMethods<'a, S> {}

impl<'a, S> RegionDiskTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified regional disk type. Gets a list of available disk types by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `diskType` - Name of the disk type to return.
    pub fn get(&self, project: &str, region: &str, disk_type: &str) -> RegionDiskTypeGetCall<'a, S> {
        RegionDiskTypeGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk_type: disk_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of regional disk types available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> RegionDiskTypeListCall<'a, S> {
        RegionDiskTypeListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionDisk* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_resource_policies(...)`, `create_snapshot(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `remove_resource_policies(...)`, `resize(...)`, `set_iam_policy(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.region_disks();
/// # }
/// ```
pub struct RegionDiskMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionDiskMethods<'a, S> {}

impl<'a, S> RegionDiskMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds existing resource policies to a regional disk. You can only add one policy which will be applied to this disk for scheduling snapshot creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `disk` - The disk name for this request.
    pub fn add_resource_policies(&self, request: RegionDisksAddResourcePoliciesRequest, project: &str, region: &str, disk: &str) -> RegionDiskAddResourcePolicyCall<'a, S> {
        RegionDiskAddResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a snapshot of a specified persistent disk. For regular snapshot creation, consider using snapshots.insert instead, as that method supports more features, such as creating snapshots in a project different from the source disk project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `disk` - Name of the regional persistent disk to snapshot.
    pub fn create_snapshot(&self, request: Snapshot, project: &str, region: &str, disk: &str) -> RegionDiskCreateSnapshotCall<'a, S> {
        RegionDiskCreateSnapshotCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified regional persistent disk. Deleting a regional disk removes all the replicas of its data permanently and is irreversible. However, deleting a disk does not delete any snapshots previously made from the disk. You must separately delete snapshots.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `disk` - Name of the regional persistent disk to delete.
    pub fn delete(&self, project: &str, region: &str, disk: &str) -> RegionDiskDeleteCall<'a, S> {
        RegionDiskDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a specified regional persistent disk.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `disk` - Name of the regional persistent disk to return.
    pub fn get(&self, project: &str, region: &str, disk: &str) -> RegionDiskGetCall<'a, S> {
        RegionDiskGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> RegionDiskGetIamPolicyCall<'a, S> {
        RegionDiskGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a persistent regional disk in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: Disk, project: &str, region: &str) -> RegionDiskInsertCall<'a, S> {
        RegionDiskInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _source_image: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of persistent disks contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> RegionDiskListCall<'a, S> {
        RegionDiskListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes resource policies from a regional disk.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `disk` - The disk name for this request.
    pub fn remove_resource_policies(&self, request: RegionDisksRemoveResourcePoliciesRequest, project: &str, region: &str, disk: &str) -> RegionDiskRemoveResourcePolicyCall<'a, S> {
        RegionDiskRemoveResourcePolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the specified regional persistent disk.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `disk` - Name of the regional persistent disk.
    pub fn resize(&self, request: RegionDisksResizeRequest, project: &str, region: &str, disk: &str) -> RegionDiskResizeCall<'a, S> {
        RegionDiskResizeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _disk: disk.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> RegionDiskSetIamPolicyCall<'a, S> {
        RegionDiskSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on the target regional disk.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> RegionDiskSetLabelCall<'a, S> {
        RegionDiskSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> RegionDiskTestIamPermissionCall<'a, S> {
        RegionDiskTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionHealthCheckService* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.region_health_check_services();
/// # }
/// ```
pub struct RegionHealthCheckServiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionHealthCheckServiceMethods<'a, S> {}

impl<'a, S> RegionHealthCheckServiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified regional HealthCheckService.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheckService` - Name of the HealthCheckService to delete. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn delete(&self, project: &str, region: &str, health_check_service: &str) -> RegionHealthCheckServiceDeleteCall<'a, S> {
        RegionHealthCheckServiceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check_service: health_check_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified regional HealthCheckService resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheckService` - Name of the HealthCheckService to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn get(&self, project: &str, region: &str, health_check_service: &str) -> RegionHealthCheckServiceGetCall<'a, S> {
        RegionHealthCheckServiceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check_service: health_check_service.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a regional HealthCheckService resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: HealthCheckService, project: &str, region: &str) -> RegionHealthCheckServiceInsertCall<'a, S> {
        RegionHealthCheckServiceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the HealthCheckService resources that have been configured for the specified project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionHealthCheckServiceListCall<'a, S> {
        RegionHealthCheckServiceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified regional HealthCheckService resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheckService` - Name of the HealthCheckService to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn patch(&self, request: HealthCheckService, project: &str, region: &str, health_check_service: &str) -> RegionHealthCheckServicePatchCall<'a, S> {
        RegionHealthCheckServicePatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check_service: health_check_service.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionHealthCheck* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.region_health_checks();
/// # }
/// ```
pub struct RegionHealthCheckMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionHealthCheckMethods<'a, S> {}

impl<'a, S> RegionHealthCheckMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified HealthCheck resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheck` - Name of the HealthCheck resource to delete.
    pub fn delete(&self, project: &str, region: &str, health_check: &str) -> RegionHealthCheckDeleteCall<'a, S> {
        RegionHealthCheckDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified HealthCheck resource. Gets a list of available health checks by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheck` - Name of the HealthCheck resource to return.
    pub fn get(&self, project: &str, region: &str, health_check: &str) -> RegionHealthCheckGetCall<'a, S> {
        RegionHealthCheckGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check: health_check.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a HealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: HealthCheck, project: &str, region: &str) -> RegionHealthCheckInsertCall<'a, S> {
        RegionHealthCheckInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of HealthCheck resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionHealthCheckListCall<'a, S> {
        RegionHealthCheckListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HealthCheck resource in the specified project using the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheck` - Name of the HealthCheck resource to patch.
    pub fn patch(&self, request: HealthCheck, project: &str, region: &str, health_check: &str) -> RegionHealthCheckPatchCall<'a, S> {
        RegionHealthCheckPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a HealthCheck resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `healthCheck` - Name of the HealthCheck resource to update.
    pub fn update(&self, request: HealthCheck, project: &str, region: &str, health_check: &str) -> RegionHealthCheckUpdateCall<'a, S> {
        RegionHealthCheckUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _health_check: health_check.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionInstanceGroupManager* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `abandon_instances(...)`, `apply_updates_to_instances(...)`, `create_instances(...)`, `delete(...)`, `delete_instances(...)`, `delete_per_instance_configs(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_errors(...)`, `list_managed_instances(...)`, `list_per_instance_configs(...)`, `patch(...)`, `patch_per_instance_configs(...)`, `recreate_instances(...)`, `resize(...)`, `set_instance_template(...)`, `set_target_pools(...)` and `update_per_instance_configs(...)`
/// // to build up your call.
/// let rb = hub.region_instance_group_managers();
/// # }
/// ```
pub struct RegionInstanceGroupManagerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionInstanceGroupManagerMethods<'a, S> {}

impl<'a, S> RegionInstanceGroupManagerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified instances to be immediately removed from the managed instance group. Abandoning an instance does not delete the instance, but it does remove the instance from any target pools that are applied by the managed instance group. This method reduces the targetSize of the managed instance group by the number of instances that you abandon. This operation is marked as DONE when the action is scheduled even if the instances have not yet been removed from the group. You must separately verify the status of the abandoning action with the listmanagedinstances method. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group.
    pub fn abandon_instances(&self, request: RegionInstanceGroupManagersAbandonInstancesRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerAbandonInstanceCall<'a, S> {
        RegionInstanceGroupManagerAbandonInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Apply updates to selected instances the managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request, should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group, should conform to RFC1035.
    pub fn apply_updates_to_instances(&self, request: RegionInstanceGroupManagersApplyUpdatesRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerApplyUpdatesToInstanceCall<'a, S> {
        RegionInstanceGroupManagerApplyUpdatesToInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates instances with per-instance configurations in this regional managed instance group. Instances are created using the current instance template. The create instances operation is marked DONE if the createInstances request is successful. The underlying actions take additional time. You must separately verify the status of the creating or actions with the listmanagedinstances method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region where the managed instance group is located. It should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn create_instances(&self, request: RegionInstanceGroupManagersCreateInstancesRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerCreateInstanceCall<'a, S> {
        RegionInstanceGroupManagerCreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified managed instance group and all of the instances in that group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group to delete.
    pub fn delete(&self, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerDeleteCall<'a, S> {
        RegionInstanceGroupManagerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified instances in the managed instance group to be immediately deleted. The instances are also removed from any target pools of which they were a member. This method reduces the targetSize of the managed instance group by the number of instances that you delete. The deleteInstances operation is marked DONE if the deleteInstances request is successful. The underlying actions take additional time. You must separately verify the status of the deleting action with the listmanagedinstances method. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group.
    pub fn delete_instances(&self, request: RegionInstanceGroupManagersDeleteInstancesRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerDeleteInstanceCall<'a, S> {
        RegionInstanceGroupManagerDeleteInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes selected per-instance configurations for the managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request, should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn delete_per_instance_configs(&self, request: RegionInstanceGroupManagerDeleteInstanceConfigReq, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerDeletePerInstanceConfigCall<'a, S> {
        RegionInstanceGroupManagerDeletePerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all of the details about the specified managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group to return.
    pub fn get(&self, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerGetCall<'a, S> {
        RegionInstanceGroupManagerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a managed instance group using the information that you specify in the request. After the group is created, instances in the group are created using the specified instance template. This operation is marked as DONE when the group is created even if the instances in the group have not yet been created. You must separately verify the status of the individual instances with the listmanagedinstances method. A regional managed instance group can contain up to 2000 instances.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: InstanceGroupManager, project: &str, region: &str) -> RegionInstanceGroupManagerInsertCall<'a, S> {
        RegionInstanceGroupManagerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of managed instance groups that are contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionInstanceGroupManagerListCall<'a, S> {
        RegionInstanceGroupManagerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all errors thrown by actions on instances for a given regional managed instance group. The filter and orderBy query parameters are not supported.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request. This should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It must be a string that meets the requirements in RFC1035, or an unsigned long integer: must match regexp pattern: (?:[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?)|1-9{0,19}.
    pub fn list_errors(&self, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerListErrorCall<'a, S> {
        RegionInstanceGroupManagerListErrorCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the instances in the managed instance group and instances that are scheduled to be created. The list includes any current actions that the group has scheduled for its instances. The orderBy query parameter is not supported. The `pageToken` query parameter is supported only in the alpha and beta API and only if the group's `listManagedInstancesResults` field is set to `PAGINATED`.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn list_managed_instances(&self, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerListManagedInstanceCall<'a, S> {
        RegionInstanceGroupManagerListManagedInstanceCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the per-instance configurations defined for the managed instance group. The orderBy query parameter is not supported.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request, should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn list_per_instance_configs(&self, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerListPerInstanceConfigCall<'a, S> {
        RegionInstanceGroupManagerListPerInstanceConfigCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a managed instance group using the information that you specify in the request. This operation is marked as DONE when the group is patched even if the instances in the group are still in the process of being patched. You must separately verify the status of the individual instances with the listmanagedinstances method. This method supports PATCH semantics and uses the JSON merge patch format and processing rules. If you update your group to specify a new template or instance configuration, it's possible that your intended specification for each VM in the group is different from the current state of that VM. To learn how to apply an updated configuration to the VMs in a MIG, see Updating instances in a MIG.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn patch(&self, request: InstanceGroupManager, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerPatchCall<'a, S> {
        RegionInstanceGroupManagerPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or patches per-instance configurations for the managed instance group. perInstanceConfig.name serves as a key used to distinguish whether to perform insert or patch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request, should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn patch_per_instance_configs(&self, request: RegionInstanceGroupManagerPatchInstanceConfigReq, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerPatchPerInstanceConfigCall<'a, S> {
        RegionInstanceGroupManagerPatchPerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Flags the specified VM instances in the managed instance group to be immediately recreated. Each instance is recreated using the group's current configuration. This operation is marked as DONE when the flag is set even if the instances have not yet been recreated. You must separately verify the status of each instance by checking its currentAction field; for more information, see Checking the status of managed instances. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted. You can specify a maximum of 1000 instances with this method per request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group.
    pub fn recreate_instances(&self, request: RegionInstanceGroupManagersRecreateRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerRecreateInstanceCall<'a, S> {
        RegionInstanceGroupManagerRecreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the intended size of the managed instance group. If you increase the size, the group creates new instances using the current instance template. If you decrease the size, the group deletes one or more instances. The resize operation is marked DONE if the resize request is successful. The underlying actions take additional time. You must separately verify the status of the creating or deleting actions with the listmanagedinstances method. If the group is part of a backend service that has enabled connection draining, it can take up to 60 seconds after the connection draining duration has elapsed before the VM instance is removed or deleted.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group.
    /// * `size` - Number of instances that should exist in this instance group manager.
    pub fn resize(&self, project: &str, region: &str, instance_group_manager: &str, size: i32) -> RegionInstanceGroupManagerResizeCall<'a, S> {
        RegionInstanceGroupManagerResizeCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _size: size,
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the instance template to use when creating new instances or recreating instances in this group. Existing instances are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - The name of the managed instance group.
    pub fn set_instance_template(&self, request: RegionInstanceGroupManagersSetTemplateRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerSetInstanceTemplateCall<'a, S> {
        RegionInstanceGroupManagerSetInstanceTemplateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the target pools to which all new instances in this group are assigned. Existing instances in the group are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroupManager` - Name of the managed instance group.
    pub fn set_target_pools(&self, request: RegionInstanceGroupManagersSetTargetPoolsRequest, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerSetTargetPoolCall<'a, S> {
        RegionInstanceGroupManagerSetTargetPoolCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or updates per-instance configurations for the managed instance group. perInstanceConfig.name serves as a key used to distinguish whether to perform insert or patch.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request, should conform to RFC1035.
    /// * `instanceGroupManager` - The name of the managed instance group. It should conform to RFC1035.
    pub fn update_per_instance_configs(&self, request: RegionInstanceGroupManagerUpdateInstanceConfigReq, project: &str, region: &str, instance_group_manager: &str) -> RegionInstanceGroupManagerUpdatePerInstanceConfigCall<'a, S> {
        RegionInstanceGroupManagerUpdatePerInstanceConfigCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionInstanceGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `list_instances(...)` and `set_named_ports(...)`
/// // to build up your call.
/// let rb = hub.region_instance_groups();
/// # }
/// ```
pub struct RegionInstanceGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionInstanceGroupMethods<'a, S> {}

impl<'a, S> RegionInstanceGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified instance group resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroup` - Name of the instance group resource to return.
    pub fn get(&self, project: &str, region: &str, instance_group: &str) -> RegionInstanceGroupGetCall<'a, S> {
        RegionInstanceGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group: instance_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of instance group resources contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionInstanceGroupListCall<'a, S> {
        RegionInstanceGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the instances in the specified instance group and displays information about the named ports. Depending on the specified options, this method can list all instances or only the instances that are running. The orderBy query parameter is not supported.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroup` - Name of the regional instance group for which we want to list the instances.
    pub fn list_instances(&self, request: RegionInstanceGroupsListInstancesRequest, project: &str, region: &str, instance_group: &str) -> RegionInstanceGroupListInstanceCall<'a, S> {
        RegionInstanceGroupListInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group: instance_group.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the named ports for the specified regional instance group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `instanceGroup` - The name of the regional instance group where the named ports are updated.
    pub fn set_named_ports(&self, request: RegionInstanceGroupsSetNamedPortsRequest, project: &str, region: &str, instance_group: &str) -> RegionInstanceGroupSetNamedPortCall<'a, S> {
        RegionInstanceGroupSetNamedPortCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _instance_group: instance_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionInstance* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bulk_insert(...)`
/// // to build up your call.
/// let rb = hub.region_instances();
/// # }
/// ```
pub struct RegionInstanceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionInstanceMethods<'a, S> {}

impl<'a, S> RegionInstanceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates multiple instances in a given region. Count specifies the number of instances to create.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    pub fn bulk_insert(&self, request: BulkInsertInstanceResource, project: &str, region: &str) -> RegionInstanceBulkInsertCall<'a, S> {
        RegionInstanceBulkInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionNetworkEndpointGroup* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.region_network_endpoint_groups();
/// # }
/// ```
pub struct RegionNetworkEndpointGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionNetworkEndpointGroupMethods<'a, S> {}

impl<'a, S> RegionNetworkEndpointGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified network endpoint group. Note that the NEG cannot be deleted if it is configured as a backend of a backend service.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group to delete. It should comply with RFC1035.
    pub fn delete(&self, project: &str, region: &str, network_endpoint_group: &str) -> RegionNetworkEndpointGroupDeleteCall<'a, S> {
        RegionNetworkEndpointGroupDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network endpoint group. Gets a list of available network endpoint groups by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region where the network endpoint group is located. It should comply with RFC1035.
    /// * `networkEndpointGroup` - The name of the network endpoint group. It should comply with RFC1035.
    pub fn get(&self, project: &str, region: &str, network_endpoint_group: &str) -> RegionNetworkEndpointGroupGetCall<'a, S> {
        RegionNetworkEndpointGroupGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network_endpoint_group: network_endpoint_group.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a network endpoint group in the specified project using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region where you want to create the network endpoint group. It should comply with RFC1035.
    pub fn insert(&self, request: NetworkEndpointGroup, project: &str, region: &str) -> RegionNetworkEndpointGroupInsertCall<'a, S> {
        RegionNetworkEndpointGroupInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of regional network endpoint groups available to the specified project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region where the network endpoint group is located. It should comply with RFC1035.
    pub fn list(&self, project: &str, region: &str) -> RegionNetworkEndpointGroupListCall<'a, S> {
        RegionNetworkEndpointGroupListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionNetworkFirewallPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_association(...)`, `add_rule(...)`, `clone_rules(...)`, `delete(...)`, `get(...)`, `get_association(...)`, `get_effective_firewalls(...)`, `get_iam_policy(...)`, `get_rule(...)`, `insert(...)`, `list(...)`, `patch(...)`, `patch_rule(...)`, `remove_association(...)`, `remove_rule(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.region_network_firewall_policies();
/// # }
/// ```
pub struct RegionNetworkFirewallPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionNetworkFirewallPolicyMethods<'a, S> {}

impl<'a, S> RegionNetworkFirewallPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an association for the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_association(&self, request: FirewallPolicyAssociation, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyAddAssociationCall<'a, S> {
        RegionNetworkFirewallPolicyAddAssociationCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _replace_existing_association: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a rule into a network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn add_rule(&self, request: FirewallPolicyRule, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyAddRuleCall<'a, S> {
        RegionNetworkFirewallPolicyAddRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _min_priority: Default::default(),
            _max_priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies rules to the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn clone_rules(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyCloneRuleCall<'a, S> {
        RegionNetworkFirewallPolicyCloneRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _source_firewall_policy: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to delete.
    pub fn delete(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyDeleteCall<'a, S> {
        RegionNetworkFirewallPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to get.
    pub fn get(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyGetCall<'a, S> {
        RegionNetworkFirewallPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an association with the specified name.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to which the queried association belongs.
    pub fn get_association(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyGetAssociationCall<'a, S> {
        RegionNetworkFirewallPolicyGetAssociationCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the effective firewalls on a given network.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `network` - Network reference
    pub fn get_effective_firewalls(&self, project: &str, region: &str, network: &str) -> RegionNetworkFirewallPolicyGetEffectiveFirewallCall<'a, S> {
        RegionNetworkFirewallPolicyGetEffectiveFirewallCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _network: network.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> RegionNetworkFirewallPolicyGetIamPolicyCall<'a, S> {
        RegionNetworkFirewallPolicyGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to which the queried rule belongs.
    pub fn get_rule(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyGetRuleCall<'a, S> {
        RegionNetworkFirewallPolicyGetRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new network firewall policy in the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: FirewallPolicy, project: &str, region: &str) -> RegionNetworkFirewallPolicyInsertCall<'a, S> {
        RegionNetworkFirewallPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the network firewall policies that have been configured for the specified project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionNetworkFirewallPolicyListCall<'a, S> {
        RegionNetworkFirewallPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch(&self, request: FirewallPolicy, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyPatchCall<'a, S> {
        RegionNetworkFirewallPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn patch_rule(&self, request: FirewallPolicyRule, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyPatchRuleCall<'a, S> {
        RegionNetworkFirewallPolicyPatchRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an association for the specified network firewall policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_association(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyRemoveAssociationCall<'a, S> {
        RegionNetworkFirewallPolicyRemoveAssociationCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a rule of the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `firewallPolicy` - Name of the firewall policy to update.
    pub fn remove_rule(&self, project: &str, region: &str, firewall_policy: &str) -> RegionNetworkFirewallPolicyRemoveRuleCall<'a, S> {
        RegionNetworkFirewallPolicyRemoveRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _firewall_policy: firewall_policy.to_string(),
            _request_id: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> RegionNetworkFirewallPolicySetIamPolicyCall<'a, S> {
        RegionNetworkFirewallPolicySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> RegionNetworkFirewallPolicyTestIamPermissionCall<'a, S> {
        RegionNetworkFirewallPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionNotificationEndpoint* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.region_notification_endpoints();
/// # }
/// ```
pub struct RegionNotificationEndpointMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionNotificationEndpointMethods<'a, S> {}

impl<'a, S> RegionNotificationEndpointMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified NotificationEndpoint in the given region
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `notificationEndpoint` - Name of the NotificationEndpoint resource to delete.
    pub fn delete(&self, project: &str, region: &str, notification_endpoint: &str) -> RegionNotificationEndpointDeleteCall<'a, S> {
        RegionNotificationEndpointDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _notification_endpoint: notification_endpoint.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified NotificationEndpoint resource in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `notificationEndpoint` - Name of the NotificationEndpoint resource to return.
    pub fn get(&self, project: &str, region: &str, notification_endpoint: &str) -> RegionNotificationEndpointGetCall<'a, S> {
        RegionNotificationEndpointGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _notification_endpoint: notification_endpoint.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a NotificationEndpoint in the specified project in the given region using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: NotificationEndpoint, project: &str, region: &str) -> RegionNotificationEndpointInsertCall<'a, S> {
        RegionNotificationEndpointInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the NotificationEndpoints for a project in the given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionNotificationEndpointListCall<'a, S> {
        RegionNotificationEndpointListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionOperation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `wait(...)`
/// // to build up your call.
/// let rb = hub.region_operations();
/// # }
/// ```
pub struct RegionOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionOperationMethods<'a, S> {}

impl<'a, S> RegionOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified region-specific Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `operation` - Name of the Operations resource to delete.
    pub fn delete(&self, project: &str, region: &str, operation: &str) -> RegionOperationDeleteCall<'a, S> {
        RegionOperationDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified region-specific Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn get(&self, project: &str, region: &str, operation: &str) -> RegionOperationGetCall<'a, S> {
        RegionOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of Operation resources contained within the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> RegionOperationListCall<'a, S> {
        RegionOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method differs from the `GET` method in that it waits for no more than the default deadline (2 minutes) and then returns the current state of the operation, which might be `DONE` or still in progress. This method is called on a best-effort basis. Specifically: - In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. - If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn wait(&self, project: &str, region: &str, operation: &str) -> RegionOperationWaitCall<'a, S> {
        RegionOperationWaitCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionSecurityPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.region_security_policies();
/// # }
/// ```
pub struct RegionSecurityPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionSecurityPolicyMethods<'a, S> {}

impl<'a, S> RegionSecurityPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `securityPolicy` - Name of the security policy to delete.
    pub fn delete(&self, project: &str, region: &str, security_policy: &str) -> RegionSecurityPolicyDeleteCall<'a, S> {
        RegionSecurityPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _security_policy: security_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the ordered rules present in a single specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `securityPolicy` - Name of the security policy to get.
    pub fn get(&self, project: &str, region: &str, security_policy: &str) -> RegionSecurityPolicyGetCall<'a, S> {
        RegionSecurityPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _security_policy: security_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: SecurityPolicy, project: &str, region: &str) -> RegionSecurityPolicyInsertCall<'a, S> {
        RegionSecurityPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the policies that have been configured for the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionSecurityPolicyListCall<'a, S> {
        RegionSecurityPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified policy with the data included in the request. To clear fields in the rule, leave the fields empty and specify them in the updateMask. This cannot be used to be update the rules in the policy. Please use the per rule methods like addRule, patchRule, and removeRule instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `securityPolicy` - Name of the security policy to update.
    pub fn patch(&self, request: SecurityPolicy, project: &str, region: &str, security_policy: &str) -> RegionSecurityPolicyPatchCall<'a, S> {
        RegionSecurityPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _security_policy: security_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionSslCertificate* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.region_ssl_certificates();
/// # }
/// ```
pub struct RegionSslCertificateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionSslCertificateMethods<'a, S> {}

impl<'a, S> RegionSslCertificateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified SslCertificate resource in the region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `sslCertificate` - Name of the SslCertificate resource to delete.
    pub fn delete(&self, project: &str, region: &str, ssl_certificate: &str) -> RegionSslCertificateDeleteCall<'a, S> {
        RegionSslCertificateDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _ssl_certificate: ssl_certificate.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified SslCertificate resource in the specified region. Get a list of available SSL certificates by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `sslCertificate` - Name of the SslCertificate resource to return.
    pub fn get(&self, project: &str, region: &str, ssl_certificate: &str) -> RegionSslCertificateGetCall<'a, S> {
        RegionSslCertificateGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _ssl_certificate: ssl_certificate.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a SslCertificate resource in the specified project and region using the data included in the request
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: SslCertificate, project: &str, region: &str) -> RegionSslCertificateInsertCall<'a, S> {
        RegionSslCertificateInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of SslCertificate resources available to the specified project in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionSslCertificateListCall<'a, S> {
        RegionSslCertificateListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionSslPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_available_features(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.region_ssl_policies();
/// # }
/// ```
pub struct RegionSslPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionSslPolicyMethods<'a, S> {}

impl<'a, S> RegionSslPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified SSL policy. The SSL policy resource can be deleted only if it is not in use by any TargetHttpsProxy or TargetSslProxy resources.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `sslPolicy` - Name of the SSL policy to delete. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn delete(&self, project: &str, region: &str, ssl_policy: &str) -> RegionSslPolicyDeleteCall<'a, S> {
        RegionSslPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the ordered rules present in a single specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `sslPolicy` - Name of the SSL policy to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn get(&self, project: &str, region: &str, ssl_policy: &str) -> RegionSslPolicyGetCall<'a, S> {
        RegionSslPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: SslPolicy, project: &str, region: &str) -> RegionSslPolicyInsertCall<'a, S> {
        RegionSslPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the SSL policies that have been configured for the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionSslPolicyListCall<'a, S> {
        RegionSslPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all features that can be specified in the SSL policy when using custom profile.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list_available_features(&self, project: &str, region: &str) -> RegionSslPolicyListAvailableFeatureCall<'a, S> {
        RegionSslPolicyListAvailableFeatureCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified SSL policy with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `sslPolicy` - Name of the SSL policy to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn patch(&self, request: SslPolicy, project: &str, region: &str, ssl_policy: &str) -> RegionSslPolicyPatchCall<'a, S> {
        RegionSslPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionTargetHttpProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `set_url_map(...)`
/// // to build up your call.
/// let rb = hub.region_target_http_proxies();
/// # }
/// ```
pub struct RegionTargetHttpProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionTargetHttpProxyMethods<'a, S> {}

impl<'a, S> RegionTargetHttpProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetHttpProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy resource to delete.
    pub fn delete(&self, project: &str, region: &str, target_http_proxy: &str) -> RegionTargetHttpProxyDeleteCall<'a, S> {
        RegionTargetHttpProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetHttpProxy resource in the specified region. Gets a list of available target HTTP proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy resource to return.
    pub fn get(&self, project: &str, region: &str, target_http_proxy: &str) -> RegionTargetHttpProxyGetCall<'a, S> {
        RegionTargetHttpProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetHttpProxy resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: TargetHttpProxy, project: &str, region: &str) -> RegionTargetHttpProxyInsertCall<'a, S> {
        RegionTargetHttpProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetHttpProxy resources available to the specified project in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionTargetHttpProxyListCall<'a, S> {
        RegionTargetHttpProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the URL map for TargetHttpProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy to set a URL map for.
    pub fn set_url_map(&self, request: UrlMapReference, project: &str, region: &str, target_http_proxy: &str) -> RegionTargetHttpProxySetUrlMapCall<'a, S> {
        RegionTargetHttpProxySetUrlMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionTargetHttpsProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_ssl_certificates(...)` and `set_url_map(...)`
/// // to build up your call.
/// let rb = hub.region_target_https_proxies();
/// # }
/// ```
pub struct RegionTargetHttpsProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionTargetHttpsProxyMethods<'a, S> {}

impl<'a, S> RegionTargetHttpsProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetHttpsProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to delete.
    pub fn delete(&self, project: &str, region: &str, target_https_proxy: &str) -> RegionTargetHttpsProxyDeleteCall<'a, S> {
        RegionTargetHttpsProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetHttpsProxy resource in the specified region. Gets a list of available target HTTP proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to return.
    pub fn get(&self, project: &str, region: &str, target_https_proxy: &str) -> RegionTargetHttpsProxyGetCall<'a, S> {
        RegionTargetHttpsProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetHttpsProxy resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: TargetHttpsProxy, project: &str, region: &str) -> RegionTargetHttpsProxyInsertCall<'a, S> {
        RegionTargetHttpsProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetHttpsProxy resources available to the specified project in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionTargetHttpsProxyListCall<'a, S> {
        RegionTargetHttpsProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified regional TargetHttpsProxy resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to patch.
    pub fn patch(&self, request: TargetHttpsProxy, project: &str, region: &str, target_https_proxy: &str) -> RegionTargetHttpsProxyPatchCall<'a, S> {
        RegionTargetHttpsProxyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces SslCertificates for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to set an SslCertificates resource for.
    pub fn set_ssl_certificates(&self, request: RegionTargetHttpsProxiesSetSslCertificatesRequest, project: &str, region: &str, target_https_proxy: &str) -> RegionTargetHttpsProxySetSslCertificateCall<'a, S> {
        RegionTargetHttpsProxySetSslCertificateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the URL map for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy to set a URL map for.
    pub fn set_url_map(&self, request: UrlMapReference, project: &str, region: &str, target_https_proxy: &str) -> RegionTargetHttpsProxySetUrlMapCall<'a, S> {
        RegionTargetHttpsProxySetUrlMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionTargetTcpProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.region_target_tcp_proxies();
/// # }
/// ```
pub struct RegionTargetTcpProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionTargetTcpProxyMethods<'a, S> {}

impl<'a, S> RegionTargetTcpProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetTcpProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource to delete.
    pub fn delete(&self, project: &str, region: &str, target_tcp_proxy: &str) -> RegionTargetTcpProxyDeleteCall<'a, S> {
        RegionTargetTcpProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetTcpProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource to return.
    pub fn get(&self, project: &str, region: &str, target_tcp_proxy: &str) -> RegionTargetTcpProxyGetCall<'a, S> {
        RegionTargetTcpProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetTcpProxy resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: TargetTcpProxy, project: &str, region: &str) -> RegionTargetTcpProxyInsertCall<'a, S> {
        RegionTargetTcpProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of TargetTcpProxy resources available to the specified project in a given region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionTargetTcpProxyListCall<'a, S> {
        RegionTargetTcpProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *regionUrlMap* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `update(...)` and `validate(...)`
/// // to build up your call.
/// let rb = hub.region_url_maps();
/// # }
/// ```
pub struct RegionUrlMapMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionUrlMapMethods<'a, S> {}

impl<'a, S> RegionUrlMapMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified UrlMap resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `urlMap` - Name of the UrlMap resource to delete.
    pub fn delete(&self, project: &str, region: &str, url_map: &str) -> RegionUrlMapDeleteCall<'a, S> {
        RegionUrlMapDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified UrlMap resource. Gets a list of available URL maps by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `urlMap` - Name of the UrlMap resource to return.
    pub fn get(&self, project: &str, region: &str, url_map: &str) -> RegionUrlMapGetCall<'a, S> {
        RegionUrlMapGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _url_map: url_map.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a UrlMap resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: UrlMap, project: &str, region: &str) -> RegionUrlMapInsertCall<'a, S> {
        RegionUrlMapInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of UrlMap resources available to the specified project in the specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> RegionUrlMapListCall<'a, S> {
        RegionUrlMapListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified UrlMap resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `urlMap` - Name of the UrlMap resource to patch.
    pub fn patch(&self, request: UrlMap, project: &str, region: &str, url_map: &str) -> RegionUrlMapPatchCall<'a, S> {
        RegionUrlMapPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified UrlMap resource with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `urlMap` - Name of the UrlMap resource to update.
    pub fn update(&self, request: UrlMap, project: &str, region: &str, url_map: &str) -> RegionUrlMapUpdateCall<'a, S> {
        RegionUrlMapUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs static validation for the UrlMap. In particular, the tests of the provided UrlMap will be run. Calling this method does NOT create the UrlMap.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `urlMap` - Name of the UrlMap resource to be validated as.
    pub fn validate(&self, request: RegionUrlMapsValidateRequest, project: &str, region: &str, url_map: &str) -> RegionUrlMapValidateCall<'a, S> {
        RegionUrlMapValidateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _url_map: url_map.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *region* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.regions();
/// # }
/// ```
pub struct RegionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RegionMethods<'a, S> {}

impl<'a, S> RegionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Region resource. Gets a list of available regions by making a list() request. To decrease latency for this method, you can optionally omit any unneeded information from the response by using a field mask. This practice is especially recommended for unused quota information (the `quotas` field). To exclude one or more fields, set your request's `fields` query parameter to only include the fields you need. For example, to only include the `id` and `selfLink` fields, add the query parameter `?fields=id,selfLink` to your request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region resource to return.
    pub fn get(&self, project: &str, region: &str) -> RegionGetCall<'a, S> {
        RegionGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of region resources available to the specified project. To decrease latency for this method, you can optionally omit any unneeded information from the response by using a field mask. This practice is especially recommended for unused quota information (the `items.quotas` field). To exclude one or more fields, set your request's `fields` query parameter to only include the fields you need. For example, to only include the `id` and `selfLink` fields, add the query parameter `?fields=id,selfLink` to your request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> RegionListCall<'a, S> {
        RegionListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *reservation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `resize(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.reservations();
/// # }
/// ```
pub struct ReservationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ReservationMethods<'a, S> {}

impl<'a, S> ReservationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of reservations.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> ReservationAggregatedListCall<'a, S> {
        ReservationAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified reservation.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `reservation` - Name of the reservation to delete.
    pub fn delete(&self, project: &str, zone: &str, reservation: &str) -> ReservationDeleteCall<'a, S> {
        ReservationDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _reservation: reservation.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information about the specified reservation.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `reservation` - Name of the reservation to retrieve.
    pub fn get(&self, project: &str, zone: &str, reservation: &str) -> ReservationGetCall<'a, S> {
        ReservationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _reservation: reservation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, zone: &str, resource: &str) -> ReservationGetIamPolicyCall<'a, S> {
        ReservationGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new reservation. For more information, read Reserving zonal resources.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn insert(&self, request: Reservation, project: &str, zone: &str) -> ReservationInsertCall<'a, S> {
        ReservationInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// A list of all the reservations that have been configured for the specified project in specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    pub fn list(&self, project: &str, zone: &str) -> ReservationListCall<'a, S> {
        ReservationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the reservation (applicable to standalone reservations only). For more information, read Modifying reservations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `reservation` - Name of the reservation to update.
    pub fn resize(&self, request: ReservationsResizeRequest, project: &str, zone: &str, reservation: &str) -> ReservationResizeCall<'a, S> {
        ReservationResizeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _reservation: reservation.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: ZoneSetPolicyRequest, project: &str, zone: &str, resource: &str) -> ReservationSetIamPolicyCall<'a, S> {
        ReservationSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - The name of the zone for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, zone: &str, resource: &str) -> ReservationTestIamPermissionCall<'a, S> {
        ReservationTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update share settings of the reservation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `reservation` - Name of the reservation to update.
    pub fn update(&self, request: Reservation, project: &str, zone: &str, reservation: &str) -> ReservationUpdateCall<'a, S> {
        ReservationUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _reservation: reservation.to_string(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _paths: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *resourcePolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.resource_policies();
/// # }
/// ```
pub struct ResourcePolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ResourcePolicyMethods<'a, S> {}

impl<'a, S> ResourcePolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of resource policies.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> ResourcePolicyAggregatedListCall<'a, S> {
        ResourcePolicyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified resource policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `resourcePolicy` - Name of the resource policy to delete.
    pub fn delete(&self, project: &str, region: &str, resource_policy: &str) -> ResourcePolicyDeleteCall<'a, S> {
        ResourcePolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource_policy: resource_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves all information of the specified resource policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `resourcePolicy` - Name of the resource policy to retrieve.
    pub fn get(&self, project: &str, region: &str, resource_policy: &str) -> ResourcePolicyGetCall<'a, S> {
        ResourcePolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource_policy: resource_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> ResourcePolicyGetIamPolicyCall<'a, S> {
        ResourcePolicyGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new resource policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: ResourcePolicy, project: &str, region: &str) -> ResourcePolicyInsertCall<'a, S> {
        ResourcePolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// A list all the resource policies that have been configured for the specified project in specified region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> ResourcePolicyListCall<'a, S> {
        ResourcePolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> ResourcePolicySetIamPolicyCall<'a, S> {
        ResourcePolicySetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> ResourcePolicyTestIamPermissionCall<'a, S> {
        ResourcePolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *router* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_nat_mapping_info(...)`, `get_router_status(...)`, `insert(...)`, `list(...)`, `patch(...)`, `preview(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.routers();
/// # }
/// ```
pub struct RouterMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RouterMethods<'a, S> {}

impl<'a, S> RouterMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of routers.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> RouterAggregatedListCall<'a, S> {
        RouterAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Router resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to delete.
    pub fn delete(&self, project: &str, region: &str, router: &str) -> RouterDeleteCall<'a, S> {
        RouterDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Router resource. Gets a list of available routers by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to return.
    pub fn get(&self, project: &str, region: &str, router: &str) -> RouterGetCall<'a, S> {
        RouterGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves runtime Nat mapping information of VM endpoints.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to query for Nat Mapping information of VM endpoints.
    pub fn get_nat_mapping_info(&self, project: &str, region: &str, router: &str) -> RouterGetNatMappingInfoCall<'a, S> {
        RouterGetNatMappingInfoCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves runtime information of the specified router.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to query.
    pub fn get_router_status(&self, project: &str, region: &str, router: &str) -> RouterGetRouterStatuCall<'a, S> {
        RouterGetRouterStatuCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Router resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: Router, project: &str, region: &str) -> RouterInsertCall<'a, S> {
        RouterInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of Router resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> RouterListCall<'a, S> {
        RouterListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified Router resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to patch.
    pub fn patch(&self, request: Router, project: &str, region: &str, router: &str) -> RouterPatchCall<'a, S> {
        RouterPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Preview fields auto-generated during router create and update operations. Calling this method does NOT create or update the router.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to query.
    pub fn preview(&self, request: Router, project: &str, region: &str, router: &str) -> RouterPreviewCall<'a, S> {
        RouterPreviewCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Router resource with the data included in the request. This method conforms to PUT semantics, which requests that the state of the target resource be created or replaced with the state defined by the representation enclosed in the request message payload.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `router` - Name of the Router resource to update.
    pub fn update(&self, request: Router, project: &str, region: &str, router: &str) -> RouterUpdateCall<'a, S> {
        RouterUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _router: router.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *route* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.routes();
/// # }
/// ```
pub struct RouteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for RouteMethods<'a, S> {}

impl<'a, S> RouteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Route resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `route` - Name of the Route resource to delete.
    pub fn delete(&self, project: &str, route: &str) -> RouteDeleteCall<'a, S> {
        RouteDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _route: route.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Route resource. Gets a list of available routes by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `route` - Name of the Route resource to return.
    pub fn get(&self, project: &str, route: &str) -> RouteGetCall<'a, S> {
        RouteGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _route: route.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Route resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Route, project: &str) -> RouteInsertCall<'a, S> {
        RouteInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Route resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> RouteListCall<'a, S> {
        RouteListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *securityPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_rule(...)`, `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_rule(...)`, `insert(...)`, `list(...)`, `list_preconfigured_expression_sets(...)`, `patch(...)`, `patch_rule(...)`, `remove_rule(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.security_policies();
/// # }
/// ```
pub struct SecurityPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for SecurityPolicyMethods<'a, S> {}

impl<'a, S> SecurityPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a rule into a security policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to update.
    pub fn add_rule(&self, request: SecurityPolicyRule, project: &str, security_policy: &str) -> SecurityPolicyAddRuleCall<'a, S> {
        SecurityPolicyAddRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all SecurityPolicy resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> SecurityPolicyAggregatedListCall<'a, S> {
        SecurityPolicyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to delete.
    pub fn delete(&self, project: &str, security_policy: &str) -> SecurityPolicyDeleteCall<'a, S> {
        SecurityPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the ordered rules present in a single specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to get.
    pub fn get(&self, project: &str, security_policy: &str) -> SecurityPolicyGetCall<'a, S> {
        SecurityPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a rule at the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to which the queried rule belongs.
    pub fn get_rule(&self, project: &str, security_policy: &str) -> SecurityPolicyGetRuleCall<'a, S> {
        SecurityPolicyGetRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new policy in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: SecurityPolicy, project: &str) -> SecurityPolicyInsertCall<'a, S> {
        SecurityPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the policies that have been configured for the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> SecurityPolicyListCall<'a, S> {
        SecurityPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current list of preconfigured Web Application Firewall (WAF) expressions.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list_preconfigured_expression_sets(&self, project: &str) -> SecurityPolicyListPreconfiguredExpressionSetCall<'a, S> {
        SecurityPolicyListPreconfiguredExpressionSetCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified policy with the data included in the request. To clear fields in the rule, leave the fields empty and specify them in the updateMask. This cannot be used to be update the rules in the policy. Please use the per rule methods like addRule, patchRule, and removeRule instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to update.
    pub fn patch(&self, request: SecurityPolicy, project: &str, security_policy: &str) -> SecurityPolicyPatchCall<'a, S> {
        SecurityPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a rule at the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to update.
    pub fn patch_rule(&self, request: SecurityPolicyRule, project: &str, security_policy: &str) -> SecurityPolicyPatchRuleCall<'a, S> {
        SecurityPolicyPatchRuleCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _validate_only: Default::default(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a rule at the specified priority.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `securityPolicy` - Name of the security policy to update.
    pub fn remove_rule(&self, project: &str, security_policy: &str) -> SecurityPolicyRemoveRuleCall<'a, S> {
        SecurityPolicyRemoveRuleCall {
            hub: self.hub,
            _project: project.to_string(),
            _security_policy: security_policy.to_string(),
            _priority: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a security policy. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> SecurityPolicySetLabelCall<'a, S> {
        SecurityPolicySetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *serviceAttachment* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.service_attachments();
/// # }
/// ```
pub struct ServiceAttachmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ServiceAttachmentMethods<'a, S> {}

impl<'a, S> ServiceAttachmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all ServiceAttachment resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> ServiceAttachmentAggregatedListCall<'a, S> {
        ServiceAttachmentAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified ServiceAttachment in the given scope
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `serviceAttachment` - Name of the ServiceAttachment resource to delete.
    pub fn delete(&self, project: &str, region: &str, service_attachment: &str) -> ServiceAttachmentDeleteCall<'a, S> {
        ServiceAttachmentDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _service_attachment: service_attachment.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified ServiceAttachment resource in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    /// * `serviceAttachment` - Name of the ServiceAttachment resource to return.
    pub fn get(&self, project: &str, region: &str, service_attachment: &str) -> ServiceAttachmentGetCall<'a, S> {
        ServiceAttachmentGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _service_attachment: service_attachment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> ServiceAttachmentGetIamPolicyCall<'a, S> {
        ServiceAttachmentGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a ServiceAttachment in the specified project in the given scope using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn insert(&self, request: ServiceAttachment, project: &str, region: &str) -> ServiceAttachmentInsertCall<'a, S> {
        ServiceAttachmentInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the ServiceAttachments for a project in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region of this request.
    pub fn list(&self, project: &str, region: &str) -> ServiceAttachmentListCall<'a, S> {
        ServiceAttachmentListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified ServiceAttachment resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region scoping this request and should conform to RFC1035.
    /// * `serviceAttachment` - The resource id of the ServiceAttachment to patch. It should conform to RFC1035 resource name or be a string form on an unsigned long number.
    pub fn patch(&self, request: ServiceAttachment, project: &str, region: &str, service_attachment: &str) -> ServiceAttachmentPatchCall<'a, S> {
        ServiceAttachmentPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _service_attachment: service_attachment.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> ServiceAttachmentSetIamPolicyCall<'a, S> {
        ServiceAttachmentSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> ServiceAttachmentTestIamPermissionCall<'a, S> {
        ServiceAttachmentTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *snapshot* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `set_iam_policy(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.snapshots();
/// # }
/// ```
pub struct SnapshotMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for SnapshotMethods<'a, S> {}

impl<'a, S> SnapshotMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified Snapshot resource. Keep in mind that deleting a single snapshot might not necessarily delete all the data on that snapshot. If any data on the snapshot that is marked for deletion is needed for subsequent snapshots, the data will be moved to the next corresponding snapshot. For more information, see Deleting snapshots.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `snapshot` - Name of the Snapshot resource to delete.
    pub fn delete(&self, project: &str, snapshot: &str) -> SnapshotDeleteCall<'a, S> {
        SnapshotDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _snapshot: snapshot.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Snapshot resource. Gets a list of available snapshots by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `snapshot` - Name of the Snapshot resource to return.
    pub fn get(&self, project: &str, snapshot: &str) -> SnapshotGetCall<'a, S> {
        SnapshotGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _snapshot: snapshot.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, resource: &str) -> SnapshotGetIamPolicyCall<'a, S> {
        SnapshotGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a snapshot in the specified project using the data included in the request. For regular snapshot creation, consider using this method instead of disks.createSnapshot, as this method supports more features, such as creating snapshots in a project different from the source disk project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: Snapshot, project: &str) -> SnapshotInsertCall<'a, S> {
        SnapshotInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Snapshot resources contained within the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> SnapshotListCall<'a, S> {
        SnapshotListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: GlobalSetPolicyRequest, project: &str, resource: &str) -> SnapshotSetIamPolicyCall<'a, S> {
        SnapshotSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a snapshot. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: GlobalSetLabelsRequest, project: &str, resource: &str) -> SnapshotSetLabelCall<'a, S> {
        SnapshotSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, resource: &str) -> SnapshotTestIamPermissionCall<'a, S> {
        SnapshotTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sslCertificate* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.ssl_certificates();
/// # }
/// ```
pub struct SslCertificateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for SslCertificateMethods<'a, S> {}

impl<'a, S> SslCertificateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all SslCertificate resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> SslCertificateAggregatedListCall<'a, S> {
        SslCertificateAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified SslCertificate resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `sslCertificate` - Name of the SslCertificate resource to delete.
    pub fn delete(&self, project: &str, ssl_certificate: &str) -> SslCertificateDeleteCall<'a, S> {
        SslCertificateDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _ssl_certificate: ssl_certificate.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified SslCertificate resource. Gets a list of available SSL certificates by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `sslCertificate` - Name of the SslCertificate resource to return.
    pub fn get(&self, project: &str, ssl_certificate: &str) -> SslCertificateGetCall<'a, S> {
        SslCertificateGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _ssl_certificate: ssl_certificate.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a SslCertificate resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: SslCertificate, project: &str) -> SslCertificateInsertCall<'a, S> {
        SslCertificateInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of SslCertificate resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> SslCertificateListCall<'a, S> {
        SslCertificateListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sslPolicy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_available_features(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.ssl_policies();
/// # }
/// ```
pub struct SslPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for SslPolicyMethods<'a, S> {}

impl<'a, S> SslPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all SslPolicy resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> SslPolicyAggregatedListCall<'a, S> {
        SslPolicyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified SSL policy. The SSL policy resource can be deleted only if it is not in use by any TargetHttpsProxy or TargetSslProxy resources.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `sslPolicy` - Name of the SSL policy to delete. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn delete(&self, project: &str, ssl_policy: &str) -> SslPolicyDeleteCall<'a, S> {
        SslPolicyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the ordered rules present in a single specified policy.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `sslPolicy` - Name of the SSL policy to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn get(&self, project: &str, ssl_policy: &str) -> SslPolicyGetCall<'a, S> {
        SslPolicyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified SSL policy resource. Gets a list of available SSL policies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: SslPolicy, project: &str) -> SslPolicyInsertCall<'a, S> {
        SslPolicyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the SSL policies that have been configured for the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> SslPolicyListCall<'a, S> {
        SslPolicyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all features that can be specified in the SSL policy when using custom profile.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list_available_features(&self, project: &str) -> SslPolicyListAvailableFeatureCall<'a, S> {
        SslPolicyListAvailableFeatureCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified SSL policy with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `sslPolicy` - Name of the SSL policy to update. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn patch(&self, request: SslPolicy, project: &str, ssl_policy: &str) -> SslPolicyPatchCall<'a, S> {
        SslPolicyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _ssl_policy: ssl_policy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subnetwork* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `expand_ip_cidr_range(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `list_usable(...)`, `patch(...)`, `set_iam_policy(...)`, `set_private_ip_google_access(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.subnetworks();
/// # }
/// ```
pub struct SubnetworkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for SubnetworkMethods<'a, S> {}

impl<'a, S> SubnetworkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of subnetworks.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> SubnetworkAggregatedListCall<'a, S> {
        SubnetworkAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified subnetwork.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `subnetwork` - Name of the Subnetwork resource to delete.
    pub fn delete(&self, project: &str, region: &str, subnetwork: &str) -> SubnetworkDeleteCall<'a, S> {
        SubnetworkDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _subnetwork: subnetwork.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Expands the IP CIDR range of the subnetwork to a specified value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `subnetwork` - Name of the Subnetwork resource to update.
    pub fn expand_ip_cidr_range(&self, request: SubnetworksExpandIpCidrRangeRequest, project: &str, region: &str, subnetwork: &str) -> SubnetworkExpandIpCidrRangeCall<'a, S> {
        SubnetworkExpandIpCidrRangeCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _subnetwork: subnetwork.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified subnetwork. Gets a list of available subnetworks list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `subnetwork` - Name of the Subnetwork resource to return.
    pub fn get(&self, project: &str, region: &str, subnetwork: &str) -> SubnetworkGetCall<'a, S> {
        SubnetworkGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _subnetwork: subnetwork.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. May be empty if no such policy or resource exists.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn get_iam_policy(&self, project: &str, region: &str, resource: &str) -> SubnetworkGetIamPolicyCall<'a, S> {
        SubnetworkGetIamPolicyCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a subnetwork in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: Subnetwork, project: &str, region: &str) -> SubnetworkInsertCall<'a, S> {
        SubnetworkInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of subnetworks available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> SubnetworkListCall<'a, S> {
        SubnetworkListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of all usable subnetworks in the project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list_usable(&self, project: &str) -> SubnetworkListUsableCall<'a, S> {
        SubnetworkListUsableCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified subnetwork with the data included in the request. Only certain fields can be updated with a patch request as indicated in the field descriptions. You must specify the current fingerprint of the subnetwork resource being patched.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `subnetwork` - Name of the Subnetwork resource to patch.
    pub fn patch(&self, request: Subnetwork, project: &str, region: &str, subnetwork: &str) -> SubnetworkPatchCall<'a, S> {
        SubnetworkPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _subnetwork: subnetwork.to_string(),
            _request_id: Default::default(),
            _drain_timeout_seconds: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_iam_policy(&self, request: RegionSetPolicyRequest, project: &str, region: &str, resource: &str) -> SubnetworkSetIamPolicyCall<'a, S> {
        SubnetworkSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set whether VMs in this subnet can access Google services without assigning external IP addresses through Private Google Access.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `subnetwork` - Name of the Subnetwork resource.
    pub fn set_private_ip_google_access(&self, request: SubnetworksSetPrivateIpGoogleAccessRequest, project: &str, region: &str, subnetwork: &str) -> SubnetworkSetPrivateIpGoogleAccesCall<'a, S> {
        SubnetworkSetPrivateIpGoogleAccesCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _subnetwork: subnetwork.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> SubnetworkTestIamPermissionCall<'a, S> {
        SubnetworkTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetGrpcProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.target_grpc_proxies();
/// # }
/// ```
pub struct TargetGrpcProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetGrpcProxyMethods<'a, S> {}

impl<'a, S> TargetGrpcProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetGrpcProxy in the given scope
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetGrpcProxy` - Name of the TargetGrpcProxy resource to delete.
    pub fn delete(&self, project: &str, target_grpc_proxy: &str) -> TargetGrpcProxyDeleteCall<'a, S> {
        TargetGrpcProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_grpc_proxy: target_grpc_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetGrpcProxy resource in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetGrpcProxy` - Name of the TargetGrpcProxy resource to return.
    pub fn get(&self, project: &str, target_grpc_proxy: &str) -> TargetGrpcProxyGetCall<'a, S> {
        TargetGrpcProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_grpc_proxy: target_grpc_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetGrpcProxy in the specified project in the given scope using the parameters that are included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: TargetGrpcProxy, project: &str) -> TargetGrpcProxyInsertCall<'a, S> {
        TargetGrpcProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the TargetGrpcProxies for a project in the given scope.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> TargetGrpcProxyListCall<'a, S> {
        TargetGrpcProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified TargetGrpcProxy resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetGrpcProxy` - Name of the TargetGrpcProxy resource to patch.
    pub fn patch(&self, request: TargetGrpcProxy, project: &str, target_grpc_proxy: &str) -> TargetGrpcProxyPatchCall<'a, S> {
        TargetGrpcProxyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_grpc_proxy: target_grpc_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetHttpProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `set_url_map(...)`
/// // to build up your call.
/// let rb = hub.target_http_proxies();
/// # }
/// ```
pub struct TargetHttpProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetHttpProxyMethods<'a, S> {}

impl<'a, S> TargetHttpProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all TargetHttpProxy resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> TargetHttpProxyAggregatedListCall<'a, S> {
        TargetHttpProxyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetHttpProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy resource to delete.
    pub fn delete(&self, project: &str, target_http_proxy: &str) -> TargetHttpProxyDeleteCall<'a, S> {
        TargetHttpProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetHttpProxy resource. Gets a list of available target HTTP proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy resource to return.
    pub fn get(&self, project: &str, target_http_proxy: &str) -> TargetHttpProxyGetCall<'a, S> {
        TargetHttpProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetHttpProxy resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: TargetHttpProxy, project: &str) -> TargetHttpProxyInsertCall<'a, S> {
        TargetHttpProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetHttpProxy resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> TargetHttpProxyListCall<'a, S> {
        TargetHttpProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified TargetHttpProxy resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy resource to patch.
    pub fn patch(&self, request: TargetHttpProxy, project: &str, target_http_proxy: &str) -> TargetHttpProxyPatchCall<'a, S> {
        TargetHttpProxyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the URL map for TargetHttpProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpProxy` - Name of the TargetHttpProxy to set a URL map for.
    pub fn set_url_map(&self, request: UrlMapReference, project: &str, target_http_proxy: &str) -> TargetHttpProxySetUrlMapCall<'a, S> {
        TargetHttpProxySetUrlMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_http_proxy: target_http_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetHttpsProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_certificate_map(...)`, `set_quic_override(...)`, `set_ssl_certificates(...)`, `set_ssl_policy(...)` and `set_url_map(...)`
/// // to build up your call.
/// let rb = hub.target_https_proxies();
/// # }
/// ```
pub struct TargetHttpsProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetHttpsProxyMethods<'a, S> {}

impl<'a, S> TargetHttpsProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all TargetHttpsProxy resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> TargetHttpsProxyAggregatedListCall<'a, S> {
        TargetHttpsProxyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetHttpsProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to delete.
    pub fn delete(&self, project: &str, target_https_proxy: &str) -> TargetHttpsProxyDeleteCall<'a, S> {
        TargetHttpsProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetHttpsProxy resource. Gets a list of available target HTTPS proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to return.
    pub fn get(&self, project: &str, target_https_proxy: &str) -> TargetHttpsProxyGetCall<'a, S> {
        TargetHttpsProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetHttpsProxy resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: TargetHttpsProxy, project: &str) -> TargetHttpsProxyInsertCall<'a, S> {
        TargetHttpsProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetHttpsProxy resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> TargetHttpsProxyListCall<'a, S> {
        TargetHttpsProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified TargetHttpsProxy resource with the data included in the request. This method supports PATCH semantics and uses JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to patch.
    pub fn patch(&self, request: TargetHttpsProxy, project: &str, target_https_proxy: &str) -> TargetHttpsProxyPatchCall<'a, S> {
        TargetHttpsProxyPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the Certificate Map for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource whose CertificateMap is to be set. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn set_certificate_map(&self, request: TargetHttpsProxiesSetCertificateMapRequest, project: &str, target_https_proxy: &str) -> TargetHttpsProxySetCertificateMapCall<'a, S> {
        TargetHttpsProxySetCertificateMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the QUIC override policy for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to set the QUIC override policy for. The name should conform to RFC1035.
    pub fn set_quic_override(&self, request: TargetHttpsProxiesSetQuicOverrideRequest, project: &str, target_https_proxy: &str) -> TargetHttpsProxySetQuicOverrideCall<'a, S> {
        TargetHttpsProxySetQuicOverrideCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces SslCertificates for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource to set an SslCertificates resource for.
    pub fn set_ssl_certificates(&self, request: TargetHttpsProxiesSetSslCertificatesRequest, project: &str, target_https_proxy: &str) -> TargetHttpsProxySetSslCertificateCall<'a, S> {
        TargetHttpsProxySetSslCertificateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the SSL policy for TargetHttpsProxy. The SSL policy specifies the server-side support for SSL features. This affects connections between clients and the HTTPS proxy load balancer. They do not affect the connection between the load balancer and the backends.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource whose SSL policy is to be set. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn set_ssl_policy(&self, request: SslPolicyReference, project: &str, target_https_proxy: &str) -> TargetHttpsProxySetSslPolicyCall<'a, S> {
        TargetHttpsProxySetSslPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the URL map for TargetHttpsProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetHttpsProxy` - Name of the TargetHttpsProxy resource whose URL map is to be set.
    pub fn set_url_map(&self, request: UrlMapReference, project: &str, target_https_proxy: &str) -> TargetHttpsProxySetUrlMapCall<'a, S> {
        TargetHttpsProxySetUrlMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_https_proxy: target_https_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetInstance* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.target_instances();
/// # }
/// ```
pub struct TargetInstanceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetInstanceMethods<'a, S> {}

impl<'a, S> TargetInstanceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of target instances.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> TargetInstanceAggregatedListCall<'a, S> {
        TargetInstanceAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetInstance resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone scoping this request.
    /// * `targetInstance` - Name of the TargetInstance resource to delete.
    pub fn delete(&self, project: &str, zone: &str, target_instance: &str) -> TargetInstanceDeleteCall<'a, S> {
        TargetInstanceDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _target_instance: target_instance.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetInstance resource. Gets a list of available target instances by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone scoping this request.
    /// * `targetInstance` - Name of the TargetInstance resource to return.
    pub fn get(&self, project: &str, zone: &str, target_instance: &str) -> TargetInstanceGetCall<'a, S> {
        TargetInstanceGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _target_instance: target_instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetInstance resource in the specified project and zone using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone scoping this request.
    pub fn insert(&self, request: TargetInstance, project: &str, zone: &str) -> TargetInstanceInsertCall<'a, S> {
        TargetInstanceInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of TargetInstance resources available to the specified project and zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone scoping this request.
    pub fn list(&self, project: &str, zone: &str) -> TargetInstanceListCall<'a, S> {
        TargetInstanceListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetPool* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_health_check(...)`, `add_instance(...)`, `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_health(...)`, `insert(...)`, `list(...)`, `remove_health_check(...)`, `remove_instance(...)` and `set_backup(...)`
/// // to build up your call.
/// let rb = hub.target_pools();
/// # }
/// ```
pub struct TargetPoolMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetPoolMethods<'a, S> {}

impl<'a, S> TargetPoolMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds health check URLs to a target pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the target pool to add a health check to.
    pub fn add_health_check(&self, request: TargetPoolsAddHealthCheckRequest, project: &str, region: &str, target_pool: &str) -> TargetPoolAddHealthCheckCall<'a, S> {
        TargetPoolAddHealthCheckCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an instance to a target pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to add instances to.
    pub fn add_instance(&self, request: TargetPoolsAddInstanceRequest, project: &str, region: &str, target_pool: &str) -> TargetPoolAddInstanceCall<'a, S> {
        TargetPoolAddInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of target pools.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> TargetPoolAggregatedListCall<'a, S> {
        TargetPoolAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified target pool.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to delete.
    pub fn delete(&self, project: &str, region: &str, target_pool: &str) -> TargetPoolDeleteCall<'a, S> {
        TargetPoolDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified target pool. Gets a list of available target pools by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to return.
    pub fn get(&self, project: &str, region: &str, target_pool: &str) -> TargetPoolGetCall<'a, S> {
        TargetPoolGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the most recent health check results for each IP for the instance that is referenced by the given target pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to which the queried instance belongs.
    pub fn get_health(&self, request: InstanceReference, project: &str, region: &str, target_pool: &str) -> TargetPoolGetHealthCall<'a, S> {
        TargetPoolGetHealthCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a target pool in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn insert(&self, request: TargetPool, project: &str, region: &str) -> TargetPoolInsertCall<'a, S> {
        TargetPoolInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of target pools available to the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    pub fn list(&self, project: &str, region: &str) -> TargetPoolListCall<'a, S> {
        TargetPoolListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes health check URL from a target pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `targetPool` - Name of the target pool to remove health checks from.
    pub fn remove_health_check(&self, request: TargetPoolsRemoveHealthCheckRequest, project: &str, region: &str, target_pool: &str) -> TargetPoolRemoveHealthCheckCall<'a, S> {
        TargetPoolRemoveHealthCheckCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes instance URL from a target pool.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to remove instances from.
    pub fn remove_instance(&self, request: TargetPoolsRemoveInstanceRequest, project: &str, region: &str, target_pool: &str) -> TargetPoolRemoveInstanceCall<'a, S> {
        TargetPoolRemoveInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes a backup target pool's configurations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region scoping this request.
    /// * `targetPool` - Name of the TargetPool resource to set a backup pool for.
    pub fn set_backup(&self, request: TargetReference, project: &str, region: &str, target_pool: &str) -> TargetPoolSetBackupCall<'a, S> {
        TargetPoolSetBackupCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_pool: target_pool.to_string(),
            _request_id: Default::default(),
            _failover_ratio: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetSslProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `set_backend_service(...)`, `set_certificate_map(...)`, `set_proxy_header(...)`, `set_ssl_certificates(...)` and `set_ssl_policy(...)`
/// // to build up your call.
/// let rb = hub.target_ssl_proxies();
/// # }
/// ```
pub struct TargetSslProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetSslProxyMethods<'a, S> {}

impl<'a, S> TargetSslProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetSslProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource to delete.
    pub fn delete(&self, project: &str, target_ssl_proxy: &str) -> TargetSslProxyDeleteCall<'a, S> {
        TargetSslProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetSslProxy resource. Gets a list of available target SSL proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource to return.
    pub fn get(&self, project: &str, target_ssl_proxy: &str) -> TargetSslProxyGetCall<'a, S> {
        TargetSslProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetSslProxy resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: TargetSslProxy, project: &str) -> TargetSslProxyInsertCall<'a, S> {
        TargetSslProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetSslProxy resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> TargetSslProxyListCall<'a, S> {
        TargetSslProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the BackendService for TargetSslProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource whose BackendService resource is to be set.
    pub fn set_backend_service(&self, request: TargetSslProxiesSetBackendServiceRequest, project: &str, target_ssl_proxy: &str) -> TargetSslProxySetBackendServiceCall<'a, S> {
        TargetSslProxySetBackendServiceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the Certificate Map for TargetSslProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource whose CertificateMap is to be set. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn set_certificate_map(&self, request: TargetSslProxiesSetCertificateMapRequest, project: &str, target_ssl_proxy: &str) -> TargetSslProxySetCertificateMapCall<'a, S> {
        TargetSslProxySetCertificateMapCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the ProxyHeaderType for TargetSslProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource whose ProxyHeader is to be set.
    pub fn set_proxy_header(&self, request: TargetSslProxiesSetProxyHeaderRequest, project: &str, target_ssl_proxy: &str) -> TargetSslProxySetProxyHeaderCall<'a, S> {
        TargetSslProxySetProxyHeaderCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes SslCertificates for TargetSslProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource whose SslCertificate resource is to be set.
    pub fn set_ssl_certificates(&self, request: TargetSslProxiesSetSslCertificatesRequest, project: &str, target_ssl_proxy: &str) -> TargetSslProxySetSslCertificateCall<'a, S> {
        TargetSslProxySetSslCertificateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the SSL policy for TargetSslProxy. The SSL policy specifies the server-side support for SSL features. This affects connections between clients and the SSL proxy load balancer. They do not affect the connection between the load balancer and the backends.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetSslProxy` - Name of the TargetSslProxy resource whose SSL policy is to be set. The name must be 1-63 characters long, and comply with RFC1035.
    pub fn set_ssl_policy(&self, request: SslPolicyReference, project: &str, target_ssl_proxy: &str) -> TargetSslProxySetSslPolicyCall<'a, S> {
        TargetSslProxySetSslPolicyCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_ssl_proxy: target_ssl_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetTcpProxy* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `set_backend_service(...)` and `set_proxy_header(...)`
/// // to build up your call.
/// let rb = hub.target_tcp_proxies();
/// # }
/// ```
pub struct TargetTcpProxyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetTcpProxyMethods<'a, S> {}

impl<'a, S> TargetTcpProxyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all TargetTcpProxy resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> TargetTcpProxyAggregatedListCall<'a, S> {
        TargetTcpProxyAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified TargetTcpProxy resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource to delete.
    pub fn delete(&self, project: &str, target_tcp_proxy: &str) -> TargetTcpProxyDeleteCall<'a, S> {
        TargetTcpProxyDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified TargetTcpProxy resource. Gets a list of available target TCP proxies by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource to return.
    pub fn get(&self, project: &str, target_tcp_proxy: &str) -> TargetTcpProxyGetCall<'a, S> {
        TargetTcpProxyGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetTcpProxy resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: TargetTcpProxy, project: &str) -> TargetTcpProxyInsertCall<'a, S> {
        TargetTcpProxyInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of TargetTcpProxy resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> TargetTcpProxyListCall<'a, S> {
        TargetTcpProxyListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the BackendService for TargetTcpProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource whose BackendService resource is to be set.
    pub fn set_backend_service(&self, request: TargetTcpProxiesSetBackendServiceRequest, project: &str, target_tcp_proxy: &str) -> TargetTcpProxySetBackendServiceCall<'a, S> {
        TargetTcpProxySetBackendServiceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the ProxyHeaderType for TargetTcpProxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `targetTcpProxy` - Name of the TargetTcpProxy resource whose ProxyHeader is to be set.
    pub fn set_proxy_header(&self, request: TargetTcpProxiesSetProxyHeaderRequest, project: &str, target_tcp_proxy: &str) -> TargetTcpProxySetProxyHeaderCall<'a, S> {
        TargetTcpProxySetProxyHeaderCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _target_tcp_proxy: target_tcp_proxy.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetVpnGateway* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.target_vpn_gateways();
/// # }
/// ```
pub struct TargetVpnGatewayMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for TargetVpnGatewayMethods<'a, S> {}

impl<'a, S> TargetVpnGatewayMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of target VPN gateways.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> TargetVpnGatewayAggregatedListCall<'a, S> {
        TargetVpnGatewayAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified target VPN gateway.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `targetVpnGateway` - Name of the target VPN gateway to delete.
    pub fn delete(&self, project: &str, region: &str, target_vpn_gateway: &str) -> TargetVpnGatewayDeleteCall<'a, S> {
        TargetVpnGatewayDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_vpn_gateway: target_vpn_gateway.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified target VPN gateway. Gets a list of available target VPN gateways by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `targetVpnGateway` - Name of the target VPN gateway to return.
    pub fn get(&self, project: &str, region: &str, target_vpn_gateway: &str) -> TargetVpnGatewayGetCall<'a, S> {
        TargetVpnGatewayGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _target_vpn_gateway: target_vpn_gateway.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a target VPN gateway in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: TargetVpnGateway, project: &str, region: &str) -> TargetVpnGatewayInsertCall<'a, S> {
        TargetVpnGatewayInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of target VPN gateways available to the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> TargetVpnGatewayListCall<'a, S> {
        TargetVpnGatewayListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a TargetVpnGateway. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> TargetVpnGatewaySetLabelCall<'a, S> {
        TargetVpnGatewaySetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *urlMap* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `invalidate_cache(...)`, `list(...)`, `patch(...)`, `update(...)` and `validate(...)`
/// // to build up your call.
/// let rb = hub.url_maps();
/// # }
/// ```
pub struct UrlMapMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for UrlMapMethods<'a, S> {}

impl<'a, S> UrlMapMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of all UrlMap resources, regional and global, available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    pub fn aggregated_list(&self, project: &str) -> UrlMapAggregatedListCall<'a, S> {
        UrlMapAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified UrlMap resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap resource to delete.
    pub fn delete(&self, project: &str, url_map: &str) -> UrlMapDeleteCall<'a, S> {
        UrlMapDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified UrlMap resource. Gets a list of available URL maps by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap resource to return.
    pub fn get(&self, project: &str, url_map: &str) -> UrlMapGetCall<'a, S> {
        UrlMapGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a UrlMap resource in the specified project using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    pub fn insert(&self, request: UrlMap, project: &str) -> UrlMapInsertCall<'a, S> {
        UrlMapInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates a cache invalidation operation, invalidating the specified path, scoped to the specified UrlMap. For more information, see [Invalidating cached content](https://cloud.google.com/cdn/docs/invalidating-cached-content).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap scoping this request.
    pub fn invalidate_cache(&self, request: CacheInvalidationRule, project: &str, url_map: &str) -> UrlMapInvalidateCacheCall<'a, S> {
        UrlMapInvalidateCacheCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of UrlMap resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> UrlMapListCall<'a, S> {
        UrlMapListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the specified UrlMap resource with the data included in the request. This method supports PATCH semantics and uses the JSON merge patch format and processing rules.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap resource to patch.
    pub fn patch(&self, request: UrlMap, project: &str, url_map: &str) -> UrlMapPatchCall<'a, S> {
        UrlMapPatchCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified UrlMap resource with the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap resource to update.
    pub fn update(&self, request: UrlMap, project: &str, url_map: &str) -> UrlMapUpdateCall<'a, S> {
        UrlMapUpdateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs static validation for the UrlMap. In particular, the tests of the provided UrlMap will be run. Calling this method does NOT create the UrlMap.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `urlMap` - Name of the UrlMap resource to be validated as.
    pub fn validate(&self, request: UrlMapsValidateRequest, project: &str, url_map: &str) -> UrlMapValidateCall<'a, S> {
        UrlMapValidateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _url_map: url_map.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *vpnGateway* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `get_status(...)`, `insert(...)`, `list(...)`, `set_labels(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.vpn_gateways();
/// # }
/// ```
pub struct VpnGatewayMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for VpnGatewayMethods<'a, S> {}

impl<'a, S> VpnGatewayMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of VPN gateways.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> VpnGatewayAggregatedListCall<'a, S> {
        VpnGatewayAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified VPN gateway.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `vpnGateway` - Name of the VPN gateway to delete.
    pub fn delete(&self, project: &str, region: &str, vpn_gateway: &str) -> VpnGatewayDeleteCall<'a, S> {
        VpnGatewayDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _vpn_gateway: vpn_gateway.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified VPN gateway. Gets a list of available VPN gateways by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `vpnGateway` - Name of the VPN gateway to return.
    pub fn get(&self, project: &str, region: &str, vpn_gateway: &str) -> VpnGatewayGetCall<'a, S> {
        VpnGatewayGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _vpn_gateway: vpn_gateway.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the status for the specified VPN gateway.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `vpnGateway` - Name of the VPN gateway to return.
    pub fn get_status(&self, project: &str, region: &str, vpn_gateway: &str) -> VpnGatewayGetStatuCall<'a, S> {
        VpnGatewayGetStatuCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _vpn_gateway: vpn_gateway.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a VPN gateway in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: VpnGateway, project: &str, region: &str) -> VpnGatewayInsertCall<'a, S> {
        VpnGatewayInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of VPN gateways available to the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> VpnGatewayListCall<'a, S> {
        VpnGatewayListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a VpnGateway. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> VpnGatewaySetLabelCall<'a, S> {
        VpnGatewaySetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The name of the region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn test_iam_permissions(&self, request: TestPermissionsRequest, project: &str, region: &str, resource: &str) -> VpnGatewayTestIamPermissionCall<'a, S> {
        VpnGatewayTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *vpnTunnel* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `aggregated_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `set_labels(...)`
/// // to build up your call.
/// let rb = hub.vpn_tunnels();
/// # }
/// ```
pub struct VpnTunnelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for VpnTunnelMethods<'a, S> {}

impl<'a, S> VpnTunnelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an aggregated list of VPN tunnels.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn aggregated_list(&self, project: &str) -> VpnTunnelAggregatedListCall<'a, S> {
        VpnTunnelAggregatedListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_all_scopes: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified VpnTunnel resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `vpnTunnel` - Name of the VpnTunnel resource to delete.
    pub fn delete(&self, project: &str, region: &str, vpn_tunnel: &str) -> VpnTunnelDeleteCall<'a, S> {
        VpnTunnelDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _vpn_tunnel: vpn_tunnel.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified VpnTunnel resource. Gets a list of available VPN tunnels by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    /// * `vpnTunnel` - Name of the VpnTunnel resource to return.
    pub fn get(&self, project: &str, region: &str, vpn_tunnel: &str) -> VpnTunnelGetCall<'a, S> {
        VpnTunnelGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _vpn_tunnel: vpn_tunnel.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a VpnTunnel resource in the specified project and region using the data included in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn insert(&self, request: VpnTunnel, project: &str, region: &str) -> VpnTunnelInsertCall<'a, S> {
        VpnTunnelInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of VpnTunnel resources contained in the specified project and region.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `region` - Name of the region for this request.
    pub fn list(&self, project: &str, region: &str) -> VpnTunnelListCall<'a, S> {
        VpnTunnelListCall {
            hub: self.hub,
            _project: project.to_string(),
            _region: region.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the labels on a VpnTunnel. To learn more about labels, read the Labeling Resources documentation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Project ID for this request.
    /// * `region` - The region for this request.
    /// * `resource` - Name or id of the resource for this request.
    pub fn set_labels(&self, request: RegionSetLabelsRequest, project: &str, region: &str, resource: &str) -> VpnTunnelSetLabelCall<'a, S> {
        VpnTunnelSetLabelCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _region: region.to_string(),
            _resource: resource.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `wait(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneOperationMethods<'a, S> {}

impl<'a, S> ZoneOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified zone-specific Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `operation` - Name of the Operations resource to delete.
    pub fn delete(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationDeleteCall<'a, S> {
        ZoneOperationDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified zone-specific Operations resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn get(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationGetCall<'a, S> {
        ZoneOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of Operation resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for request.
    pub fn list(&self, project: &str, zone: &str) -> ZoneOperationListCall<'a, S> {
        ZoneOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Waits for the specified Operation resource to return as `DONE` or for the request to approach the 2 minute deadline, and retrieves the specified Operation resource. This method waits for no more than the 2 minutes and then returns the current state of the operation, which might be `DONE` or still in progress. This method is called on a best-effort basis. Specifically: - In uncommon cases, when the server is overloaded, the request might return before the default deadline is reached, or might return after zero seconds. - If the default deadline is reached, there is no guarantee that the operation is actually done when the method returns. Be prepared to retry if the operation is not `DONE`. 
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone for this request.
    /// * `operation` - Name of the Operations resource to return.
    pub fn wait(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationWaitCall<'a, S> {
        ZoneOperationWaitCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zone* resources.
/// It is not used directly, but through the [`Compute`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_compute1 as compute1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use compute1::{Compute, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Compute::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zones();
/// # }
/// ```
pub struct ZoneMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Compute<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneMethods<'a, S> {}

impl<'a, S> ZoneMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Zone resource. Gets a list of available zones by making a list() request.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    /// * `zone` - Name of the zone resource to return.
    pub fn get(&self, project: &str, zone: &str) -> ZoneGetCall<'a, S> {
        ZoneGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Zone resources available to the specified project.
    /// 
    /// # Arguments
    ///
    /// * `project` - Project ID for this request.
    pub fn list(&self, project: &str) -> ZoneListCall<'a, S> {
        ZoneListCall {
            hub: self.hub,
            _project: project.to_string(),
            _return_partial_success: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



