use super::*;
/// A builder providing access to all methods supported on *application* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `device_tier_configs_create(...)`, `device_tier_configs_get(...)` and `device_tier_configs_list(...)`
/// // to build up your call.
/// let rb = hub.applications();
/// # }
/// ```
pub struct ApplicationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationMethods<'a, S> {}

impl<'a, S> ApplicationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new device tier config for an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    pub fn device_tier_configs_create(&self, request: DeviceTierConfig, package_name: &str) -> ApplicationDeviceTierConfigCreateCall<'a, S> {
        ApplicationDeviceTierConfigCreateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _allow_unknown_devices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a particular device tier config.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `deviceTierConfigId` - Required. Id of an existing device tier config.
    pub fn device_tier_configs_get(&self, package_name: &str, device_tier_config_id: i64) -> ApplicationDeviceTierConfigGetCall<'a, S> {
        ApplicationDeviceTierConfigGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _device_tier_config_id: device_tier_config_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns created device tier configs, ordered by descending creation time.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    pub fn device_tier_configs_list(&self, package_name: &str) -> ApplicationDeviceTierConfigListCall<'a, S> {
        ApplicationDeviceTierConfigListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *edit* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apks_addexternallyhosted(...)`, `apks_list(...)`, `apks_upload(...)`, `bundles_list(...)`, `bundles_upload(...)`, `commit(...)`, `countryavailability_get(...)`, `delete(...)`, `deobfuscationfiles_upload(...)`, `details_get(...)`, `details_patch(...)`, `details_update(...)`, `expansionfiles_get(...)`, `expansionfiles_patch(...)`, `expansionfiles_update(...)`, `expansionfiles_upload(...)`, `get(...)`, `images_delete(...)`, `images_deleteall(...)`, `images_list(...)`, `images_upload(...)`, `insert(...)`, `listings_delete(...)`, `listings_deleteall(...)`, `listings_get(...)`, `listings_list(...)`, `listings_patch(...)`, `listings_update(...)`, `testers_get(...)`, `testers_patch(...)`, `testers_update(...)`, `tracks_get(...)`, `tracks_list(...)`, `tracks_patch(...)`, `tracks_update(...)` and `validate(...)`
/// // to build up your call.
/// let rb = hub.edits();
/// # }
/// ```
pub struct EditMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for EditMethods<'a, S> {}

impl<'a, S> EditMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to organizations using Managed Play whose application is configured to restrict distribution to the organizations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn apks_addexternallyhosted(&self, request: ApksAddExternallyHostedRequest, package_name: &str, edit_id: &str) -> EditApkAddexternallyhostedCall<'a, S> {
        EditApkAddexternallyhostedCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all current APKs of the app and edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn apks_list(&self, package_name: &str, edit_id: &str) -> EditApkListCall<'a, S> {
        EditApkListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads an APK and adds to the current edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn apks_upload(&self, package_name: &str, edit_id: &str) -> EditApkUploadCall<'a, S> {
        EditApkUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all current Android App Bundles of the app and edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn bundles_list(&self, package_name: &str, edit_id: &str) -> EditBundleListCall<'a, S> {
        EditBundleListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn bundles_upload(&self, package_name: &str, edit_id: &str) -> EditBundleUploadCall<'a, S> {
        EditBundleUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _device_tier_config_id: Default::default(),
            _ack_bundle_installation_warning: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets country availability.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - The track to read from.
    pub fn countryavailability_get(&self, package_name: &str, edit_id: &str, track: &str) -> EditCountryavailabilityGetCall<'a, S> {
        EditCountryavailabilityGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new deobfuscation file and attaches to the specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app.
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Deobfuscation File is being uploaded.
    /// * `deobfuscationFileType` - The type of the deobfuscation file.
    pub fn deobfuscationfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, deobfuscation_file_type: &str) -> EditDeobfuscationfileUploadCall<'a, S> {
        EditDeobfuscationfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _deobfuscation_file_type: deobfuscation_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details of an app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn details_get(&self, package_name: &str, edit_id: &str) -> EditDetailGetCall<'a, S> {
        EditDetailGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches details of an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn details_patch(&self, request: AppDetails, package_name: &str, edit_id: &str) -> EditDetailPatchCall<'a, S> {
        EditDetailPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates details of an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn details_update(&self, request: AppDetails, package_name: &str, edit_id: &str) -> EditDetailUpdateCall<'a, S> {
        EditDetailUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the expansion file configuration for the specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `apkVersionCode` - The version code of the APK whose expansion file configuration is being read or modified.
    /// * `expansionFileType` - The file type of the file configuration which is being read or modified.
    pub fn expansionfiles_get(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileGetCall<'a, S> {
        EditExpansionfileGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches the APK's expansion file configuration to reference another APK's expansion file. To add a new expansion file use the Upload method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `apkVersionCode` - The version code of the APK whose expansion file configuration is being read or modified.
    /// * `expansionFileType` - The file type of the expansion file configuration which is being updated.
    pub fn expansionfiles_patch(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfilePatchCall<'a, S> {
        EditExpansionfilePatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the APK's expansion file configuration to reference another APK's expansion file. To add a new expansion file use the Upload method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `apkVersionCode` - The version code of the APK whose expansion file configuration is being read or modified.
    /// * `expansionFileType` - The file type of the file configuration which is being read or modified.
    pub fn expansionfiles_update(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileUpdateCall<'a, S> {
        EditExpansionfileUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new expansion file and attaches to the specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `apkVersionCode` - The version code of the APK whose expansion file configuration is being read or modified.
    /// * `expansionFileType` - The file type of the expansion file configuration which is being updated.
    pub fn expansionfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &str) -> EditExpansionfileUploadCall<'a, S> {
        EditExpansionfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the image (specified by id) from the edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    /// * `imageType` - Type of the Image.
    /// * `imageId` - Unique identifier an image within the set of images attached to this edit.
    pub fn images_delete(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str, image_id: &str) -> EditImageDeleteCall<'a, S> {
        EditImageDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _image_id: image_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all images for the specified language and image type. Returns an empty response if no images are found.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). Providing a language that is not supported by the App is a no-op.
    /// * `imageType` - Type of the Image. Providing an image type that refers to no images is a no-op.
    pub fn images_deleteall(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageDeleteallCall<'a, S> {
        EditImageDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all images. The response may be empty.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). There must be a store listing for the specified language.
    /// * `imageType` - Type of the Image. Providing an image type that refers to no images will return an empty response.
    pub fn images_list(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageListCall<'a, S> {
        EditImageListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads an image of the specified language and image type, and adds to the edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). Providing a language that is not supported by the App is a no-op.
    /// * `imageType` - Type of the Image.
    pub fn images_upload(&self, package_name: &str, edit_id: &str, language: &str, image_type: &str) -> EditImageUploadCall<'a, S> {
        EditImageUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    pub fn listings_delete(&self, package_name: &str, edit_id: &str, language: &str) -> EditListingDeleteCall<'a, S> {
        EditListingDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all store listings.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn listings_deleteall(&self, package_name: &str, edit_id: &str) -> EditListingDeleteallCall<'a, S> {
        EditListingDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    pub fn listings_get(&self, package_name: &str, edit_id: &str, language: &str) -> EditListingGetCall<'a, S> {
        EditListingGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all localized store listings.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn listings_list(&self, package_name: &str, edit_id: &str) -> EditListingListCall<'a, S> {
        EditListingListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    pub fn listings_patch(&self, request: Listing, package_name: &str, edit_id: &str, language: &str) -> EditListingPatchCall<'a, S> {
        EditListingPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates or updates a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `language` - Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German).
    pub fn listings_update(&self, request: Listing, package_name: &str, edit_id: &str, language: &str) -> EditListingUpdateCall<'a, S> {
        EditListingUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets testers. Note: Testers resource does not support email lists.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - The track to read from.
    pub fn testers_get(&self, package_name: &str, edit_id: &str, track: &str) -> EditTesterGetCall<'a, S> {
        EditTesterGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches testers. Note: Testers resource does not support email lists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - The track to update.
    pub fn testers_patch(&self, request: Testers, package_name: &str, edit_id: &str, track: &str) -> EditTesterPatchCall<'a, S> {
        EditTesterPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates testers. Note: Testers resource does not support email lists.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - The track to update.
    pub fn testers_update(&self, request: Testers, package_name: &str, edit_id: &str, track: &str) -> EditTesterUpdateCall<'a, S> {
        EditTesterUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a track.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - Identifier of the track.
    pub fn tracks_get(&self, package_name: &str, edit_id: &str, track: &str) -> EditTrackGetCall<'a, S> {
        EditTrackGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all tracks.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn tracks_list(&self, package_name: &str, edit_id: &str) -> EditTrackListCall<'a, S> {
        EditTrackListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a track.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - Identifier of the track.
    pub fn tracks_patch(&self, request: Track, package_name: &str, edit_id: &str, track: &str) -> EditTrackPatchCall<'a, S> {
        EditTrackPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a track.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    /// * `track` - Identifier of the track.
    pub fn tracks_update(&self, request: Track, package_name: &str, edit_id: &str, track: &str) -> EditTrackUpdateCall<'a, S> {
        EditTrackUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _track: track.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Commits an app edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn commit(&self, package_name: &str, edit_id: &str) -> EditCommitCall<'a, S> {
        EditCommitCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _changes_not_sent_for_review: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an app edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn delete(&self, package_name: &str, edit_id: &str) -> EditDeleteCall<'a, S> {
        EditDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an app edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn get(&self, package_name: &str, edit_id: &str) -> EditGetCall<'a, S> {
        EditGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new edit for an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    pub fn insert(&self, request: AppEdit, package_name: &str) -> EditInsertCall<'a, S> {
        EditInsertCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Validates an app edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `editId` - Identifier of the edit.
    pub fn validate(&self, package_name: &str, edit_id: &str) -> EditValidateCall<'a, S> {
        EditValidateCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *generatedapk* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `download(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.generatedapks();
/// # }
/// ```
pub struct GeneratedapkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for GeneratedapkMethods<'a, S> {}

impl<'a, S> GeneratedapkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads a single signed APK generated from an app bundle.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - Version code of the app bundle.
    /// * `downloadId` - Download ID, which uniquely identifies the APK to download. Can be obtained from the response of `generatedapks.list` method.
    pub fn download(&self, package_name: &str, version_code: i32, download_id: &str) -> GeneratedapkDownloadCall<'a, S> {
        GeneratedapkDownloadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _download_id: download_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns download metadata for all APKs that were generated from a given app bundle.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - Version code of the app bundle.
    pub fn list(&self, package_name: &str, version_code: i32) -> GeneratedapkListCall<'a, S> {
        GeneratedapkListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *grant* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.grants();
/// # }
/// ```
pub struct GrantMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for GrantMethods<'a, S> {}

impl<'a, S> GrantMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Grant access for a user to the given package.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The user which needs permission. Format: developers/{developer}/users/{user}
    pub fn create(&self, request: Grant, parent: &str) -> GrantCreateCall<'a, S> {
        GrantCreateCall {
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
    /// Removes all access for the user to the given package or developer account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the grant to delete. Format: developers/{developer}/users/{email}/grants/{package_name}
    pub fn delete(&self, name: &str) -> GrantDeleteCall<'a, S> {
        GrantDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates access for the user to the given package.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name for this grant, following the pattern "developers/{developer}/users/{email}/grants/{package_name}". If this grant is for a draft app, the app ID will be used in this resource name instead of the package name.
    pub fn patch(&self, request: Grant, name: &str) -> GrantPatchCall<'a, S> {
        GrantPatchCall {
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



/// A builder providing access to all methods supported on *inappproduct* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.inappproducts();
/// # }
/// ```
pub struct InappproductMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for InappproductMethods<'a, S> {}

impl<'a, S> InappproductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an in-app product (i.e. a managed product or a subscriptions).
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `sku` - Unique identifier for the in-app product.
    pub fn delete(&self, package_name: &str, sku: &str) -> InappproductDeleteCall<'a, S> {
        InappproductDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an in-app product, which can be a managed product or a subscription.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `sku` - Unique identifier for the in-app product.
    pub fn get(&self, package_name: &str, sku: &str) -> InappproductGetCall<'a, S> {
        InappproductGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an in-app product (i.e. a managed product or a subscriptions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    pub fn insert(&self, request: InAppProduct, package_name: &str) -> InappproductInsertCall<'a, S> {
        InappproductInsertCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all in-app products - both managed products and subscriptions. If an app has a large number of in-app products, the response may be paginated. In this case the response field `tokenPagination.nextPageToken` will be set and the caller should provide its value as a `token` request parameter to retrieve the next page.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    pub fn list(&self, package_name: &str) -> InappproductListCall<'a, S> {
        InappproductListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an in-app product (i.e. a managed product or a subscriptions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `sku` - Unique identifier for the in-app product.
    pub fn patch(&self, request: InAppProduct, package_name: &str, sku: &str) -> InappproductPatchCall<'a, S> {
        InappproductPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an in-app product (i.e. a managed product or a subscriptions).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `sku` - Unique identifier for the in-app product.
    pub fn update(&self, request: InAppProduct, package_name: &str, sku: &str) -> InappproductUpdateCall<'a, S> {
        InappproductUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _sku: sku.to_string(),
            _auto_convert_missing_prices: Default::default(),
            _allow_missing: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *internalappsharingartifact* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `uploadapk(...)` and `uploadbundle(...)`
/// // to build up your call.
/// let rb = hub.internalappsharingartifacts();
/// # }
/// ```
pub struct InternalappsharingartifactMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for InternalappsharingartifactMethods<'a, S> {}

impl<'a, S> InternalappsharingartifactMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads an APK to internal app sharing. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    pub fn uploadapk(&self, package_name: &str) -> InternalappsharingartifactUploadapkCall<'a, S> {
        InternalappsharingartifactUploadapkCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads an app bundle to internal app sharing. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    pub fn uploadbundle(&self, package_name: &str) -> InternalappsharingartifactUploadbundleCall<'a, S> {
        InternalappsharingartifactUploadbundleCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *monetization* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `convert_region_prices(...)`, `subscriptions_archive(...)`, `subscriptions_base_plans_activate(...)`, `subscriptions_base_plans_deactivate(...)`, `subscriptions_base_plans_delete(...)`, `subscriptions_base_plans_migrate_prices(...)`, `subscriptions_base_plans_offers_activate(...)`, `subscriptions_base_plans_offers_create(...)`, `subscriptions_base_plans_offers_deactivate(...)`, `subscriptions_base_plans_offers_delete(...)`, `subscriptions_base_plans_offers_get(...)`, `subscriptions_base_plans_offers_list(...)`, `subscriptions_base_plans_offers_patch(...)`, `subscriptions_create(...)`, `subscriptions_delete(...)`, `subscriptions_get(...)`, `subscriptions_list(...)` and `subscriptions_patch(...)`
/// // to build up your call.
/// let rb = hub.monetization();
/// # }
/// ```
pub struct MonetizationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for MonetizationMethods<'a, S> {}

impl<'a, S> MonetizationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a subscription offer. Once activated, subscription offers will be available to new subscribers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) of the offer to activate.
    /// * `productId` - Required. The parent subscription (ID) of the offer to activate.
    /// * `basePlanId` - Required. The parent base plan (ID) of the offer to activate.
    /// * `offerId` - Required. The unique offer ID of the offer to activate.
    pub fn subscriptions_base_plans_offers_activate(&self, request: ActivateSubscriptionOfferRequest, package_name: &str, product_id: &str, base_plan_id: &str, offer_id: &str) -> MonetizationSubscriptionBasePlanOfferActivateCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferActivateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _offer_id: offer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new subscription offer. Only auto-renewing base plans can have subscription offers. The offer state will be DRAFT until it is activated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) for which the offer should be created. Must be equal to the package_name field on the Subscription resource.
    /// * `productId` - Required. The parent subscription (ID) for which the offer should be created. Must be equal to the product_id field on the SubscriptionOffer resource.
    /// * `basePlanId` - Required. The parent base plan (ID) for which the offer should be created. Must be equal to the base_plan_id field on the SubscriptionOffer resource.
    pub fn subscriptions_base_plans_offers_create(&self, request: SubscriptionOffer, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanOfferCreateCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferCreateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _regions_version_version: Default::default(),
            _offer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deactivates a subscription offer. Once deactivated, existing subscribers will maintain their subscription, but the offer will become unavailable to new subscribers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) of the offer to deactivate.
    /// * `productId` - Required. The parent subscription (ID) of the offer to deactivate.
    /// * `basePlanId` - Required. The parent base plan (ID) of the offer to deactivate.
    /// * `offerId` - Required. The unique offer ID of the offer to deactivate.
    pub fn subscriptions_base_plans_offers_deactivate(&self, request: DeactivateSubscriptionOfferRequest, package_name: &str, product_id: &str, base_plan_id: &str, offer_id: &str) -> MonetizationSubscriptionBasePlanOfferDeactivateCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferDeactivateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _offer_id: offer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscription offer. Can only be done for draft offers. This action is irreversible.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) of the offer to delete.
    /// * `productId` - Required. The parent subscription (ID) of the offer to delete.
    /// * `basePlanId` - Required. The parent base plan (ID) of the offer to delete.
    /// * `offerId` - Required. The unique offer ID of the offer to delete.
    pub fn subscriptions_base_plans_offers_delete(&self, package_name: &str, product_id: &str, base_plan_id: &str, offer_id: &str) -> MonetizationSubscriptionBasePlanOfferDeleteCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _offer_id: offer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reads a single offer
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) of the offer to get.
    /// * `productId` - Required. The parent subscription (ID) of the offer to get.
    /// * `basePlanId` - Required. The parent base plan (ID) of the offer to get.
    /// * `offerId` - Required. The unique offer ID of the offer to get.
    pub fn subscriptions_base_plans_offers_get(&self, package_name: &str, product_id: &str, base_plan_id: &str, offer_id: &str) -> MonetizationSubscriptionBasePlanOfferGetCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _offer_id: offer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all offers under a given subscription.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) for which the subscriptions should be read.
    /// * `productId` - Required. The parent subscription (ID) for which the offers should be read.
    /// * `basePlanId` - Required. The parent base plan (ID) for which the offers should be read. May be specified as '-' to read all offers under a subscription.
    pub fn subscriptions_base_plans_offers_list(&self, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanOfferListCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subscription offer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. Immutable. The package name of the app the parent subscription belongs to.
    /// * `productId` - Required. Immutable. The ID of the parent subscription this offer belongs to.
    /// * `basePlanId` - Required. Immutable. The ID of the base plan to which this offer is an extension.
    /// * `offerId` - Required. Immutable. Unique ID of this subscription offer. Must be unique within the base plan.
    pub fn subscriptions_base_plans_offers_patch(&self, request: SubscriptionOffer, package_name: &str, product_id: &str, base_plan_id: &str, offer_id: &str) -> MonetizationSubscriptionBasePlanOfferPatchCall<'a, S> {
        MonetizationSubscriptionBasePlanOfferPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _offer_id: offer_id.to_string(),
            _update_mask: Default::default(),
            _regions_version_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a base plan. Once activated, base plans will be available to new subscribers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) of the base plan to activate.
    /// * `productId` - Required. The parent subscription (ID) of the base plan to activate.
    /// * `basePlanId` - Required. The unique base plan ID of the base plan to activate.
    pub fn subscriptions_base_plans_activate(&self, request: ActivateBasePlanRequest, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanActivateCall<'a, S> {
        MonetizationSubscriptionBasePlanActivateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deactivates a base plan. Once deactivated, the base plan will become unavailable to new subscribers, but existing subscribers will maintain their subscription
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) of the base plan to deactivate.
    /// * `productId` - Required. The parent subscription (ID) of the base plan to deactivate.
    /// * `basePlanId` - Required. The unique base plan ID of the base plan to deactivate.
    pub fn subscriptions_base_plans_deactivate(&self, request: DeactivateBasePlanRequest, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanDeactivateCall<'a, S> {
        MonetizationSubscriptionBasePlanDeactivateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a base plan. Can only be done for draft base plans. This action is irreversible.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) of the base plan to delete.
    /// * `productId` - Required. The parent subscription (ID) of the base plan to delete.
    /// * `basePlanId` - Required. The unique offer ID of the base plan to delete.
    pub fn subscriptions_base_plans_delete(&self, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanDeleteCall<'a, S> {
        MonetizationSubscriptionBasePlanDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Migrates subscribers who are receiving an historical subscription price to the currently-offered price for the specified region. Requests will cause price change notifications to be sent to users who are currently receiving an historical price older than the supplied timestamp. Subscribers who do not agree to the new price will have their subscription ended at the next renewal.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. Package name of the parent app. Must be equal to the package_name field on the Subscription resource.
    /// * `productId` - Required. The ID of the subscription to update. Must be equal to the product_id field on the Subscription resource.
    /// * `basePlanId` - Required. The unique base plan ID of the base plan to update prices on.
    pub fn subscriptions_base_plans_migrate_prices(&self, request: MigrateBasePlanPricesRequest, package_name: &str, product_id: &str, base_plan_id: &str) -> MonetizationSubscriptionBasePlanMigratePriceCall<'a, S> {
        MonetizationSubscriptionBasePlanMigratePriceCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _base_plan_id: base_plan_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Archives a subscription. Can only be done if at least one base plan was active in the past, and no base plan is available for new or existing subscribers currently. This action is irreversible, and the subscription ID will remain reserved.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) of the app of the subscription to delete.
    /// * `productId` - Required. The unique product ID of the subscription to delete.
    pub fn subscriptions_archive(&self, request: ArchiveSubscriptionRequest, package_name: &str, product_id: &str) -> MonetizationSubscriptionArchiveCall<'a, S> {
        MonetizationSubscriptionArchiveCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new subscription. Newly added base plans will remain in draft state until activated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The parent app (package name) for which the subscription should be created. Must be equal to the package_name field on the Subscription resource.
    pub fn subscriptions_create(&self, request: Subscription, package_name: &str) -> MonetizationSubscriptionCreateCall<'a, S> {
        MonetizationSubscriptionCreateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _regions_version_version: Default::default(),
            _product_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscription. A subscription can only be deleted if it has never had a base plan published.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) of the app of the subscription to delete.
    /// * `productId` - Required. The unique product ID of the subscription to delete.
    pub fn subscriptions_delete(&self, package_name: &str, product_id: &str) -> MonetizationSubscriptionDeleteCall<'a, S> {
        MonetizationSubscriptionDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reads a single subscription.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) of the subscription to get.
    /// * `productId` - Required. The unique product ID of the subscription to get.
    pub fn subscriptions_get(&self, package_name: &str, product_id: &str) -> MonetizationSubscriptionGetCall<'a, S> {
        MonetizationSubscriptionGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all subscriptions under a given app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Required. The parent app (package name) for which the subscriptions should be read.
    pub fn subscriptions_list(&self, package_name: &str) -> MonetizationSubscriptionListCall<'a, S> {
        MonetizationSubscriptionListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _show_archived: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subscription.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Immutable. Package name of the parent app.
    /// * `productId` - Immutable. Unique product ID of the product. Unique within the parent app. Product IDs must be composed of lower-case letters (a-z), numbers (0-9), underscores (_) and dots (.). It must start with a lower-case letter or number, and be between 1 and 40 (inclusive) characters in length.
    pub fn subscriptions_patch(&self, request: Subscription, package_name: &str, product_id: &str) -> MonetizationSubscriptionPatchCall<'a, S> {
        MonetizationSubscriptionPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _update_mask: Default::default(),
            _regions_version_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Calculates the region prices, using today's exchange rate and country-specific pricing patterns, based on the price in the request for a set of regions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Required. The app package name.
    pub fn convert_region_prices(&self, request: ConvertRegionPricesRequest, package_name: &str) -> MonetizationConvertRegionPriceCall<'a, S> {
        MonetizationConvertRegionPriceCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `refund(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refunds a user's subscription or in-app purchase order. Orders older than 1 year cannot be refunded.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription or in-app item was purchased (for example, 'com.some.thing').
    /// * `orderId` - The order ID provided to the user when the subscription or in-app order was purchased.
    pub fn refund(&self, package_name: &str, order_id: &str) -> OrderRefundCall<'a, S> {
        OrderRefundCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _order_id: order_id.to_string(),
            _revoke: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *purchase* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_acknowledge(...)`, `products_get(...)`, `subscriptions_acknowledge(...)`, `subscriptions_cancel(...)`, `subscriptions_defer(...)`, `subscriptions_get(...)`, `subscriptions_refund(...)`, `subscriptions_revoke(...)`, `subscriptionsv2_get(...)` and `voidedpurchases_list(...)`
/// // to build up your call.
/// let rb = hub.purchases();
/// # }
/// ```
pub struct PurchaseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for PurchaseMethods<'a, S> {}

impl<'a, S> PurchaseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Acknowledges a purchase of an inapp item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - The package name of the application the inapp product was sold in (for example, 'com.some.thing').
    /// * `productId` - The inapp product SKU (for example, 'com.some.thing.inapp1').
    /// * `token` - The token provided to the user's device when the inapp product was purchased.
    pub fn products_acknowledge(&self, request: ProductPurchasesAcknowledgeRequest, package_name: &str, product_id: &str, token: &str) -> PurchaseProductAcknowledgeCall<'a, S> {
        PurchaseProductAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks the purchase and consumption status of an inapp item.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application the inapp product was sold in (for example, 'com.some.thing').
    /// * `productId` - The inapp product SKU (for example, 'com.some.thing.inapp1').
    /// * `token` - The token provided to the user's device when the inapp product was purchased.
    pub fn products_get(&self, package_name: &str, product_id: &str, token: &str) -> PurchaseProductGetCall<'a, S> {
        PurchaseProductGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _product_id: product_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Acknowledges a subscription purchase.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_acknowledge(&self, request: SubscriptionPurchasesAcknowledgeRequest, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionAcknowledgeCall<'a, S> {
        PurchaseSubscriptionAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a user's subscription purchase. The subscription remains valid until its expiration time.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_cancel(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionCancelCall<'a, S> {
        PurchaseSubscriptionCancelCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Defers a user's subscription purchase until a specified future expiration time.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_defer(&self, request: SubscriptionPurchasesDeferRequest, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionDeferCall<'a, S> {
        PurchaseSubscriptionDeferCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether a user's subscription purchase is valid and returns its expiry time.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_get(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionGetCall<'a, S> {
        PurchaseSubscriptionGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refunds a user's subscription purchase, but the subscription remains valid until its expiration time and it will continue to recur.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - "The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_refund(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionRefundCall<'a, S> {
        PurchaseSubscriptionRefundCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Refunds and immediately revokes a user's subscription purchase. Access to the subscription will be terminated immediately and it will stop recurring.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
    /// * `token` - The token provided to the user's device when the subscription was purchased.
    pub fn subscriptions_revoke(&self, package_name: &str, subscription_id: &str, token: &str) -> PurchaseSubscriptionRevokeCall<'a, S> {
        PurchaseSubscriptionRevokeCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _subscription_id: subscription_id.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get metadata about a subscription
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package of the application for which this subscription was purchased (for example, 'com.some.thing').
    /// * `token` - Required. The token provided to the user's device when the subscription was purchased.
    pub fn subscriptionsv2_get(&self, package_name: &str, token: &str) -> PurchaseSubscriptionsv2GetCall<'a, S> {
        PurchaseSubscriptionsv2GetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the purchases that were canceled, refunded or charged-back.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which voided purchases need to be returned (for example, 'com.some.thing').
    pub fn voidedpurchases_list(&self, package_name: &str) -> PurchaseVoidedpurchaseListCall<'a, S> {
        PurchaseVoidedpurchaseListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _type_: Default::default(),
            _token: Default::default(),
            _start_time: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _end_time: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *review* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `reply(...)`
/// // to build up your call.
/// let rb = hub.reviews();
/// # }
/// ```
pub struct ReviewMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for ReviewMethods<'a, S> {}

impl<'a, S> ReviewMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a single review.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `reviewId` - Unique identifier for a review.
    pub fn get(&self, package_name: &str, review_id: &str) -> ReviewGetCall<'a, S> {
        ReviewGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _review_id: review_id.to_string(),
            _translation_language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all reviews.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    pub fn list(&self, package_name: &str) -> ReviewListCall<'a, S> {
        ReviewListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _translation_language: Default::default(),
            _token: Default::default(),
            _start_index: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replies to a single review, or updates an existing reply.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `reviewId` - Unique identifier for a review.
    pub fn reply(&self, request: ReviewsReplyRequest, package_name: &str, review_id: &str) -> ReviewReplyCall<'a, S> {
        ReviewReplyCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _review_id: review_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *systemapk* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `variants_create(...)`, `variants_download(...)`, `variants_get(...)` and `variants_list(...)`
/// // to build up your call.
/// let rb = hub.systemapks();
/// # }
/// ```
pub struct SystemapkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for SystemapkMethods<'a, S> {}

impl<'a, S> SystemapkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an APK which is suitable for inclusion in a system image from an already uploaded Android App Bundle.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - The version code of the App Bundle.
    pub fn variants_create(&self, request: Variant, package_name: &str, version_code: i64) -> SystemapkVariantCreateCall<'a, S> {
        SystemapkVariantCreateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads a previously created system APK which is suitable for inclusion in a system image.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - The version code of the App Bundle.
    /// * `variantId` - The ID of a previously created system APK variant.
    pub fn variants_download(&self, package_name: &str, version_code: i64, variant_id: u32) -> SystemapkVariantDownloadCall<'a, S> {
        SystemapkVariantDownloadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _variant_id: variant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a previously created system APK variant.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - The version code of the App Bundle.
    /// * `variantId` - The ID of a previously created system APK variant.
    pub fn variants_get(&self, package_name: &str, version_code: i64, variant_id: u32) -> SystemapkVariantGetCall<'a, S> {
        SystemapkVariantGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _variant_id: variant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of previously created system APK variants.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Package name of the app.
    /// * `versionCode` - The version code of the App Bundle.
    pub fn variants_list(&self, package_name: &str, version_code: i64) -> SystemapkVariantListCall<'a, S> {
        SystemapkVariantListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _version_code: version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`AndroidPublisher`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidPublisher<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Grant access for a user to the given developer account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The developer account to add the user to. Format: developers/{developer}
    pub fn create(&self, request: User, parent: &str) -> UserCreateCall<'a, S> {
        UserCreateCall {
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
    /// Removes all access for the user to the given developer account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the user to delete. Format: developers/{developer}/users/{email}
    pub fn delete(&self, name: &str) -> UserDeleteCall<'a, S> {
        UserDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all users with access to a developer account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The developer account to fetch users from. Format: developers/{developer}
    pub fn list(&self, parent: &str) -> UserListCall<'a, S> {
        UserListCall {
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
    /// Updates access for the user to the developer account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Resource name for this user, following the pattern "developers/{developer}/users/{email}".
    pub fn patch(&self, request: User, name: &str) -> UserPatchCall<'a, S> {
        UserPatchCall {
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



