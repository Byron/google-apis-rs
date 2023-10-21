use super::*;
/// A builder providing access to all methods supported on *album* resources.
/// It is not used directly, but through the [`PhotosLibrary`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_photoslibrary1 as photoslibrary1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use photoslibrary1::{PhotosLibrary, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PhotosLibrary::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_enrichment(...)`, `batch_add_media_items(...)`, `batch_remove_media_items(...)`, `create(...)`, `get(...)`, `list(...)`, `patch(...)`, `share(...)` and `unshare(...)`
/// // to build up your call.
/// let rb = hub.albums();
/// # }
/// ```
pub struct AlbumMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PhotosLibrary<S>,
}

impl<'a, S> client::MethodsBuilder for AlbumMethods<'a, S> {}

impl<'a, S> AlbumMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an enrichment at a specified position in a defined album.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `albumId` - Required. Identifier of the album where the enrichment is to be added.
    pub fn add_enrichment(&self, request: AddEnrichmentToAlbumRequest, album_id: &str) -> AlbumAddEnrichmentCall<'a, S> {
        AlbumAddEnrichmentCall {
            hub: self.hub,
            _request: request,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds one or more media items in a user's Google Photos library to an album. The media items and albums must have been created by the developer via the API. Media items are added to the end of the album. If multiple media items are given, they are added in the order specified in this call. Each album can contain up to 20,000 media items. Only media items that are in the user's library can be added to an album. For albums that are shared, the album must either be owned by the user or the user must have joined the album as a collaborator. Partial success is not supported. The entire request will fail if an invalid media item or album is specified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `albumId` - Required. Identifier of the Album that the media items are added to.
    pub fn batch_add_media_items(&self, request: BatchAddMediaItemsToAlbumRequest, album_id: &str) -> AlbumBatchAddMediaItemCall<'a, S> {
        AlbumBatchAddMediaItemCall {
            hub: self.hub,
            _request: request,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes one or more media items from a specified album. The media items and the album must have been created by the developer via the API. For albums that are shared, this action is only supported for media items that were added to the album by this user, or for all media items if the album was created by this user. Partial success is not supported. The entire request will fail and no action will be performed on the album if an invalid media item or album is specified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `albumId` - Required. Identifier of the Album that the media items are to be removed from.
    pub fn batch_remove_media_items(&self, request: BatchRemoveMediaItemsFromAlbumRequest, album_id: &str) -> AlbumBatchRemoveMediaItemCall<'a, S> {
        AlbumBatchRemoveMediaItemCall {
            hub: self.hub,
            _request: request,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an album in a user's Google Photos library.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateAlbumRequest) -> AlbumCreateCall<'a, S> {
        AlbumCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the album based on the specified `albumId`. The `albumId` must be the ID of an album owned by the user or a shared album that the user has joined.
    /// 
    /// # Arguments
    ///
    /// * `albumId` - Required. Identifier of the album to be requested.
    pub fn get(&self, album_id: &str) -> AlbumGetCall<'a, S> {
        AlbumGetCall {
            hub: self.hub,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all albums shown to a user in the Albums tab of the Google Photos app.
    pub fn list(&self) -> AlbumListCall<'a, S> {
        AlbumListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _exclude_non_app_created_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the album with the specified `id`. Only the `id`, `title` and `cover_photo_media_item_id` fields of the album are read. The album must have been created by the developer via the API and must be owned by the user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier for the album. This is a persistent identifier that can be used between sessions to identify this album.
    pub fn patch(&self, request: Album, id: &str) -> AlbumPatchCall<'a, S> {
        AlbumPatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks an album as shared and accessible to other users. This action can only be performed on albums which were created by the developer via the API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `albumId` - Required. Identifier of the album to be shared. This `albumId` must belong to an album created by the developer.
    pub fn share(&self, request: ShareAlbumRequest, album_id: &str) -> AlbumShareCall<'a, S> {
        AlbumShareCall {
            hub: self.hub,
            _request: request,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks a previously shared album as private. This means that the album is no longer shared and all the non-owners will lose access to the album. All non-owner content will be removed from the album. If a non-owner has previously added the album to their library, they will retain all photos in their library. This action can only be performed on albums which were created by the developer via the API.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `albumId` - Required. Identifier of the album to be unshared. This album id must belong to an album created by the developer.
    pub fn unshare(&self, request: UnshareAlbumRequest, album_id: &str) -> AlbumUnshareCall<'a, S> {
        AlbumUnshareCall {
            hub: self.hub,
            _request: request,
            _album_id: album_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mediaItem* resources.
/// It is not used directly, but through the [`PhotosLibrary`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_photoslibrary1 as photoslibrary1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use photoslibrary1::{PhotosLibrary, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PhotosLibrary::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_create(...)`, `batch_get(...)`, `get(...)`, `list(...)`, `patch(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.media_items();
/// # }
/// ```
pub struct MediaItemMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PhotosLibrary<S>,
}

impl<'a, S> client::MethodsBuilder for MediaItemMethods<'a, S> {}

impl<'a, S> MediaItemMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates one or more media items in a user's Google Photos library. This is the second step for creating a media item. For details regarding Step 1, uploading the raw bytes to a Google Server, see Uploading media. This call adds the media item to the library. If an album `id` is specified, the call adds the media item to the album too. Each album can contain up to 20,000 media items. By default, the media item will be added to the end of the library or album. If an album `id` and position are both defined, the media item is added to the album at the specified position. If the call contains multiple media items, they're added at the specified position. If you are creating a media item in a shared album where you are not the owner, you are not allowed to position the media item. Doing so will result in a `BAD REQUEST` error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_create(&self, request: BatchCreateMediaItemsRequest) -> MediaItemBatchCreateCall<'a, S> {
        MediaItemBatchCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of media items for the specified media item identifiers. Items are returned in the same order as the supplied identifiers.
    pub fn batch_get(&self) -> MediaItemBatchGetCall<'a, S> {
        MediaItemBatchGetCall {
            hub: self.hub,
            _media_item_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the media item for the specified media item identifier.
    /// 
    /// # Arguments
    ///
    /// * `mediaItemId` - Required. Identifier of the media item to be requested.
    pub fn get(&self, media_item_id: &str) -> MediaItemGetCall<'a, S> {
        MediaItemGetCall {
            hub: self.hub,
            _media_item_id: media_item_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all media items from a user's Google Photos library.
    pub fn list(&self) -> MediaItemListCall<'a, S> {
        MediaItemListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the media item with the specified `id`. Only the `id` and `description` fields of the media item are read. The media item must have been created by the developer via the API and must be owned by the user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `id` - Identifier for the media item. This is a persistent identifier that can be used between sessions to identify this media item.
    pub fn patch(&self, request: MediaItem, id: &str) -> MediaItemPatchCall<'a, S> {
        MediaItemPatchCall {
            hub: self.hub,
            _request: request,
            _id: id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for media items in a user's Google Photos library. If no filters are set, then all media items in the user's library are returned. If an album is set, all media items in the specified album are returned. If filters are specified, media items that match the filters from the user's library are listed. If you set both the album and the filters, the request results in an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchMediaItemsRequest) -> MediaItemSearchCall<'a, S> {
        MediaItemSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sharedAlbum* resources.
/// It is not used directly, but through the [`PhotosLibrary`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_photoslibrary1 as photoslibrary1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use photoslibrary1::{PhotosLibrary, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PhotosLibrary::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `join(...)`, `leave(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.shared_albums();
/// # }
/// ```
pub struct SharedAlbumMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PhotosLibrary<S>,
}

impl<'a, S> client::MethodsBuilder for SharedAlbumMethods<'a, S> {}

impl<'a, S> SharedAlbumMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the album based on the specified `shareToken`.
    /// 
    /// # Arguments
    ///
    /// * `shareToken` - Required. Share token of the album to be requested.
    pub fn get(&self, share_token: &str) -> SharedAlbumGetCall<'a, S> {
        SharedAlbumGetCall {
            hub: self.hub,
            _share_token: share_token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Joins a shared album on behalf of the Google Photos user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn join(&self, request: JoinSharedAlbumRequest) -> SharedAlbumJoinCall<'a, S> {
        SharedAlbumJoinCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Leaves a previously-joined shared album on behalf of the Google Photos user. The user must not own this album.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn leave(&self, request: LeaveSharedAlbumRequest) -> SharedAlbumLeaveCall<'a, S> {
        SharedAlbumLeaveCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all shared albums available in the Sharing tab of the user's Google Photos app.
    pub fn list(&self) -> SharedAlbumListCall<'a, S> {
        SharedAlbumListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _exclude_non_app_created_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



