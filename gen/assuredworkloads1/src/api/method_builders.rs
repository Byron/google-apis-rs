use super::*;
/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Assuredworkloads`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_assuredworkloads1 as assuredworkloads1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use assuredworkloads1::{Assuredworkloads, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Assuredworkloads::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_workloads_create(...)`, `locations_workloads_delete(...)`, `locations_workloads_get(...)`, `locations_workloads_list(...)`, `locations_workloads_mutate_partner_permissions(...)`, `locations_workloads_patch(...)`, `locations_workloads_restrict_allowed_resources(...)`, `locations_workloads_violations_acknowledge(...)`, `locations_workloads_violations_get(...)` and `locations_workloads_violations_list(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Assuredworkloads<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn locations_operations_get(&self, name: &str) -> OrganizationLocationOperationGetCall<'a, S> {
        OrganizationLocationOperationGetCall {
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
    pub fn locations_operations_list(&self, name: &str) -> OrganizationLocationOperationListCall<'a, S> {
        OrganizationLocationOperationListCall {
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
    /// Acknowledges an existing violation. By acknowledging a violation, users acknowledge the existence of a compliance violation in their workload and decide to ignore it due to a valid business justification. Acknowledgement is a permanent operation and it cannot be reverted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Violation to acknowledge. Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
    pub fn locations_workloads_violations_acknowledge(&self, request: GoogleCloudAssuredworkloadsV1AcknowledgeViolationRequest, name: &str) -> OrganizationLocationWorkloadViolationAcknowledgeCall<'a, S> {
        OrganizationLocationWorkloadViolationAcknowledgeCall {
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
    /// Retrieves Assured Workload Violation based on ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Violation to fetch (ie. Violation.name). Format: organizations/{organization}/locations/{location}/workloads/{workload}/violations/{violation}
    pub fn locations_workloads_violations_get(&self, name: &str) -> OrganizationLocationWorkloadViolationGetCall<'a, S> {
        OrganizationLocationWorkloadViolationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Violations in the AssuredWorkload Environment. Callers may also choose to read across multiple Workloads as per [AIP-159](https://google.aip.dev/159) by using '-' (the hyphen or dash character) as a wildcard character instead of workload-id in the parent. Format `organizations/{org_id}/locations/{location}/workloads/-`
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The Workload name. Format `organizations/{org_id}/locations/{location}/workloads/{workload}`.
    pub fn locations_workloads_violations_list(&self, parent: &str) -> OrganizationLocationWorkloadViolationListCall<'a, S> {
        OrganizationLocationWorkloadViolationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _interval_start_time: Default::default(),
            _interval_end_time: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates Assured Workload.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the new Workload's parent. Must be of the form `organizations/{org_id}/locations/{location_id}`.
    pub fn locations_workloads_create(&self, request: GoogleCloudAssuredworkloadsV1Workload, parent: &str) -> OrganizationLocationWorkloadCreateCall<'a, S> {
        OrganizationLocationWorkloadCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _external_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the workload. Make sure that workload's direct children are already in a deleted state, otherwise the request will fail with a FAILED_PRECONDITION error.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The `name` field is used to identify the workload. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    pub fn locations_workloads_delete(&self, name: &str) -> OrganizationLocationWorkloadDeleteCall<'a, S> {
        OrganizationLocationWorkloadDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _etag: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets Assured Workload associated with a CRM Node
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the Workload to fetch. This is the workload's relative path in the API, formatted as "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}". For example, "organizations/123/locations/us-east1/workloads/assured-workload-1".
    pub fn locations_workloads_get(&self, name: &str) -> OrganizationLocationWorkloadGetCall<'a, S> {
        OrganizationLocationWorkloadGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists Assured Workloads under a CRM Node.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent Resource to list workloads from. Must be of the form `organizations/{org_id}/locations/{location}`.
    pub fn locations_workloads_list(&self, parent: &str) -> OrganizationLocationWorkloadListCall<'a, S> {
        OrganizationLocationWorkloadListCall {
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
    /// Update the permissions settings for an existing partner workload. For force updates don't set etag field in the Workload. Only one update operation per workload can be in progress.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The `name` field is used to identify the workload. Format: organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    pub fn locations_workloads_mutate_partner_permissions(&self, request: GoogleCloudAssuredworkloadsV1MutatePartnerPermissionsRequest, name: &str) -> OrganizationLocationWorkloadMutatePartnerPermissionCall<'a, S> {
        OrganizationLocationWorkloadMutatePartnerPermissionCall {
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
    /// Updates an existing workload. Currently allows updating of workload display_name and labels. For force updates don't set etag field in the Workload. Only one update operation per workload can be in progress.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. The resource name of the workload. Format: organizations/{organization}/locations/{location}/workloads/{workload} Read-only.
    pub fn locations_workloads_patch(&self, request: GoogleCloudAssuredworkloadsV1Workload, name: &str) -> OrganizationLocationWorkloadPatchCall<'a, S> {
        OrganizationLocationWorkloadPatchCall {
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
    /// Restrict the list of resources allowed in the Workload environment. The current list of allowed products can be found at https://cloud.google.com/assured-workloads/docs/supported-products In addition to assuredworkloads.workload.update permission, the user should also have orgpolicy.policy.set permission on the folder resource to use this functionality.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the Workload. This is the workloads's relative path in the API, formatted as "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}". For example, "organizations/123/locations/us-east1/workloads/assured-workload-1".
    pub fn locations_workloads_restrict_allowed_resources(&self, request: GoogleCloudAssuredworkloadsV1RestrictAllowedResourcesRequest, name: &str) -> OrganizationLocationWorkloadRestrictAllowedResourceCall<'a, S> {
        OrganizationLocationWorkloadRestrictAllowedResourceCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



