use super::*;
/// A builder providing access to all methods supported on *about* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.about();
/// # }
/// ```
pub struct AboutMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for AboutMethods<'a, S> {}

impl<'a, S> AboutMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the information about the current user along with Drive API settings
    pub fn get(&self) -> AboutGetCall<'a, S> {
        AboutGetCall {
            hub: self.hub,
            _start_change_id: Default::default(),
            _max_change_id_count: Default::default(),
            _include_subscribed: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *app* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.apps();
/// # }
/// ```
pub struct AppMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for AppMethods<'a, S> {}

impl<'a, S> AppMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific app.
    /// 
    /// # Arguments
    ///
    /// * `appId` - The ID of the app.
    pub fn get(&self, app_id: &str) -> AppGetCall<'a, S> {
        AppGetCall {
            hub: self.hub,
            _app_id: app_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a user's installed apps.
    pub fn list(&self) -> AppListCall<'a, S> {
        AppListCall {
            hub: self.hub,
            _language_code: Default::default(),
            _app_filter_mime_types: Default::default(),
            _app_filter_extensions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *change* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_start_page_token(...)`, `list(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.changes();
/// # }
/// ```
pub struct ChangeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ChangeMethods<'a, S> {}

impl<'a, S> ChangeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated - Use changes.getStartPageToken and changes.list to retrieve recent changes.
    /// 
    /// # Arguments
    ///
    /// * `changeId` - The ID of the change.
    pub fn get(&self, change_id: &str) -> ChangeGetCall<'a, S> {
        ChangeGetCall {
            hub: self.hub,
            _change_id: change_id.to_string(),
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the starting pageToken for listing future changes.
    pub fn get_start_page_token(&self) -> ChangeGetStartPageTokenCall<'a, S> {
        ChangeGetStartPageTokenCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the changes for a user or shared drive.
    pub fn list(&self) -> ChangeListCall<'a, S> {
        ChangeListCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _start_change_id: Default::default(),
            _spaces: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_subscribed: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_deleted: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribe to changes for a user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn watch(&self, request: Channel) -> ChangeWatchCall<'a, S> {
        ChangeWatchCall {
            hub: self.hub,
            _request: request,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _start_change_id: Default::default(),
            _spaces: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_subscribed: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _include_deleted: Default::default(),
            _include_corpus_removals: Default::default(),
            _drive_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
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



/// A builder providing access to all methods supported on *child* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.children();
/// # }
/// ```
pub struct ChildMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ChildMethods<'a, S> {}

impl<'a, S> ChildMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a child from a folder.
    /// 
    /// # Arguments
    ///
    /// * `folderId` - The ID of the folder.
    /// * `childId` - The ID of the child.
    pub fn delete(&self, folder_id: &str, child_id: &str) -> ChildDeleteCall<'a, S> {
        ChildDeleteCall {
            hub: self.hub,
            _folder_id: folder_id.to_string(),
            _child_id: child_id.to_string(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific child reference.
    /// 
    /// # Arguments
    ///
    /// * `folderId` - The ID of the folder.
    /// * `childId` - The ID of the child.
    pub fn get(&self, folder_id: &str, child_id: &str) -> ChildGetCall<'a, S> {
        ChildGetCall {
            hub: self.hub,
            _folder_id: folder_id.to_string(),
            _child_id: child_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a file into a folder.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `folderId` - The ID of the folder.
    pub fn insert(&self, request: ChildReference, folder_id: &str) -> ChildInsertCall<'a, S> {
        ChildInsertCall {
            hub: self.hub,
            _request: request,
            _folder_id: folder_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a folder's children.
    /// 
    /// # Arguments
    ///
    /// * `folderId` - The ID of the folder.
    pub fn list(&self, folder_id: &str) -> ChildListCall<'a, S> {
        ChildListCall {
            hub: self.hub,
            _folder_id: folder_id.to_string(),
            _q: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a comment.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn delete(&self, file_id: &str, comment_id: &str) -> CommentDeleteCall<'a, S> {
        CommentDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a comment by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn get(&self, file_id: &str, comment_id: &str) -> CommentGetCall<'a, S> {
        CommentGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new comment on the given file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn insert(&self, request: Comment, file_id: &str) -> CommentInsertCall<'a, S> {
        CommentInsertCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's comments.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> CommentListCall<'a, S> {
        CommentListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _updated_min: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing comment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn patch(&self, request: Comment, file_id: &str, comment_id: &str) -> CommentPatchCall<'a, S> {
        CommentPatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing comment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn update(&self, request: Comment, file_id: &str, comment_id: &str) -> CommentUpdateCall<'a, S> {
        CommentUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *drive* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `hide(...)`, `insert(...)`, `list(...)`, `unhide(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.drives();
/// # }
/// ```
pub struct DriveMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for DriveMethods<'a, S> {}

impl<'a, S> DriveMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn delete(&self, drive_id: &str) -> DriveDeleteCall<'a, S> {
        DriveDeleteCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _allow_item_deletion: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a shared drive's metadata by ID.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn get(&self, drive_id: &str) -> DriveGetCall<'a, S> {
        DriveGetCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Hides a shared drive from the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn hide(&self, drive_id: &str) -> DriveHideCall<'a, S> {
        DriveHideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.
    pub fn insert(&self, request: Drive, request_id: &str) -> DriveInsertCall<'a, S> {
        DriveInsertCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user's shared drives.
    pub fn list(&self) -> DriveListCall<'a, S> {
        DriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a shared drive to the default view.
    /// 
    /// # Arguments
    ///
    /// * `driveId` - The ID of the shared drive.
    pub fn unhide(&self, drive_id: &str) -> DriveUnhideCall<'a, S> {
        DriveUnhideCall {
            hub: self.hub,
            _drive_id: drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the metadata for a shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `driveId` - The ID of the shared drive.
    pub fn update(&self, request: Drive, drive_id: &str) -> DriveUpdateCall<'a, S> {
        DriveUpdateCall {
            hub: self.hub,
            _request: request,
            _drive_id: drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *file* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `copy(...)`, `delete(...)`, `empty_trash(...)`, `export(...)`, `generate_ids(...)`, `get(...)`, `insert(...)`, `list(...)`, `list_labels(...)`, `modify_labels(...)`, `patch(...)`, `touch(...)`, `trash(...)`, `untrash(...)`, `update(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.files();
/// # }
/// ```
pub struct FileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for FileMethods<'a, S> {}

impl<'a, S> FileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a copy of the specified file. Folders cannot be copied.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file to copy.
    pub fn copy(&self, request: File, file_id: &str) -> FileCopyCall<'a, S> {
        FileCopyCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _visibility: Default::default(),
            _timed_text_track_name: Default::default(),
            _timed_text_language: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _pinned: Default::default(),
            _ocr_language: Default::default(),
            _ocr: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _enforce_single_parent: Default::default(),
            _convert: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file by ID. Skips the trash. The currently authenticated user must own the file or be an organizer on the parent for shared drive files.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file to delete.
    pub fn delete(&self, file_id: &str) -> FileDeleteCall<'a, S> {
        FileDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes all of the user's trashed files.
    pub fn empty_trash(&self) -> FileEmptyTrashCall<'a, S> {
        FileEmptyTrashCall {
            hub: self.hub,
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports a Google Workspace document to the requested MIME type and returns exported byte content. Note that the exported content is limited to 10MB.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `mimeType` - The MIME type of the format requested for this export.
    pub fn export(&self, file_id: &str, mime_type: &str) -> FileExportCall<'a, S> {
        FileExportCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _mime_type: mime_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a set of file IDs which can be provided in insert or copy requests.
    pub fn generate_ids(&self) -> FileGenerateIdCall<'a, S> {
        FileGenerateIdCall {
            hub: self.hub,
            _type_: Default::default(),
            _space: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a file's metadata or content by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID for the file in question.
    pub fn get(&self, file_id: &str) -> FileGetCall<'a, S> {
        FileGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _update_viewed_date: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _revision_id: Default::default(),
            _projection: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert a new file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: File) -> FileInsertCall<'a, S> {
        FileInsertCall {
            hub: self.hub,
            _request: request,
            _visibility: Default::default(),
            _use_content_as_indexable_text: Default::default(),
            _timed_text_track_name: Default::default(),
            _timed_text_language: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _pinned: Default::default(),
            _ocr_language: Default::default(),
            _ocr: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _enforce_single_parent: Default::default(),
            _convert: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the user's files.
    pub fn list(&self) -> FileListCall<'a, S> {
        FileListCall {
            hub: self.hub,
            _team_drive_id: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _spaces: Default::default(),
            _q: Default::default(),
            _projection: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _include_team_drive_items: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _include_items_from_all_drives: Default::default(),
            _drive_id: Default::default(),
            _corpus: Default::default(),
            _corpora: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the labels on a file.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list_labels(&self, file_id: &str) -> FileListLabelCall<'a, S> {
        FileListLabelCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the set of labels on a file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file for which the labels are modified.
    pub fn modify_labels(&self, request: ModifyLabelsRequest, file_id: &str) -> FileModifyLabelCall<'a, S> {
        FileModifyLabelCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might change automatically, such as modifiedDate. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file to update.
    pub fn patch(&self, request: File, file_id: &str) -> FilePatchCall<'a, S> {
        FilePatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_content_as_indexable_text: Default::default(),
            _update_viewed_date: Default::default(),
            _timed_text_track_name: Default::default(),
            _timed_text_language: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _set_modified_date: Default::default(),
            _remove_parents: Default::default(),
            _pinned: Default::default(),
            _ocr_language: Default::default(),
            _ocr: Default::default(),
            _new_revision: Default::default(),
            _modified_date_behavior: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _enforce_single_parent: Default::default(),
            _convert: Default::default(),
            _add_parents: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set the file's updated time to the current server time.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file to update.
    pub fn touch(&self, file_id: &str) -> FileTouchCall<'a, S> {
        FileTouchCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a file to the trash. The currently authenticated user must own the file or be at least a fileOrganizer on the parent for shared drive files. Only the owner may trash a file. The trashed item is excluded from all files.list responses returned for any user who does not own the file. However, all users with access to the file can see the trashed item metadata in an API response. All users with access can copy, download, export, and share the file.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file to trash.
    pub fn trash(&self, file_id: &str) -> FileTrashCall<'a, S> {
        FileTrashCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Restores a file from the trash. The currently authenticated user must own the file or be at least a fileOrganizer on the parent for shared drive files. Only the owner may untrash a file.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file to untrash.
    pub fn untrash(&self, file_id: &str) -> FileUntrashCall<'a, S> {
        FileUntrashCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a file's metadata and/or content. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as modifiedDate. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file to update.
    pub fn update(&self, request: File, file_id: &str) -> FileUpdateCall<'a, S> {
        FileUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_content_as_indexable_text: Default::default(),
            _update_viewed_date: Default::default(),
            _timed_text_track_name: Default::default(),
            _timed_text_language: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _set_modified_date: Default::default(),
            _remove_parents: Default::default(),
            _pinned: Default::default(),
            _ocr_language: Default::default(),
            _ocr: Default::default(),
            _new_revision: Default::default(),
            _modified_date_behavior: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _enforce_single_parent: Default::default(),
            _convert: Default::default(),
            _add_parents: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Subscribes to changes to a file. While you can establish a channel for changes to a file on a shared drive, a change to a shared drive file won't create a notification.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file in question.
    pub fn watch(&self, request: Channel, file_id: &str) -> FileWatchCall<'a, S> {
        FileWatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _update_viewed_date: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _revision_id: Default::default(),
            _projection: Default::default(),
            _include_permissions_for_view: Default::default(),
            _include_labels: Default::default(),
            _acknowledge_abuse: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *parent* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.parents();
/// # }
/// ```
pub struct ParentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ParentMethods<'a, S> {}

impl<'a, S> ParentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a parent from a file.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `parentId` - The ID of the parent.
    pub fn delete(&self, file_id: &str, parent_id: &str) -> ParentDeleteCall<'a, S> {
        ParentDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _parent_id: parent_id.to_string(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific parent reference.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `parentId` - The ID of the parent.
    pub fn get(&self, file_id: &str, parent_id: &str) -> ParentGetCall<'a, S> {
        ParentGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _parent_id: parent_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a parent folder for a file.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn insert(&self, request: ParentReference, file_id: &str) -> ParentInsertCall<'a, S> {
        ParentInsertCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _enforce_single_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's parents.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> ParentListCall<'a, S> {
        ParentListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *permission* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_id_for_email(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.permissions();
/// # }
/// ```
pub struct PermissionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for PermissionMethods<'a, S> {}

impl<'a, S> PermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a permission from a file or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID for the file or shared drive.
    /// * `permissionId` - The ID for the permission.
    pub fn delete(&self, file_id: &str, permission_id: &str) -> PermissionDeleteCall<'a, S> {
        PermissionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a permission by ID.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID for the file or shared drive.
    /// * `permissionId` - The ID for the permission.
    pub fn get(&self, file_id: &str, permission_id: &str) -> PermissionGetCall<'a, S> {
        PermissionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the permission ID for an email address.
    /// 
    /// # Arguments
    ///
    /// * `email` - The email address for which to return a permission ID
    pub fn get_id_for_email(&self, email: &str) -> PermissionGetIdForEmailCall<'a, S> {
        PermissionGetIdForEmailCall {
            hub: self.hub,
            _email: email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a permission for a file or shared drive.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file or shared drive.
    pub fn insert(&self, request: Permission, file_id: &str) -> PermissionInsertCall<'a, S> {
        PermissionInsertCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _send_notification_emails: Default::default(),
            _move_to_new_owners_root: Default::default(),
            _enforce_single_parent: Default::default(),
            _email_message: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's or shared drive's permissions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID for the file or shared drive.
    pub fn list(&self, file_id: &str) -> PermissionListCall<'a, S> {
        PermissionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_permissions_for_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a permission using patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file or shared drive.
    /// * `permissionId` - The ID for the permission.
    pub fn patch(&self, request: Permission, file_id: &str, permission_id: &str) -> PermissionPatchCall<'a, S> {
        PermissionPatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_expiration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a permission.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file or shared drive.
    /// * `permissionId` - The ID for the permission.
    pub fn update(&self, request: Permission, file_id: &str, permission_id: &str) -> PermissionUpdateCall<'a, S> {
        PermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _permission_id: permission_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _transfer_ownership: Default::default(),
            _supports_team_drives: Default::default(),
            _supports_all_drives: Default::default(),
            _remove_expiration: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *property* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.properties();
/// # }
/// ```
pub struct PropertyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for PropertyMethods<'a, S> {}

impl<'a, S> PropertyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a property.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `propertyKey` - The key of the property.
    pub fn delete(&self, file_id: &str, property_key: &str) -> PropertyDeleteCall<'a, S> {
        PropertyDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _property_key: property_key.to_string(),
            _visibility: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a property by its key.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `propertyKey` - The key of the property.
    pub fn get(&self, file_id: &str, property_key: &str) -> PropertyGetCall<'a, S> {
        PropertyGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _property_key: property_key.to_string(),
            _visibility: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a property to a file, or updates it if it already exists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    pub fn insert(&self, request: Property, file_id: &str) -> PropertyInsertCall<'a, S> {
        PropertyInsertCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's properties.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> PropertyListCall<'a, S> {
        PropertyListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `propertyKey` - The key of the property.
    pub fn patch(&self, request: Property, file_id: &str, property_key: &str) -> PropertyPatchCall<'a, S> {
        PropertyPatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _property_key: property_key.to_string(),
            _visibility: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `propertyKey` - The key of the property.
    pub fn update(&self, request: Property, file_id: &str, property_key: &str) -> PropertyUpdateCall<'a, S> {
        PropertyUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _property_key: property_key.to_string(),
            _visibility: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *reply* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.replies();
/// # }
/// ```
pub struct ReplyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for ReplyMethods<'a, S> {}

impl<'a, S> ReplyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a reply.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn delete(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyDeleteCall<'a, S> {
        ReplyDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a reply.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn get(&self, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyGetCall<'a, S> {
        ReplyGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new reply to the given comment.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn insert(&self, request: CommentReply, file_id: &str, comment_id: &str) -> ReplyInsertCall<'a, S> {
        ReplyInsertCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the replies to a comment.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    pub fn list(&self, file_id: &str, comment_id: &str) -> ReplyListCall<'a, S> {
        ReplyListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing reply.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn patch(&self, request: CommentReply, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyPatchCall<'a, S> {
        ReplyPatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing reply.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID of the file.
    /// * `commentId` - The ID of the comment.
    /// * `replyId` - The ID of the reply.
    pub fn update(&self, request: CommentReply, file_id: &str, comment_id: &str, reply_id: &str) -> ReplyUpdateCall<'a, S> {
        ReplyUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _comment_id: comment_id.to_string(),
            _reply_id: reply_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *revision* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.revisions();
/// # }
/// ```
pub struct RevisionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for RevisionMethods<'a, S> {}

impl<'a, S> RevisionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a file version. You can only delete revisions for files with binary content, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn delete(&self, file_id: &str, revision_id: &str) -> RevisionDeleteCall<'a, S> {
        RevisionDeleteCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific revision.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    /// * `revisionId` - The ID of the revision.
    pub fn get(&self, file_id: &str, revision_id: &str) -> RevisionGetCall<'a, S> {
        RevisionGetCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists a file's revisions.
    /// 
    /// # Arguments
    ///
    /// * `fileId` - The ID of the file.
    pub fn list(&self, file_id: &str) -> RevisionListCall<'a, S> {
        RevisionListCall {
            hub: self.hub,
            _file_id: file_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a revision.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file.
    /// * `revisionId` - The ID for the revision.
    pub fn patch(&self, request: Revision, file_id: &str, revision_id: &str) -> RevisionPatchCall<'a, S> {
        RevisionPatchCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a revision.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `fileId` - The ID for the file.
    /// * `revisionId` - The ID for the revision.
    pub fn update(&self, request: Revision, file_id: &str, revision_id: &str) -> RevisionUpdateCall<'a, S> {
        RevisionUpdateCall {
            hub: self.hub,
            _request: request,
            _file_id: file_id.to_string(),
            _revision_id: revision_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *teamdrive* resources.
/// It is not used directly, but through the [`DriveHub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.teamdrives();
/// # }
/// ```
pub struct TeamdriveMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DriveHub<S>,
}

impl<'a, S> client::MethodsBuilder for TeamdriveMethods<'a, S> {}

impl<'a, S> TeamdriveMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.delete instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn delete(&self, team_drive_id: &str) -> TeamdriveDeleteCall<'a, S> {
        TeamdriveDeleteCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.get instead.
    /// 
    /// # Arguments
    ///
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn get(&self, team_drive_id: &str) -> TeamdriveGetCall<'a, S> {
        TeamdriveGetCall {
            hub: self.hub,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.insert instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `requestId` - An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a Team Drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same Team Drive. If the Team Drive already exists a 409 error will be returned.
    pub fn insert(&self, request: TeamDrive, request_id: &str) -> TeamdriveInsertCall<'a, S> {
        TeamdriveInsertCall {
            hub: self.hub,
            _request: request,
            _request_id: request_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.list instead.
    pub fn list(&self) -> TeamdriveListCall<'a, S> {
        TeamdriveListCall {
            hub: self.hub,
            _use_domain_admin_access: Default::default(),
            _q: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated use drives.update instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `teamDriveId` - The ID of the Team Drive
    pub fn update(&self, request: TeamDrive, team_drive_id: &str) -> TeamdriveUpdateCall<'a, S> {
        TeamdriveUpdateCall {
            hub: self.hub,
            _request: request,
            _team_drive_id: team_drive_id.to_string(),
            _use_domain_admin_access: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



