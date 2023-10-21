use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`CertificateManager`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_certificatemanager1 as certificatemanager1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use certificatemanager1::{CertificateManager, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CertificateManager::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_certificate_issuance_configs_create(...)`, `locations_certificate_issuance_configs_delete(...)`, `locations_certificate_issuance_configs_get(...)`, `locations_certificate_issuance_configs_list(...)`, `locations_certificate_maps_certificate_map_entries_create(...)`, `locations_certificate_maps_certificate_map_entries_delete(...)`, `locations_certificate_maps_certificate_map_entries_get(...)`, `locations_certificate_maps_certificate_map_entries_list(...)`, `locations_certificate_maps_certificate_map_entries_patch(...)`, `locations_certificate_maps_create(...)`, `locations_certificate_maps_delete(...)`, `locations_certificate_maps_get(...)`, `locations_certificate_maps_list(...)`, `locations_certificate_maps_patch(...)`, `locations_certificates_create(...)`, `locations_certificates_delete(...)`, `locations_certificates_get(...)`, `locations_certificates_list(...)`, `locations_certificates_patch(...)`, `locations_dns_authorizations_create(...)`, `locations_dns_authorizations_delete(...)`, `locations_dns_authorizations_get(...)`, `locations_dns_authorizations_list(...)`, `locations_dns_authorizations_patch(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)` and `locations_operations_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CertificateManager<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new CertificateIssuanceConfig in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the certificate issuance config. Must be in the format `projects/*/locations/*`.
    pub fn locations_certificate_issuance_configs_create(&self, request: CertificateIssuanceConfig, parent: &str) -> ProjectLocationCertificateIssuanceConfigCreateCall<'a, S> {
        ProjectLocationCertificateIssuanceConfigCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _certificate_issuance_config_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single CertificateIssuanceConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate issuance config to delete. Must be in the format `projects/*/locations/*/certificateIssuanceConfigs/*`.
    pub fn locations_certificate_issuance_configs_delete(&self, name: &str) -> ProjectLocationCertificateIssuanceConfigDeleteCall<'a, S> {
        ProjectLocationCertificateIssuanceConfigDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single CertificateIssuanceConfig.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate issuance config to describe. Must be in the format `projects/*/locations/*/certificateIssuanceConfigs/*`.
    pub fn locations_certificate_issuance_configs_get(&self, name: &str) -> ProjectLocationCertificateIssuanceConfigGetCall<'a, S> {
        ProjectLocationCertificateIssuanceConfigGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CertificateIssuanceConfigs in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the certificate should be listed, specified in the format `projects/*/locations/*`.
    pub fn locations_certificate_issuance_configs_list(&self, parent: &str) -> ProjectLocationCertificateIssuanceConfigListCall<'a, S> {
        ProjectLocationCertificateIssuanceConfigListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new CertificateMapEntry in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the certificate map entry. Must be in the format `projects/*/locations/*/certificateMaps/*`.
    pub fn locations_certificate_maps_certificate_map_entries_create(&self, request: CertificateMapEntry, parent: &str) -> ProjectLocationCertificateMapCertificateMapEntryCreateCall<'a, S> {
        ProjectLocationCertificateMapCertificateMapEntryCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _certificate_map_entry_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single CertificateMapEntry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate map entry to delete. Must be in the format `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    pub fn locations_certificate_maps_certificate_map_entries_delete(&self, name: &str) -> ProjectLocationCertificateMapCertificateMapEntryDeleteCall<'a, S> {
        ProjectLocationCertificateMapCertificateMapEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single CertificateMapEntry.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate map entry to describe. Must be in the format `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    pub fn locations_certificate_maps_certificate_map_entries_get(&self, name: &str) -> ProjectLocationCertificateMapCertificateMapEntryGetCall<'a, S> {
        ProjectLocationCertificateMapCertificateMapEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CertificateMapEntries in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project, location and certificate map from which the certificate map entries should be listed, specified in the format `projects/*/locations/*/certificateMaps/*`.
    pub fn locations_certificate_maps_certificate_map_entries_list(&self, parent: &str) -> ProjectLocationCertificateMapCertificateMapEntryListCall<'a, S> {
        ProjectLocationCertificateMapCertificateMapEntryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a CertificateMapEntry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A user-defined name of the Certificate Map Entry. Certificate Map Entry names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`.
    pub fn locations_certificate_maps_certificate_map_entries_patch(&self, request: CertificateMapEntry, name: &str) -> ProjectLocationCertificateMapCertificateMapEntryPatchCall<'a, S> {
        ProjectLocationCertificateMapCertificateMapEntryPatchCall {
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
    /// Creates a new CertificateMap in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the certificate map. Must be in the format `projects/*/locations/*`.
    pub fn locations_certificate_maps_create(&self, request: CertificateMap, parent: &str) -> ProjectLocationCertificateMapCreateCall<'a, S> {
        ProjectLocationCertificateMapCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _certificate_map_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single CertificateMap. A Certificate Map can't be deleted if it contains Certificate Map Entries. Remove all the entries from the map before calling this method.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate map to delete. Must be in the format `projects/*/locations/*/certificateMaps/*`.
    pub fn locations_certificate_maps_delete(&self, name: &str) -> ProjectLocationCertificateMapDeleteCall<'a, S> {
        ProjectLocationCertificateMapDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single CertificateMap.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate map to describe. Must be in the format `projects/*/locations/*/certificateMaps/*`.
    pub fn locations_certificate_maps_get(&self, name: &str) -> ProjectLocationCertificateMapGetCall<'a, S> {
        ProjectLocationCertificateMapGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists CertificateMaps in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the certificate maps should be listed, specified in the format `projects/*/locations/*`.
    pub fn locations_certificate_maps_list(&self, parent: &str) -> ProjectLocationCertificateMapListCall<'a, S> {
        ProjectLocationCertificateMapListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a CertificateMap.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A user-defined name of the Certificate Map. Certificate Map names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*`.
    pub fn locations_certificate_maps_patch(&self, request: CertificateMap, name: &str) -> ProjectLocationCertificateMapPatchCall<'a, S> {
        ProjectLocationCertificateMapPatchCall {
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
    /// Creates a new Certificate in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the certificate. Must be in the format `projects/*/locations/*`.
    pub fn locations_certificates_create(&self, request: Certificate, parent: &str) -> ProjectLocationCertificateCreateCall<'a, S> {
        ProjectLocationCertificateCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _certificate_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single Certificate.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate to delete. Must be in the format `projects/*/locations/*/certificates/*`.
    pub fn locations_certificates_delete(&self, name: &str) -> ProjectLocationCertificateDeleteCall<'a, S> {
        ProjectLocationCertificateDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single Certificate.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the certificate to describe. Must be in the format `projects/*/locations/*/certificates/*`.
    pub fn locations_certificates_get(&self, name: &str) -> ProjectLocationCertificateGetCall<'a, S> {
        ProjectLocationCertificateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Certificates in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the certificate should be listed, specified in the format `projects/*/locations/*`.
    pub fn locations_certificates_list(&self, parent: &str) -> ProjectLocationCertificateListCall<'a, S> {
        ProjectLocationCertificateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a Certificate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A user-defined name of the certificate. Certificate names must be unique globally and match pattern `projects/*/locations/*/certificates/*`.
    pub fn locations_certificates_patch(&self, request: Certificate, name: &str) -> ProjectLocationCertificatePatchCall<'a, S> {
        ProjectLocationCertificatePatchCall {
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
    /// Creates a new DnsAuthorization in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the dns authorization. Must be in the format `projects/*/locations/*`.
    pub fn locations_dns_authorizations_create(&self, request: DnsAuthorization, parent: &str) -> ProjectLocationDnsAuthorizationCreateCall<'a, S> {
        ProjectLocationDnsAuthorizationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _dns_authorization_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a single DnsAuthorization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the dns authorization to delete. Must be in the format `projects/*/locations/*/dnsAuthorizations/*`.
    pub fn locations_dns_authorizations_delete(&self, name: &str) -> ProjectLocationDnsAuthorizationDeleteCall<'a, S> {
        ProjectLocationDnsAuthorizationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of a single DnsAuthorization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. A name of the dns authorization to describe. Must be in the format `projects/*/locations/*/dnsAuthorizations/*`.
    pub fn locations_dns_authorizations_get(&self, name: &str) -> ProjectLocationDnsAuthorizationGetCall<'a, S> {
        ProjectLocationDnsAuthorizationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists DnsAuthorizations in a given project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The project and location from which the dns authorizations should be listed, specified in the format `projects/*/locations/*`.
    pub fn locations_dns_authorizations_list(&self, parent: &str) -> ProjectLocationDnsAuthorizationListCall<'a, S> {
        ProjectLocationDnsAuthorizationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a DnsAuthorization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - A user-defined name of the dns authorization. DnsAuthorization names must be unique globally and match pattern `projects/*/locations/*/dnsAuthorizations/*`.
    pub fn locations_dns_authorizations_patch(&self, request: DnsAuthorization, name: &str) -> ProjectLocationDnsAuthorizationPatchCall<'a, S> {
        ProjectLocationDnsAuthorizationPatchCall {
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
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> ProjectLocationOperationCancelCall<'a, S> {
        ProjectLocationOperationCancelCall {
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
    /// Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource to be deleted.
    pub fn locations_operations_delete(&self, name: &str) -> ProjectLocationOperationDeleteCall<'a, S> {
        ProjectLocationOperationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// Gets information about a location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name for the location.
    pub fn locations_get(&self, name: &str) -> ProjectLocationGetCall<'a, S> {
        ProjectLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists information about the supported locations for this service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, name: &str) -> ProjectLocationListCall<'a, S> {
        ProjectLocationListCall {
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



