use super::*;
/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for ActivityMethods<'a, S> {}

impl<'a, S> ActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get.
    pub fn get(&self, activity_id: &str) -> ActivityGetCall<'a, S> {
        ActivityGetCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user.
    /// * `collection` - The collection of activities to list.
    pub fn list(&self, user_id: &str, collection: &ActivityCollectionEnum) -> ActivityListCall<'a, S> {
        ActivityListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *audience* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.audiences();
/// # }
/// ```
pub struct AudienceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for AudienceMethods<'a, S> {}

impl<'a, S> AudienceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get audiences for. The special value "me" can be used to indicate the authenticated user.
    pub fn list(&self, user_id: &str) -> AudienceListCall<'a, S> {
        AudienceListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *circle* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.circles();
/// # }
/// ```
pub struct CircleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for CircleMethods<'a, S> {}

impl<'a, S> CircleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get circles for. The special value "me" can be used to indicate the authenticated user.
    pub fn list(&self, user_id: &str) -> CircleListCall<'a, S> {
        CircleListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `commentId` - The ID of the comment to get.
    pub fn get(&self, comment_id: &str) -> CommentGetCall<'a, S> {
        CommentGetCall {
            hub: self.hub,
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get comments for.
    pub fn list(&self, activity_id: &str) -> CommentListCall<'a, S> {
        CommentListCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _sort_order: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The ID of the user to create the activity on behalf of.
    /// * `collection` - No description provided.
    pub fn insert(&self, request: Media, user_id: &str, collection: &MediaCollectionEnum) -> MediaInsertCall<'a, S> {
        MediaInsertCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _collection: collection.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *person* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use plusdomains1::{PlusDomains, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlusDomains::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `list_by_activity(...)`
/// // to build up your call.
/// let rb = hub.people();
/// # }
/// ```
pub struct PersonMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlusDomains<S>,
}

impl<'a, S> client::MethodsBuilder for PersonMethods<'a, S> {}

impl<'a, S> PersonMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a person's profile.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user.
    pub fn get(&self, user_id: &str) -> PersonGetCall<'a, S> {
        PersonGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all of the people in the specified collection.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Get the collection of people for the person identified. Use "me" to indicate the authenticated user.
    /// * `collection` - The collection of people to list.
    pub fn list(&self, user_id: &str, collection: &PersonCollectionEnum) -> PersonListCall<'a, S> {
        PersonListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.clone(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    /// 
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get the list of people for.
    /// * `collection` - The collection of people to list.
    pub fn list_by_activity(&self, activity_id: &str, collection: &PersonCollectionEnum) -> PersonListByActivityCall<'a, S> {
        PersonListByActivityCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _collection: collection.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



