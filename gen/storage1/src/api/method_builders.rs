use super::*;
/// A builder providing access to all methods supported on *bucketAccessControl* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.bucket_access_controls();
/// # }
/// ```
pub struct BucketAccessControlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for BucketAccessControlMethods<'a, S> {}

impl<'a, S> BucketAccessControlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> BucketAccessControlDeleteCall<'a, S> {
        BucketAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> BucketAccessControlGetCall<'a, S> {
        BucketAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: BucketAccessControl, bucket: &str) -> BucketAccessControlInsertCall<'a, S> {
        BucketAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> BucketAccessControlListCall<'a, S> {
        BucketAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlPatchCall<'a, S> {
        BucketAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlUpdateCall<'a, S> {
        BucketAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bucket* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `lock_retention_policy(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.buckets();
/// # }
/// ```
pub struct BucketMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for BucketMethods<'a, S> {}

impl<'a, S> BucketMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes an empty bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn delete(&self, bucket: &str) -> BucketDeleteCall<'a, S> {
        BucketDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get(&self, bucket: &str) -> BucketGetCall<'a, S> {
        BucketGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get_iam_policy(&self, bucket: &str) -> BucketGetIamPolicyCall<'a, S> {
        BucketGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - A valid API project identifier.
    pub fn insert(&self, request: Bucket, project: &str) -> BucketInsertCall<'a, S> {
        BucketInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of buckets for a given project.
    /// 
    /// # Arguments
    ///
    /// * `project` - A valid API project identifier.
    pub fn list(&self, project: &str) -> BucketListCall<'a, S> {
        BucketListCall {
            hub: self.hub,
            _project: project.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Locks retention policy on a bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `ifMetagenerationMatch` - Makes the operation conditional on whether bucket's current metageneration matches the given value.
    pub fn lock_retention_policy(&self, bucket: &str, if_metageneration_match: i64) -> BucketLockRetentionPolicyCall<'a, S> {
        BucketLockRetentionPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _if_metageneration_match: if_metageneration_match,
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn patch(&self, request: Bucket, bucket: &str) -> BucketPatchCall<'a, S> {
        BucketPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str) -> BucketSetIamPolicyCall<'a, S> {
        BucketSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests a set of permissions on the given bucket to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, permissions: &Vec<String>) -> BucketTestIamPermissionCall<'a, S> {
        BucketTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn update(&self, request: Bucket, bucket: &str) -> BucketUpdateCall<'a, S> {
        BucketUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelMethods<'a, S> {}

impl<'a, S> ChannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a, S> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *defaultObjectAccessControl* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.default_object_access_controls();
/// # }
/// ```
pub struct DefaultObjectAccessControlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for DefaultObjectAccessControlMethods<'a, S> {}

impl<'a, S> DefaultObjectAccessControlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlDeleteCall<'a, S> {
        DefaultObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlGetCall<'a, S> {
        DefaultObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str) -> DefaultObjectAccessControlInsertCall<'a, S> {
        DefaultObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves default object ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> DefaultObjectAccessControlListCall<'a, S> {
        DefaultObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlPatchCall<'a, S> {
        DefaultObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlUpdateCall<'a, S> {
        DefaultObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.notifications();
/// # }
/// ```
pub struct NotificationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for NotificationMethods<'a, S> {}

impl<'a, S> NotificationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a notification subscription.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - ID of the notification to delete.
    pub fn delete(&self, bucket: &str, notification: &str) -> NotificationDeleteCall<'a, S> {
        NotificationDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// View a notification configuration.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - Notification ID
    pub fn get(&self, bucket: &str, notification: &str) -> NotificationGetCall<'a, S> {
        NotificationGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a notification subscription for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - The parent bucket of the notification.
    pub fn insert(&self, request: Notification, bucket: &str) -> NotificationInsertCall<'a, S> {
        NotificationInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of notification subscriptions for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a Google Cloud Storage bucket.
    pub fn list(&self, bucket: &str) -> NotificationListCall<'a, S> {
        NotificationListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *objectAccessControl* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.object_access_controls();
/// # }
/// ```
pub struct ObjectAccessControlMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for ObjectAccessControlMethods<'a, S> {}

impl<'a, S> ObjectAccessControlMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlDeleteCall<'a, S> {
        ObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlGetCall<'a, S> {
        ObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str, object: &str) -> ObjectAccessControlInsertCall<'a, S> {
        ObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn list(&self, bucket: &str, object: &str) -> ObjectAccessControlListCall<'a, S> {
        ObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlPatchCall<'a, S> {
        ObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlUpdateCall<'a, S> {
        ObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *object* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `compose(...)`, `copy(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `rewrite(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)`, `update(...)` and `watch_all(...)`
/// // to build up your call.
/// let rb = hub.objects();
/// # }
/// ```
pub struct ObjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for ObjectMethods<'a, S> {}

impl<'a, S> ObjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Concatenates a list of existing objects into a new object in the same bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `destinationBucket` - Name of the bucket containing the source objects. The destination object is stored in this bucket.
    /// * `destinationObject` - Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn compose(&self, request: ComposeRequest, destination_bucket: &str, destination_object: &str) -> ObjectComposeCall<'a, S> {
        ObjectComposeCall {
            hub: self.hub,
            _request: request,
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _kms_key_name: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.
    pub fn copy(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectCopyCall<'a, S> {
        ObjectCopyCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _source_generation: Default::default(),
            _projection: Default::default(),
            _if_source_metageneration_not_match: Default::default(),
            _if_source_metageneration_match: Default::default(),
            _if_source_generation_not_match: Default::default(),
            _if_source_generation_match: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _destination_kms_key_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn delete(&self, bucket: &str, object: &str) -> ObjectDeleteCall<'a, S> {
        ObjectDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an object or its metadata.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get(&self, bucket: &str, object: &str) -> ObjectGetCall<'a, S> {
        ObjectGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get_iam_policy(&self, bucket: &str, object: &str) -> ObjectGetIamPolicyCall<'a, S> {
        ObjectGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stores a new object and metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    pub fn insert(&self, request: Object, bucket: &str) -> ObjectInsertCall<'a, S> {
        ObjectInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _name: Default::default(),
            _kms_key_name: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _content_encoding: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of objects matching the criteria.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn list(&self, bucket: &str) -> ObjectListCall<'a, S> {
        ObjectListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
            _start_offset: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_trailing_delimiter: Default::default(),
            _end_offset: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an object's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn patch(&self, request: Object, bucket: &str, object: &str) -> ObjectPatchCall<'a, S> {
        ObjectPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rewrites a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn rewrite(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectRewriteCall<'a, S> {
        ObjectRewriteCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _source_generation: Default::default(),
            _rewrite_token: Default::default(),
            _projection: Default::default(),
            _max_bytes_rewritten_per_call: Default::default(),
            _if_source_metageneration_not_match: Default::default(),
            _if_source_metageneration_match: Default::default(),
            _if_source_generation_not_match: Default::default(),
            _if_source_generation_match: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _destination_kms_key_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str, object: &str) -> ObjectSetIamPolicyCall<'a, S> {
        ObjectSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests a set of permissions on the given object to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, object: &str, permissions: &Vec<String>) -> ObjectTestIamPermissionCall<'a, S> {
        ObjectTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an object's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn update(&self, request: Object, bucket: &str, object: &str) -> ObjectUpdateCall<'a, S> {
        ObjectUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes on all objects in a bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn watch_all(&self, request: Channel, bucket: &str) -> ObjectWatchAllCall<'a, S> {
        ObjectWatchAllCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
            _start_offset: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_trailing_delimiter: Default::default(),
            _end_offset: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Storage`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `hmac_keys_create(...)`, `hmac_keys_delete(...)`, `hmac_keys_get(...)`, `hmac_keys_list(...)`, `hmac_keys_update(...)` and `service_account_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Storage<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new HMAC key for the specified service account.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the service account.
    /// * `serviceAccountEmail` - Email address of the service account.
    pub fn hmac_keys_create(&self, project_id: &str, service_account_email: &str) -> ProjectHmacKeyCreateCall<'a, S> {
        ProjectHmacKeyCreateCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _service_account_email: service_account_email.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an HMAC key.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the requested key
    /// * `accessId` - Name of the HMAC key to be deleted.
    pub fn hmac_keys_delete(&self, project_id: &str, access_id: &str) -> ProjectHmacKeyDeleteCall<'a, S> {
        ProjectHmacKeyDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an HMAC key's metadata
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the service account of the requested key.
    /// * `accessId` - Name of the HMAC key.
    pub fn hmac_keys_get(&self, project_id: &str, access_id: &str) -> ProjectHmacKeyGetCall<'a, S> {
        ProjectHmacKeyGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of HMAC keys matching the criteria.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Name of the project in which to look for HMAC keys.
    pub fn hmac_keys_list(&self, project_id: &str) -> ProjectHmacKeyListCall<'a, S> {
        ProjectHmacKeyListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _user_project: Default::default(),
            _show_deleted_keys: Default::default(),
            _service_account_email: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID owning the service account of the updated key.
    /// * `accessId` - Name of the HMAC key being updated.
    pub fn hmac_keys_update(&self, request: HmacKeyMetadata, project_id: &str, access_id: &str) -> ProjectHmacKeyUpdateCall<'a, S> {
        ProjectHmacKeyUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the email address of this project's Google Cloud Storage service account.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID
    pub fn service_account_get(&self, project_id: &str) -> ProjectServiceAccountGetCall<'a, S> {
        ProjectServiceAccountGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



