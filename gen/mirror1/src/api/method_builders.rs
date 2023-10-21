use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new account for a user
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userToken` - The ID for the user.
    /// * `accountType` - Account type to be passed to Android Account Manager.
    /// * `accountName` - The name of the account to be passed to the Android Account Manager.
    pub fn insert(&self, request: Account, user_token: &str, account_type: &str, account_name: &str) -> AccountInsertCall<'a, S> {
        AccountInsertCall {
            hub: self.hub,
            _request: request,
            _user_token: user_token.to_string(),
            _account_type: account_type.to_string(),
            _account_name: account_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *contact* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.contacts();
/// # }
/// ```
pub struct ContactMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for ContactMethods<'a, S> {}

impl<'a, S> ContactMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a contact.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the contact.
    pub fn delete(&self, id: &str) -> ContactDeleteCall<'a, S> {
        ContactDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single contact by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the contact.
    pub fn get(&self, id: &str) -> ContactGetCall<'a, S> {
        ContactGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new contact.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Contact) -> ContactInsertCall<'a, S> {
        ContactInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of contacts for the authenticated user.
    pub fn list(&self) -> ContactListCall<'a, S> {
        ContactListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a contact in place. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The ID of the contact.
    pub fn patch(&self, request: Contact, id: &str) -> ContactPatchCall<'a, S> {
        ContactPatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a contact in place.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The ID of the contact.
    pub fn update(&self, request: Contact, id: &str) -> ContactUpdateCall<'a, S> {
        ContactUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single location by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the location or latest for the last known location.
    pub fn get(&self, id: &str) -> LocationGetCall<'a, S> {
        LocationGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of locations for the user.
    pub fn list(&self) -> LocationListCall<'a, S> {
        LocationListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *setting* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.settings();
/// # }
/// ```
pub struct SettingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for SettingMethods<'a, S> {}

impl<'a, S> SettingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single setting by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the setting. The following IDs are valid: 
    ///          - locale - The key to the user’s language/locale (BCP 47 identifier) that Glassware should use to render localized content. 
    ///          - timezone - The key to the user’s current time zone region as defined in the tz database. Example: America/Los_Angeles.
    pub fn get(&self, id: &str) -> SettingGetCall<'a, S> {
        SettingGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subscription* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.subscriptions();
/// # }
/// ```
pub struct SubscriptionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for SubscriptionMethods<'a, S> {}

impl<'a, S> SubscriptionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscription.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the subscription.
    pub fn delete(&self, id: &str) -> SubscriptionDeleteCall<'a, S> {
        SubscriptionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
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
    pub fn insert(&self, request: Subscription) -> SubscriptionInsertCall<'a, S> {
        SubscriptionInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of subscriptions for the authenticated user and service.
    pub fn list(&self) -> SubscriptionListCall<'a, S> {
        SubscriptionListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subscription in place.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The ID of the subscription.
    pub fn update(&self, request: Subscription, id: &str) -> SubscriptionUpdateCall<'a, S> {
        SubscriptionUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *timeline* resources.
/// It is not used directly, but through the [`Mirror`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mirror1 as mirror1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mirror1::{Mirror, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Mirror::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `attachments_delete(...)`, `attachments_get(...)`, `attachments_insert(...)`, `attachments_list(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.timeline();
/// # }
/// ```
pub struct TimelineMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Mirror<S>,
}

impl<'a, S> client::MethodsBuilder for TimelineMethods<'a, S> {}

impl<'a, S> TimelineMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an attachment from a timeline item.
    /// 
    /// # Arguments
    ///
    /// * `itemId` - The ID of the timeline item the attachment belongs to.
    /// * `attachmentId` - The ID of the attachment.
    pub fn attachments_delete(&self, item_id: &str, attachment_id: &str) -> TimelineAttachmentDeleteCall<'a, S> {
        TimelineAttachmentDeleteCall {
            hub: self.hub,
            _item_id: item_id.to_string(),
            _attachment_id: attachment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an attachment on a timeline item by item ID and attachment ID.
    /// 
    /// # Arguments
    ///
    /// * `itemId` - The ID of the timeline item the attachment belongs to.
    /// * `attachmentId` - The ID of the attachment.
    pub fn attachments_get(&self, item_id: &str, attachment_id: &str) -> TimelineAttachmentGetCall<'a, S> {
        TimelineAttachmentGetCall {
            hub: self.hub,
            _item_id: item_id.to_string(),
            _attachment_id: attachment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a new attachment to a timeline item.
    /// 
    /// # Arguments
    ///
    /// * `itemId` - The ID of the timeline item the attachment belongs to.
    pub fn attachments_insert(&self, item_id: &str) -> TimelineAttachmentInsertCall<'a, S> {
        TimelineAttachmentInsertCall {
            hub: self.hub,
            _item_id: item_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of attachments for a timeline item.
    /// 
    /// # Arguments
    ///
    /// * `itemId` - The ID of the timeline item whose attachments should be listed.
    pub fn attachments_list(&self, item_id: &str) -> TimelineAttachmentListCall<'a, S> {
        TimelineAttachmentListCall {
            hub: self.hub,
            _item_id: item_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a timeline item.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the timeline item.
    pub fn delete(&self, id: &str) -> TimelineDeleteCall<'a, S> {
        TimelineDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single timeline item by ID.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the timeline item.
    pub fn get(&self, id: &str) -> TimelineGetCall<'a, S> {
        TimelineGetCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new item into the timeline.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: TimelineItem) -> TimelineInsertCall<'a, S> {
        TimelineInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of timeline items for the authenticated user.
    pub fn list(&self) -> TimelineListCall<'a, S> {
        TimelineListCall {
            hub: self.hub,
            _source_item_id: Default::default(),
            _pinned_only: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_deleted: Default::default(),
            _bundle_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a timeline item in place. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The ID of the timeline item.
    pub fn patch(&self, request: TimelineItem, id: &str) -> TimelinePatchCall<'a, S> {
        TimelinePatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a timeline item in place.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - The ID of the timeline item.
    pub fn update(&self, request: TimelineItem, id: &str) -> TimelineUpdateCall<'a, S> {
        TimelineUpdateCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



