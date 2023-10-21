use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Pubsub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_pubsub1 as pubsub1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use pubsub1::{Pubsub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Pubsub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `schemas_commit(...)`, `schemas_create(...)`, `schemas_delete(...)`, `schemas_delete_revision(...)`, `schemas_get(...)`, `schemas_get_iam_policy(...)`, `schemas_list(...)`, `schemas_list_revisions(...)`, `schemas_rollback(...)`, `schemas_set_iam_policy(...)`, `schemas_test_iam_permissions(...)`, `schemas_validate(...)`, `schemas_validate_message(...)`, `snapshots_create(...)`, `snapshots_delete(...)`, `snapshots_get(...)`, `snapshots_get_iam_policy(...)`, `snapshots_list(...)`, `snapshots_patch(...)`, `snapshots_set_iam_policy(...)`, `snapshots_test_iam_permissions(...)`, `subscriptions_acknowledge(...)`, `subscriptions_create(...)`, `subscriptions_delete(...)`, `subscriptions_detach(...)`, `subscriptions_get(...)`, `subscriptions_get_iam_policy(...)`, `subscriptions_list(...)`, `subscriptions_modify_ack_deadline(...)`, `subscriptions_modify_push_config(...)`, `subscriptions_patch(...)`, `subscriptions_pull(...)`, `subscriptions_seek(...)`, `subscriptions_set_iam_policy(...)`, `subscriptions_test_iam_permissions(...)`, `topics_create(...)`, `topics_delete(...)`, `topics_get(...)`, `topics_get_iam_policy(...)`, `topics_list(...)`, `topics_patch(...)`, `topics_publish(...)`, `topics_set_iam_policy(...)`, `topics_snapshots_list(...)`, `topics_subscriptions_list(...)` and `topics_test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Pubsub<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits a new schema revision to an existing schema.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the schema we are revising. Format is `projects/{project}/schemas/{schema}`.
    pub fn schemas_commit(&self, request: CommitSchemaRequest, name: &str) -> ProjectSchemaCommitCall<'a, S> {
        ProjectSchemaCommitCall {
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
    /// Creates a schema.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which to create the schema. Format is `projects/{project-id}`.
    pub fn schemas_create(&self, request: Schema, parent: &str) -> ProjectSchemaCreateCall<'a, S> {
        ProjectSchemaCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _schema_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a schema.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the schema to delete. Format is `projects/{project}/schemas/{schema}`.
    pub fn schemas_delete(&self, name: &str) -> ProjectSchemaDeleteCall<'a, S> {
        ProjectSchemaDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a specific schema revision.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the schema revision to be deleted, with a revision ID explicitly included. Example: projects/123/schemas/my-schema@c7cfa2a8
    pub fn schemas_delete_revision(&self, name: &str) -> ProjectSchemaDeleteRevisionCall<'a, S> {
        ProjectSchemaDeleteRevisionCall {
            hub: self.hub,
            _name: name.to_string(),
            _revision_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a schema.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the schema to get. Format is `projects/{project}/schemas/{schema}`.
    pub fn schemas_get(&self, name: &str) -> ProjectSchemaGetCall<'a, S> {
        ProjectSchemaGetCall {
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
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn schemas_get_iam_policy(&self, resource: &str) -> ProjectSchemaGetIamPolicyCall<'a, S> {
        ProjectSchemaGetIamPolicyCall {
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
    /// Lists schemas in a project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the project in which to list schemas. Format is `projects/{project-id}`.
    pub fn schemas_list(&self, parent: &str) -> ProjectSchemaListCall<'a, S> {
        ProjectSchemaListCall {
            hub: self.hub,
            _parent: parent.to_string(),
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
    /// Lists all schema revisions for the named schema.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the schema to list revisions for.
    pub fn schemas_list_revisions(&self, name: &str) -> ProjectSchemaListRevisionCall<'a, S> {
        ProjectSchemaListRevisionCall {
            hub: self.hub,
            _name: name.to_string(),
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
    /// Creates a new schema revision that is a copy of the provided revision_id.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The schema being rolled back with revision id.
    pub fn schemas_rollback(&self, request: RollbackSchemaRequest, name: &str) -> ProjectSchemaRollbackCall<'a, S> {
        ProjectSchemaRollbackCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn schemas_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectSchemaSetIamPolicyCall<'a, S> {
        ProjectSchemaSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn schemas_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectSchemaTestIamPermissionCall<'a, S> {
        ProjectSchemaTestIamPermissionCall {
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
    /// Validates a schema.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which to validate schemas. Format is `projects/{project-id}`.
    pub fn schemas_validate(&self, request: ValidateSchemaRequest, parent: &str) -> ProjectSchemaValidateCall<'a, S> {
        ProjectSchemaValidateCall {
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
    /// Validates a message against a schema.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project in which to validate schemas. Format is `projects/{project-id}`.
    pub fn schemas_validate_message(&self, request: ValidateMessageRequest, parent: &str) -> ProjectSchemaValidateMessageCall<'a, S> {
        ProjectSchemaValidateMessageCall {
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
    /// Creates a snapshot from the requested subscription. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot. If the snapshot already exists, returns `ALREADY_EXISTS`. If the requested subscription doesn't exist, returns `NOT_FOUND`. If the backlog in the subscription is too old -- and the resulting snapshot would expire in less than 1 hour -- then `FAILED_PRECONDITION` is returned. See also the `Snapshot.expire_time` field. If the name is not provided in the request, the server will assign a random name for this snapshot on the same project as the subscription, conforming to the [resource name format] (https://cloud.google.com/pubsub/docs/admin#resource_names). The generated name is populated in the returned Snapshot object. Note that for REST API requests, you must specify a name in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. User-provided name for this snapshot. If the name is not provided in the request, the server will assign a random name for this snapshot on the same project as the subscription. Note that for REST API requests, you must specify a name. See the [resource name rules](https://cloud.google.com/pubsub/docs/admin#resource_names). Format is `projects/{project}/snapshots/{snap}`.
    pub fn snapshots_create(&self, request: CreateSnapshotRequest, name: &str) -> ProjectSnapshotCreateCall<'a, S> {
        ProjectSnapshotCreateCall {
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
    /// Removes an existing snapshot. Snapshots are used in [Seek] (https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot. When the snapshot is deleted, all messages retained in the snapshot are immediately dropped. After a snapshot is deleted, a new one may be created with the same name, but the new one has no association with the old snapshot or its subscription, unless the same subscription is specified.
    /// 
    /// # Arguments
    ///
    /// * `snapshot` - Required. The name of the snapshot to delete. Format is `projects/{project}/snapshots/{snap}`.
    pub fn snapshots_delete(&self, snapshot: &str) -> ProjectSnapshotDeleteCall<'a, S> {
        ProjectSnapshotDeleteCall {
            hub: self.hub,
            _snapshot: snapshot.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration details of a snapshot. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `snapshot` - Required. The name of the snapshot to get. Format is `projects/{project}/snapshots/{snap}`.
    pub fn snapshots_get(&self, snapshot: &str) -> ProjectSnapshotGetCall<'a, S> {
        ProjectSnapshotGetCall {
            hub: self.hub,
            _snapshot: snapshot.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn snapshots_get_iam_policy(&self, resource: &str) -> ProjectSnapshotGetIamPolicyCall<'a, S> {
        ProjectSnapshotGetIamPolicyCall {
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
    /// Lists the existing snapshots. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `project` - Required. The name of the project in which to list snapshots. Format is `projects/{project-id}`.
    pub fn snapshots_list(&self, project: &str) -> ProjectSnapshotListCall<'a, S> {
        ProjectSnapshotListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing snapshot. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the snapshot.
    pub fn snapshots_patch(&self, request: UpdateSnapshotRequest, name: &str) -> ProjectSnapshotPatchCall<'a, S> {
        ProjectSnapshotPatchCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn snapshots_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectSnapshotSetIamPolicyCall<'a, S> {
        ProjectSnapshotSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn snapshots_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectSnapshotTestIamPermissionCall<'a, S> {
        ProjectSnapshotTestIamPermissionCall {
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
    /// Acknowledges the messages associated with the `ack_ids` in the `AcknowledgeRequest`. The Pub/Sub system can remove the relevant messages from the subscription. Acknowledging a message whose ack deadline has expired may succeed, but such a message may be redelivered later. Acknowledging a message more than once will not result in an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - Required. The subscription whose message is being acknowledged. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_acknowledge(&self, request: AcknowledgeRequest, subscription: &str) -> ProjectSubscriptionAcknowledgeCall<'a, S> {
        ProjectSubscriptionAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a subscription to a given topic. See the [resource name rules] (https://cloud.google.com/pubsub/docs/admin#resource_names). If the subscription already exists, returns `ALREADY_EXISTS`. If the corresponding topic doesn't exist, returns `NOT_FOUND`. If the name is not provided in the request, the server will assign a random name for this subscription on the same project as the topic, conforming to the [resource name format] (https://cloud.google.com/pubsub/docs/admin#resource_names). The generated name is populated in the returned Subscription object. Note that for REST API requests, you must specify a name in the request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription. It must have the format `"projects/{project}/subscriptions/{subscription}"`. `{subscription}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `"goog"`.
    pub fn subscriptions_create(&self, request: Subscription, name: &str) -> ProjectSubscriptionCreateCall<'a, S> {
        ProjectSubscriptionCreateCall {
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
    /// Deletes an existing subscription. All messages retained in the subscription are immediately dropped. Calls to `Pull` after deletion will return `NOT_FOUND`. After a subscription is deleted, a new one may be created with the same name, but the new one has no association with the old subscription or its topic unless the same topic is specified.
    /// 
    /// # Arguments
    ///
    /// * `subscription` - Required. The subscription to delete. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_delete(&self, subscription: &str) -> ProjectSubscriptionDeleteCall<'a, S> {
        ProjectSubscriptionDeleteCall {
            hub: self.hub,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Detaches a subscription from this topic. All messages retained in the subscription are dropped. Subsequent `Pull` and `StreamingPull` requests will return FAILED_PRECONDITION. If the subscription is a push subscription, pushes to the endpoint will stop.
    /// 
    /// # Arguments
    ///
    /// * `subscription` - Required. The subscription to detach. Format is `projects/{project}/subscriptions/{subscription}`.
    pub fn subscriptions_detach(&self, subscription: &str) -> ProjectSubscriptionDetachCall<'a, S> {
        ProjectSubscriptionDetachCall {
            hub: self.hub,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration details of a subscription.
    /// 
    /// # Arguments
    ///
    /// * `subscription` - Required. The name of the subscription to get. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_get(&self, subscription: &str) -> ProjectSubscriptionGetCall<'a, S> {
        ProjectSubscriptionGetCall {
            hub: self.hub,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn subscriptions_get_iam_policy(&self, resource: &str) -> ProjectSubscriptionGetIamPolicyCall<'a, S> {
        ProjectSubscriptionGetIamPolicyCall {
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
    /// Lists matching subscriptions.
    /// 
    /// # Arguments
    ///
    /// * `project` - Required. The name of the project in which to list subscriptions. Format is `projects/{project-id}`.
    pub fn subscriptions_list(&self, project: &str) -> ProjectSubscriptionListCall<'a, S> {
        ProjectSubscriptionListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the ack deadline for a specific message. This method is useful to indicate that more time is needed to process a message by the subscriber, or to make the message available for redelivery if the processing was interrupted. Note that this does not modify the subscription-level `ackDeadlineSeconds` used for subsequent messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - Required. The name of the subscription. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_modify_ack_deadline(&self, request: ModifyAckDeadlineRequest, subscription: &str) -> ProjectSubscriptionModifyAckDeadlineCall<'a, S> {
        ProjectSubscriptionModifyAckDeadlineCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the `PushConfig` for a specified subscription. This may be used to change a push subscription to a pull one (signified by an empty `PushConfig`) or vice versa, or change the endpoint URL and other attributes of a push subscription. Messages will accumulate for delivery continuously through the call regardless of changes to the `PushConfig`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - Required. The name of the subscription. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_modify_push_config(&self, request: ModifyPushConfigRequest, subscription: &str) -> ProjectSubscriptionModifyPushConfigCall<'a, S> {
        ProjectSubscriptionModifyPushConfigCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subscription. Note that certain properties of a subscription, such as its topic, are not modifiable.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription. It must have the format `"projects/{project}/subscriptions/{subscription}"`. `{subscription}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `"goog"`.
    pub fn subscriptions_patch(&self, request: UpdateSubscriptionRequest, name: &str) -> ProjectSubscriptionPatchCall<'a, S> {
        ProjectSubscriptionPatchCall {
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
    /// Pulls messages from the server.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - Required. The subscription from which messages should be pulled. Format is `projects/{project}/subscriptions/{sub}`.
    pub fn subscriptions_pull(&self, request: PullRequest, subscription: &str) -> ProjectSubscriptionPullCall<'a, S> {
        ProjectSubscriptionPullCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Seeks an existing subscription to a point in time or to a given snapshot, whichever is provided in the request. Snapshots are used in [Seek] (https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot. Note that both the subscription and the snapshot must be on the same topic.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `subscription` - Required. The subscription to affect.
    pub fn subscriptions_seek(&self, request: SeekRequest, subscription: &str) -> ProjectSubscriptionSeekCall<'a, S> {
        ProjectSubscriptionSeekCall {
            hub: self.hub,
            _request: request,
            _subscription: subscription.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn subscriptions_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectSubscriptionSetIamPolicyCall<'a, S> {
        ProjectSubscriptionSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn subscriptions_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectSubscriptionTestIamPermissionCall<'a, S> {
        ProjectSubscriptionTestIamPermissionCall {
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
    /// Lists the names of the snapshots on this topic. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot.
    /// 
    /// # Arguments
    ///
    /// * `topic` - Required. The name of the topic that snapshots are attached to. Format is `projects/{project}/topics/{topic}`.
    pub fn topics_snapshots_list(&self, topic: &str) -> ProjectTopicSnapshotListCall<'a, S> {
        ProjectTopicSnapshotListCall {
            hub: self.hub,
            _topic: topic.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the names of the attached subscriptions on this topic.
    /// 
    /// # Arguments
    ///
    /// * `topic` - Required. The name of the topic that subscriptions are attached to. Format is `projects/{project}/topics/{topic}`.
    pub fn topics_subscriptions_list(&self, topic: &str) -> ProjectTopicSubscriptionListCall<'a, S> {
        ProjectTopicSubscriptionListCall {
            hub: self.hub,
            _topic: topic.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates the given topic with the given name. See the [resource name rules] (https://cloud.google.com/pubsub/docs/admin#resource_names).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the topic. It must have the format `"projects/{project}/topics/{topic}"`. `{topic}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `"goog"`.
    pub fn topics_create(&self, request: Topic, name: &str) -> ProjectTopicCreateCall<'a, S> {
        ProjectTopicCreateCall {
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
    /// Deletes the topic with the given name. Returns `NOT_FOUND` if the topic does not exist. After a topic is deleted, a new topic may be created with the same name; this is an entirely new topic with none of the old configuration or subscriptions. Existing subscriptions to this topic are not deleted, but their `topic` field is set to `_deleted-topic_`.
    /// 
    /// # Arguments
    ///
    /// * `topic` - Required. Name of the topic to delete. Format is `projects/{project}/topics/{topic}`.
    pub fn topics_delete(&self, topic: &str) -> ProjectTopicDeleteCall<'a, S> {
        ProjectTopicDeleteCall {
            hub: self.hub,
            _topic: topic.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the configuration of a topic.
    /// 
    /// # Arguments
    ///
    /// * `topic` - Required. The name of the topic to get. Format is `projects/{project}/topics/{topic}`.
    pub fn topics_get(&self, topic: &str) -> ProjectTopicGetCall<'a, S> {
        ProjectTopicGetCall {
            hub: self.hub,
            _topic: topic.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn topics_get_iam_policy(&self, resource: &str) -> ProjectTopicGetIamPolicyCall<'a, S> {
        ProjectTopicGetIamPolicyCall {
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
    /// Lists matching topics.
    /// 
    /// # Arguments
    ///
    /// * `project` - Required. The name of the project in which to list topics. Format is `projects/{project-id}`.
    pub fn topics_list(&self, project: &str) -> ProjectTopicListCall<'a, S> {
        ProjectTopicListCall {
            hub: self.hub,
            _project: project.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing topic. Note that certain properties of a topic are not modifiable.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the topic. It must have the format `"projects/{project}/topics/{topic}"`. `{topic}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `"goog"`.
    pub fn topics_patch(&self, request: UpdateTopicRequest, name: &str) -> ProjectTopicPatchCall<'a, S> {
        ProjectTopicPatchCall {
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
    /// Adds one or more messages to the topic. Returns `NOT_FOUND` if the topic does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `topic` - Required. The messages in the request will be published on this topic. Format is `projects/{project}/topics/{topic}`.
    pub fn topics_publish(&self, request: PublishRequest, topic: &str) -> ProjectTopicPublishCall<'a, S> {
        ProjectTopicPublishCall {
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
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn topics_set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> ProjectTopicSetIamPolicyCall<'a, S> {
        ProjectTopicSetIamPolicyCall {
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
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn topics_test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> ProjectTopicTestIamPermissionCall<'a, S> {
        ProjectTopicTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



