use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`RecommendationsAI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_recommendationengine1_beta1 as recommendationengine1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use recommendationengine1_beta1::{RecommendationsAI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = RecommendationsAI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_catalogs_catalog_items_create(...)`, `locations_catalogs_catalog_items_delete(...)`, `locations_catalogs_catalog_items_get(...)`, `locations_catalogs_catalog_items_import(...)`, `locations_catalogs_catalog_items_list(...)`, `locations_catalogs_catalog_items_patch(...)`, `locations_catalogs_event_stores_operations_get(...)`, `locations_catalogs_event_stores_operations_list(...)`, `locations_catalogs_event_stores_placements_predict(...)`, `locations_catalogs_event_stores_prediction_api_key_registrations_create(...)`, `locations_catalogs_event_stores_prediction_api_key_registrations_delete(...)`, `locations_catalogs_event_stores_prediction_api_key_registrations_list(...)`, `locations_catalogs_event_stores_user_events_collect(...)`, `locations_catalogs_event_stores_user_events_import(...)`, `locations_catalogs_event_stores_user_events_list(...)`, `locations_catalogs_event_stores_user_events_purge(...)`, `locations_catalogs_event_stores_user_events_rejoin(...)`, `locations_catalogs_event_stores_user_events_write(...)`, `locations_catalogs_list(...)`, `locations_catalogs_operations_get(...)`, `locations_catalogs_operations_list(...)` and `locations_catalogs_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a RecommendationsAI<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a catalog item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_catalog_items_create(&self, request: GoogleCloudRecommendationengineV1beta1CatalogItem, parent: &str) -> ProjectLocationCatalogCatalogItemCreateCall<'a, S> {
        ProjectLocationCatalogCatalogItemCreateCall {
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
    /// Deletes a catalog item.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full resource name of catalog item, such as `projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id`.
    pub fn locations_catalogs_catalog_items_delete(&self, name: &str) -> ProjectLocationCatalogCatalogItemDeleteCall<'a, S> {
        ProjectLocationCatalogCatalogItemDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific catalog item.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Full resource name of catalog item, such as `projects/*/locations/global/catalogs/default_catalog/catalogitems/some_catalog_item_id`.
    pub fn locations_catalogs_catalog_items_get(&self, name: &str) -> ProjectLocationCatalogCatalogItemGetCall<'a, S> {
        ProjectLocationCatalogCatalogItemGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk import of multiple catalog items. Request processing may be synchronous. No partial updating supported. Non-existing items will be created. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. `projects/1234/locations/global/catalogs/default_catalog` If no updateMask is specified, requires catalogItems.create permission. If updateMask is specified, requires catalogItems.update permission.
    pub fn locations_catalogs_catalog_items_import(&self, request: GoogleCloudRecommendationengineV1beta1ImportCatalogItemsRequest, parent: &str) -> ProjectLocationCatalogCatalogItemImportCall<'a, S> {
        ProjectLocationCatalogCatalogItemImportCall {
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
    /// Gets a list of catalog items.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`.
    pub fn locations_catalogs_catalog_items_list(&self, parent: &str) -> ProjectLocationCatalogCatalogItemListCall<'a, S> {
        ProjectLocationCatalogCatalogItemListCall {
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
    /// Updates a catalog item. Partial updating is supported. Non-existing items will be created.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Full resource name of catalog item, such as `projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id`.
    pub fn locations_catalogs_catalog_items_patch(&self, request: GoogleCloudRecommendationengineV1beta1CatalogItem, name: &str) -> ProjectLocationCatalogCatalogItemPatchCall<'a, S> {
        ProjectLocationCatalogCatalogItemPatchCall {
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
    pub fn locations_catalogs_event_stores_operations_get(&self, name: &str) -> ProjectLocationCatalogEventStoreOperationGetCall<'a, S> {
        ProjectLocationCatalogEventStoreOperationGetCall {
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
    pub fn locations_catalogs_event_stores_operations_list(&self, name: &str) -> ProjectLocationCatalogEventStoreOperationListCall<'a, S> {
        ProjectLocationCatalogEventStoreOperationListCall {
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
    /// Makes a recommendation prediction. If using API Key based authentication, the API Key must be registered using the PredictionApiKeyRegistry service. [Learn more](https://cloud.google.com/recommendations-ai/docs/setting-up#register-key).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - No description provided.
    pub fn locations_catalogs_event_stores_placements_predict(&self, request: GoogleCloudRecommendationengineV1beta1PredictRequest, name: &str) -> ProjectLocationCatalogEventStorePlacementPredictCall<'a, S> {
        ProjectLocationCatalogEventStorePlacementPredictCall {
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
    /// Register an API key for use with predict method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource path. `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
    pub fn locations_catalogs_event_stores_prediction_api_key_registrations_create(&self, request: GoogleCloudRecommendationengineV1beta1CreatePredictionApiKeyRegistrationRequest, parent: &str) -> ProjectLocationCatalogEventStorePredictionApiKeyRegistrationCreateCall<'a, S> {
        ProjectLocationCatalogEventStorePredictionApiKeyRegistrationCreateCall {
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
    /// Unregister an apiKey from using for predict method.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The API key to unregister including full resource path. `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/predictionApiKeyRegistrations/`
    pub fn locations_catalogs_event_stores_prediction_api_key_registrations_delete(&self, name: &str) -> ProjectLocationCatalogEventStorePredictionApiKeyRegistrationDeleteCall<'a, S> {
        ProjectLocationCatalogEventStorePredictionApiKeyRegistrationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the registered apiKeys for use with predict method.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent placement resource name such as `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`
    pub fn locations_catalogs_event_stores_prediction_api_key_registrations_list(&self, parent: &str) -> ProjectLocationCatalogEventStorePredictionApiKeyRegistrationListCall<'a, S> {
        ProjectLocationCatalogEventStorePredictionApiKeyRegistrationListCall {
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
    /// Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Recommendations AI JavaScript pixel. Users should not call this method directly.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent eventStore name, such as `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
    pub fn locations_catalogs_event_stores_user_events_collect(&self, parent: &str) -> ProjectLocationCatalogEventStoreUserEventCollectCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventCollectCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _user_event: Default::default(),
            _uri: Default::default(),
            _ets: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. `projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store`
    pub fn locations_catalogs_event_stores_user_events_import(&self, request: GoogleCloudRecommendationengineV1beta1ImportUserEventsRequest, parent: &str) -> ProjectLocationCatalogEventStoreUserEventImportCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventImportCall {
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
    /// Gets a list of user events within a time range, with potential filtering. The method does not list unjoined user events. Unjoined user event definition: when a user event is ingested from Recommendations AI User Event APIs, the catalog item included in the user event is connected with the current catalog. If a catalog item of the ingested event is not in the current catalog, it could lead to degraded model quality. This is called an unjoined event.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent eventStore resource name, such as `projects/*/locations/*/catalogs/default_catalog/eventStores/default_event_store`.
    pub fn locations_catalogs_event_stores_user_events_list(&self, parent: &str) -> ProjectLocationCatalogEventStoreUserEventListCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventListCall {
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
    /// Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The resource name of the event_store under which the events are created. The format is `projects/${projectId}/locations/global/catalogs/${catalogId}/eventStores/${eventStoreId}`
    pub fn locations_catalogs_event_stores_user_events_purge(&self, request: GoogleCloudRecommendationengineV1beta1PurgeUserEventsRequest, parent: &str) -> ProjectLocationCatalogEventStoreUserEventPurgeCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventPurgeCall {
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
    /// Triggers a user event rejoin operation with latest catalog data. Events will not be annotated with detailed catalog information if catalog item is missing at the time the user event is ingested, and these events are stored as unjoined events with a limited usage on training and serving. This API can be used to trigger a 'join' operation on specified events with latest version of catalog items. It can also be used to correct events joined with wrong catalog items.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Full resource name of user event, such as `projects/*/locations/*/catalogs/default_catalog/eventStores/default_event_store`.
    pub fn locations_catalogs_event_stores_user_events_rejoin(&self, request: GoogleCloudRecommendationengineV1beta1RejoinUserEventsRequest, parent: &str) -> ProjectLocationCatalogEventStoreUserEventRejoinCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventRejoinCall {
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
    /// * `parent` - Required. The parent eventStore resource name, such as "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store".
    pub fn locations_catalogs_event_stores_user_events_write(&self, request: GoogleCloudRecommendationengineV1beta1UserEvent, parent: &str) -> ProjectLocationCatalogEventStoreUserEventWriteCall<'a, S> {
        ProjectLocationCatalogEventStoreUserEventWriteCall {
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
    /// Lists all the catalog configurations associated with the project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The account resource name with an associated location.
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
    /// Updates the catalog configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The fully qualified resource name of the catalog.
    pub fn locations_catalogs_patch(&self, request: GoogleCloudRecommendationengineV1beta1Catalog, name: &str) -> ProjectLocationCatalogPatchCall<'a, S> {
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
}



