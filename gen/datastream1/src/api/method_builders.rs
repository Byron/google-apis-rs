use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Datastream`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_datastream1 as datastream1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use datastream1::{Datastream, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Datastream::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_connection_profiles_create(...)`, `locations_connection_profiles_delete(...)`, `locations_connection_profiles_discover(...)`, `locations_connection_profiles_get(...)`, `locations_connection_profiles_list(...)`, `locations_connection_profiles_patch(...)`, `locations_fetch_static_ips(...)`, `locations_get(...)`, `locations_list(...)`, `locations_operations_cancel(...)`, `locations_operations_delete(...)`, `locations_operations_get(...)`, `locations_operations_list(...)`, `locations_private_connections_create(...)`, `locations_private_connections_delete(...)`, `locations_private_connections_get(...)`, `locations_private_connections_list(...)`, `locations_private_connections_routes_create(...)`, `locations_private_connections_routes_delete(...)`, `locations_private_connections_routes_get(...)`, `locations_private_connections_routes_list(...)`, `locations_streams_create(...)`, `locations_streams_delete(...)`, `locations_streams_get(...)`, `locations_streams_list(...)`, `locations_streams_objects_get(...)`, `locations_streams_objects_list(...)`, `locations_streams_objects_lookup(...)`, `locations_streams_objects_start_backfill_job(...)`, `locations_streams_objects_stop_backfill_job(...)` and `locations_streams_patch(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Datastream<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to create a connection profile in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent that owns the collection of ConnectionProfiles.
    pub fn locations_connection_profiles_create(&self, request: ConnectionProfile, parent: &str) -> ProjectLocationConnectionProfileCreateCall<'a, S> {
        ProjectLocationConnectionProfileCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _force: Default::default(),
            _connection_profile_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to delete a connection profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the connection profile resource to delete.
    pub fn locations_connection_profiles_delete(&self, name: &str) -> ProjectLocationConnectionProfileDeleteCall<'a, S> {
        ProjectLocationConnectionProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to discover a connection profile. The discover API call exposes the data objects and metadata belonging to the profile. Typically, a request returns children data objects of a parent data object that's optionally supplied in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource of the connection profile type. Must be in the format `projects/*/locations/*`.
    pub fn locations_connection_profiles_discover(&self, request: DiscoverConnectionProfileRequest, parent: &str) -> ProjectLocationConnectionProfileDiscoverCall<'a, S> {
        ProjectLocationConnectionProfileDiscoverCall {
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
    /// Use this method to get details about a connection profile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the connection profile resource to get.
    pub fn locations_connection_profiles_get(&self, name: &str) -> ProjectLocationConnectionProfileGetCall<'a, S> {
        ProjectLocationConnectionProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to list connection profiles created in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent that owns the collection of connection profiles.
    pub fn locations_connection_profiles_list(&self, parent: &str) -> ProjectLocationConnectionProfileListCall<'a, S> {
        ProjectLocationConnectionProfileListCall {
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
    /// Use this method to update the parameters of a connection profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource's name.
    pub fn locations_connection_profiles_patch(&self, request: ConnectionProfile, name: &str) -> ProjectLocationConnectionProfilePatchCall<'a, S> {
        ProjectLocationConnectionProfilePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _force: Default::default(),
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
    /// Use this method to create a route for a private connectivity configuration in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent that owns the collection of Routes.
    pub fn locations_private_connections_routes_create(&self, request: Route, parent: &str) -> ProjectLocationPrivateConnectionRouteCreateCall<'a, S> {
        ProjectLocationPrivateConnectionRouteCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _route_id: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to delete a route.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Route resource to delete.
    pub fn locations_private_connections_routes_delete(&self, name: &str) -> ProjectLocationPrivateConnectionRouteDeleteCall<'a, S> {
        ProjectLocationPrivateConnectionRouteDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to get details about a route.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Route resource to get.
    pub fn locations_private_connections_routes_get(&self, name: &str) -> ProjectLocationPrivateConnectionRouteGetCall<'a, S> {
        ProjectLocationPrivateConnectionRouteGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to list routes created for a private connectivity configuration in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent that owns the collection of Routess.
    pub fn locations_private_connections_routes_list(&self, parent: &str) -> ProjectLocationPrivateConnectionRouteListCall<'a, S> {
        ProjectLocationPrivateConnectionRouteListCall {
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
    /// Use this method to create a private connectivity configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent that owns the collection of PrivateConnections.
    pub fn locations_private_connections_create(&self, request: PrivateConnection, parent: &str) -> ProjectLocationPrivateConnectionCreateCall<'a, S> {
        ProjectLocationPrivateConnectionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _request_id: Default::default(),
            _private_connection_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to delete a private connectivity configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the private connectivity configuration to delete.
    pub fn locations_private_connections_delete(&self, name: &str) -> ProjectLocationPrivateConnectionDeleteCall<'a, S> {
        ProjectLocationPrivateConnectionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to get details about a private connectivity configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the private connectivity configuration to get.
    pub fn locations_private_connections_get(&self, name: &str) -> ProjectLocationPrivateConnectionGetCall<'a, S> {
        ProjectLocationPrivateConnectionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to list private connectivity configurations in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent that owns the collection of private connectivity configurations.
    pub fn locations_private_connections_list(&self, parent: &str) -> ProjectLocationPrivateConnectionListCall<'a, S> {
        ProjectLocationPrivateConnectionListCall {
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
    /// Use this method to get details about a stream object.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the stream object resource to get.
    pub fn locations_streams_objects_get(&self, name: &str) -> ProjectLocationStreamObjectGetCall<'a, S> {
        ProjectLocationStreamObjectGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to list the objects of a specific stream.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent stream that owns the collection of objects.
    pub fn locations_streams_objects_list(&self, parent: &str) -> ProjectLocationStreamObjectListCall<'a, S> {
        ProjectLocationStreamObjectListCall {
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
    /// Use this method to look up a stream object by its source object identifier.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent stream that owns the collection of objects.
    pub fn locations_streams_objects_lookup(&self, request: LookupStreamObjectRequest, parent: &str) -> ProjectLocationStreamObjectLookupCall<'a, S> {
        ProjectLocationStreamObjectLookupCall {
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
    /// Use this method to start a backfill job for the specified stream object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `object` - Required. The name of the stream object resource to start a backfill job for.
    pub fn locations_streams_objects_start_backfill_job(&self, request: StartBackfillJobRequest, object: &str) -> ProjectLocationStreamObjectStartBackfillJobCall<'a, S> {
        ProjectLocationStreamObjectStartBackfillJobCall {
            hub: self.hub,
            _request: request,
            _object: object.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to stop a backfill job for the specified stream object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `object` - Required. The name of the stream object resource to stop the backfill job for.
    pub fn locations_streams_objects_stop_backfill_job(&self, request: StopBackfillJobRequest, object: &str) -> ProjectLocationStreamObjectStopBackfillJobCall<'a, S> {
        ProjectLocationStreamObjectStopBackfillJobCall {
            hub: self.hub,
            _request: request,
            _object: object.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to create a stream.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent that owns the collection of streams.
    pub fn locations_streams_create(&self, request: Stream, parent: &str) -> ProjectLocationStreamCreateCall<'a, S> {
        ProjectLocationStreamCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _stream_id: Default::default(),
            _request_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to delete a stream.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the stream resource to delete.
    pub fn locations_streams_delete(&self, name: &str) -> ProjectLocationStreamDeleteCall<'a, S> {
        ProjectLocationStreamDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to get details about a stream.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the stream resource to get.
    pub fn locations_streams_get(&self, name: &str) -> ProjectLocationStreamGetCall<'a, S> {
        ProjectLocationStreamGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Use this method to list streams in a project and location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent that owns the collection of streams.
    pub fn locations_streams_list(&self, parent: &str) -> ProjectLocationStreamListCall<'a, S> {
        ProjectLocationStreamListCall {
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
    /// Use this method to update the configuration of a stream.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The stream's name.
    pub fn locations_streams_patch(&self, request: Stream, name: &str) -> ProjectLocationStreamPatchCall<'a, S> {
        ProjectLocationStreamPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _request_id: Default::default(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The FetchStaticIps API call exposes the static IP addresses used by Datastream.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name for the location for which static IPs should be returned. Must be in the format `projects/*/locations/*`.
    pub fn locations_fetch_static_ips(&self, name: &str) -> ProjectLocationFetchStaticIpCall<'a, S> {
        ProjectLocationFetchStaticIpCall {
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



