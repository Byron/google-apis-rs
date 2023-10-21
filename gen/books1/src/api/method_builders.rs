use super::*;
/// A builder providing access to all methods supported on *bookshelf* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `volumes_list(...)`
/// // to build up your call.
/// let rb = hub.bookshelves();
/// # }
/// ```
pub struct BookshelfMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for BookshelfMethods<'a, S> {}

impl<'a, S> BookshelfMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves volumes in a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelf volumes.
    /// * `shelf` - ID of bookshelf to retrieve volumes.
    pub fn volumes_list(&self, user_id: &str, shelf: &str) -> BookshelfVolumeListCall<'a, S> {
        BookshelfVolumeListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn get(&self, user_id: &str, shelf: &str) -> BookshelfGetCall<'a, S> {
        BookshelfGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of public bookshelves for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - ID of user for whom to retrieve bookshelves.
    pub fn list(&self, user_id: &str) -> BookshelfListCall<'a, S> {
        BookshelfListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *cloudloading* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add_book(...)`, `delete_book(...)` and `update_book(...)`
/// // to build up your call.
/// let rb = hub.cloudloading();
/// # }
/// ```
pub struct CloudloadingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for CloudloadingMethods<'a, S> {}

impl<'a, S> CloudloadingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a user-upload volume and triggers processing.
    pub fn add_book(&self) -> CloudloadingAddBookCall<'a, S> {
        CloudloadingAddBookCall {
            hub: self.hub,
            _upload_client_token: Default::default(),
            _name: Default::default(),
            _mime_type: Default::default(),
            _drive_document_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Remove the book and its contents
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The id of the book to be removed.
    pub fn delete_book(&self, volume_id: &str) -> CloudloadingDeleteBookCall<'a, S> {
        CloudloadingDeleteBookCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user-upload volume.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_book(&self, request: BooksCloudloadingResource) -> CloudloadingUpdateBookCall<'a, S> {
        CloudloadingUpdateBookCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dictionary* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_offline_metadata(...)`
/// // to build up your call.
/// let rb = hub.dictionary();
/// # }
/// ```
pub struct DictionaryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for DictionaryMethods<'a, S> {}

impl<'a, S> DictionaryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of offline dictionary metadata available
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the data.
    pub fn list_offline_metadata(&self, cpksver: &str) -> DictionaryListOfflineMetadataCall<'a, S> {
        DictionaryListOfflineMetadataCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *familysharing* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_family_info(...)`, `share(...)` and `unshare(...)`
/// // to build up your call.
/// let rb = hub.familysharing();
/// # }
/// ```
pub struct FamilysharingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for FamilysharingMethods<'a, S> {}

impl<'a, S> FamilysharingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information regarding the family that the user is part of.
    pub fn get_family_info(&self) -> FamilysharingGetFamilyInfoCall<'a, S> {
        FamilysharingGetFamilyInfoCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates sharing of the content with the user's family. Empty response indicates success.
    pub fn share(&self) -> FamilysharingShareCall<'a, S> {
        FamilysharingShareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Initiates revoking content that has already been shared with the user's family. Empty response indicates success.
    pub fn unshare(&self) -> FamilysharingUnshareCall<'a, S> {
        FamilysharingUnshareCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _source: Default::default(),
            _doc_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *layer* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotation_data_get(...)`, `annotation_data_list(...)`, `get(...)`, `list(...)`, `volume_annotations_get(...)` and `volume_annotations_list(...)`
/// // to build up your call.
/// let rb = hub.layers();
/// # }
/// ```
pub struct LayerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for LayerMethods<'a, S> {}

impl<'a, S> LayerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationDataId` - The ID of the annotation data to retrieve.
    /// * `contentVersion` - The content version for the volume you are trying to retrieve.
    pub fn annotation_data_get(&self, volume_id: &str, layer_id: &str, annotation_data_id: &str, content_version: &str) -> LayerAnnotationDataGetCall<'a, S> {
        LayerAnnotationDataGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_data_id: annotation_data_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _allow_web_definitions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the annotation data for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotation data for.
    /// * `layerId` - The ID for the layer to get the annotation data.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn annotation_data_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerAnnotationDataListCall<'a, S> {
        LayerAnnotationDataListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _w: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _scale: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _h: Default::default(),
            _annotation_data_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotation.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `annotationId` - The ID of the volume annotation to retrieve.
    pub fn volume_annotations_get(&self, volume_id: &str, layer_id: &str, annotation_id: &str) -> LayerVolumeAnnotationGetCall<'a, S> {
        LayerVolumeAnnotationGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the volume annotations for a volume and layer.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve annotations for.
    /// * `layerId` - The ID for the layer to get the annotations.
    /// * `contentVersion` - The content version for the requested volume.
    pub fn volume_annotations_list(&self, volume_id: &str, layer_id: &str, content_version: &str) -> LayerVolumeAnnotationListCall<'a, S> {
        LayerVolumeAnnotationListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _layer_id: layer_id.to_string(),
            _content_version: content_version.to_string(),
            _volume_annotations_version: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _start_position: Default::default(),
            _start_offset: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _end_position: Default::default(),
            _end_offset: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the layer summary for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    /// * `summaryId` - The ID for the layer to get the summary for.
    pub fn get(&self, volume_id: &str, summary_id: &str) -> LayerGetCall<'a, S> {
        LayerGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _summary_id: summary_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the layer summaries for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - The volume to retrieve layers for.
    pub fn list(&self, volume_id: &str) -> LayerListCall<'a, S> {
        LayerListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *myconfig* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_user_settings(...)`, `release_download_access(...)`, `request_access(...)`, `sync_volume_licenses(...)` and `update_user_settings(...)`
/// // to build up your call.
/// let rb = hub.myconfig();
/// # }
/// ```
pub struct MyconfigMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for MyconfigMethods<'a, S> {}

impl<'a, S> MyconfigMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current settings for the user.
    pub fn get_user_settings(&self) -> MyconfigGetUserSettingCall<'a, S> {
        MyconfigGetUserSettingCall {
            hub: self.hub,
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Release downloaded content access restriction.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to release the restriction.
    /// * `volumeIds` - The volume(s) to release restrictions for.
    pub fn release_download_access(&self, cpksver: &str, volume_ids: &Vec<String>) -> MyconfigReleaseDownloadAccesCall<'a, S> {
        MyconfigReleaseDownloadAccesCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _volume_ids: volume_ids.clone(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request concurrent and download access restrictions.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to request the restrictions.
    /// * `nonce` - The client nonce value.
    /// * `source` - String to identify the originator of this request.
    /// * `volumeId` - The volume to request concurrent/download restrictions for.
    pub fn request_access(&self, cpksver: &str, nonce: &str, source: &str, volume_id: &str) -> MyconfigRequestAccesCall<'a, S> {
        MyconfigRequestAccesCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _nonce: nonce.to_string(),
            _source: source.to_string(),
            _volume_id: volume_id.to_string(),
            _locale: Default::default(),
            _license_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Request downloaded content access for specified volumes on the My eBooks shelf.
    /// 
    /// # Arguments
    ///
    /// * `cpksver` - The device/version ID from which to release the restriction.
    /// * `nonce` - The client nonce value.
    /// * `source` - String to identify the originator of this request.
    pub fn sync_volume_licenses(&self, cpksver: &str, nonce: &str, source: &str) -> MyconfigSyncVolumeLicenseCall<'a, S> {
        MyconfigSyncVolumeLicenseCall {
            hub: self.hub,
            _cpksver: cpksver.to_string(),
            _nonce: nonce.to_string(),
            _source: source.to_string(),
            _volume_ids: Default::default(),
            _show_preorders: Default::default(),
            _locale: Default::default(),
            _include_non_comics_series: Default::default(),
            _features: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_user_settings(&self, request: Usersettings) -> MyconfigUpdateUserSettingCall<'a, S> {
        MyconfigUpdateUserSettingCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mylibrary* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotations_delete(...)`, `annotations_insert(...)`, `annotations_list(...)`, `annotations_summary(...)`, `annotations_update(...)`, `bookshelves_add_volume(...)`, `bookshelves_clear_volumes(...)`, `bookshelves_get(...)`, `bookshelves_list(...)`, `bookshelves_move_volume(...)`, `bookshelves_remove_volume(...)`, `bookshelves_volumes_list(...)`, `readingpositions_get(...)` and `readingpositions_set_position(...)`
/// // to build up your call.
/// let rb = hub.mylibrary();
/// # }
/// ```
pub struct MylibraryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for MylibraryMethods<'a, S> {}

impl<'a, S> MylibraryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an annotation.
    /// 
    /// # Arguments
    ///
    /// * `annotationId` - The ID for the annotation to delete.
    pub fn annotations_delete(&self, annotation_id: &str) -> MylibraryAnnotationDeleteCall<'a, S> {
        MylibraryAnnotationDeleteCall {
            hub: self.hub,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotations_insert(&self, request: Annotation) -> MylibraryAnnotationInsertCall<'a, S> {
        MylibraryAnnotationInsertCall {
            hub: self.hub,
            _request: request,
            _source: Default::default(),
            _show_only_summary_in_response: Default::default(),
            _country: Default::default(),
            _annotation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of annotations, possibly filtered.
    pub fn annotations_list(&self) -> MylibraryAnnotationListCall<'a, S> {
        MylibraryAnnotationListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _updated_min: Default::default(),
            _updated_max: Default::default(),
            _source: Default::default(),
            _show_deleted: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _layer_ids: Default::default(),
            _layer_id: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the summary of specified layers.
    /// 
    /// # Arguments
    ///
    /// * `layerIds` - Array of layer IDs to get the summary for.
    /// * `volumeId` - Volume id to get the summary for.
    pub fn annotations_summary(&self, layer_ids: &Vec<String>, volume_id: &str) -> MylibraryAnnotationSummaryCall<'a, S> {
        MylibraryAnnotationSummaryCall {
            hub: self.hub,
            _layer_ids: layer_ids.clone(),
            _volume_id: volume_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing annotation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `annotationId` - The ID for the annotation to update.
    pub fn annotations_update(&self, request: Annotation, annotation_id: &str) -> MylibraryAnnotationUpdateCall<'a, S> {
        MylibraryAnnotationUpdateCall {
            hub: self.hub,
            _request: request,
            _annotation_id: annotation_id.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for volumes on a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - The bookshelf ID or name retrieve volumes for.
    pub fn bookshelves_volumes_list(&self, shelf: &str) -> MylibraryBookshelfVolumeListCall<'a, S> {
        MylibraryBookshelfVolumeListCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _q: Default::default(),
            _projection: Default::default(),
            _max_results: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a volume to a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to which to add a volume.
    /// * `volumeId` - ID of volume to add.
    pub fn bookshelves_add_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelfAddVolumeCall<'a, S> {
        MylibraryBookshelfAddVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears all volumes from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    pub fn bookshelves_clear_volumes(&self, shelf: &str) -> MylibraryBookshelfClearVolumeCall<'a, S> {
        MylibraryBookshelfClearVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves metadata for a specific bookshelf belonging to the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf to retrieve.
    pub fn bookshelves_get(&self, shelf: &str) -> MylibraryBookshelfGetCall<'a, S> {
        MylibraryBookshelfGetCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of bookshelves belonging to the authenticated user.
    pub fn bookshelves_list(&self) -> MylibraryBookshelfListCall<'a, S> {
        MylibraryBookshelfListCall {
            hub: self.hub,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a volume within a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf with the volume.
    /// * `volumeId` - ID of volume to move.
    /// * `volumePosition` - Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)
    pub fn bookshelves_move_volume(&self, shelf: &str, volume_id: &str, volume_position: i32) -> MylibraryBookshelfMoveVolumeCall<'a, S> {
        MylibraryBookshelfMoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _volume_position: volume_position,
            _source: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a volume from a bookshelf.
    /// 
    /// # Arguments
    ///
    /// * `shelf` - ID of bookshelf from which to remove a volume.
    /// * `volumeId` - ID of volume to remove.
    pub fn bookshelves_remove_volume(&self, shelf: &str, volume_id: &str) -> MylibraryBookshelfRemoveVolumeCall<'a, S> {
        MylibraryBookshelfRemoveVolumeCall {
            hub: self.hub,
            _shelf: shelf.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _reason: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to retrieve a reading position.
    pub fn readingpositions_get(&self, volume_id: &str) -> MylibraryReadingpositionGetCall<'a, S> {
        MylibraryReadingpositionGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _content_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets my reading position information for a volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume for which to update the reading position.
    /// * `position` - Position string for the new volume reading position.
    /// * `timestamp` - RFC 3339 UTC format timestamp associated with this reading position.
    pub fn readingpositions_set_position(&self, volume_id: &str, position: &str, timestamp: &str) -> MylibraryReadingpositionSetPositionCall<'a, S> {
        MylibraryReadingpositionSetPositionCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _position: position.to_string(),
            _timestamp: timestamp.to_string(),
            _source: Default::default(),
            _device_cookie: Default::default(),
            _content_version: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.notification();
/// # }
/// ```
pub struct NotificationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for NotificationMethods<'a, S> {}

impl<'a, S> NotificationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns notification details for a given notification id.
    /// 
    /// # Arguments
    ///
    /// * `notification_id` - String to identify the notification.
    pub fn get(&self, notification_id: &str) -> NotificationGetCall<'a, S> {
        NotificationGetCall {
            hub: self.hub,
            _notification_id: notification_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *onboarding* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_categories(...)` and `list_category_volumes(...)`
/// // to build up your call.
/// let rb = hub.onboarding();
/// # }
/// ```
pub struct OnboardingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for OnboardingMethods<'a, S> {}

impl<'a, S> OnboardingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List categories for onboarding experience.
    pub fn list_categories(&self) -> OnboardingListCategoryCall<'a, S> {
        OnboardingListCategoryCall {
            hub: self.hub,
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List available volumes under categories for onboarding experience.
    pub fn list_category_volumes(&self) -> OnboardingListCategoryVolumeCall<'a, S> {
        OnboardingListCategoryVolumeCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *personalizedstream* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.personalizedstream();
/// # }
/// ```
pub struct PersonalizedstreamMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for PersonalizedstreamMethods<'a, S> {}

impl<'a, S> PersonalizedstreamMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a stream of personalized book clusters
    pub fn get(&self) -> PersonalizedstreamGetCall<'a, S> {
        PersonalizedstreamGetCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *promooffer* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `accept(...)`, `dismiss(...)` and `get(...)`
/// // to build up your call.
/// let rb = hub.promooffer();
/// # }
/// ```
pub struct PromoofferMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for PromoofferMethods<'a, S> {}

impl<'a, S> PromoofferMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts the promo offer.
    pub fn accept(&self) -> PromoofferAcceptCall<'a, S> {
        PromoofferAcceptCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks the promo offer as dismissed.
    pub fn dismiss(&self) -> PromoofferDismisCall<'a, S> {
        PromoofferDismisCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _offer_id: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of promo offers available to the user
    pub fn get(&self) -> PromoofferGetCall<'a, S> {
        PromoofferGetCall {
            hub: self.hub,
            _serial: Default::default(),
            _product: Default::default(),
            _model: Default::default(),
            _manufacturer: Default::default(),
            _device: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *series* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `membership_get(...)`
/// // to build up your call.
/// let rb = hub.series();
/// # }
/// ```
pub struct SeriesMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for SeriesMethods<'a, S> {}

impl<'a, S> SeriesMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series membership data given the series id.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn membership_get(&self, series_id: &str) -> SeriesMembershipGetCall<'a, S> {
        SeriesMembershipGetCall {
            hub: self.hub,
            _series_id: series_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns Series metadata for the given series ids.
    /// 
    /// # Arguments
    ///
    /// * `series_id` - String that identifies the series
    pub fn get(&self, series_id: &Vec<String>) -> SeriesGetCall<'a, S> {
        SeriesGetCall {
            hub: self.hub,
            _series_id: series_id.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *volume* resources.
/// It is not used directly, but through the [`Books`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associated_list(...)`, `get(...)`, `list(...)`, `mybooks_list(...)`, `recommended_list(...)`, `recommended_rate(...)` and `useruploaded_list(...)`
/// // to build up your call.
/// let rb = hub.volumes();
/// # }
/// ```
pub struct VolumeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Books<S>,
}

impl<'a, S> client::MethodsBuilder for VolumeMethods<'a, S> {}

impl<'a, S> VolumeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of associated books.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of the source volume.
    pub fn associated_list(&self, volume_id: &str) -> VolumeAssociatedListCall<'a, S> {
        VolumeAssociatedListCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _association: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books in My Library.
    pub fn mybooks_list(&self) -> VolumeMybookListCall<'a, S> {
        VolumeMybookListCall {
            hub: self.hub,
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _country: Default::default(),
            _acquire_method: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of recommended books for the current user.
    pub fn recommended_list(&self) -> VolumeRecommendedListCall<'a, S> {
        VolumeRecommendedListCall {
            hub: self.hub,
            _source: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rate a recommended book for the current user.
    /// 
    /// # Arguments
    ///
    /// * `rating` - Rating to be given to the volume.
    /// * `volumeId` - ID of the source volume.
    pub fn recommended_rate(&self, rating: &str, volume_id: &str) -> VolumeRecommendedRateCall<'a, S> {
        VolumeRecommendedRateCall {
            hub: self.hub,
            _rating: rating.to_string(),
            _volume_id: volume_id.to_string(),
            _source: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return a list of books uploaded by the current user.
    pub fn useruploaded_list(&self) -> VolumeUseruploadedListCall<'a, S> {
        VolumeUseruploadedListCall {
            hub: self.hub,
            _volume_id: Default::default(),
            _start_index: Default::default(),
            _source: Default::default(),
            _processing_state: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets volume information for a single volume.
    /// 
    /// # Arguments
    ///
    /// * `volumeId` - ID of volume to retrieve.
    pub fn get(&self, volume_id: &str) -> VolumeGetCall<'a, S> {
        VolumeGetCall {
            hub: self.hub,
            _volume_id: volume_id.to_string(),
            _user_library_consistent_read: Default::default(),
            _source: Default::default(),
            _projection: Default::default(),
            _partner: Default::default(),
            _include_non_comics_series: Default::default(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Performs a book search.
    /// 
    /// # Arguments
    ///
    /// * `q` - Full-text search query string.
    pub fn list(&self, q: &str) -> VolumeListCall<'a, S> {
        VolumeListCall {
            hub: self.hub,
            _q: q.to_string(),
            _start_index: Default::default(),
            _source: Default::default(),
            _show_preorders: Default::default(),
            _projection: Default::default(),
            _print_type: Default::default(),
            _partner: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _max_allowed_maturity_rating: Default::default(),
            _library_restrict: Default::default(),
            _lang_restrict: Default::default(),
            _filter: Default::default(),
            _download: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



