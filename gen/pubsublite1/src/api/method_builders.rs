use super::*;
/// A builder providing access to all methods supported on *admin* resources.
/// It is not used directly, but through the [`PubsubLite`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pubsublite1 as pubsublite1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pubsublite1::{PubsubLite, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PubsubLite::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `projects_locations_operations_cancel(...)`, `projects_locations_operations_delete(...)`, `projects_locations_operations_get(...)`, `projects_locations_operations_list(...)`, `projects_locations_reservations_create(...)`, `projects_locations_reservations_delete(...)`, `projects_locations_reservations_get(...)`, `projects_locations_reservations_list(...)`, `projects_locations_reservations_patch(...)`, `projects_locations_reservations_topics_list(...)`, `projects_locations_subscriptions_create(...)`, `projects_locations_subscriptions_delete(...)`, `projects_locations_subscriptions_get(...)`, `projects_locations_subscriptions_list(...)`, `projects_locations_subscriptions_patch(...)`, `projects_locations_subscriptions_seek(...)`, `projects_locations_topics_create(...)`, `projects_locations_topics_delete(...)`, `projects_locations_topics_get(...)`, `projects_locations_topics_get_partitions(...)`, `projects_locations_topics_list(...)`, `projects_locations_topics_patch(...)` and `projects_locations_topics_subscriptions_list(...)`
/// // to build up your call.
/// let rb = hub.admin();
/// # }
/// ```
pub struct AdminMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PubsubLite<S>,
}

impl<'a, S> client::MethodsBuilder for AdminMethods<'a, S> {}

impl<'a, S> AdminMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the operation resource to be cancelled.
    pub fn projects_locations_operations_cancel(&self, request: CancelOperationRequest, name: &str) -> AdminProjectLocationOperationCancelCall<'a, S> {
        AdminProjectLocationOperationCancelCall {
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
    pub fn projects_locations_operations_delete(&self, name: &str) -> AdminProjectLocationOperationDeleteCall<'a, S> {
        AdminProjectLocationOperationDeleteCall {
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
    pub fn projects_locations_operations_get(&self, name: &str) -> AdminProjectLocationOperationGetCall<'a, S> {
        AdminProjectLocationOperationGetCall {
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
    pub fn projects_locations_operations_list(&self, name: &str) -> AdminProjectLocationOperationListCall<'a, S> {
        AdminProjectLocationOperationListCall {
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
    /// Lists the topics attached to the specified reservation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the reservation whose topics to list. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    pub fn projects_locations_reservations_topics_list(&self, name: &str) -> AdminProjectLocationReservationTopicListCall<'a, S> {
        AdminProjectLocationReservationTopicListCall {
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
    /// Creates a new reservation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent location in which to create the reservation. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_reservations_create(&self, request: Reservation, parent: &str) -> AdminProjectLocationReservationCreateCall<'a, S> {
        AdminProjectLocationReservationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _reservation_id: Default::default(),
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
    /// * `name` - Required. The name of the reservation to delete. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    pub fn projects_locations_reservations_delete(&self, name: &str) -> AdminProjectLocationReservationDeleteCall<'a, S> {
        AdminProjectLocationReservationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the reservation configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the reservation whose configuration to return. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    pub fn projects_locations_reservations_get(&self, name: &str) -> AdminProjectLocationReservationGetCall<'a, S> {
        AdminProjectLocationReservationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of reservations for the given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent whose reservations are to be listed. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_reservations_list(&self, parent: &str) -> AdminProjectLocationReservationListCall<'a, S> {
        AdminProjectLocationReservationListCall {
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
    /// Updates properties of the specified reservation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the reservation. Structured like: projects/{project_number}/locations/{location}/reservations/{reservation_id}
    pub fn projects_locations_reservations_patch(&self, request: Reservation, name: &str) -> AdminProjectLocationReservationPatchCall<'a, S> {
        AdminProjectLocationReservationPatchCall {
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
    /// Creates a new subscription.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent location in which to create the subscription. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_subscriptions_create(&self, request: Subscription, parent: &str) -> AdminProjectLocationSubscriptionCreateCall<'a, S> {
        AdminProjectLocationSubscriptionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _skip_backlog: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified subscription.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the subscription to delete.
    pub fn projects_locations_subscriptions_delete(&self, name: &str) -> AdminProjectLocationSubscriptionDeleteCall<'a, S> {
        AdminProjectLocationSubscriptionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the subscription configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the subscription whose configuration to return.
    pub fn projects_locations_subscriptions_get(&self, name: &str) -> AdminProjectLocationSubscriptionGetCall<'a, S> {
        AdminProjectLocationSubscriptionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of subscriptions for the given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent whose subscriptions are to be listed. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_subscriptions_list(&self, parent: &str) -> AdminProjectLocationSubscriptionListCall<'a, S> {
        AdminProjectLocationSubscriptionListCall {
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
    /// Updates properties of the specified subscription.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id}
    pub fn projects_locations_subscriptions_patch(&self, request: Subscription, name: &str) -> AdminProjectLocationSubscriptionPatchCall<'a, S> {
        AdminProjectLocationSubscriptionPatchCall {
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
    /// Performs an out-of-band seek for a subscription to a specified target, which may be timestamps or named positions within the message backlog. Seek translates these targets to cursors for each partition and orchestrates subscribers to start consuming messages from these seek cursors. If an operation is returned, the seek has been registered and subscribers will eventually receive messages from the seek cursors (i.e. eventual consistency), as long as they are using a minimum supported client library version and not a system that tracks cursors independently of Pub/Sub Lite (e.g. Apache Beam, Dataflow, Spark). The seek operation will fail for unsupported clients. If clients would like to know when subscribers react to the seek (or not), they can poll the operation. The seek operation will succeed and complete once subscribers are ready to receive messages from the seek cursors for all partitions of the topic. This means that the seek operation will not complete until all subscribers come online. If the previous seek operation has not yet completed, it will be aborted and the new invocation of seek will supersede it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription to seek.
    pub fn projects_locations_subscriptions_seek(&self, request: SeekSubscriptionRequest, name: &str) -> AdminProjectLocationSubscriptionSeekCall<'a, S> {
        AdminProjectLocationSubscriptionSeekCall {
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
    /// Lists the subscriptions attached to the specified topic.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the topic whose subscriptions to list.
    pub fn projects_locations_topics_subscriptions_list(&self, name: &str) -> AdminProjectLocationTopicSubscriptionListCall<'a, S> {
        AdminProjectLocationTopicSubscriptionListCall {
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
    /// Creates a new topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent location in which to create the topic. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_topics_create(&self, request: Topic, parent: &str) -> AdminProjectLocationTopicCreateCall<'a, S> {
        AdminProjectLocationTopicCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _topic_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified topic.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the topic to delete.
    pub fn projects_locations_topics_delete(&self, name: &str) -> AdminProjectLocationTopicDeleteCall<'a, S> {
        AdminProjectLocationTopicDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the topic configuration.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the topic whose configuration to return.
    pub fn projects_locations_topics_get(&self, name: &str) -> AdminProjectLocationTopicGetCall<'a, S> {
        AdminProjectLocationTopicGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the partition information for the requested topic.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The topic whose partition information to return.
    pub fn projects_locations_topics_get_partitions(&self, name: &str) -> AdminProjectLocationTopicGetPartitionCall<'a, S> {
        AdminProjectLocationTopicGetPartitionCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of topics for the given project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent whose topics are to be listed. Structured like `projects/{project_number}/locations/{location}`.
    pub fn projects_locations_topics_list(&self, parent: &str) -> AdminProjectLocationTopicListCall<'a, S> {
        AdminProjectLocationTopicListCall {
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
    /// Updates properties of the specified topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}
    pub fn projects_locations_topics_patch(&self, request: Topic, name: &str) -> AdminProjectLocationTopicPatchCall<'a, S> {
        AdminProjectLocationTopicPatchCall {
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



/// A builder providing access to all methods supported on *cursor* resources.
/// It is not used directly, but through the [`PubsubLite`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pubsublite1 as pubsublite1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pubsublite1::{PubsubLite, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PubsubLite::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `projects_locations_subscriptions_commit_cursor(...)` and `projects_locations_subscriptions_cursors_list(...)`
/// // to build up your call.
/// let rb = hub.cursor();
/// # }
/// ```
pub struct CursorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PubsubLite<S>,
}

impl<'a, S> client::MethodsBuilder for CursorMethods<'a, S> {}

impl<'a, S> CursorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all committed cursor information for a subscription.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The subscription for which to retrieve cursors. Structured like `projects/{project_number}/locations/{location}/subscriptions/{subscription_id}`.
    pub fn projects_locations_subscriptions_cursors_list(&self, parent: &str) -> CursorProjectLocationSubscriptionCursorListCall<'a, S> {
        CursorProjectLocationSubscriptionCursorListCall {
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
    /// Updates the committed cursor.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - The subscription for which to update the cursor.
    pub fn projects_locations_subscriptions_commit_cursor(&self, request: CommitCursorRequest, subscription: &str) -> CursorProjectLocationSubscriptionCommitCursorCall<'a, S> {
        CursorProjectLocationSubscriptionCommitCursorCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *topicStat* resources.
/// It is not used directly, but through the [`PubsubLite`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pubsublite1 as pubsublite1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pubsublite1::{PubsubLite, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PubsubLite::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `projects_locations_topics_compute_head_cursor(...)`, `projects_locations_topics_compute_message_stats(...)` and `projects_locations_topics_compute_time_cursor(...)`
/// // to build up your call.
/// let rb = hub.topic_stats();
/// # }
/// ```
pub struct TopicStatMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PubsubLite<S>,
}

impl<'a, S> client::MethodsBuilder for TopicStatMethods<'a, S> {}

impl<'a, S> TopicStatMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Compute the head cursor for the partition. The head cursor's offset is guaranteed to be less than or equal to all messages which have not yet been acknowledged as published, and greater than the offset of any message whose publish has already been acknowledged. It is zero if there have never been messages in the partition.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `topic` - Required. The topic for which we should compute the head cursor.
    pub fn projects_locations_topics_compute_head_cursor(&self, request: ComputeHeadCursorRequest, topic: &str) -> TopicStatProjectLocationTopicComputeHeadCursorCall<'a, S> {
        TopicStatProjectLocationTopicComputeHeadCursorCall {
            hub: self.hub,
            _request: request,
            _topic: topic.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Compute statistics about a range of messages in a given topic and partition.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `topic` - Required. The topic for which we should compute message stats.
    pub fn projects_locations_topics_compute_message_stats(&self, request: ComputeMessageStatsRequest, topic: &str) -> TopicStatProjectLocationTopicComputeMessageStatCall<'a, S> {
        TopicStatProjectLocationTopicComputeMessageStatCall {
            hub: self.hub,
            _request: request,
            _topic: topic.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Compute the corresponding cursor for a publish or event time in a topic partition.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `topic` - Required. The topic for which we should compute the cursor.
    pub fn projects_locations_topics_compute_time_cursor(&self, request: ComputeTimeCursorRequest, topic: &str) -> TopicStatProjectLocationTopicComputeTimeCursorCall<'a, S> {
        TopicStatProjectLocationTopicComputeTimeCursorCall {
            hub: self.hub,
            _request: request,
            _topic: topic.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



