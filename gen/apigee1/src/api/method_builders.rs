use super::*;
/// A builder providing access to all methods supported on *hybrid* resources.
/// It is not used directly, but through the [`Apigee`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apigee1 as apigee1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apigee1::{Apigee, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Apigee::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `issuers_list(...)`
/// // to build up your call.
/// let rb = hub.hybrid();
/// # }
/// ```
pub struct HybridMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Apigee<S>,
}

impl<'a, S> client::MethodsBuilder for HybridMethods<'a, S> {}

impl<'a, S> HybridMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists hybrid services and its trusted issuers service account ids. This api is authenticated and unauthorized(allow all the users) and used by runtime authn-authz service to query control plane's issuer service account ids.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Must be of the form `hybrid/issuers`.
    pub fn issuers_list(&self, name: &str) -> HybridIssuerListCall<'a, S> {
        HybridIssuerListCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *organization* resources.
/// It is not used directly, but through the [`Apigee`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apigee1 as apigee1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apigee1::{Apigee, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Apigee::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analytics_datastores_create(...)`, `analytics_datastores_delete(...)`, `analytics_datastores_get(...)`, `analytics_datastores_list(...)`, `analytics_datastores_test(...)`, `analytics_datastores_update(...)`, `apiproducts_attributes(...)`, `apiproducts_attributes_delete(...)`, `apiproducts_attributes_get(...)`, `apiproducts_attributes_list(...)`, `apiproducts_attributes_update_api_product_attribute(...)`, `apiproducts_create(...)`, `apiproducts_delete(...)`, `apiproducts_get(...)`, `apiproducts_list(...)`, `apiproducts_rateplans_create(...)`, `apiproducts_rateplans_delete(...)`, `apiproducts_rateplans_get(...)`, `apiproducts_rateplans_list(...)`, `apiproducts_rateplans_update(...)`, `apiproducts_update(...)`, `apis_create(...)`, `apis_delete(...)`, `apis_deployments_list(...)`, `apis_get(...)`, `apis_keyvaluemaps_create(...)`, `apis_keyvaluemaps_delete(...)`, `apis_keyvaluemaps_entries_create(...)`, `apis_keyvaluemaps_entries_delete(...)`, `apis_keyvaluemaps_entries_get(...)`, `apis_keyvaluemaps_entries_list(...)`, `apis_list(...)`, `apis_patch(...)`, `apis_revisions_delete(...)`, `apis_revisions_deployments_list(...)`, `apis_revisions_get(...)`, `apis_revisions_update_api_proxy_revision(...)`, `apps_get(...)`, `apps_list(...)`, `create(...)`, `datacollectors_create(...)`, `datacollectors_delete(...)`, `datacollectors_get(...)`, `datacollectors_list(...)`, `datacollectors_patch(...)`, `delete(...)`, `deployments_list(...)`, `developers_apps_attributes(...)`, `developers_apps_attributes_delete(...)`, `developers_apps_attributes_get(...)`, `developers_apps_attributes_list(...)`, `developers_apps_attributes_update_developer_app_attribute(...)`, `developers_apps_create(...)`, `developers_apps_delete(...)`, `developers_apps_generate_key_pair_or_update_developer_app_status(...)`, `developers_apps_get(...)`, `developers_apps_keys_apiproducts_delete(...)`, `developers_apps_keys_apiproducts_update_developer_app_key_api_product(...)`, `developers_apps_keys_create(...)`, `developers_apps_keys_create_create(...)`, `developers_apps_keys_delete(...)`, `developers_apps_keys_get(...)`, `developers_apps_keys_replace_developer_app_key(...)`, `developers_apps_keys_update_developer_app_key(...)`, `developers_apps_list(...)`, `developers_apps_update(...)`, `developers_attributes(...)`, `developers_attributes_delete(...)`, `developers_attributes_get(...)`, `developers_attributes_list(...)`, `developers_attributes_update_developer_attribute(...)`, `developers_balance_adjust(...)`, `developers_balance_credit(...)`, `developers_create(...)`, `developers_delete(...)`, `developers_get(...)`, `developers_get_balance(...)`, `developers_get_monetization_config(...)`, `developers_list(...)`, `developers_set_developer_status(...)`, `developers_subscriptions_create(...)`, `developers_subscriptions_expire(...)`, `developers_subscriptions_get(...)`, `developers_subscriptions_list(...)`, `developers_update(...)`, `developers_update_monetization_config(...)`, `endpoint_attachments_create(...)`, `endpoint_attachments_delete(...)`, `endpoint_attachments_get(...)`, `endpoint_attachments_list(...)`, `envgroups_attachments_create(...)`, `envgroups_attachments_delete(...)`, `envgroups_attachments_get(...)`, `envgroups_attachments_list(...)`, `envgroups_create(...)`, `envgroups_delete(...)`, `envgroups_get(...)`, `envgroups_get_deployed_ingress_config(...)`, `envgroups_list(...)`, `envgroups_patch(...)`, `environments_analytics_admin_get_schemav2(...)`, `environments_analytics_exports_create(...)`, `environments_analytics_exports_get(...)`, `environments_analytics_exports_list(...)`, `environments_apis_deployments_list(...)`, `environments_apis_revisions_debugsessions_create(...)`, `environments_apis_revisions_debugsessions_data_get(...)`, `environments_apis_revisions_debugsessions_delete_data(...)`, `environments_apis_revisions_debugsessions_get(...)`, `environments_apis_revisions_debugsessions_list(...)`, `environments_apis_revisions_deploy(...)`, `environments_apis_revisions_deployments_generate_deploy_change_report(...)`, `environments_apis_revisions_deployments_generate_undeploy_change_report(...)`, `environments_apis_revisions_get_deployments(...)`, `environments_apis_revisions_undeploy(...)`, `environments_archive_deployments_create(...)`, `environments_archive_deployments_delete(...)`, `environments_archive_deployments_generate_download_url(...)`, `environments_archive_deployments_generate_upload_url(...)`, `environments_archive_deployments_get(...)`, `environments_archive_deployments_list(...)`, `environments_archive_deployments_patch(...)`, `environments_caches_delete(...)`, `environments_create(...)`, `environments_delete(...)`, `environments_deployments_list(...)`, `environments_flowhooks_attach_shared_flow_to_flow_hook(...)`, `environments_flowhooks_detach_shared_flow_from_flow_hook(...)`, `environments_flowhooks_get(...)`, `environments_get(...)`, `environments_get_api_security_runtime_config(...)`, `environments_get_debugmask(...)`, `environments_get_deployed_config(...)`, `environments_get_iam_policy(...)`, `environments_get_trace_config(...)`, `environments_keystores_aliases_create(...)`, `environments_keystores_aliases_csr(...)`, `environments_keystores_aliases_delete(...)`, `environments_keystores_aliases_get(...)`, `environments_keystores_aliases_get_certificate(...)`, `environments_keystores_aliases_update(...)`, `environments_keystores_create(...)`, `environments_keystores_delete(...)`, `environments_keystores_get(...)`, `environments_keyvaluemaps_create(...)`, `environments_keyvaluemaps_delete(...)`, `environments_keyvaluemaps_entries_create(...)`, `environments_keyvaluemaps_entries_delete(...)`, `environments_keyvaluemaps_entries_get(...)`, `environments_keyvaluemaps_entries_list(...)`, `environments_modify_environment(...)`, `environments_optimized_stats_get(...)`, `environments_queries_create(...)`, `environments_queries_get(...)`, `environments_queries_get_result(...)`, `environments_queries_get_resulturl(...)`, `environments_queries_list(...)`, `environments_references_create(...)`, `environments_references_delete(...)`, `environments_references_get(...)`, `environments_references_update(...)`, `environments_resourcefiles_create(...)`, `environments_resourcefiles_delete(...)`, `environments_resourcefiles_get(...)`, `environments_resourcefiles_list(...)`, `environments_resourcefiles_list_environment_resources(...)`, `environments_resourcefiles_update(...)`, `environments_security_reports_create(...)`, `environments_security_reports_get(...)`, `environments_security_reports_get_result(...)`, `environments_security_reports_get_result_view(...)`, `environments_security_reports_list(...)`, `environments_security_stats_query_tabular_stats(...)`, `environments_security_stats_query_time_series_stats(...)`, `environments_set_iam_policy(...)`, `environments_sharedflows_deployments_list(...)`, `environments_sharedflows_revisions_deploy(...)`, `environments_sharedflows_revisions_get_deployments(...)`, `environments_sharedflows_revisions_undeploy(...)`, `environments_stats_get(...)`, `environments_subscribe(...)`, `environments_targetservers_create(...)`, `environments_targetservers_delete(...)`, `environments_targetservers_get(...)`, `environments_targetservers_update(...)`, `environments_test_iam_permissions(...)`, `environments_trace_config_overrides_create(...)`, `environments_trace_config_overrides_delete(...)`, `environments_trace_config_overrides_get(...)`, `environments_trace_config_overrides_list(...)`, `environments_trace_config_overrides_patch(...)`, `environments_unsubscribe(...)`, `environments_update(...)`, `environments_update_debugmask(...)`, `environments_update_environment(...)`, `environments_update_trace_config(...)`, `get(...)`, `get_deployed_ingress_config(...)`, `get_project_mapping(...)`, `get_runtime_config(...)`, `get_sync_authorization(...)`, `host_queries_create(...)`, `host_queries_get(...)`, `host_queries_get_result(...)`, `host_queries_get_result_view(...)`, `host_queries_list(...)`, `host_security_reports_create(...)`, `host_security_reports_get(...)`, `host_security_reports_get_result(...)`, `host_security_reports_get_result_view(...)`, `host_security_reports_list(...)`, `host_stats_get(...)`, `instances_attachments_create(...)`, `instances_attachments_delete(...)`, `instances_attachments_get(...)`, `instances_attachments_list(...)`, `instances_canaryevaluations_create(...)`, `instances_canaryevaluations_get(...)`, `instances_create(...)`, `instances_delete(...)`, `instances_get(...)`, `instances_list(...)`, `instances_nat_addresses_activate(...)`, `instances_nat_addresses_create(...)`, `instances_nat_addresses_delete(...)`, `instances_nat_addresses_get(...)`, `instances_nat_addresses_list(...)`, `instances_patch(...)`, `instances_report_status(...)`, `keyvaluemaps_create(...)`, `keyvaluemaps_delete(...)`, `keyvaluemaps_entries_create(...)`, `keyvaluemaps_entries_delete(...)`, `keyvaluemaps_entries_get(...)`, `keyvaluemaps_entries_list(...)`, `list(...)`, `operations_get(...)`, `operations_list(...)`, `optimized_host_stats_get(...)`, `reports_create(...)`, `reports_delete(...)`, `reports_get(...)`, `reports_list(...)`, `reports_update(...)`, `security_profiles_environments_compute_environment_scores(...)`, `security_profiles_environments_create(...)`, `security_profiles_environments_delete(...)`, `security_profiles_get(...)`, `security_profiles_list(...)`, `security_profiles_list_revisions(...)`, `set_addons(...)`, `set_sync_authorization(...)`, `sharedflows_create(...)`, `sharedflows_delete(...)`, `sharedflows_deployments_list(...)`, `sharedflows_get(...)`, `sharedflows_list(...)`, `sharedflows_revisions_delete(...)`, `sharedflows_revisions_deployments_list(...)`, `sharedflows_revisions_get(...)`, `sharedflows_revisions_update_shared_flow_revision(...)`, `sites_apicategories_create(...)`, `sites_apicategories_delete(...)`, `sites_apicategories_get(...)`, `sites_apicategories_list(...)`, `sites_apicategories_patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.organizations();
/// # }
/// ```
pub struct OrganizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Apigee<S>,
}

impl<'a, S> client::MethodsBuilder for OrganizationMethods<'a, S> {}

impl<'a, S> OrganizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Create a Datastore for an org
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent organization name. Must be of the form `organizations/{org}`.
    pub fn analytics_datastores_create(&self, request: GoogleCloudApigeeV1Datastore, parent: &str) -> OrganizationAnalyticDatastoreCreateCall<'a, S> {
        OrganizationAnalyticDatastoreCreateCall {
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
    /// Delete a Datastore from an org.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the Datastore to be deleted. Must be of the form `organizations/{org}/analytics/datastores/{datastoreId}`
    pub fn analytics_datastores_delete(&self, name: &str) -> OrganizationAnalyticDatastoreDeleteCall<'a, S> {
        OrganizationAnalyticDatastoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a Datastore
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the Datastore to be get. Must be of the form `organizations/{org}/analytics/datastores/{datastoreId}`
    pub fn analytics_datastores_get(&self, name: &str) -> OrganizationAnalyticDatastoreGetCall<'a, S> {
        OrganizationAnalyticDatastoreGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Datastores
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent organization name. Must be of the form `organizations/{org}`.
    pub fn analytics_datastores_list(&self, parent: &str) -> OrganizationAnalyticDatastoreListCall<'a, S> {
        OrganizationAnalyticDatastoreListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _target_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Test if Datastore configuration is correct. This includes checking if credentials provided by customer have required permissions in target destination storage
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent organization name Must be of the form `organizations/{org}`
    pub fn analytics_datastores_test(&self, request: GoogleCloudApigeeV1Datastore, parent: &str) -> OrganizationAnalyticDatastoreTestCall<'a, S> {
        OrganizationAnalyticDatastoreTestCall {
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
    /// Update a Datastore
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of datastore to be updated. Must be of the form `organizations/{org}/analytics/datastores/{datastoreId}`
    pub fn analytics_datastores_update(&self, request: GoogleCloudApigeeV1Datastore, name: &str) -> OrganizationAnalyticDatastoreUpdateCall<'a, S> {
        OrganizationAnalyticDatastoreUpdateCall {
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
    /// Deletes an API product attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API product attribute. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}/attributes/{attribute}`
    pub fn apiproducts_attributes_delete(&self, name: &str) -> OrganizationApiproductAttributeDeleteCall<'a, S> {
        OrganizationApiproductAttributeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the value of an API product attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API product attribute. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}/attributes/{attribute}`
    pub fn apiproducts_attributes_get(&self, name: &str) -> OrganizationApiproductAttributeGetCall<'a, S> {
        OrganizationApiproductAttributeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all API product attributes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_attributes_list(&self, parent: &str) -> OrganizationApiproductAttributeListCall<'a, S> {
        OrganizationApiproductAttributeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the value of an API product attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with entities also get cached for at least 180 seconds after entity is accessed during runtime. In this case, the `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_attributes_update_api_product_attribute(&self, request: GoogleCloudApigeeV1Attribute, name: &str) -> OrganizationApiproductAttributeUpdateApiProductAttributeCall<'a, S> {
        OrganizationApiproductAttributeUpdateApiProductAttributeCall {
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
    /// Create a rate plan that is associated with an API product in an organization. Using rate plans, API product owners can monetize their API products by configuring one or more of the following: - Billing frequency - Initial setup fees for using an API product - Payment funding model (postpaid only) - Fixed recurring or consumption-based charges for using an API product - Revenue sharing with developer partners An API product can have multiple rate plans associated with it but *only one* rate plan can be active at any point of time. **Note: From the developer's perspective, they purchase API products not rate plans.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the API product that is associated with the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_rateplans_create(&self, request: GoogleCloudApigeeV1RatePlan, parent: &str) -> OrganizationApiproductRateplanCreateCall<'a, S> {
        OrganizationApiproductRateplanCreateCall {
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
    /// Deletes a rate plan.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. ID of the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}/rateplans/{rateplan}`
    pub fn apiproducts_rateplans_delete(&self, name: &str) -> OrganizationApiproductRateplanDeleteCall<'a, S> {
        OrganizationApiproductRateplanDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a rate plan.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}/rateplans/{rateplan}`
    pub fn apiproducts_rateplans_get(&self, name: &str) -> OrganizationApiproductRateplanGetCall<'a, S> {
        OrganizationApiproductRateplanGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the rate plans for an API product.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}` Use `organizations/{org}/apiproducts/-` to return rate plans for all API products within the organization.
    pub fn apiproducts_rateplans_list(&self, parent: &str) -> OrganizationApiproductRateplanListCall<'a, S> {
        OrganizationApiproductRateplanListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _state: Default::default(),
            _start_key: Default::default(),
            _order_by: Default::default(),
            _expand: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing rate plan.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}/rateplans/{rateplan}`
    pub fn apiproducts_rateplans_update(&self, request: GoogleCloudApigeeV1RatePlan, name: &str) -> OrganizationApiproductRateplanUpdateCall<'a, S> {
        OrganizationApiproductRateplanUpdateCall {
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
    /// Updates or creates API product attributes. This API **replaces** the current list of attributes with the attributes specified in the request body. In this way, you can update existing attributes, add new attributes, or delete existing attributes by omitting them from the request body. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with entities also get cached for at least 180 seconds after entity is accessed during runtime. In this case, the `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_attributes(&self, request: GoogleCloudApigeeV1Attributes, name: &str) -> OrganizationApiproductAttributeCall<'a, S> {
        OrganizationApiproductAttributeCall {
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
    /// Creates an API product in an organization. You create API products after you have proxied backend services using API proxies. An API product is a collection of API resources combined with quota settings and metadata that you can use to deliver customized and productized API bundles to your developer community. This metadata can include: - Scope - Environments - API proxies - Extensible profile API products enable you repackage APIs on the fly, without having to do any additional coding or configuration. Apigee recommends that you start with a simple API product including only required elements. You then provision credentials to apps to enable them to start testing your APIs. After you have authentication and authorization working against a simple API product, you can iterate to create finer-grained API products, defining different sets of API resources for each API product. **WARNING:** - If you don't specify an API proxy in the request body, *any* app associated with the product can make calls to *any* API in your entire organization. - If you don't specify an environment in the request body, the product allows access to all environments. For more information, see What is an API product?
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in which the API product will be created. Use the following structure in your request: `organizations/{org}`
    pub fn apiproducts_create(&self, request: GoogleCloudApigeeV1ApiProduct, parent: &str) -> OrganizationApiproductCreateCall<'a, S> {
        OrganizationApiproductCreateCall {
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
    /// Deletes an API product from an organization. Deleting an API product causes app requests to the resource URIs defined in the API product to fail. Ensure that you create a new API product to serve existing apps, unless your intention is to disable access to the resources defined in the API product. The API product name required in the request URL is the internal name of the product, not the display name. While they may be the same, it depends on whether the API product was created via the UI or the API. View the list of API products to verify the internal name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_delete(&self, name: &str) -> OrganizationApiproductDeleteCall<'a, S> {
        OrganizationApiproductDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets configuration details for an API product. The API product name required in the request URL is the internal name of the product, not the display name. While they may be the same, it depends on whether the API product was created via the UI or the API. View the list of API products to verify the internal name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_get(&self, name: &str) -> OrganizationApiproductGetCall<'a, S> {
        OrganizationApiproductGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all API product names for an organization. Filter the list by passing an `attributename` and `attibutevalue`. The maximum number of API products returned is 1000. You can paginate the list of API products returned using the `startKey` and `count` query parameters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}`
    pub fn apiproducts_list(&self, parent: &str) -> OrganizationApiproductListCall<'a, S> {
        OrganizationApiproductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_key: Default::default(),
            _expand: Default::default(),
            _count: Default::default(),
            _attributevalue: Default::default(),
            _attributename: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing API product. You must include all required values, whether or not you are updating them, as well as any optional values that you are updating. The API product name required in the request URL is the internal name of the product, not the display name. While they may be the same, it depends on whether the API product was created via UI or API. View the list of API products to identify their internal names.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the API product. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}`
    pub fn apiproducts_update(&self, request: GoogleCloudApigeeV1ApiProduct, name: &str) -> OrganizationApiproductUpdateCall<'a, S> {
        OrganizationApiproductUpdateCall {
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
    /// Lists all deployments of an API proxy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the API proxy for which to return deployment information in the following format: `organizations/{org}/apis/{api}`
    pub fn apis_deployments_list(&self, parent: &str) -> OrganizationApiDeploymentListCall<'a, S> {
        OrganizationApiDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates key value entries in a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Scope as indicated by the URI in which to create the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn apis_keyvaluemaps_entries_create(&self, request: GoogleCloudApigeeV1KeyValueEntry, parent: &str) -> OrganizationApiKeyvaluemapEntryCreateCall<'a, S> {
        OrganizationApiKeyvaluemapEntryCreateCall {
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
    /// Deletes a key value entry from a key value map scoped to an organization, environment, or API proxy. **Notes:** * After you delete the key value entry, the policy consuming the entry will continue to function with its cached values for a few minutes. This is expected behavior. * Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to delete the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn apis_keyvaluemaps_entries_delete(&self, name: &str) -> OrganizationApiKeyvaluemapEntryDeleteCall<'a, S> {
        OrganizationApiKeyvaluemapEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the key value entry value for a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to fetch the key value map entry/value. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn apis_keyvaluemaps_entries_get(&self, name: &str) -> OrganizationApiKeyvaluemapEntryGetCall<'a, S> {
        OrganizationApiKeyvaluemapEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists key value entries for key values maps scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Scope as indicated by the URI in which to list key value maps. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn apis_keyvaluemaps_entries_list(&self, parent: &str) -> OrganizationApiKeyvaluemapEntryListCall<'a, S> {
        OrganizationApiKeyvaluemapEntryListCall {
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
    /// Creates a key value map in an API proxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment in which to create the key value map. Use the following structure in your request: `organizations/{org}/apis/{api}`
    pub fn apis_keyvaluemaps_create(&self, request: GoogleCloudApigeeV1KeyValueMap, parent: &str) -> OrganizationApiKeyvaluemapCreateCall<'a, S> {
        OrganizationApiKeyvaluemapCreateCall {
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
    /// Deletes a key value map from an API proxy.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the key value map. Use the following structure in your request: `organizations/{org}/apis/{api}/keyvaluemaps/{keyvaluemap}`
    pub fn apis_keyvaluemaps_delete(&self, name: &str) -> OrganizationApiKeyvaluemapDeleteCall<'a, S> {
        OrganizationApiKeyvaluemapDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of an API proxy revision.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the API proxy revision for which to return deployment information in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`.
    pub fn apis_revisions_deployments_list(&self, parent: &str) -> OrganizationApiRevisionDeploymentListCall<'a, S> {
        OrganizationApiRevisionDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an API proxy revision and all policies, resources, endpoints, and revisions associated with it. The API proxy revision must be undeployed before you can delete it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. API proxy revision in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`
    pub fn apis_revisions_delete(&self, name: &str) -> OrganizationApiRevisionDeleteCall<'a, S> {
        OrganizationApiRevisionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an API proxy revision. To download the API proxy configuration bundle for the specified revision as a zip file, set the `format` query parameter to `bundle`. If you are using curl, specify `-o filename.zip` to save the output to a file; otherwise, it displays to `stdout`. Then, develop the API proxy configuration locally and upload the updated API proxy configuration revision, as described in [updateApiProxyRevision](updateApiProxyRevision).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. API proxy revision in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`
    pub fn apis_revisions_get(&self, name: &str) -> OrganizationApiRevisionGetCall<'a, S> {
        OrganizationApiRevisionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing API proxy revision by uploading the API proxy configuration bundle as a zip file from your local machine. You can update only API proxy revisions that have never been deployed. After deployment, an API proxy revision becomes immutable, even if it is undeployed. Set the `Content-Type` header to either `multipart/form-data` or `application/octet-stream`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. API proxy revision to update in the following format: `organizations/{org}/apis/{api}/revisions/{rev}`
    pub fn apis_revisions_update_api_proxy_revision(&self, request: GoogleApiHttpBody, name: &str) -> OrganizationApiRevisionUpdateApiProxyRevisionCall<'a, S> {
        OrganizationApiRevisionUpdateApiProxyRevisionCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an API proxy. The API proxy created will not be accessible at runtime until it is deployed to an environment. Create a new API proxy by setting the `name` query parameter to the name of the API proxy. Import an API proxy configuration bundle stored in zip format on your local machine to your organization by doing the following: * Set the `name` query parameter to the name of the API proxy. * Set the `action` query parameter to `import`. * Set the `Content-Type` header to `multipart/form-data`. * Pass as a file the name of API proxy configuration bundle stored in zip format on your local machine using the `file` form field. **Note**: To validate the API proxy configuration bundle only without importing it, set the `action` query parameter to `validate`. When importing an API proxy configuration bundle, if the API proxy does not exist, it will be created. If the API proxy exists, then a new revision is created. Invalid API proxy configurations are rejected, and a list of validation errors is returned to the client.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in the following format: `organizations/{org}`
    pub fn apis_create(&self, request: GoogleApiHttpBody, parent: &str) -> OrganizationApiCreateCall<'a, S> {
        OrganizationApiCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate: Default::default(),
            _name: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an API proxy and all associated endpoints, policies, resources, and revisions. The API proxy must be undeployed before you can delete it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API proxy in the following format: `organizations/{org}/apis/{api}`
    pub fn apis_delete(&self, name: &str) -> OrganizationApiDeleteCall<'a, S> {
        OrganizationApiDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an API proxy including a list of existing revisions.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API proxy in the following format: `organizations/{org}/apis/{api}`
    pub fn apis_get(&self, name: &str) -> OrganizationApiGetCall<'a, S> {
        OrganizationApiGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the names of all API proxies in an organization. The names returned correspond to the names defined in the configuration files for each API proxy.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization in the following format: `organizations/{org}`
    pub fn apis_list(&self, parent: &str) -> OrganizationApiListCall<'a, S> {
        OrganizationApiListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _include_revisions: Default::default(),
            _include_meta_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing API proxy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. API proxy to update in the following format: `organizations/{org}/apis/{api}`
    pub fn apis_patch(&self, request: GoogleCloudApigeeV1ApiProxy, name: &str) -> OrganizationApiPatchCall<'a, S> {
        OrganizationApiPatchCall {
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
    /// Gets the app profile for the specified app ID.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. App ID in the following format: `organizations/{org}/apps/{app}`
    pub fn apps_get(&self, name: &str) -> OrganizationAppGetCall<'a, S> {
        OrganizationAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists IDs of apps within an organization that have the specified app status (approved or revoked) or are of the specified app type (developer or company).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource path of the parent in the following format: `organizations/{org}`
    pub fn apps_list(&self, parent: &str) -> OrganizationAppListCall<'a, S> {
        OrganizationAppListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _status: Default::default(),
            _start_key: Default::default(),
            _rows: Default::default(),
            _key_status: Default::default(),
            _include_cred: Default::default(),
            _ids: Default::default(),
            _expand: Default::default(),
            _apptype: Default::default(),
            _api_product: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new data collector.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in which to create the data collector in the following format: `organizations/{org}`.
    pub fn datacollectors_create(&self, request: GoogleCloudApigeeV1DataCollector, parent: &str) -> OrganizationDatacollectorCreateCall<'a, S> {
        OrganizationDatacollectorCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _data_collector_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a data collector.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the data collector in the following format: `organizations/{org}/datacollectors/{data_collector_id}`.
    pub fn datacollectors_delete(&self, name: &str) -> OrganizationDatacollectorDeleteCall<'a, S> {
        OrganizationDatacollectorDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a data collector.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the data collector in the following format: `organizations/{org}/datacollectors/{data_collector_id}`.
    pub fn datacollectors_get(&self, name: &str) -> OrganizationDatacollectorGetCall<'a, S> {
        OrganizationDatacollectorGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all data collectors.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization for which to list data collectors in the following format: `organizations/{org}`.
    pub fn datacollectors_list(&self, parent: &str) -> OrganizationDatacollectorListCall<'a, S> {
        OrganizationDatacollectorListCall {
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
    /// Updates a data collector.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the data collector in the following format: `organizations/{org}/datacollectors/{data_collector_id}`.
    pub fn datacollectors_patch(&self, request: GoogleCloudApigeeV1DataCollector, name: &str) -> OrganizationDatacollectorPatchCall<'a, S> {
        OrganizationDatacollectorPatchCall {
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
    /// Lists all deployments of API proxies or shared flows.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization for which to return deployment information in the following format: `organizations/{org}`
    pub fn deployments_list(&self, parent: &str) -> OrganizationDeploymentListCall<'a, S> {
        OrganizationDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _shared_flows: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a developer app attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer app attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/attributes/{attribute}`
    pub fn developers_apps_attributes_delete(&self, name: &str) -> OrganizationDeveloperAppAttributeDeleteCall<'a, S> {
        OrganizationDeveloperAppAttributeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a developer app attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer app attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/attributes/{attribute}`
    pub fn developers_apps_attributes_get(&self, name: &str) -> OrganizationDeveloperAppAttributeGetCall<'a, S> {
        OrganizationDeveloperAppAttributeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all developer app attributes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_attributes_list(&self, parent: &str) -> OrganizationDeveloperAppAttributeListCall<'a, S> {
        OrganizationDeveloperAppAttributeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a developer app attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the developer app attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/attributes/{attribute}`
    pub fn developers_apps_attributes_update_developer_app_attribute(&self, request: GoogleCloudApigeeV1Attribute, name: &str) -> OrganizationDeveloperAppAttributeUpdateDeveloperAppAttributeCall<'a, S> {
        OrganizationDeveloperAppAttributeUpdateDeveloperAppAttributeCall {
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
    /// Removes an API product from an app's consumer key. After the API product is removed, the app cannot access the API resources defined in that API product. **Note**: The consumer key is not removed, only its association with the API product.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the API product in the developer app key in the following format: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}/apiproducts/{apiproduct}`
    pub fn developers_apps_keys_apiproducts_delete(&self, name: &str) -> OrganizationDeveloperAppKeyApiproductDeleteCall<'a, S> {
        OrganizationDeveloperAppKeyApiproductDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Approves or revokes the consumer key for an API product. After a consumer key is approved, the app can use it to access APIs. A consumer key that is revoked or pending cannot be used to access an API. Any access tokens associated with a revoked consumer key will remain active. However, Apigee checks the status of the consumer key and if set to `revoked` will not allow access to the API.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the API product in the developer app key in the following format: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}/apiproducts/{apiproduct}`
    pub fn developers_apps_keys_apiproducts_update_developer_app_key_api_product(&self, name: &str) -> OrganizationDeveloperAppKeyApiproductUpdateDeveloperAppKeyApiProductCall<'a, S> {
        OrganizationDeveloperAppKeyApiproductUpdateDeveloperAppKeyApiProductCall {
            hub: self.hub,
            _name: name.to_string(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps`
    pub fn developers_apps_keys_create_create(&self, request: GoogleCloudApigeeV1DeveloperAppKey, parent: &str) -> OrganizationDeveloperAppKeyCreateCreateCall<'a, S> {
        OrganizationDeveloperAppKeyCreateCreateCall {
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
    /// Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps`
    pub fn developers_apps_keys_create(&self, request: GoogleCloudApigeeV1DeveloperAppKey, parent: &str) -> OrganizationDeveloperAppKeyCreateCall<'a, S> {
        OrganizationDeveloperAppKeyCreateCall {
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
    /// Deletes an app's consumer key and removes all API products associated with the app. After the consumer key is deleted, it cannot be used to access any APIs. **Note**: After you delete a consumer key, you may want to: 1. Create a new consumer key and secret for the developer app using the CreateDeveloperAppKey API, and subsequently add an API product to the key using the UpdateDeveloperAppKey API. 2. Delete the developer app, if it is no longer required.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}`
    pub fn developers_apps_keys_delete(&self, name: &str) -> OrganizationDeveloperAppKeyDeleteCall<'a, S> {
        OrganizationDeveloperAppKeyDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details for a consumer key for a developer app, including the key and secret value, associated API products, and other information.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}`
    pub fn developers_apps_keys_get(&self, name: &str) -> OrganizationDeveloperAppKeyGetCall<'a, S> {
        OrganizationDeveloperAppKeyGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the scope of an app. This API replaces the existing scopes with those specified in the request. Include or exclude any existing scopes that you want to retain or delete, respectively. The specified scopes must already be defined for the API products associated with the app. This API sets the `scopes` element under the `apiProducts` element in the attributes of the app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}`
    pub fn developers_apps_keys_replace_developer_app_key(&self, request: GoogleCloudApigeeV1DeveloperAppKey, name: &str) -> OrganizationDeveloperAppKeyReplaceDeveloperAppKeyCall<'a, S> {
        OrganizationDeveloperAppKeyReplaceDeveloperAppKeyCall {
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
    /// Adds an API product to a developer app key, enabling the app that holds the key to access the API resources bundled in the API product. In addition, you can add attributes to a developer app key. This API replaces the existing attributes with those specified in the request. Include or exclude any existing attributes that you want to retain or delete, respectively. You can use the same key to access all API products associated with the app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the developer app key. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}/keys/{key}`
    pub fn developers_apps_keys_update_developer_app_key(&self, request: GoogleCloudApigeeV1DeveloperAppKey, name: &str) -> OrganizationDeveloperAppKeyUpdateDeveloperAppKeyCall<'a, S> {
        OrganizationDeveloperAppKeyUpdateDeveloperAppKeyCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates attributes for a developer app. This API replaces the current attributes with those specified in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_attributes(&self, request: GoogleCloudApigeeV1Attributes, name: &str) -> OrganizationDeveloperAppAttributeCall<'a, S> {
        OrganizationDeveloperAppAttributeCall {
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
    /// Creates an app associated with a developer. This API associates the developer app with the specified API product and auto-generates an API key for the app to use in calls to API proxies inside that API product. The `name` is the unique ID of the app that you can use in API calls. The `DisplayName` (set as an attribute) appears in the UI. If you don't set the `DisplayName` attribute, the `name` appears in the UI.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_apps_create(&self, request: GoogleCloudApigeeV1DeveloperApp, parent: &str) -> OrganizationDeveloperAppCreateCall<'a, S> {
        OrganizationDeveloperAppCreateCall {
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
    /// Deletes a developer app. **Note**: The delete operation is asynchronous. The developer app is deleted immediately, but its associated resources, such as app keys or access tokens, may take anywhere from a few seconds to a few minutes to be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_delete(&self, name: &str) -> OrganizationDeveloperAppDeleteCall<'a, S> {
        OrganizationDeveloperAppDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Manages access to a developer app by enabling you to: * Approve or revoke a developer app * Generate a new consumer key and secret for a developer app To approve or revoke a developer app, set the `action` query parameter to `approve` or `revoke`, respectively, and the `Content-Type` header to `application/octet-stream`. If a developer app is revoked, none of its API keys are valid for API calls even though the keys are still approved. If successful, the API call returns the following HTTP status code: `204 No Content` To generate a new consumer key and secret for a developer app, pass the new key/secret details. Rather than replace an existing key, this API generates a new key. In this case, multiple key pairs may be associated with a single developer app. Each key pair has an independent status (`approve` or `revoke`) and expiration time. Any approved, non-expired key can be used in an API call. For example, if you're using API key rotation, you can generate new keys with expiration times that overlap keys that are going to expire. You might also generate a new consumer key/secret if the security of the original key/secret is compromised. The `keyExpiresIn` property defines the expiration time for the API key in milliseconds. If you don't set this property or set it to `-1`, the API key never expires. **Notes**: * When generating a new key/secret, this API replaces the existing attributes, notes, and callback URLs with those specified in the request. Include or exclude any existing information that you want to retain or delete, respectively. * To migrate existing consumer keys and secrets to hybrid from another system, see the CreateDeveloperAppKey API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_generate_key_pair_or_update_developer_app_status(&self, request: GoogleCloudApigeeV1DeveloperApp, name: &str) -> OrganizationDeveloperAppGenerateKeyPairOrUpdateDeveloperAppStatuCall<'a, S> {
        OrganizationDeveloperAppGenerateKeyPairOrUpdateDeveloperAppStatuCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the details for a developer app.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_get(&self, name: &str) -> OrganizationDeveloperAppGetCall<'a, S> {
        OrganizationDeveloperAppGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _query: Default::default(),
            _entity: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all apps created by a developer in an Apigee organization. Optionally, you can request an expanded view of the developer apps. A maximum of 100 developer apps are returned per API call. You can paginate the list of deveoper apps returned using the `startKey` and `count` query parameters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_apps_list(&self, parent: &str) -> OrganizationDeveloperAppListCall<'a, S> {
        OrganizationDeveloperAppListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_key: Default::default(),
            _shallow_expand: Default::default(),
            _expand: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the details for a developer app. In addition, you can add an API product to a developer app and automatically generate an API key for the app to use when calling APIs in the API product. If you want to use an existing API key for the API product, add the API product to the API key using the UpdateDeveloperAppKey API. Using this API, you cannot update the following: * App name as it is the primary key used to identify the app and cannot be changed. * Scopes associated with the app. Instead, use the ReplaceDeveloperAppKey API. This API replaces the existing attributes with those specified in the request. Include or exclude any existing attributes that you want to retain or delete, respectively.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the developer app. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/apps/{app}`
    pub fn developers_apps_update(&self, request: GoogleCloudApigeeV1DeveloperApp, name: &str) -> OrganizationDeveloperAppUpdateCall<'a, S> {
        OrganizationDeveloperAppUpdateCall {
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
    /// Deletes a developer attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/attributes/{attribute}`
    pub fn developers_attributes_delete(&self, name: &str) -> OrganizationDeveloperAttributeDeleteCall<'a, S> {
        OrganizationDeveloperAttributeDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the value of the specified developer attribute.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/attributes/{attribute}`
    pub fn developers_attributes_get(&self, name: &str) -> OrganizationDeveloperAttributeGetCall<'a, S> {
        OrganizationDeveloperAttributeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all developer attributes.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Email address of the developer for which attributes are being listed. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_attributes_list(&self, parent: &str) -> OrganizationDeveloperAttributeListCall<'a, S> {
        OrganizationDeveloperAttributeListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a developer attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the developer attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/attributes/{attribute}`
    pub fn developers_attributes_update_developer_attribute(&self, request: GoogleCloudApigeeV1Attribute, name: &str) -> OrganizationDeveloperAttributeUpdateDeveloperAttributeCall<'a, S> {
        OrganizationDeveloperAttributeUpdateDeveloperAttributeCall {
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
    /// Adjust the prepaid balance for the developer. This API will be used in scenarios where the developer has been under-charged or over-charged.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Account balance for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/balance`
    pub fn developers_balance_adjust(&self, request: GoogleCloudApigeeV1AdjustDeveloperBalanceRequest, name: &str) -> OrganizationDeveloperBalanceAdjustCall<'a, S> {
        OrganizationDeveloperBalanceAdjustCall {
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
    /// Credits the account balance for the developer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Account balance for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/balance`
    pub fn developers_balance_credit(&self, request: GoogleCloudApigeeV1CreditDeveloperBalanceRequest, name: &str) -> OrganizationDeveloperBalanceCreditCall<'a, S> {
        OrganizationDeveloperBalanceCreditCall {
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
    /// Creates a subscription to an API product. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Email address of the developer that is purchasing a subscription to the API product. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_subscriptions_create(&self, request: GoogleCloudApigeeV1DeveloperSubscription, parent: &str) -> OrganizationDeveloperSubscriptionCreateCall<'a, S> {
        OrganizationDeveloperSubscriptionCreateCall {
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
    /// Expires an API product subscription immediately.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the API product subscription. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/subscriptions/{subscription}`
    pub fn developers_subscriptions_expire(&self, request: GoogleCloudApigeeV1ExpireDeveloperSubscriptionRequest, name: &str) -> OrganizationDeveloperSubscriptionExpireCall<'a, S> {
        OrganizationDeveloperSubscriptionExpireCall {
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
    /// Gets details for an API product subscription.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API product subscription. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/subscriptions/{subscription}`
    pub fn developers_subscriptions_get(&self, name: &str) -> OrganizationDeveloperSubscriptionGetCall<'a, S> {
        OrganizationDeveloperSubscriptionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all API product subscriptions for a developer.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Email address of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_subscriptions_list(&self, parent: &str) -> OrganizationDeveloperSubscriptionListCall<'a, S> {
        OrganizationDeveloperSubscriptionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_key: Default::default(),
            _count: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates developer attributes. This API replaces the existing attributes with those specified in the request. Add new attributes, and include or exclude any existing attributes that you want to retain or remove, respectively. The custom attribute limit is 18. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Email address of the developer for which attributes are being updated. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_attributes(&self, request: GoogleCloudApigeeV1Attributes, parent: &str) -> OrganizationDeveloperAttributeCall<'a, S> {
        OrganizationDeveloperAttributeCall {
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
    /// Creates a developer. Once created, the developer can register an app and obtain an API key. At creation time, a developer is set as `active`. To change the developer status, use the SetDeveloperStatus API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the Apigee organization in which the developer is created. Use the following structure in your request: `organizations/{org}`.
    pub fn developers_create(&self, request: GoogleCloudApigeeV1Developer, parent: &str) -> OrganizationDeveloperCreateCall<'a, S> {
        OrganizationDeveloperCreateCall {
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
    /// Deletes a developer. All apps and API keys associated with the developer are also removed. **Warning**: This API will permanently delete the developer and related artifacts. To avoid permanently deleting developers and their artifacts, set the developer status to `inactive` using the SetDeveloperStatus API. **Note**: The delete operation is asynchronous. The developer app is deleted immediately, but its associated resources, such as apps and API keys, may take anywhere from a few seconds to a few minutes to be deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Email address of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_delete(&self, name: &str) -> OrganizationDeveloperDeleteCall<'a, S> {
        OrganizationDeveloperDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the developer details, including the developer's name, email address, apps, and other information. **Note**: The response includes only the first 100 developer apps.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Email address of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_get(&self, name: &str) -> OrganizationDeveloperGetCall<'a, S> {
        OrganizationDeveloperGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the account balance for the developer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Account balance for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/balance`
    pub fn developers_get_balance(&self, name: &str) -> OrganizationDeveloperGetBalanceCall<'a, S> {
        OrganizationDeveloperGetBalanceCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the monetization configuration for the developer.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Monetization configuration for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/monetizationConfig`
    pub fn developers_get_monetization_config(&self, name: &str) -> OrganizationDeveloperGetMonetizationConfigCall<'a, S> {
        OrganizationDeveloperGetMonetizationConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all developers in an organization by email address. By default, the response does not include company developers. Set the `includeCompany` query parameter to `true` to include company developers. **Note**: A maximum of 1000 developers are returned in the response. You paginate the list of developers returned using the `startKey` and `count` query parameters.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the Apigee organization. Use the following structure in your request: `organizations/{org}`.
    pub fn developers_list(&self, parent: &str) -> OrganizationDeveloperListCall<'a, S> {
        OrganizationDeveloperListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_key: Default::default(),
            _include_company: Default::default(),
            _ids: Default::default(),
            _expand: Default::default(),
            _count: Default::default(),
            _app: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the status of a developer. A developer is `active` by default. If you set a developer's status to `inactive`, the API keys assigned to the developer apps are no longer valid even though the API keys are set to `approved`. Inactive developers can still sign in to the developer portal and create apps; however, any new API keys generated during app creation won't work. To set the status of a developer, set the `action` query parameter to `active` or `inactive`, and the `Content-Type` header to `application/octet-stream`. If successful, the API call returns the following HTTP status code: `204 No Content`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_id}`
    pub fn developers_set_developer_status(&self, name: &str) -> OrganizationDeveloperSetDeveloperStatuCall<'a, S> {
        OrganizationDeveloperSetDeveloperStatuCall {
            hub: self.hub,
            _name: name.to_string(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a developer. This API replaces the existing developer details with those specified in the request. Include or exclude any existing details that you want to retain or delete, respectively. The custom attribute limit is 18. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (current default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Email address of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
    pub fn developers_update(&self, request: GoogleCloudApigeeV1Developer, name: &str) -> OrganizationDeveloperUpdateCall<'a, S> {
        OrganizationDeveloperUpdateCall {
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
    /// Updates the monetization configuration for the developer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Monetization configuration for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/monetizationConfig`
    pub fn developers_update_monetization_config(&self, request: GoogleCloudApigeeV1DeveloperMonetizationConfig, name: &str) -> OrganizationDeveloperUpdateMonetizationConfigCall<'a, S> {
        OrganizationDeveloperUpdateMonetizationConfigCall {
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
    /// Creates an endpoint attachment. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Organization the endpoint attachment will be created in.
    pub fn endpoint_attachments_create(&self, request: GoogleCloudApigeeV1EndpointAttachment, parent: &str) -> OrganizationEndpointAttachmentCreateCall<'a, S> {
        OrganizationEndpointAttachmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _endpoint_attachment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an endpoint attachment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the endpoint attachment. Use the following structure in your request: `organizations/{org}/endpointAttachments/{endpoint_attachment}`
    pub fn endpoint_attachments_delete(&self, name: &str) -> OrganizationEndpointAttachmentDeleteCall<'a, S> {
        OrganizationEndpointAttachmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the endpoint attachment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the endpoint attachment. Use the following structure in your request: `organizations/{org}/endpointAttachments/{endpoint_attachment}`
    pub fn endpoint_attachments_get(&self, name: &str) -> OrganizationEndpointAttachmentGetCall<'a, S> {
        OrganizationEndpointAttachmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the endpoint attachments in an organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization for which to list endpoint attachments. Use the following structure in your request: `organizations/{org}`
    pub fn endpoint_attachments_list(&self, parent: &str) -> OrganizationEndpointAttachmentListCall<'a, S> {
        OrganizationEndpointAttachmentListCall {
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
    /// Creates a new attachment of an environment to an environment group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. EnvironmentGroup under which to create the attachment in the following format: `organizations/{org}/envgroups/{envgroup}`.
    pub fn envgroups_attachments_create(&self, request: GoogleCloudApigeeV1EnvironmentGroupAttachment, parent: &str) -> OrganizationEnvgroupAttachmentCreateCall<'a, S> {
        OrganizationEnvgroupAttachmentCreateCall {
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
    /// Deletes an environment group attachment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment group attachment to delete in the following format: `organizations/{org}/envgroups/{envgroup}/attachments/{attachment}`.
    pub fn envgroups_attachments_delete(&self, name: &str) -> OrganizationEnvgroupAttachmentDeleteCall<'a, S> {
        OrganizationEnvgroupAttachmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an environment group attachment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment group attachment in the following format: `organizations/{org}/envgroups/{envgroup}/attachments/{attachment}`
    pub fn envgroups_attachments_get(&self, name: &str) -> OrganizationEnvgroupAttachmentGetCall<'a, S> {
        OrganizationEnvgroupAttachmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all attachments of an environment group.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`.
    pub fn envgroups_attachments_list(&self, parent: &str) -> OrganizationEnvgroupAttachmentListCall<'a, S> {
        OrganizationEnvgroupAttachmentListCall {
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
    /// Creates a new environment group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in which to create the environment group in the following format: `organizations/{org}`.
    pub fn envgroups_create(&self, request: GoogleCloudApigeeV1EnvironmentGroup, parent: &str) -> OrganizationEnvgroupCreateCall<'a, S> {
        OrganizationEnvgroupCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an environment group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`.
    pub fn envgroups_delete(&self, name: &str) -> OrganizationEnvgroupDeleteCall<'a, S> {
        OrganizationEnvgroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an environment group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment group in the following format: `organizations/{org}/envgroups/{envgroup}`.
    pub fn envgroups_get(&self, name: &str) -> OrganizationEnvgroupGetCall<'a, S> {
        OrganizationEnvgroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the deployed ingress configuration for an environment group.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the deployed configuration for the environment group in the following format: 'organizations/{org}/envgroups/{envgroup}/deployedIngressConfig'.
    pub fn envgroups_get_deployed_ingress_config(&self, name: &str) -> OrganizationEnvgroupGetDeployedIngressConfigCall<'a, S> {
        OrganizationEnvgroupGetDeployedIngressConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all environment groups.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization for which to list environment groups in the following format: `organizations/{org}`.
    pub fn envgroups_list(&self, parent: &str) -> OrganizationEnvgroupListCall<'a, S> {
        OrganizationEnvgroupListCall {
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
    /// Updates an environment group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the environment group to update in the format: `organizations/{org}/envgroups/{envgroup}.
    pub fn envgroups_patch(&self, request: GoogleCloudApigeeV1EnvironmentGroup, name: &str) -> OrganizationEnvgroupPatchCall<'a, S> {
        OrganizationEnvgroupPatchCall {
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
    /// Gets a list of metrics and dimensions that can be used to create analytics queries and reports. Each schema element contains the name of the field, its associated type, and a flag indicating whether it is a standard or custom field.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Path to the schema. Use the following structure in your request: `organizations/{org}/environments/{env}/analytics/admin/schemav2`.
    pub fn environments_analytics_admin_get_schemav2(&self, name: &str) -> OrganizationEnvironmentAnalyticAdminGetSchemav2Call<'a, S> {
        OrganizationEnvironmentAnalyticAdminGetSchemav2Call {
            hub: self.hub,
            _name: name.to_string(),
            _type_: Default::default(),
            _disable_cache: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a data export job to be processed in the background. If the request is successful, the API returns a 201 status, a URI that can be used to retrieve the status of the export job, and the `state` value of "enqueued".
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Names of the parent organization and environment. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_analytics_exports_create(&self, request: GoogleCloudApigeeV1ExportRequest, parent: &str) -> OrganizationEnvironmentAnalyticExportCreateCall<'a, S> {
        OrganizationEnvironmentAnalyticExportCreateCall {
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
    /// Gets the details and status of an analytics export job. If the export job is still in progress, its `state` is set to "running". After the export job has completed successfully, its `state` is set to "completed". If the export job fails, its `state` is set to `failed`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the export to get.
    pub fn environments_analytics_exports_get(&self, name: &str) -> OrganizationEnvironmentAnalyticExportGetCall<'a, S> {
        OrganizationEnvironmentAnalyticExportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the details and status of all analytics export jobs belonging to the parent organization and environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Names of the parent organization and environment. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_analytics_exports_list(&self, parent: &str) -> OrganizationEnvironmentAnalyticExportListCall<'a, S> {
        OrganizationEnvironmentAnalyticExportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of an API proxy in an environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name representing an API proxy in an environment in the following format: `organizations/{org}/environments/{env}/apis/{api}`
    pub fn environments_apis_deployments_list(&self, parent: &str) -> OrganizationEnvironmentApiDeploymentListCall<'a, S> {
        OrganizationEnvironmentApiDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the debug data from a transaction.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the debug session transaction. Must be of the form: `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}/debugsessions/{session}/data/{transaction}`.
    pub fn environments_apis_revisions_debugsessions_data_get(&self, name: &str) -> OrganizationEnvironmentApiRevisionDebugsessionDataGetCall<'a, S> {
        OrganizationEnvironmentApiRevisionDebugsessionDataGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a debug session for a deployed API Proxy revision.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the API Proxy revision deployment for which to create the DebugSession. Must be of the form `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}`.
    pub fn environments_apis_revisions_debugsessions_create(&self, request: GoogleCloudApigeeV1DebugSession, parent: &str) -> OrganizationEnvironmentApiRevisionDebugsessionCreateCall<'a, S> {
        OrganizationEnvironmentApiRevisionDebugsessionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _timeout: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the data from a debug session. This does not cancel the debug session or prevent further data from being collected if the session is still active in runtime pods.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the debug session to delete. Must be of the form: `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}/debugsessions/{debugsession}`.
    pub fn environments_apis_revisions_debugsessions_delete_data(&self, name: &str) -> OrganizationEnvironmentApiRevisionDebugsessionDeleteDataCall<'a, S> {
        OrganizationEnvironmentApiRevisionDebugsessionDeleteDataCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a debug session.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the debug session to retrieve. Must be of the form: `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}/debugsessions/{session}`.
    pub fn environments_apis_revisions_debugsessions_get(&self, name: &str) -> OrganizationEnvironmentApiRevisionDebugsessionGetCall<'a, S> {
        OrganizationEnvironmentApiRevisionDebugsessionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists debug sessions that are currently active in the given API Proxy revision.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the API Proxy revision deployment for which to list debug sessions. Must be of the form: `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}`.
    pub fn environments_apis_revisions_debugsessions_list(&self, parent: &str) -> OrganizationEnvironmentApiRevisionDebugsessionListCall<'a, S> {
        OrganizationEnvironmentApiRevisionDebugsessionListCall {
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
    /// Generates a report for a dry run analysis of a DeployApiProxy request without committing the deployment. In addition to the standard validations performed when adding deployments, additional analysis will be done to detect possible traffic routing changes that would result from this deployment being created. Any potential routing conflicts or unsafe changes will be reported in the response. This routing analysis is not performed for a non-dry-run DeployApiProxy request. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments:generateDeployChangeReport`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.deploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the API proxy revision deployment in the following format: `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}`
    pub fn environments_apis_revisions_deployments_generate_deploy_change_report(&self, name: &str) -> OrganizationEnvironmentApiRevisionDeploymentGenerateDeployChangeReportCall<'a, S> {
        OrganizationEnvironmentApiRevisionDeploymentGenerateDeployChangeReportCall {
            hub: self.hub,
            _name: name.to_string(),
            _override_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a report for a dry run analysis of an UndeployApiProxy request without committing the undeploy. In addition to the standard validations performed when removing deployments, additional analysis will be done to detect possible traffic routing changes that would result from this deployment being removed. Any potential routing conflicts or unsafe changes will be reported in the response. This routing analysis is not performed for a non-dry-run UndeployApiProxy request. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments:generateUndeployChangeReport`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.undeploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the API proxy revision deployment in the following format: `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}`
    pub fn environments_apis_revisions_deployments_generate_undeploy_change_report(&self, name: &str) -> OrganizationEnvironmentApiRevisionDeploymentGenerateUndeployChangeReportCall<'a, S> {
        OrganizationEnvironmentApiRevisionDeploymentGenerateUndeployChangeReportCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys a revision of an API proxy. If another revision of the same API proxy revision is currently deployed, set the `override` parameter to `true` to have this revision replace the currently deployed revision. You cannot invoke an API proxy until it has been deployed to an environment. After you deploy an API proxy revision, you cannot edit it. To edit the API proxy, you must create and deploy a new revision. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.deploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}` 
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API proxy revision deployment in the following format: `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}`
    pub fn environments_apis_revisions_deploy(&self, name: &str) -> OrganizationEnvironmentApiRevisionDeployCall<'a, S> {
        OrganizationEnvironmentApiRevisionDeployCall {
            hub: self.hub,
            _name: name.to_string(),
            _service_account: Default::default(),
            _sequenced_rollout: Default::default(),
            _override_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the deployment of an API proxy revision and actual state reported by runtime pods.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name representing an API proxy revision in an environment in the following format: `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}`
    pub fn environments_apis_revisions_get_deployments(&self, name: &str) -> OrganizationEnvironmentApiRevisionGetDeploymentCall<'a, S> {
        OrganizationEnvironmentApiRevisionGetDeploymentCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeploys an API proxy revision from an environment. For a request path `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.proxyrevisions.undeploy` on the resource `organizations/{org}/apis/{api}/revisions/{rev}`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the API proxy revision deployment in the following format: `organizations/{org}/environments/{env}/apis/{api}/revisions/{rev}`
    pub fn environments_apis_revisions_undeploy(&self, name: &str) -> OrganizationEnvironmentApiRevisionUndeployCall<'a, S> {
        OrganizationEnvironmentApiRevisionUndeployCall {
            hub: self.hub,
            _name: name.to_string(),
            _sequenced_rollout: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ArchiveDeployment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Environment this Archive Deployment will be created in.
    pub fn environments_archive_deployments_create(&self, request: GoogleCloudApigeeV1ArchiveDeployment, parent: &str) -> OrganizationEnvironmentArchiveDeploymentCreateCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentCreateCall {
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
    /// Deletes an archive deployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`.
    pub fn environments_archive_deployments_delete(&self, name: &str) -> OrganizationEnvironmentArchiveDeploymentDeleteCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a signed URL for downloading the original zip file used to create an Archive Deployment. The URL is only valid for a limited period and should be used within minutes after generation. Each call returns a new upload URL.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the Archive Deployment you want to download.
    pub fn environments_archive_deployments_generate_download_url(&self, request: GoogleCloudApigeeV1GenerateDownloadUrlRequest, name: &str) -> OrganizationEnvironmentArchiveDeploymentGenerateDownloadUrlCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentGenerateDownloadUrlCall {
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
    /// Generates a signed URL for uploading an Archive zip file to Google Cloud Storage. Once the upload is complete, the signed URL should be passed to CreateArchiveDeployment. When uploading to the generated signed URL, please follow these restrictions: * Source file type should be a zip file. * Source file size should not exceed 1GB limit. * No credentials should be attached - the signed URLs provide access to the target bucket using internal service identity; if credentials were attached, the identity from the credentials would be used, but that identity does not have permissions to upload files to the URL. When making a HTTP PUT request, these two headers need to be specified: * `content-type: application/zip` * `x-goog-content-length-range: 0,1073741824` And this header SHOULD NOT be specified: * `Authorization: Bearer YOUR_TOKEN`
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The organization and environment to upload to.
    pub fn environments_archive_deployments_generate_upload_url(&self, request: GoogleCloudApigeeV1GenerateUploadUrlRequest, parent: &str) -> OrganizationEnvironmentArchiveDeploymentGenerateUploadUrlCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentGenerateUploadUrlCall {
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
    /// Gets the specified ArchiveDeployment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`.
    pub fn environments_archive_deployments_get(&self, name: &str) -> OrganizationEnvironmentArchiveDeploymentGetCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the ArchiveDeployments in the specified Environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the Environment for which to list Archive Deployments in the format: `organizations/{org}/environments/{env}`.
    pub fn environments_archive_deployments_list(&self, parent: &str) -> OrganizationEnvironmentArchiveDeploymentListCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentListCall {
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
    /// Updates an existing ArchiveDeployment. Labels can modified but most of the other fields are not modifiable.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`.
    pub fn environments_archive_deployments_patch(&self, request: GoogleCloudApigeeV1ArchiveDeployment, name: &str) -> OrganizationEnvironmentArchiveDeploymentPatchCall<'a, S> {
        OrganizationEnvironmentArchiveDeploymentPatchCall {
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
    /// Deletes a cache.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Cache resource name of the form: `organizations/{organization_id}/environments/{environment_id}/caches/{cache_id}`
    pub fn environments_caches_delete(&self, name: &str) -> OrganizationEnvironmentCacheDeleteCall<'a, S> {
        OrganizationEnvironmentCacheDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of API proxies or shared flows in an environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment for which to return deployment information in the following format: `organizations/{org}/environments/{env}`
    pub fn environments_deployments_list(&self, parent: &str) -> OrganizationEnvironmentDeploymentListCall<'a, S> {
        OrganizationEnvironmentDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _shared_flows: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Attaches a shared flow to a flow hook.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the flow hook to which the shared flow should be attached in the following format: `organizations/{org}/environments/{env}/flowhooks/{flowhook}`
    pub fn environments_flowhooks_attach_shared_flow_to_flow_hook(&self, request: GoogleCloudApigeeV1FlowHook, name: &str) -> OrganizationEnvironmentFlowhookAttachSharedFlowToFlowHookCall<'a, S> {
        OrganizationEnvironmentFlowhookAttachSharedFlowToFlowHookCall {
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
    /// Detaches a shared flow from a flow hook.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the flow hook to detach in the following format: `organizations/{org}/environments/{env}/flowhooks/{flowhook}`
    pub fn environments_flowhooks_detach_shared_flow_from_flow_hook(&self, name: &str) -> OrganizationEnvironmentFlowhookDetachSharedFlowFromFlowHookCall<'a, S> {
        OrganizationEnvironmentFlowhookDetachSharedFlowFromFlowHookCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the name of the shared flow attached to the specified flow hook. If there's no shared flow attached to the flow hook, the API does not return an error; it simply does not return a name in the response.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the flow hook in the following format: `organizations/{org}/environments/{env}/flowhooks/{flowhook}`
    pub fn environments_flowhooks_get(&self, name: &str) -> OrganizationEnvironmentFlowhookGetCall<'a, S> {
        OrganizationEnvironmentFlowhookGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an alias from a key/certificate pair. The structure of the request is controlled by the `format` query parameter: - `keycertfile` - Separate PEM-encoded key and certificate files are uploaded. Set `Content-Type: multipart/form-data` and include the `keyFile`, `certFile`, and `password` (if keys are encrypted) fields in the request body. If uploading to a truststore, omit `keyFile`. - `pkcs12` - A PKCS12 file is uploaded. Set `Content-Type: multipart/form-data`, provide the file in the `file` field, and include the `password` field if the file is encrypted in the request body. - `selfsignedcert` - A new private key and certificate are generated. Set `Content-Type: application/json` and include CertificateGenerationSpec in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the keystore. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}`.
    pub fn environments_keystores_aliases_create(&self, request: GoogleApiHttpBody, parent: &str) -> OrganizationEnvironmentKeystoreAliasCreateCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _ignore_newline_validation: Default::default(),
            _ignore_expiry_validation: Default::default(),
            _format: Default::default(),
            _alias: Default::default(),
            __password: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a PKCS #10 Certificate Signing Request for the private key in an alias.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the alias. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`.
    pub fn environments_keystores_aliases_csr(&self, name: &str) -> OrganizationEnvironmentKeystoreAliasCsrCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasCsrCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an alias.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the alias. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`.
    pub fn environments_keystores_aliases_delete(&self, name: &str) -> OrganizationEnvironmentKeystoreAliasDeleteCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an alias.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the alias. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`.
    pub fn environments_keystores_aliases_get(&self, name: &str) -> OrganizationEnvironmentKeystoreAliasGetCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the certificate from an alias in PEM-encoded form.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the alias. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`.
    pub fn environments_keystores_aliases_get_certificate(&self, name: &str) -> OrganizationEnvironmentKeystoreAliasGetCertificateCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasGetCertificateCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the certificate in an alias.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the alias. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}/aliases/{alias}`
    pub fn environments_keystores_aliases_update(&self, request: GoogleApiHttpBody, name: &str) -> OrganizationEnvironmentKeystoreAliasUpdateCall<'a, S> {
        OrganizationEnvironmentKeystoreAliasUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _ignore_newline_validation: Default::default(),
            _ignore_expiry_validation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a keystore or truststore. - Keystore: Contains certificates and their associated keys. - Truststore: Contains trusted certificates used to validate a server's certificate. These certificates are typically self-signed certificates or certificates that are not signed by a trusted CA.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment in which to create the keystore. Use the following format in your request: `organizations/{org}/environments/{env}`
    pub fn environments_keystores_create(&self, request: GoogleCloudApigeeV1Keystore, parent: &str) -> OrganizationEnvironmentKeystoreCreateCall<'a, S> {
        OrganizationEnvironmentKeystoreCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a keystore or truststore.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the keystore. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}`
    pub fn environments_keystores_delete(&self, name: &str) -> OrganizationEnvironmentKeystoreDeleteCall<'a, S> {
        OrganizationEnvironmentKeystoreDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a keystore or truststore.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the keystore. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}`.
    pub fn environments_keystores_get(&self, name: &str) -> OrganizationEnvironmentKeystoreGetCall<'a, S> {
        OrganizationEnvironmentKeystoreGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates key value entries in a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Scope as indicated by the URI in which to create the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn environments_keyvaluemaps_entries_create(&self, request: GoogleCloudApigeeV1KeyValueEntry, parent: &str) -> OrganizationEnvironmentKeyvaluemapEntryCreateCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapEntryCreateCall {
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
    /// Deletes a key value entry from a key value map scoped to an organization, environment, or API proxy. **Notes:** * After you delete the key value entry, the policy consuming the entry will continue to function with its cached values for a few minutes. This is expected behavior. * Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to delete the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn environments_keyvaluemaps_entries_delete(&self, name: &str) -> OrganizationEnvironmentKeyvaluemapEntryDeleteCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the key value entry value for a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to fetch the key value map entry/value. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn environments_keyvaluemaps_entries_get(&self, name: &str) -> OrganizationEnvironmentKeyvaluemapEntryGetCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists key value entries for key values maps scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Scope as indicated by the URI in which to list key value maps. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn environments_keyvaluemaps_entries_list(&self, parent: &str) -> OrganizationEnvironmentKeyvaluemapEntryListCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapEntryListCall {
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
    /// Creates a key value map in an environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment in which to create the key value map. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_keyvaluemaps_create(&self, request: GoogleCloudApigeeV1KeyValueMap, parent: &str) -> OrganizationEnvironmentKeyvaluemapCreateCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapCreateCall {
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
    /// Deletes a key value map from an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the key value map. Use the following structure in your request: `organizations/{org}/environments/{env}/keyvaluemaps/{keyvaluemap}`
    pub fn environments_keyvaluemaps_delete(&self, name: &str) -> OrganizationEnvironmentKeyvaluemapDeleteCall<'a, S> {
        OrganizationEnvironmentKeyvaluemapDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Similar to GetStats except that the response is less verbose.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for which the interactive query will be executed. Use the following format in your request: `organizations/{org}/environments/{env}/optimizedStats/{dimensions}` Dimensions let you view metrics in meaningful groupings, such as `apiproxy`, `target_host`. The value of `dimensions` should be a comma-separated list as shown below: `organizations/{org}/environments/{env}/optimizedStats/apiproxy,request_verb`
    pub fn environments_optimized_stats_get(&self, name: &str) -> OrganizationEnvironmentOptimizedStatGetCall<'a, S> {
        OrganizationEnvironmentOptimizedStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _tzo: Default::default(),
            _ts_ascending: Default::default(),
            _topk: Default::default(),
            _time_unit: Default::default(),
            _time_range: Default::default(),
            _sortby: Default::default(),
            _sort: Default::default(),
            _sonar: Default::default(),
            _select: Default::default(),
            _realtime: Default::default(),
            _offset: Default::default(),
            _limit: Default::default(),
            _filter: Default::default(),
            _agg_table: Default::default(),
            _accuracy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a query to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_queries_create(&self, request: GoogleCloudApigeeV1Query, parent: &str) -> OrganizationEnvironmentQueryCreateCall<'a, S> {
        OrganizationEnvironmentQueryCreateCall {
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
    /// Get query status If the query is still in progress, the `state` is set to "running" After the query has completed successfully, `state` is set to "completed"
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query to get. Must be of the form `organizations/{org}/environments/{env}/queries/{queryId}`.
    pub fn environments_queries_get(&self, name: &str) -> OrganizationEnvironmentQueryGetCall<'a, S> {
        OrganizationEnvironmentQueryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query result to get. Must be of the form `organizations/{org}/environments/{env}/queries/{queryId}/result`.
    pub fn environments_queries_get_result(&self, name: &str) -> OrganizationEnvironmentQueryGetResultCall<'a, S> {
        OrganizationEnvironmentQueryGetResultCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is sent to the client as a list of urls to JSON files.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query result to get. Must be of the form `organizations/{org}/environments/{env}/queries/{queryId}/resulturl`.
    pub fn environments_queries_get_resulturl(&self, name: &str) -> OrganizationEnvironmentQueryGetResulturlCall<'a, S> {
        OrganizationEnvironmentQueryGetResulturlCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of Asynchronous Queries
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_queries_list(&self, parent: &str) -> OrganizationEnvironmentQueryListCall<'a, S> {
        OrganizationEnvironmentQueryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _to: Default::default(),
            _submitted_by: Default::default(),
            _status: Default::default(),
            _incl_queries_without_report: Default::default(),
            _from: Default::default(),
            _dataset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Reference in the specified environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent environment name under which the Reference will be created. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_references_create(&self, request: GoogleCloudApigeeV1Reference, parent: &str) -> OrganizationEnvironmentReferenceCreateCall<'a, S> {
        OrganizationEnvironmentReferenceCreateCall {
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
    /// Deletes a Reference from an environment. Returns the deleted Reference resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Reference to delete. Must be of the form `organizations/{org}/environments/{env}/references/{ref}`.
    pub fn environments_references_delete(&self, name: &str) -> OrganizationEnvironmentReferenceDeleteCall<'a, S> {
        OrganizationEnvironmentReferenceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a Reference resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Reference to get. Must be of the form `organizations/{org}/environments/{env}/references/{ref}`.
    pub fn environments_references_get(&self, name: &str) -> OrganizationEnvironmentReferenceGetCall<'a, S> {
        OrganizationEnvironmentReferenceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing Reference. Note that this operation has PUT semantics; it will replace the entirety of the existing Reference with the resource in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the Reference to update. Must be of the form `organizations/{org}/environments/{env}/references/{ref}`.
    pub fn environments_references_update(&self, request: GoogleCloudApigeeV1Reference, name: &str) -> OrganizationEnvironmentReferenceUpdateCall<'a, S> {
        OrganizationEnvironmentReferenceUpdateCall {
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
    /// Creates a resource file. Specify the `Content-Type` as `application/octet-stream` or `multipart/form-data`. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment in which to create the resource file in the following format: `organizations/{org}/environments/{env}`.
    pub fn environments_resourcefiles_create(&self, request: GoogleApiHttpBody, parent: &str) -> OrganizationEnvironmentResourcefileCreateCall<'a, S> {
        OrganizationEnvironmentResourcefileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource file. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment in the following format: `organizations/{org}/environments/{env}`.
    /// * `type` - Required. Resource file type. {{ resource_file_type }}
    /// * `name` - Required. ID of the resource file to delete. Must match the regular expression: \[a-zA-Z0-9:/\\!@#$%^&{}\[\]()+-=,.~’\` \]{1,255}
    pub fn environments_resourcefiles_delete(&self, parent: &str, type_: &str, name: &str) -> OrganizationEnvironmentResourcefileDeleteCall<'a, S> {
        OrganizationEnvironmentResourcefileDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the contents of a resource file. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment in the following format: `organizations/{org}/environments/{env}`.
    /// * `type` - Required. Resource file type. {{ resource_file_type }}
    /// * `name` - Required. ID of the resource file. Must match the regular expression: \[a-zA-Z0-9:/\\!@#$%^&{}\[\]()+-=,.~’\` \]{1,255}
    pub fn environments_resourcefiles_get(&self, parent: &str, type_: &str, name: &str) -> OrganizationEnvironmentResourcefileGetCall<'a, S> {
        OrganizationEnvironmentResourcefileGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all resource files, optionally filtering by type. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment in which to list resource files in the following format: `organizations/{org}/environments/{env}`.
    pub fn environments_resourcefiles_list(&self, parent: &str) -> OrganizationEnvironmentResourcefileListCall<'a, S> {
        OrganizationEnvironmentResourcefileListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all resource files, optionally filtering by type. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment in which to list resource files in the following format: `organizations/{org}/environments/{env}`.
    /// * `type` - Optional. Type of resource files to list. {{ resource_file_type }}
    pub fn environments_resourcefiles_list_environment_resources(&self, parent: &str, type_: &str) -> OrganizationEnvironmentResourcefileListEnvironmentResourceCall<'a, S> {
        OrganizationEnvironmentResourcefileListEnvironmentResourceCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a resource file. Specify the `Content-Type` as `application/octet-stream` or `multipart/form-data`. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment in the following format: `organizations/{org}/environments/{env}`.
    /// * `type` - Required. Resource file type. {{ resource_file_type }}
    /// * `name` - Required. ID of the resource file to update. Must match the regular expression: \[a-zA-Z0-9:/\\!@#$%^&{}\[\]()+-=,.~’\` \]{1,255}
    pub fn environments_resourcefiles_update(&self, request: GoogleApiHttpBody, parent: &str, type_: &str, name: &str) -> OrganizationEnvironmentResourcefileUpdateCall<'a, S> {
        OrganizationEnvironmentResourcefileUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _type_: type_.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a report request to be processed in the background. If the submission succeeds, the API returns a 200 status and an ID that refer to the report request. In addition to the HTTP status 200, the `state` of "enqueued" means that the request succeeded.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_security_reports_create(&self, request: GoogleCloudApigeeV1SecurityReportQuery, parent: &str) -> OrganizationEnvironmentSecurityReportCreateCall<'a, S> {
        OrganizationEnvironmentSecurityReportCreateCall {
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
    /// Get security report status If the query is still in progress, the `state` is set to "running" After the query has completed successfully, `state` is set to "completed"
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report to get. Must be of the form `organizations/{org}/environments/{env}/securityReports/{reportId}`.
    pub fn environments_security_reports_get(&self, name: &str) -> OrganizationEnvironmentSecurityReportGetCall<'a, S> {
        OrganizationEnvironmentSecurityReportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to retrieve the results as file. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report result to get. Must be of the form `organizations/{org}/environments/{env}/securityReports/{reportId}/result`.
    pub fn environments_security_reports_get_result(&self, name: &str) -> OrganizationEnvironmentSecurityReportGetResultCall<'a, S> {
        OrganizationEnvironmentSecurityReportGetResultCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to view the query result when result size is small.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report result view to get. Must be of the form `organizations/{org}/environments/{env}/securityReports/{reportId}/resultView`.
    pub fn environments_security_reports_get_result_view(&self, name: &str) -> OrganizationEnvironmentSecurityReportGetResultViewCall<'a, S> {
        OrganizationEnvironmentSecurityReportGetResultViewCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of Security Reports
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_security_reports_list(&self, parent: &str) -> OrganizationEnvironmentSecurityReportListCall<'a, S> {
        OrganizationEnvironmentSecurityReportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _to: Default::default(),
            _submitted_by: Default::default(),
            _status: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _from: Default::default(),
            _dataset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve security statistics as tabular rows.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `orgenv` - Required. Should be of the form organizations//environments/.
    pub fn environments_security_stats_query_tabular_stats(&self, request: GoogleCloudApigeeV1QueryTabularStatsRequest, orgenv: &str) -> OrganizationEnvironmentSecurityStatQueryTabularStatCall<'a, S> {
        OrganizationEnvironmentSecurityStatQueryTabularStatCall {
            hub: self.hub,
            _request: request,
            _orgenv: orgenv.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve security statistics as a collection of time series.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `orgenv` - Required. Should be of the form organizations//environments/.
    pub fn environments_security_stats_query_time_series_stats(&self, request: GoogleCloudApigeeV1QueryTimeSeriesStatsRequest, orgenv: &str) -> OrganizationEnvironmentSecurityStatQueryTimeSeriesStatCall<'a, S> {
        OrganizationEnvironmentSecurityStatQueryTimeSeriesStatCall {
            hub: self.hub,
            _request: request,
            _orgenv: orgenv.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of a shared flow in an environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name representing a shared flow in an environment in the following format: `organizations/{org}/environments/{env}/sharedflows/{sharedflow}`
    pub fn environments_sharedflows_deployments_list(&self, parent: &str) -> OrganizationEnvironmentSharedflowDeploymentListCall<'a, S> {
        OrganizationEnvironmentSharedflowDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deploys a revision of a shared flow. If another revision of the same shared flow is currently deployed, set the `override` parameter to `true` to have this revision replace the currently deployed revision. You cannot use a shared flow until it has been deployed to an environment. For a request path `organizations/{org}/environments/{env}/sharedflows/{sf}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.create` on the resource `organizations/{org}/environments/{env}` * `apigee.sharedflowrevisions.deploy` on the resource `organizations/{org}/sharedflows/{sf}/revisions/{rev}`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the shared flow revision to deploy in the following format: `organizations/{org}/environments/{env}/sharedflows/{sharedflow}/revisions/{rev}`
    pub fn environments_sharedflows_revisions_deploy(&self, name: &str) -> OrganizationEnvironmentSharedflowRevisionDeployCall<'a, S> {
        OrganizationEnvironmentSharedflowRevisionDeployCall {
            hub: self.hub,
            _name: name.to_string(),
            _service_account: Default::default(),
            _override_: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the deployment of a shared flow revision and actual state reported by runtime pods.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name representing a shared flow in an environment in the following format: `organizations/{org}/environments/{env}/sharedflows/{sharedflow}/revisions/{rev}`
    pub fn environments_sharedflows_revisions_get_deployments(&self, name: &str) -> OrganizationEnvironmentSharedflowRevisionGetDeploymentCall<'a, S> {
        OrganizationEnvironmentSharedflowRevisionGetDeploymentCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeploys a shared flow revision from an environment. For a request path `organizations/{org}/environments/{env}/sharedflows/{sf}/revisions/{rev}/deployments`, two permissions are required: * `apigee.deployments.delete` on the resource `organizations/{org}/environments/{env}` * `apigee.sharedflowrevisions.undeploy` on the resource `organizations/{org}/sharedflows/{sf}/revisions/{rev}`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the shared flow revision to undeploy in the following format: `organizations/{org}/environments/{env}/sharedflows/{sharedflow}/revisions/{rev}`
    pub fn environments_sharedflows_revisions_undeploy(&self, name: &str) -> OrganizationEnvironmentSharedflowRevisionUndeployCall<'a, S> {
        OrganizationEnvironmentSharedflowRevisionUndeployCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve metrics grouped by dimensions. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. You can optionally pass dimensions as path parameters to the `stats` API. If dimensions are not specified, the metrics are computed on the entire set of data for the given time range.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for which the interactive query will be executed. Use the following format in your request: `organizations/{org}/environments/{env}/stats/{dimensions}` Dimensions let you view metrics in meaningful groupings, such as `apiproxy` or `target_host`. The value of dimensions should be a comma-separated list, as shown below: `organizations/{org}/environments/{env}/stats/apiproxy,request_verb`
    pub fn environments_stats_get(&self, name: &str) -> OrganizationEnvironmentStatGetCall<'a, S> {
        OrganizationEnvironmentStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _tzo: Default::default(),
            _ts_ascending: Default::default(),
            _topk: Default::default(),
            _time_unit: Default::default(),
            _time_range: Default::default(),
            _sortby: Default::default(),
            _sort: Default::default(),
            _sonar: Default::default(),
            _select: Default::default(),
            _realtime: Default::default(),
            _offset: Default::default(),
            _limit: Default::default(),
            _filter: Default::default(),
            _agg_table: Default::default(),
            _accuracy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a TargetServer in the specified environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent environment name under which the TargetServer will be created. Must be of the form `organizations/{org}/environments/{env}`.
    pub fn environments_targetservers_create(&self, request: GoogleCloudApigeeV1TargetServer, parent: &str) -> OrganizationEnvironmentTargetserverCreateCall<'a, S> {
        OrganizationEnvironmentTargetserverCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a TargetServer from an environment. Returns the deleted TargetServer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the TargetServer to delete. Must be of the form `organizations/{org}/environments/{env}/targetservers/{target_server_id}`.
    pub fn environments_targetservers_delete(&self, name: &str) -> OrganizationEnvironmentTargetserverDeleteCall<'a, S> {
        OrganizationEnvironmentTargetserverDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a TargetServer resource.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the TargetServer to get. Must be of the form `organizations/{org}/environments/{env}/targetservers/{target_server_id}`.
    pub fn environments_targetservers_get(&self, name: &str) -> OrganizationEnvironmentTargetserverGetCall<'a, S> {
        OrganizationEnvironmentTargetserverGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing TargetServer. Note that this operation has PUT semantics; it will replace the entirety of the existing TargetServer with the resource in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the TargetServer to replace. Must be of the form `organizations/{org}/environments/{env}/targetservers/{target_server_id}`.
    pub fn environments_targetservers_update(&self, request: GoogleCloudApigeeV1TargetServer, name: &str) -> OrganizationEnvironmentTargetserverUpdateCall<'a, S> {
        OrganizationEnvironmentTargetserverUpdateCall {
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
    /// Creates a trace configuration override. The response contains a system-generated UUID, that can be used to view, update, or delete the configuration override. Use the List API to view the existing trace configuration overrides.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Parent resource of the trace configuration override. Use the following structure in your request. "organizations/*/environments/*/traceConfig".
    pub fn environments_trace_config_overrides_create(&self, request: GoogleCloudApigeeV1TraceConfigOverride, parent: &str) -> OrganizationEnvironmentTraceConfigOverrideCreateCall<'a, S> {
        OrganizationEnvironmentTraceConfigOverrideCreateCall {
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
    /// Deletes a distributed trace configuration override.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the trace configuration override. Use the following structure in your request: "organizations/*/environments/*/traceConfig/overrides/*".
    pub fn environments_trace_config_overrides_delete(&self, name: &str) -> OrganizationEnvironmentTraceConfigOverrideDeleteCall<'a, S> {
        OrganizationEnvironmentTraceConfigOverrideDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a trace configuration override.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the trace configuration override. Use the following structure in your request: "organizations/*/environments/*/traceConfig/overrides/*".
    pub fn environments_trace_config_overrides_get(&self, name: &str) -> OrganizationEnvironmentTraceConfigOverrideGetCall<'a, S> {
        OrganizationEnvironmentTraceConfigOverrideGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the distributed trace configuration overrides in an environment.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Parent resource of the trace configuration override. Use the following structure in your request: "organizations/*/environments/*/traceConfig".
    pub fn environments_trace_config_overrides_list(&self, parent: &str) -> OrganizationEnvironmentTraceConfigOverrideListCall<'a, S> {
        OrganizationEnvironmentTraceConfigOverrideListCall {
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
    /// Updates a distributed trace configuration override. Note that the repeated fields have replace semantics when included in the field mask and that they will be overwritten by the value of the fields in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the trace configuration override. Use the following structure in your request: "organizations/*/environments/*/traceConfig/overrides/*".
    pub fn environments_trace_config_overrides_patch(&self, request: GoogleCloudApigeeV1TraceConfigOverride, name: &str) -> OrganizationEnvironmentTraceConfigOverridePatchCall<'a, S> {
        OrganizationEnvironmentTraceConfigOverridePatchCall {
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
    /// Creates an environment in an organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in which the environment will be created. Use the following structure in your request: `organizations/{org}`
    pub fn environments_create(&self, request: GoogleCloudApigeeV1Environment, parent: &str) -> OrganizationEnvironmentCreateCall<'a, S> {
        OrganizationEnvironmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an environment from an organization. **Warning: You must delete all key value maps and key value entries before you delete an environment.** Otherwise, if you re-create the environment the key value map entry operations will encounter encryption/decryption discrepancies.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_delete(&self, name: &str) -> OrganizationEnvironmentDeleteCall<'a, S> {
        OrganizationEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets environment details.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_get(&self, name: &str) -> OrganizationEnvironmentGetCall<'a, S> {
        OrganizationEnvironmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the API Security runtime configuration for an environment. This named ApiSecurityRuntimeConfig to prevent conflicts with ApiSecurityConfig from addon config.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment API Security Runtime configuration resource. Use the following structure in your request: `organizations/{org}/environments/{env}/apiSecurityRuntimeConfig`
    pub fn environments_get_api_security_runtime_config(&self, name: &str) -> OrganizationEnvironmentGetApiSecurityRuntimeConfigCall<'a, S> {
        OrganizationEnvironmentGetApiSecurityRuntimeConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the debug mask singleton resource for an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the debug mask. Use the following structure in your request: `organizations/{org}/environments/{env}/debugmask`.
    pub fn environments_get_debugmask(&self, name: &str) -> OrganizationEnvironmentGetDebugmaskCall<'a, S> {
        OrganizationEnvironmentGetDebugmaskCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the deployed configuration for an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the environment deployed configuration resource. Use the following structure in your request: `organizations/{org}/environments/{env}/deployedConfig`
    pub fn environments_get_deployed_config(&self, name: &str) -> OrganizationEnvironmentGetDeployedConfigCall<'a, S> {
        OrganizationEnvironmentGetDeployedConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the IAM policy on an environment. For more information, see [Manage users, roles, and permissions using the API](https://cloud.google.com/apigee/docs/api-platform/system-administration/manage-users-roles). You must have the `apigee.environments.getIamPolicy` permission to call this API.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn environments_get_iam_policy(&self, resource: &str) -> OrganizationEnvironmentGetIamPolicyCall<'a, S> {
        OrganizationEnvironmentGetIamPolicyCall {
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
    /// Get distributed trace configuration in an environment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the trace configuration. Use the following structure in your request: "organizations/*/environments/*/traceConfig".
    pub fn environments_get_trace_config(&self, name: &str) -> OrganizationEnvironmentGetTraceConfigCall<'a, S> {
        OrganizationEnvironmentGetTraceConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates properties for an Apigee environment with patch semantics using a field mask. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{environment}`.
    pub fn environments_modify_environment(&self, request: GoogleCloudApigeeV1Environment, name: &str) -> OrganizationEnvironmentModifyEnvironmentCall<'a, S> {
        OrganizationEnvironmentModifyEnvironmentCall {
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
    /// Sets the IAM policy on an environment, if the policy already exists it will be replaced. For more information, see [Manage users, roles, and permissions using the API](https://cloud.google.com/apigee/docs/api-platform/system-administration/manage-users-roles). You must have the `apigee.environments.setIamPolicy` permission to call this API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn environments_set_iam_policy(&self, request: GoogleIamV1SetIamPolicyRequest, resource: &str) -> OrganizationEnvironmentSetIamPolicyCall<'a, S> {
        OrganizationEnvironmentSetIamPolicyCall {
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
    /// Creates a subscription for the environment's Pub/Sub topic. The server will assign a random name for this subscription. The "name" and "push_config" must *not* be specified.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_subscribe(&self, parent: &str) -> OrganizationEnvironmentSubscribeCall<'a, S> {
        OrganizationEnvironmentSubscribeCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests the permissions of a user on an environment, and returns a subset of permissions that the user has on the environment. If the environment does not exist, an empty permission set is returned (a NOT_FOUND error is not returned).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn environments_test_iam_permissions(&self, request: GoogleIamV1TestIamPermissionsRequest, resource: &str) -> OrganizationEnvironmentTestIamPermissionCall<'a, S> {
        OrganizationEnvironmentTestIamPermissionCall {
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
    /// Deletes a subscription for the environment's Pub/Sub topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_unsubscribe(&self, request: GoogleCloudApigeeV1Subscription, parent: &str) -> OrganizationEnvironmentUnsubscribeCall<'a, S> {
        OrganizationEnvironmentUnsubscribeCall {
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
    /// Updates an existing environment. When updating properties, you must pass all existing properties to the API, even if they are not being changed. If you omit properties from the payload, the properties are removed. To get the current list of properties for the environment, use the [Get Environment API](get). **Note**: Both `PUT` and `POST` methods are supported for updating an existing environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_update(&self, request: GoogleCloudApigeeV1Environment, name: &str) -> OrganizationEnvironmentUpdateCall<'a, S> {
        OrganizationEnvironmentUpdateCall {
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
    /// Updates the debug mask singleton resource for an environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Name of the debug mask.
    pub fn environments_update_debugmask(&self, request: GoogleCloudApigeeV1DebugMask, name: &str) -> OrganizationEnvironmentUpdateDebugmaskCall<'a, S> {
        OrganizationEnvironmentUpdateDebugmaskCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _replace_repeated_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing environment. When updating properties, you must pass all existing properties to the API, even if they are not being changed. If you omit properties from the payload, the properties are removed. To get the current list of properties for the environment, use the [Get Environment API](get). **Note**: Both `PUT` and `POST` methods are supported for updating an existing environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the environment. Use the following structure in your request: `organizations/{org}/environments/{env}`
    pub fn environments_update_environment(&self, request: GoogleCloudApigeeV1Environment, name: &str) -> OrganizationEnvironmentUpdateEnvironmentCall<'a, S> {
        OrganizationEnvironmentUpdateEnvironmentCall {
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
    /// Updates the trace configurations in an environment. Note that the repeated fields have replace semantics when included in the field mask and that they will be overwritten by the value of the fields in the request body.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the trace configuration. Use the following structure in your request: "organizations/*/environments/*/traceConfig".
    pub fn environments_update_trace_config(&self, request: GoogleCloudApigeeV1TraceConfig, name: &str) -> OrganizationEnvironmentUpdateTraceConfigCall<'a, S> {
        OrganizationEnvironmentUpdateTraceConfigCall {
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
    /// Submit a query at host level to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}`.
    pub fn host_queries_create(&self, request: GoogleCloudApigeeV1Query, parent: &str) -> OrganizationHostQueryCreateCall<'a, S> {
        OrganizationHostQueryCreateCall {
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
    /// Get status of a query submitted at host level. If the query is still in progress, the `state` is set to "running" After the query has completed successfully, `state` is set to "completed"
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query to get. Must be of the form `organizations/{org}/queries/{queryId}`.
    pub fn host_queries_get(&self, name: &str) -> OrganizationHostQueryGetCall<'a, S> {
        OrganizationHostQueryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query result to get. Must be of the form `organizations/{org}/queries/{queryId}/result`.
    pub fn host_queries_get_result(&self, name: &str) -> OrganizationHostQueryGetResultCall<'a, S> {
        OrganizationHostQueryGetResultCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// 
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the asynchronous query result view to get. Must be of the form `organizations/{org}/queries/{queryId}/resultView`.
    pub fn host_queries_get_result_view(&self, name: &str) -> OrganizationHostQueryGetResultViewCall<'a, S> {
        OrganizationHostQueryGetResultViewCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of Asynchronous Queries at host level.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}`.
    pub fn host_queries_list(&self, parent: &str) -> OrganizationHostQueryListCall<'a, S> {
        OrganizationHostQueryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _to: Default::default(),
            _submitted_by: Default::default(),
            _status: Default::default(),
            _incl_queries_without_report: Default::default(),
            _from: Default::default(),
            _envgroup_hostname: Default::default(),
            _dataset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submit a query at host level to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}`.
    pub fn host_security_reports_create(&self, request: GoogleCloudApigeeV1SecurityReportQuery, parent: &str) -> OrganizationHostSecurityReportCreateCall<'a, S> {
        OrganizationHostSecurityReportCreateCall {
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
    /// Get status of a query submitted at host level. If the query is still in progress, the `state` is set to "running" After the query has completed successfully, `state` is set to "completed"
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report to get. Must be of the form `organizations/{org}/securityReports/{reportId}`.
    pub fn host_security_reports_get(&self, name: &str) -> OrganizationHostSecurityReportGetCall<'a, S> {
        OrganizationHostSecurityReportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to retrieve the results. If the request succeeds, and there is a non-zero result set, the result is downloaded to the client as a zipped JSON file. The name of the downloaded file will be: OfflineQueryResult-.zip Example: `OfflineQueryResult-9cfc0d85-0f30-46d6-ae6f-318d0cb961bd.zip`
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report result to get. Must be of the form `organizations/{org}/securityReports/{reportId}/result`.
    pub fn host_security_reports_get_result(&self, name: &str) -> OrganizationHostSecurityReportGetResultCall<'a, S> {
        OrganizationHostSecurityReportGetResultCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// After the query is completed, use this API to view the query result when result size is small.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the security report result view to get. Must be of the form `organizations/{org}/securityReports/{reportId}/resultView`.
    pub fn host_security_reports_get_result_view(&self, name: &str) -> OrganizationHostSecurityReportGetResultViewCall<'a, S> {
        OrganizationHostSecurityReportGetResultViewCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of Security Reports at host level.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name. Must be of the form `organizations/{org}`.
    pub fn host_security_reports_list(&self, parent: &str) -> OrganizationHostSecurityReportListCall<'a, S> {
        OrganizationHostSecurityReportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _to: Default::default(),
            _submitted_by: Default::default(),
            _status: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _from: Default::default(),
            _envgroup_hostname: Default::default(),
            _dataset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve metrics grouped by dimensions in host level. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. You can optionally pass dimensions as path parameters to the `stats` API. If dimensions are not specified, the metrics are computed on the entire set of data for the given time range.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for which the interactive query will be executed. Use the following format in your request: `organizations/{org}/hostStats/{dimensions}` Dimensions let you view metrics in meaningful groupings, such as `apiproxy`, `target_host`. The value of dimensions should be a comma-separated list as shown below `organizations/{org}/hostStats/apiproxy,request_verb`
    pub fn host_stats_get(&self, name: &str) -> OrganizationHostStatGetCall<'a, S> {
        OrganizationHostStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _tzo: Default::default(),
            _ts_ascending: Default::default(),
            _topk: Default::default(),
            _time_unit: Default::default(),
            _time_range: Default::default(),
            _sortby: Default::default(),
            _sort: Default::default(),
            _select: Default::default(),
            _realtime: Default::default(),
            _offset: Default::default(),
            _limit: Default::default(),
            _filter: Default::default(),
            _envgroup_hostname: Default::default(),
            _accuracy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new attachment of an environment to an instance. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
    pub fn instances_attachments_create(&self, request: GoogleCloudApigeeV1InstanceAttachment, parent: &str) -> OrganizationInstanceAttachmentCreateCall<'a, S> {
        OrganizationInstanceAttachmentCreateCall {
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
    /// Deletes an attachment. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the attachment. Use the following structure in your request: `organizations/{org}/instances/{instance}/attachments/{attachment}`.
    pub fn instances_attachments_delete(&self, name: &str) -> OrganizationInstanceAttachmentDeleteCall<'a, S> {
        OrganizationInstanceAttachmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an attachment. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the attachment. Use the following structure in your request: `organizations/{org}/instances/{instance}/attachments/{attachment}`
    pub fn instances_attachments_get(&self, name: &str) -> OrganizationInstanceAttachmentGetCall<'a, S> {
        OrganizationInstanceAttachmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all attachments to an instance. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}/instances/{instance}`
    pub fn instances_attachments_list(&self, parent: &str) -> OrganizationInstanceAttachmentListCall<'a, S> {
        OrganizationInstanceAttachmentListCall {
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
    /// Creates a new canary evaluation for an organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
    pub fn instances_canaryevaluations_create(&self, request: GoogleCloudApigeeV1CanaryEvaluation, parent: &str) -> OrganizationInstanceCanaryevaluationCreateCall<'a, S> {
        OrganizationInstanceCanaryevaluationCreateCall {
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
    /// Gets a CanaryEvaluation for an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the CanaryEvaluation. Use the following structure in your request: `organizations/{org}/instances/*/canaryevaluations/{evaluation}`
    pub fn instances_canaryevaluations_get(&self, name: &str) -> OrganizationInstanceCanaryevaluationGetCall<'a, S> {
        OrganizationInstanceCanaryevaluationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates the NAT address. The Apigee instance can now use this for Internet egress traffic. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the nat address. Use the following structure in your request: `organizations/{org}/instances/{instances}/natAddresses/{nataddress}``
    pub fn instances_nat_addresses_activate(&self, request: GoogleCloudApigeeV1ActivateNatAddressRequest, name: &str) -> OrganizationInstanceNatAddressActivateCall<'a, S> {
        OrganizationInstanceNatAddressActivateCall {
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
    /// Creates a NAT address. The address is created in the RESERVED state and a static external IP address will be provisioned. At this time, the instance will not use this IP address for Internet egress traffic. The address can be activated for use once any required firewall IP whitelisting has been completed. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`
    pub fn instances_nat_addresses_create(&self, request: GoogleCloudApigeeV1NatAddress, parent: &str) -> OrganizationInstanceNatAddressCreateCall<'a, S> {
        OrganizationInstanceNatAddressCreateCall {
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
    /// Deletes the NAT address. Connections that are actively using the address are drained before it is removed. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the nat address. Use the following structure in your request: `organizations/{org}/instances/{instances}/natAddresses/{nataddress}``
    pub fn instances_nat_addresses_delete(&self, name: &str) -> OrganizationInstanceNatAddressDeleteCall<'a, S> {
        OrganizationInstanceNatAddressDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of a NAT address. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the nat address. Use the following structure in your request: `organizations/{org}/instances/{instances}/natAddresses/{nataddress}`
    pub fn instances_nat_addresses_get(&self, name: &str) -> OrganizationInstanceNatAddressGetCall<'a, S> {
        OrganizationInstanceNatAddressGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the NAT addresses for an Apigee instance. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`
    pub fn instances_nat_addresses_list(&self, parent: &str) -> OrganizationInstanceNatAddressListCall<'a, S> {
        OrganizationInstanceNatAddressListCall {
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
    /// Creates an Apigee runtime instance. The instance is accessible from the authorized network configured on the organization. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}`.
    pub fn instances_create(&self, request: GoogleCloudApigeeV1Instance, parent: &str) -> OrganizationInstanceCreateCall<'a, S> {
        OrganizationInstanceCreateCall {
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
    /// Deletes an Apigee runtime instance. The instance stops serving requests and the runtime data is deleted. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
    pub fn instances_delete(&self, name: &str) -> OrganizationInstanceDeleteCall<'a, S> {
        OrganizationInstanceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details for an Apigee runtime instance. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
    pub fn instances_get(&self, name: &str) -> OrganizationInstanceGetCall<'a, S> {
        OrganizationInstanceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all Apigee runtime instances for the organization. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}`.
    pub fn instances_list(&self, parent: &str) -> OrganizationInstanceListCall<'a, S> {
        OrganizationInstanceListCall {
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
    /// Updates an Apigee runtime instance. You can update the fields described in NodeConfig. No other fields will be updated. **Note:** Not supported for Apigee hybrid.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
    pub fn instances_patch(&self, request: GoogleCloudApigeeV1Instance, name: &str) -> OrganizationInstancePatchCall<'a, S> {
        OrganizationInstancePatchCall {
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
    /// Reports the latest status for a runtime instance.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `instance` - The name of the instance reporting this status. For SaaS the request will be rejected if no instance exists under this name. Format is organizations/{org}/instances/{instance}
    pub fn instances_report_status(&self, request: GoogleCloudApigeeV1ReportInstanceStatusRequest, instance: &str) -> OrganizationInstanceReportStatuCall<'a, S> {
        OrganizationInstanceReportStatuCall {
            hub: self.hub,
            _request: request,
            _instance: instance.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates key value entries in a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Scope as indicated by the URI in which to create the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn keyvaluemaps_entries_create(&self, request: GoogleCloudApigeeV1KeyValueEntry, parent: &str) -> OrganizationKeyvaluemapEntryCreateCall<'a, S> {
        OrganizationKeyvaluemapEntryCreateCall {
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
    /// Deletes a key value entry from a key value map scoped to an organization, environment, or API proxy. **Notes:** * After you delete the key value entry, the policy consuming the entry will continue to function with its cached values for a few minutes. This is expected behavior. * Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to delete the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn keyvaluemaps_entries_delete(&self, name: &str) -> OrganizationKeyvaluemapEntryDeleteCall<'a, S> {
        OrganizationKeyvaluemapEntryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the key value entry value for a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Scope as indicated by the URI in which to fetch the key value map entry/value. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}/entries/{entry}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}/entries/{entry}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}/entries/{entry}`.
    pub fn keyvaluemaps_entries_get(&self, name: &str) -> OrganizationKeyvaluemapEntryGetCall<'a, S> {
        OrganizationKeyvaluemapEntryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists key value entries for key values maps scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Scope as indicated by the URI in which to list key value maps. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`.
    pub fn keyvaluemaps_entries_list(&self, parent: &str) -> OrganizationKeyvaluemapEntryListCall<'a, S> {
        OrganizationKeyvaluemapEntryListCall {
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
    /// Creates a key value map in an organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the organization in which to create the key value map file. Use the following structure in your request: `organizations/{org}`
    pub fn keyvaluemaps_create(&self, request: GoogleCloudApigeeV1KeyValueMap, parent: &str) -> OrganizationKeyvaluemapCreateCall<'a, S> {
        OrganizationKeyvaluemapCreateCall {
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
    /// Deletes a key value map from an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the key value map. Use the following structure in your request: `organizations/{org}/keyvaluemaps/{keyvaluemap}`
    pub fn keyvaluemaps_delete(&self, name: &str) -> OrganizationKeyvaluemapDeleteCall<'a, S> {
        OrganizationKeyvaluemapDeleteCall {
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
    pub fn operations_get(&self, name: &str) -> OrganizationOperationGetCall<'a, S> {
        OrganizationOperationGetCall {
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
    pub fn operations_list(&self, name: &str) -> OrganizationOperationListCall<'a, S> {
        OrganizationOperationListCall {
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
    /// Similar to GetHostStats except that the response is less verbose.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name for which the interactive query will be executed. Use the following format in your request: `organizations/{organization_id}/optimizedHostStats/{dimensions}` Dimensions let you view metrics in meaningful groupings, such as `apiproxy`, `target_host`. The value of dimensions should be a comma-separated list as shown below: `organizations/{org}/optimizedHostStats/apiproxy,request_verb`
    pub fn optimized_host_stats_get(&self, name: &str) -> OrganizationOptimizedHostStatGetCall<'a, S> {
        OrganizationOptimizedHostStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _tzo: Default::default(),
            _ts_ascending: Default::default(),
            _topk: Default::default(),
            _time_unit: Default::default(),
            _time_range: Default::default(),
            _sortby: Default::default(),
            _sort: Default::default(),
            _select: Default::default(),
            _realtime: Default::default(),
            _offset: Default::default(),
            _limit: Default::default(),
            _filter: Default::default(),
            _envgroup_hostname: Default::default(),
            _accuracy: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a Custom Report for an Organization. A Custom Report provides Apigee Customers to create custom dashboards in addition to the standard dashboards which are provided. The Custom Report in its simplest form contains specifications about metrics, dimensions and filters. It is important to note that the custom report by itself does not provide an executable entity. The Edge UI converts the custom report definition into an analytics query and displays the result in a chart.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent organization name under which the Custom Report will be created. Must be of the form: `organizations/{organization_id}/reports`
    pub fn reports_create(&self, request: GoogleCloudApigeeV1CustomReport, parent: &str) -> OrganizationReportCreateCall<'a, S> {
        OrganizationReportCreateCall {
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
    /// Deletes an existing custom report definition
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Custom Report name of the form: `organizations/{organization_id}/reports/{report_name}`
    pub fn reports_delete(&self, name: &str) -> OrganizationReportDeleteCall<'a, S> {
        OrganizationReportDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve a custom report definition.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Custom Report name of the form: `organizations/{organization_id}/reports/{report_name}`
    pub fn reports_get(&self, name: &str) -> OrganizationReportGetCall<'a, S> {
        OrganizationReportGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of Custom Reports
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent organization name under which the API product will be listed `organizations/{organization_id}/reports`
    pub fn reports_list(&self, parent: &str) -> OrganizationReportListCall<'a, S> {
        OrganizationReportListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _expand: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an existing custom report definition
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Custom Report name of the form: `organizations/{organization_id}/reports/{report_name}`
    pub fn reports_update(&self, request: GoogleCloudApigeeV1CustomReport, name: &str) -> OrganizationReportUpdateCall<'a, S> {
        OrganizationReportUpdateCall {
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
    /// ComputeEnvironmentScores calculates scores for requested time range for the specified security profile and environment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileEnvironment` - Required. Name of organization and environment and profile id for which score needs to be computed. Format: organizations/{org}/securityProfiles/{profile}/environments/{env}
    pub fn security_profiles_environments_compute_environment_scores(&self, request: GoogleCloudApigeeV1ComputeEnvironmentScoresRequest, profile_environment: &str) -> OrganizationSecurityProfileEnvironmentComputeEnvironmentScoreCall<'a, S> {
        OrganizationSecurityProfileEnvironmentComputeEnvironmentScoreCall {
            hub: self.hub,
            _request: request,
            _profile_environment: profile_environment.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// CreateSecurityProfileEnvironmentAssociation creates profile environment association i.e. attaches environment to security profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of organization and security profile ID. Format: organizations/{org}/securityProfiles/{profile}
    pub fn security_profiles_environments_create(&self, request: GoogleCloudApigeeV1SecurityProfileEnvironmentAssociation, parent: &str) -> OrganizationSecurityProfileEnvironmentCreateCall<'a, S> {
        OrganizationSecurityProfileEnvironmentCreateCall {
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
    /// DeleteSecurityProfileEnvironmentAssociation removes profile environment association i.e. detaches environment from security profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the environment attachment to delete. Format: organizations/{org}/securityProfiles/{profile}/environments/{env}
    pub fn security_profiles_environments_delete(&self, name: &str) -> OrganizationSecurityProfileEnvironmentDeleteCall<'a, S> {
        OrganizationSecurityProfileEnvironmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// GetSecurityProfile gets the specified security profile. Returns NOT_FOUND if security profile is not present for the specified organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Security profile in the following format: `organizations/{org}/securityProfiles/{profile}'. Profile may optionally contain revision ID. If revision ID is not provided, the response will contain latest revision by default. Example: organizations/testOrg/securityProfiles/testProfile@5
    pub fn security_profiles_get(&self, name: &str) -> OrganizationSecurityProfileGetCall<'a, S> {
        OrganizationSecurityProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// ListSecurityProfiles lists all the security profiles associated with the org including attached and unattached profiles.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. For a specific organization, list of all the security profiles. Format: `organizations/{org}`
    pub fn security_profiles_list(&self, parent: &str) -> OrganizationSecurityProfileListCall<'a, S> {
        OrganizationSecurityProfileListCall {
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
    /// ListSecurityProfileRevisions lists all the revisions of the security profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. For a specific profile, list all the revisions. Format: `organizations/{org}/securityProfiles/{profile}`
    pub fn security_profiles_list_revisions(&self, name: &str) -> OrganizationSecurityProfileListRevisionCall<'a, S> {
        OrganizationSecurityProfileListRevisionCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of a shared flow.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the shared flow for which to return deployment information in the following format: `organizations/{org}/sharedflows/{sharedflow}`
    pub fn sharedflows_deployments_list(&self, parent: &str) -> OrganizationSharedflowDeploymentListCall<'a, S> {
        OrganizationSharedflowDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all deployments of a shared flow revision.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the API proxy revision for which to return deployment information in the following format: `organizations/{org}/sharedflows/{sharedflow}/revisions/{rev}`.
    pub fn sharedflows_revisions_deployments_list(&self, parent: &str) -> OrganizationSharedflowRevisionDeploymentListCall<'a, S> {
        OrganizationSharedflowRevisionDeploymentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a shared flow and all associated policies, resources, and revisions. You must undeploy the shared flow before deleting it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the shared flow revision to delete. Must be of the form: `organizations/{organization_id}/sharedflows/{shared_flow_id}/revisions/{revision_id}`
    pub fn sharedflows_revisions_delete(&self, name: &str) -> OrganizationSharedflowRevisionDeleteCall<'a, S> {
        OrganizationSharedflowRevisionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a revision of a shared flow. To download the shared flow configuration bundle for the specified revision as a zip file, set the `format` query parameter to `bundle`. If you are using curl, specify `-o filename.zip` to save the output to a file; otherwise, it displays to `stdout`. Then, develop the shared flow configuration locally and upload the updated sharedFlow configuration revision, as described in [updateSharedFlowRevision](updateSharedFlowRevision).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the shared flow revision to get. Must be of the form: `organizations/{organization_id}/sharedflows/{shared_flow_id}/revisions/{revision_id}`
    pub fn sharedflows_revisions_get(&self, name: &str) -> OrganizationSharedflowRevisionGetCall<'a, S> {
        OrganizationSharedflowRevisionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a shared flow revision. This operation is only allowed on revisions which have never been deployed. After deployment a revision becomes immutable, even if it becomes undeployed. The payload is a ZIP-formatted shared flow. Content type must be either multipart/form-data or application/octet-stream.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the shared flow revision to update. Must be of the form: `organizations/{organization_id}/sharedflows/{shared_flow_id}/revisions/{revision_id}`
    pub fn sharedflows_revisions_update_shared_flow_revision(&self, request: GoogleApiHttpBody, name: &str) -> OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall<'a, S> {
        OrganizationSharedflowRevisionUpdateSharedFlowRevisionCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a ZIP-formatted shared flow configuration bundle to an organization. If the shared flow already exists, this creates a new revision of it. If the shared flow does not exist, this creates it. Once imported, the shared flow revision must be deployed before it can be accessed at runtime. The size limit of a shared flow bundle is 15 MB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the parent organization under which to create the shared flow. Must be of the form: `organizations/{organization_id}`
    pub fn sharedflows_create(&self, request: GoogleApiHttpBody, parent: &str) -> OrganizationSharedflowCreateCall<'a, S> {
        OrganizationSharedflowCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a shared flow and all it's revisions. The shared flow must be undeployed before you can delete it.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. shared flow name of the form: `organizations/{organization_id}/sharedflows/{shared_flow_id}`
    pub fn sharedflows_delete(&self, name: &str) -> OrganizationSharedflowDeleteCall<'a, S> {
        OrganizationSharedflowDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a shared flow by name, including a list of its revisions.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the shared flow to get. Must be of the form: `organizations/{organization_id}/sharedflows/{shared_flow_id}`
    pub fn sharedflows_get(&self, name: &str) -> OrganizationSharedflowGetCall<'a, S> {
        OrganizationSharedflowGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all shared flows in the organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent organization under which to get shared flows. Must be of the form: `organizations/{organization_id}`
    pub fn sharedflows_list(&self, parent: &str) -> OrganizationSharedflowListCall<'a, S> {
        OrganizationSharedflowListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _include_revisions: Default::default(),
            _include_meta_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new category on the portal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}`
    pub fn sites_apicategories_create(&self, request: GoogleCloudApigeeV1ApiCategoryData, parent: &str) -> OrganizationSiteApicategoryCreateCall<'a, S> {
        OrganizationSiteApicategoryCreateCall {
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
    /// Deletes a category from the portal.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the category. Use the following structure in your request: `organizations/{org}/sites/{site}/apicategories/{apicategory}`
    pub fn sites_apicategories_delete(&self, name: &str) -> OrganizationSiteApicategoryDeleteCall<'a, S> {
        OrganizationSiteApicategoryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a category on the portal.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the category. Use the following structure in your request: `organizations/{org}/sites/{site}/apicategories/{apicategory}`
    pub fn sites_apicategories_get(&self, name: &str) -> OrganizationSiteApicategoryGetCall<'a, S> {
        OrganizationSiteApicategoryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the categories on the portal.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}`
    pub fn sites_apicategories_list(&self, parent: &str) -> OrganizationSiteApicategoryListCall<'a, S> {
        OrganizationSiteApicategoryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a category on the portal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the category. Use the following structure in your request: `organizations/{org}/sites/{site}/apicategories/{apicategory}`
    pub fn sites_apicategories_patch(&self, request: GoogleCloudApigeeV1ApiCategoryData, name: &str) -> OrganizationSiteApicategoryPatchCall<'a, S> {
        OrganizationSiteApicategoryPatchCall {
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
    /// Creates an Apigee organization. See [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleCloudApigeeV1Organization) -> OrganizationCreateCall<'a, S> {
        OrganizationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete an Apigee organization. For organizations with BillingType EVALUATION, an immediate deletion is performed. For paid organizations, a soft-deletion is performed. The organization can be restored within the soft-deletion period which can be controlled using the retention field in the request.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}`
    pub fn delete(&self, name: &str) -> OrganizationDeleteCall<'a, S> {
        OrganizationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _retention: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the profile for an Apigee organization. See [Understanding organizations](https://cloud.google.com/apigee/docs/api-platform/fundamentals/organization-structure).
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Apigee organization name in the following format: `organizations/{org}`
    pub fn get(&self, name: &str) -> OrganizationGetCall<'a, S> {
        OrganizationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the deployed ingress configuration for an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the deployed configuration for the organization in the following format: 'organizations/{org}/deployedIngressConfig'.
    pub fn get_deployed_ingress_config(&self, name: &str) -> OrganizationGetDeployedIngressConfigCall<'a, S> {
        OrganizationGetDeployedIngressConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the project ID and region for an Apigee organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Apigee organization name in the following format: `organizations/{org}`
    pub fn get_project_mapping(&self, name: &str) -> OrganizationGetProjectMappingCall<'a, S> {
        OrganizationGetProjectMappingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get runtime config for an organization.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the runtime config for the organization in the following format: 'organizations/{org}/runtimeConfig'.
    pub fn get_runtime_config(&self, name: &str) -> OrganizationGetRuntimeConfigCall<'a, S> {
        OrganizationGetRuntimeConfigCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the service accounts with the permissions required to allow the Synchronizer to download environment data from the control plane. An ETag is returned in the response to `getSyncAuthorization`. Pass that ETag when calling [setSyncAuthorization](setSyncAuthorization) to ensure that you are updating the correct version. If you don’t pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. For more information, see [Configure the Synchronizer](https://cloud.google.com/apigee/docs/hybrid/latest/synchronizer-access). **Note**: Available to Apigee hybrid only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Apigee organization. Use the following structure in your request: `organizations/{org}`
    pub fn get_sync_authorization(&self, request: GoogleCloudApigeeV1GetSyncAuthorizationRequest, name: &str) -> OrganizationGetSyncAuthorizationCall<'a, S> {
        OrganizationGetSyncAuthorizationCall {
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
    /// Lists the Apigee organizations and associated Google Cloud projects that you have permission to access. See [Understanding organizations](https://cloud.google.com/apigee/docs/api-platform/fundamentals/organization-structure).
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Use the following structure in your request: `organizations`
    pub fn list(&self, parent: &str) -> OrganizationListCall<'a, S> {
        OrganizationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Configures the add-ons for the Apigee organization. The existing add-on configuration will be fully replaced.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `org` - Required. Name of the organization. Use the following structure in your request: `organizations/{org}`
    pub fn set_addons(&self, request: GoogleCloudApigeeV1SetAddonsRequest, org: &str) -> OrganizationSetAddonCall<'a, S> {
        OrganizationSetAddonCall {
            hub: self.hub,
            _request: request,
            _org: org.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the permissions required to allow the Synchronizer to download environment data from the control plane. You must call this API to enable proper functioning of hybrid. Pass the ETag when calling `setSyncAuthorization` to ensure that you are updating the correct version. To get an ETag, call [getSyncAuthorization](getSyncAuthorization). If you don’t pass the ETag in the call to `setSyncAuthorization`, then the existing authorization is overwritten indiscriminately. For more information, see [Configure the Synchronizer](https://cloud.google.com/apigee/docs/hybrid/latest/synchronizer-access). **Note**: Available to Apigee hybrid only.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the Apigee organization. Use the following structure in your request: `organizations/{org}`
    pub fn set_sync_authorization(&self, request: GoogleCloudApigeeV1SyncAuthorization, name: &str) -> OrganizationSetSyncAuthorizationCall<'a, S> {
        OrganizationSetSyncAuthorizationCall {
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
    /// Updates the properties for an Apigee organization. No other fields in the organization profile will be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Apigee organization name in the following format: `organizations/{org}`
    pub fn update(&self, request: GoogleCloudApigeeV1Organization, name: &str) -> OrganizationUpdateCall<'a, S> {
        OrganizationUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Apigee`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_apigee1 as apigee1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use apigee1::{Apigee, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Apigee::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `provision_organization(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Apigee<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Provisions a new Apigee organization with a functioning runtime. This is the standard way to create trial organizations for a free Apigee trial.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - Required. Name of the GCP project with which to associate the Apigee organization.
    pub fn provision_organization(&self, request: GoogleCloudApigeeV1ProvisionOrganizationRequest, project: &str) -> ProjectProvisionOrganizationCall<'a, S> {
        ProjectProvisionOrganizationCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



