use super::*;
/// A builder providing access to all methods supported on *app* resources.
/// It is not used directly, but through the [`Appengine`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appengine1 as appengine1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appengine1::{Appengine, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appengine::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `authorized_certificates_create(...)`, `authorized_certificates_delete(...)`, `authorized_certificates_get(...)`, `authorized_certificates_list(...)`, `authorized_certificates_patch(...)`, `authorized_domains_list(...)`, `create(...)`, `domain_mappings_create(...)`, `domain_mappings_delete(...)`, `domain_mappings_get(...)`, `domain_mappings_list(...)`, `domain_mappings_patch(...)`, `firewall_ingress_rules_batch_update(...)`, `firewall_ingress_rules_create(...)`, `firewall_ingress_rules_delete(...)`, `firewall_ingress_rules_get(...)`, `firewall_ingress_rules_list(...)`, `firewall_ingress_rules_patch(...)`, `get(...)`, `locations_get(...)`, `locations_list(...)`, `operations_get(...)`, `operations_list(...)`, `patch(...)`, `repair(...)`, `services_delete(...)`, `services_get(...)`, `services_list(...)`, `services_patch(...)`, `services_versions_create(...)`, `services_versions_delete(...)`, `services_versions_get(...)`, `services_versions_instances_debug(...)`, `services_versions_instances_delete(...)`, `services_versions_instances_get(...)`, `services_versions_instances_list(...)`, `services_versions_list(...)` and `services_versions_patch(...)`
/// // to build up your call.
/// let rb = hub.apps();
/// # }
/// ```
pub struct AppMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Appengine<S>,
}

impl<'a, S> client::MethodsBuilder for AppMethods<'a, S> {}

impl<'a, S> AppMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads the specified SSL certificate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn authorized_certificates_create(&self, request: AuthorizedCertificate, apps_id: &str) -> AppAuthorizedCertificateCreateCall<'a, S> {
        AppAuthorizedCertificateCreateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified SSL certificate.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource to delete. Example: apps/myapp/authorizedCertificates/12345.
    /// * `authorizedCertificatesId` - Part of `name`. See documentation of `appsId`.
    pub fn authorized_certificates_delete(&self, apps_id: &str, authorized_certificates_id: &str) -> AppAuthorizedCertificateDeleteCall<'a, S> {
        AppAuthorizedCertificateDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _authorized_certificates_id: authorized_certificates_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified SSL certificate.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/authorizedCertificates/12345.
    /// * `authorizedCertificatesId` - Part of `name`. See documentation of `appsId`.
    pub fn authorized_certificates_get(&self, apps_id: &str, authorized_certificates_id: &str) -> AppAuthorizedCertificateGetCall<'a, S> {
        AppAuthorizedCertificateGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _authorized_certificates_id: authorized_certificates_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all SSL certificates the user is authorized to administer.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn authorized_certificates_list(&self, apps_id: &str) -> AppAuthorizedCertificateListCall<'a, S> {
        AppAuthorizedCertificateListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified SSL certificate. To renew a certificate and maintain its existing domain mappings, update certificate_data with a new certificate. The new certificate must be applicable to the same domains as the original certificate. The certificate display_name may also be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/authorizedCertificates/12345.
    /// * `authorizedCertificatesId` - Part of `name`. See documentation of `appsId`.
    pub fn authorized_certificates_patch(&self, request: AuthorizedCertificate, apps_id: &str, authorized_certificates_id: &str) -> AppAuthorizedCertificatePatchCall<'a, S> {
        AppAuthorizedCertificatePatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _authorized_certificates_id: authorized_certificates_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all domains the user is authorized to administer.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn authorized_domains_list(&self, apps_id: &str) -> AppAuthorizedDomainListCall<'a, S> {
        AppAuthorizedDomainListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn domain_mappings_create(&self, request: DomainMapping, apps_id: &str) -> AppDomainMappingCreateCall<'a, S> {
        AppDomainMappingCreateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _override_strategy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified domain mapping. A user must be authorized to administer the associated domain in order to delete a DomainMapping resource.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource to delete. Example: apps/myapp/domainMappings/example.com.
    /// * `domainMappingsId` - Part of `name`. See documentation of `appsId`.
    pub fn domain_mappings_delete(&self, apps_id: &str, domain_mappings_id: &str) -> AppDomainMappingDeleteCall<'a, S> {
        AppDomainMappingDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _domain_mappings_id: domain_mappings_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified domain mapping.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/domainMappings/example.com.
    /// * `domainMappingsId` - Part of `name`. See documentation of `appsId`.
    pub fn domain_mappings_get(&self, apps_id: &str, domain_mappings_id: &str) -> AppDomainMappingGetCall<'a, S> {
        AppDomainMappingGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _domain_mappings_id: domain_mappings_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the domain mappings on an application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn domain_mappings_list(&self, apps_id: &str) -> AppDomainMappingListCall<'a, S> {
        AppDomainMappingListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified domain mapping. To map an SSL certificate to a domain mapping, update certificate_id to point to an AuthorizedCertificate resource. A user must be authorized to administer the associated domain in order to update a DomainMapping resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/domainMappings/example.com.
    /// * `domainMappingsId` - Part of `name`. See documentation of `appsId`.
    pub fn domain_mappings_patch(&self, request: DomainMapping, apps_id: &str, domain_mappings_id: &str) -> AppDomainMappingPatchCall<'a, S> {
        AppDomainMappingPatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _domain_mappings_id: domain_mappings_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces the entire firewall ruleset in one bulk operation. This overrides and replaces the rules of an existing firewall with the new rules.If the final rule does not match traffic with the '*' wildcard IP range, then an "allow all" rule is explicitly added to the end of the list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the Firewall collection to set. Example: apps/myapp/firewall/ingressRules.
    pub fn firewall_ingress_rules_batch_update(&self, request: BatchUpdateIngressRulesRequest, apps_id: &str) -> AppFirewallIngressRuleBatchUpdateCall<'a, S> {
        AppFirewallIngressRuleBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a firewall rule for the application.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `parent`. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules.
    pub fn firewall_ingress_rules_create(&self, request: FirewallRule, apps_id: &str) -> AppFirewallIngressRuleCreateCall<'a, S> {
        AppFirewallIngressRuleCreateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified firewall rule.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the Firewall resource to delete. Example: apps/myapp/firewall/ingressRules/100.
    /// * `ingressRulesId` - Part of `name`. See documentation of `appsId`.
    pub fn firewall_ingress_rules_delete(&self, apps_id: &str, ingress_rules_id: &str) -> AppFirewallIngressRuleDeleteCall<'a, S> {
        AppFirewallIngressRuleDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _ingress_rules_id: ingress_rules_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified firewall rule.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the Firewall resource to retrieve. Example: apps/myapp/firewall/ingressRules/100.
    /// * `ingressRulesId` - Part of `name`. See documentation of `appsId`.
    pub fn firewall_ingress_rules_get(&self, apps_id: &str, ingress_rules_id: &str) -> AppFirewallIngressRuleGetCall<'a, S> {
        AppFirewallIngressRuleGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _ingress_rules_id: ingress_rules_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the firewall rules of an application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the Firewall collection to retrieve. Example: apps/myapp/firewall/ingressRules.
    pub fn firewall_ingress_rules_list(&self, apps_id: &str) -> AppFirewallIngressRuleListCall<'a, S> {
        AppFirewallIngressRuleListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _matching_address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified firewall rule.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the Firewall resource to update. Example: apps/myapp/firewall/ingressRules/100.
    /// * `ingressRulesId` - Part of `name`. See documentation of `appsId`.
    pub fn firewall_ingress_rules_patch(&self, request: FirewallRule, apps_id: &str, ingress_rules_id: &str) -> AppFirewallIngressRulePatchCall<'a, S> {
        AppFirewallIngressRulePatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _ingress_rules_id: ingress_rules_id.to_string(),
            _update_mask: Default::default(),
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
    /// * `appsId` - Part of `name`. Resource name for the location.
    /// * `locationsId` - Part of `name`. See documentation of `appsId`.
    pub fn locations_get(&self, apps_id: &str, locations_id: &str) -> AppLocationGetCall<'a, S> {
        AppLocationGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _locations_id: locations_id.to_string(),
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
    /// * `appsId` - Part of `name`. The resource that owns the locations collection, if applicable.
    pub fn locations_list(&self, apps_id: &str) -> AppLocationListCall<'a, S> {
        AppLocationListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
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
    /// * `appsId` - Part of `name`. The name of the operation resource.
    /// * `operationsId` - Part of `name`. See documentation of `appsId`.
    pub fn operations_get(&self, apps_id: &str, operations_id: &str) -> AppOperationGetCall<'a, S> {
        AppOperationGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _operations_id: operations_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as "/v1/{name=users/*}/operations" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. The name of the operation's parent resource.
    pub fn operations_list(&self, apps_id: &str) -> AppOperationListCall<'a, S> {
        AppOperationListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
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
    /// Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_instances_debug(&self, request: DebugInstanceRequest, apps_id: &str, services_id: &str, versions_id: &str, instances_id: &str) -> AppServiceVersionInstanceDebugCall<'a, S> {
        AppServiceVersionInstanceDebugCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stops a running instance.The instance might be automatically recreated based on the scaling settings of the version. For more information, see "How Instances are Managed" (standard environment (https://cloud.google.com/appengine/docs/standard/python/how-instances-are-managed) | flexible environment (https://cloud.google.com/appengine/docs/flexible/python/how-instances-are-managed)).To ensure that instances are not re-created and avoid getting billed, you can stop all instances within the target version by changing the serving status of the version to STOPPED with the apps.services.versions.patch (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions/patch) method.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_instances_delete(&self, apps_id: &str, services_id: &str, versions_id: &str, instances_id: &str) -> AppServiceVersionInstanceDeleteCall<'a, S> {
        AppServiceVersionInstanceDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets instance information.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    /// * `instancesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_instances_get(&self, apps_id: &str, services_id: &str, versions_id: &str, instances_id: &str) -> AppServiceVersionInstanceGetCall<'a, S> {
        AppServiceVersionInstanceGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _instances_id: instances_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the instances of a version.Tip: To aggregate details about instances over time, see the Stackdriver Monitoring API (https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Version resource. Example: apps/myapp/services/default/versions/v1.
    /// * `servicesId` - Part of `parent`. See documentation of `appsId`.
    /// * `versionsId` - Part of `parent`. See documentation of `appsId`.
    pub fn services_versions_instances_list(&self, apps_id: &str, services_id: &str, versions_id: &str) -> AppServiceVersionInstanceListCall<'a, S> {
        AppServiceVersionInstanceListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys code and resource files to a new version.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `parent`. Name of the parent resource to create this version under. Example: apps/myapp/services/default.
    /// * `servicesId` - Part of `parent`. See documentation of `appsId`.
    pub fn services_versions_create(&self, request: Version, apps_id: &str, services_id: &str) -> AppServiceVersionCreateCall<'a, S> {
        AppServiceVersionCreateCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing Version resource.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_delete(&self, apps_id: &str, services_id: &str, versions_id: &str) -> AppServiceVersionDeleteCall<'a, S> {
        AppServiceVersionDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_get(&self, apps_id: &str, services_id: &str, versions_id: &str) -> AppServiceVersionGetCall<'a, S> {
        AppServiceVersionGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the versions of a service.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Service resource. Example: apps/myapp/services/default.
    /// * `servicesId` - Part of `parent`. See documentation of `appsId`.
    pub fn services_versions_list(&self, apps_id: &str, services_id: &str) -> AppServiceVersionListCall<'a, S> {
        AppServiceVersionListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses:Standard environment instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class)automatic scaling in the standard environment: automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling) automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling) automaticScaling.standard_scheduler_settings.max_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings) automaticScaling.standard_scheduler_settings.min_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings) automaticScaling.standard_scheduler_settings.target_cpu_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings) automaticScaling.standard_scheduler_settings.target_throughput_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StandardSchedulerSettings)basic scaling or manual scaling in the standard environment: serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status) manual_scaling.instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#manualscaling)Flexible environment serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status)automatic scaling in the flexible environment: automatic_scaling.min_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling) automatic_scaling.max_total_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling) automatic_scaling.cool_down_period_sec (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling) automatic_scaling.cpu_utilization.target_utilization (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling)manual scaling in the flexible environment: manual_scaling.instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#manualscaling)
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/services/default/versions/1.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    /// * `versionsId` - Part of `name`. See documentation of `appsId`.
    pub fn services_versions_patch(&self, request: Version, apps_id: &str, services_id: &str, versions_id: &str) -> AppServiceVersionPatchCall<'a, S> {
        AppServiceVersionPatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _versions_id: versions_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified service and all enclosed versions.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_delete(&self, apps_id: &str, services_id: &str) -> AppServiceDeleteCall<'a, S> {
        AppServiceDeleteCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current configuration of the specified service.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the resource requested. Example: apps/myapp/services/default.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_get(&self, apps_id: &str, services_id: &str) -> AppServiceGetCall<'a, S> {
        AppServiceGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the services in the application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `parent`. Name of the parent Application resource. Example: apps/myapp.
    pub fn services_list(&self, apps_id: &str) -> AppServiceListCall<'a, S> {
        AppServiceListCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the configuration of the specified service.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the resource to update. Example: apps/myapp/services/default.
    /// * `servicesId` - Part of `name`. See documentation of `appsId`.
    pub fn services_patch(&self, request: Service, apps_id: &str, services_id: &str) -> AppServicePatchCall<'a, S> {
        AppServicePatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _services_id: services_id.to_string(),
            _update_mask: Default::default(),
            _migrate_traffic: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an App Engine application for a Google Cloud Platform project. Required fields: id - The ID of the target Cloud Platform project. location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/standard/python/console/).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Application) -> AppCreateCall<'a, S> {
        AppCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about an application.
    /// 
    /// # Arguments
    ///
    /// * `appsId` - Part of `name`. Name of the Application resource to get. Example: apps/myapp.
    pub fn get(&self, apps_id: &str) -> AppGetCall<'a, S> {
        AppGetCall {
            hub: self.hub,
            _apps_id: apps_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified Application resource. You can update the following fields: auth_domain - Google authentication domain for controlling user access to the application. default_cookie_expiration - Cookie expiration policy for the application. iap - Identity-Aware Proxy properties for the application.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the Application resource to update. Example: apps/myapp.
    pub fn patch(&self, request: Application, apps_id: &str) -> AppPatchCall<'a, S> {
        AppPatchCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Recreates the required App Engine features for the specified App Engine application, for example a Cloud Storage bucket or App Engine service account. Use this method if you receive an error message about a missing feature, for example, Error retrieving the App Engine service account. If you have deleted your App Engine service account, this will not be able to recreate it. Instead, you should attempt to use the IAM undelete API if possible at https://cloud.google.com/iam/reference/rest/v1/projects.serviceAccounts/undelete?apix_params=%7B"name"%3A"projects%2F-%2FserviceAccounts%2Funique_id"%2C"resource"%3A%7B%7D%7D . If the deletion was recent, the numeric ID can be found in the Cloud Console Activity Log.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `appsId` - Part of `name`. Name of the application to repair. Example: apps/myapp
    pub fn repair(&self, request: RepairApplicationRequest, apps_id: &str) -> AppRepairCall<'a, S> {
        AppRepairCall {
            hub: self.hub,
            _request: request,
            _apps_id: apps_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Appengine`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appengine1 as appengine1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appengine1::{Appengine, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Appengine::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_applications_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Appengine<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about an application.
    /// 
    /// # Arguments
    ///
    /// * `projectsId` - Part of `name`. Name of the Application resource to get. Example: apps/myapp.
    /// * `locationsId` - Part of `name`. See documentation of `projectsId`.
    /// * `applicationsId` - Part of `name`. See documentation of `projectsId`.
    pub fn locations_applications_get(&self, projects_id: &str, locations_id: &str, applications_id: &str) -> ProjectLocationApplicationGetCall<'a, S> {
        ProjectLocationApplicationGetCall {
            hub: self.hub,
            _projects_id: projects_id.to_string(),
            _locations_id: locations_id.to_string(),
            _applications_id: applications_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



