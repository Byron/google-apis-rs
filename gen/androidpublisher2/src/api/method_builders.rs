use super::*;
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
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apklistings_delete(...)`, `apklistings_deleteall(...)`, `apklistings_get(...)`, `apklistings_list(...)`, `apklistings_patch(...)`, `apklistings_update(...)`, `apks_addexternallyhosted(...)`, `apks_list(...)`, `apks_upload(...)`, `bundles_list(...)`, `bundles_upload(...)`, `commit(...)`, `delete(...)`, `deobfuscationfiles_upload(...)`, `details_get(...)`, `details_patch(...)`, `details_update(...)`, `expansionfiles_get(...)`, `expansionfiles_patch(...)`, `expansionfiles_update(...)`, `expansionfiles_upload(...)`, `get(...)`, `images_delete(...)`, `images_deleteall(...)`, `images_list(...)`, `images_upload(...)`, `insert(...)`, `listings_delete(...)`, `listings_deleteall(...)`, `listings_get(...)`, `listings_list(...)`, `listings_patch(...)`, `listings_update(...)`, `testers_get(...)`, `testers_patch(...)`, `testers_update(...)`, `tracks_get(...)`, `tracks_list(...)`, `tracks_patch(...)`, `tracks_update(...)` and `validate(...)`
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
    /// Deletes the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_delete(&self, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingDeleteCall<'a, S> {
        EditApklistingDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all the APK-specific localized listings for a specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    pub fn apklistings_deleteall(&self, package_name: &str, edit_id: &str, apk_version_code: i32) -> EditApklistingDeleteallCall<'a, S> {
        EditApklistingDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_get(&self, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingGetCall<'a, S> {
        EditApklistingGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the APK-specific localized listings for a specified APK.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    pub fn apklistings_list(&self, package_name: &str, edit_id: &str, apk_version_code: i32) -> EditApklistingListCall<'a, S> {
        EditApklistingListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates the APK-specific localized listing for a specified APK and language code. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_patch(&self, request: ApkListing, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingPatchCall<'a, S> {
        EditApklistingPatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates or creates the APK-specific localized listing for a specified APK and language code.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The APK version code whose APK-specific listings should be read or modified.
    /// * `language` - The language code (a BCP-47 language tag) of the APK-specific localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
    pub fn apklistings_update(&self, request: ApkListing, package_name: &str, edit_id: &str, apk_version_code: i32, language: &str) -> EditApklistingUpdateCall<'a, S> {
        EditApklistingUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _language: language.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to enterprises using Google Play for Work whose application is configured to restrict distribution to the enterprise domain.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See: https://developers.google.com/api-client-library/java/google-api-java-client/errors for an example in java.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn bundles_upload(&self, package_name: &str, edit_id: &str) -> EditBundleUploadCall<'a, S> {
        EditBundleUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _ack_bundle_installation_warning: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads the deobfuscation file of the specified APK. If a deobfuscation file already exists, it will be replaced.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier of the Android app for which the deobfuscatiuon files are being uploaded; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose deobfuscation file is being uploaded.
    /// * `deobfuscationFileType` - No description provided.
    pub fn deobfuscationfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, deobfuscation_file_type: &EditDeobfuscationFileTypeEnum) -> EditDeobfuscationfileUploadCall<'a, S> {
        EditDeobfuscationfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _deobfuscation_file_type: deobfuscation_file_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Fetches app details for this edit. This includes the default language and developer support contact information.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Updates app details for this edit. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Updates app details for this edit.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Fetches the Expansion File configuration for the APK specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_get(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &EditExpansionFileTypeEnum) -> EditExpansionfileGetCall<'a, S> {
        EditExpansionfileGetCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_patch(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &EditExpansionFileTypeEnum) -> EditExpansionfilePatchCall<'a, S> {
        EditExpansionfilePatchCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the APK's Expansion File configuration to reference another APK's Expansion Files. To add a new Expansion File use the Upload method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_update(&self, request: ExpansionFile, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &EditExpansionFileTypeEnum) -> EditExpansionfileUpdateCall<'a, S> {
        EditExpansionfileUpdateCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads and attaches a new Expansion File to the APK specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `apkVersionCode` - The version code of the APK whose Expansion File configuration is being read or modified.
    /// * `expansionFileType` - No description provided.
    pub fn expansionfiles_upload(&self, package_name: &str, edit_id: &str, apk_version_code: i32, expansion_file_type: &EditExpansionFileTypeEnum) -> EditExpansionfileUploadCall<'a, S> {
        EditExpansionfileUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _apk_version_code: apk_version_code,
            _expansion_file_type: expansion_file_type.clone(),
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
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    /// * `imageId` - Unique identifier an image within the set of images attached to this edit.
    pub fn images_delete(&self, package_name: &str, edit_id: &str, language: &str, image_type: &EditImageTypeEnum, image_id: &str) -> EditImageDeleteCall<'a, S> {
        EditImageDeleteCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.clone(),
            _image_id: image_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes all images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_deleteall(&self, package_name: &str, edit_id: &str, language: &str, image_type: &EditImageTypeEnum) -> EditImageDeleteallCall<'a, S> {
        EditImageDeleteallCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_list(&self, package_name: &str, edit_id: &str, language: &str, image_type: &EditImageTypeEnum) -> EditImageListCall<'a, S> {
        EditImageListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a new image and adds it to the list of images for the specified language and image type.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing whose images are to read or modified. For example, to select Austrian German, pass "de-AT".
    /// * `imageType` - No description provided.
    pub fn images_upload(&self, package_name: &str, edit_id: &str, language: &str, image_type: &EditImageTypeEnum) -> EditImageUploadCall<'a, S> {
        EditImageUploadCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
            _edit_id: edit_id.to_string(),
            _language: language.to_string(),
            _image_type: image_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified localized store listing from an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
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
    /// Deletes all localized listings from an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Fetches information about a localized store listing.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
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
    /// Returns all of the localized store listings attached to this edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Creates or updates a localized store listing. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
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
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `language` - The language code (a BCP-47 language tag) of the localized listing to read or modify. For example, to select Austrian German, pass "de-AT".
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
    
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    /// Fetches the track configuration for the specified track type. Includes the APK version codes that are in this track.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    /// Lists all the track configurations for this edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Updates the track configuration for the specified track type. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    /// Updates the track configuration for the specified track type.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    /// * `track` - The track to read or modify.
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
    /// Commits/applies the changes made in this edit back to the app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
    pub fn commit(&self, package_name: &str, edit_id: &str) -> EditCommitCall<'a, S> {
        EditCommitCall {
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
    /// Deletes an edit for an app. Creating a new edit will automatically delete any of your previous edits so this method need only be called if you want to preemptively abandon an edit.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Returns information about the edit specified. Calls will fail if the edit is no long active (e.g. has been deleted, superseded or expired).
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
    /// Creates a new edit for an app, populated with the app's current state.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
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
    /// Checks that the edit can be successfully committed. The edit's changes are not applied to the live app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app that is being updated; for example, "com.spiffygame".
    /// * `editId` - Unique identifier for this edit.
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
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
    /// Delete an in-app product for an app.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
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
    /// Returns information about the in-app product specified.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - No description provided.
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
    /// Creates a new in-app product for an app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app; for example, "com.spiffygame".
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
    /// List all the in-app products for an Android app, both subscriptions and managed in-app products..
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app with in-app products; for example, "com.spiffygame".
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
    /// Updates the details of an in-app product. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
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
    /// Updates the details of an in-app product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app with the in-app product; for example, "com.spiffygame".
    /// * `sku` - Unique identifier for the in-app product.
    pub fn update(&self, request: InAppProduct, package_name: &str, sku: &str) -> InappproductUpdateCall<'a, S> {
        InappproductUpdateCall {
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
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
    /// Refund a user's subscription or in-app purchase order.
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
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_get(...)`, `subscriptions_cancel(...)`, `subscriptions_defer(...)`, `subscriptions_get(...)`, `subscriptions_refund(...)`, `subscriptions_revoke(...)` and `voidedpurchases_list(...)`
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
    /// * `subscriptionId` - The purchased subscription ID (for example, 'monthly001').
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
    /// Lists the purchases that were canceled, refunded or charged-back.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - The package name of the application for which voided purchases need to be returned (for example, 'com.some.thing').
    pub fn voidedpurchases_list(&self, package_name: &str) -> PurchaseVoidedpurchaseListCall<'a, S> {
        PurchaseVoidedpurchaseListCall {
            hub: self.hub,
            _package_name: package_name.to_string(),
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
/// extern crate google_androidpublisher2 as androidpublisher2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher2::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
    /// Returns a single review.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    /// * `reviewId` - No description provided.
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
    /// Returns a list of reviews. Only reviews from last week will be returned.
    /// 
    /// # Arguments
    ///
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
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
    /// Reply to a single review, or update an existing reply.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` - Unique identifier for the Android app for which we want reviews; for example, "com.spiffygame".
    /// * `reviewId` - No description provided.
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



