use super::*;
/// A builder providing access to all methods supported on *accountActiveAdSummary* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.account_active_ad_summaries();
/// # }
/// ```
pub struct AccountActiveAdSummaryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AccountActiveAdSummaryMethods<'a, S> {}

impl<'a, S> AccountActiveAdSummaryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the account's active ad summary by account ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `summaryAccountId` - Account ID.
    pub fn get(&self, profile_id: i64, summary_account_id: i64) -> AccountActiveAdSummaryGetCall<'a, S> {
        AccountActiveAdSummaryGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _summary_account_id: summary_account_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *accountPermissionGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.account_permission_groups();
/// # }
/// ```
pub struct AccountPermissionGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AccountPermissionGroupMethods<'a, S> {}

impl<'a, S> AccountPermissionGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account permission group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Account permission group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AccountPermissionGroupGetCall<'a, S> {
        AccountPermissionGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of account permission groups.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AccountPermissionGroupListCall<'a, S> {
        AccountPermissionGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *accountPermission* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.account_permissions();
/// # }
/// ```
pub struct AccountPermissionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AccountPermissionMethods<'a, S> {}

impl<'a, S> AccountPermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account permission by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Account permission ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AccountPermissionGetCall<'a, S> {
        AccountPermissionGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of account permissions.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AccountPermissionListCall<'a, S> {
        AccountPermissionListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *accountUserProfile* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.account_user_profiles();
/// # }
/// ```
pub struct AccountUserProfileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AccountUserProfileMethods<'a, S> {}

impl<'a, S> AccountUserProfileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account user profile by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - User profile ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AccountUserProfileGetCall<'a, S> {
        AccountUserProfileGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new account user profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: AccountUserProfile, profile_id: i64) -> AccountUserProfileInsertCall<'a, S> {
        AccountUserProfileInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of account user profiles, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AccountUserProfileListCall<'a, S> {
        AccountUserProfileListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _user_role_id: Default::default(),
            _subaccount_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account user profile. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - AccountUserProfile ID.
    pub fn patch(&self, request: AccountUserProfile, profile_id: i64, id: i64) -> AccountUserProfilePatchCall<'a, S> {
        AccountUserProfilePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account user profile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: AccountUserProfile, profile_id: i64) -> AccountUserProfileUpdateCall<'a, S> {
        AccountUserProfileUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one account by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Account ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of accounts, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Account ID.
    pub fn patch(&self, request: Account, profile_id: i64, id: i64) -> AccountPatchCall<'a, S> {
        AccountPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Account, profile_id: i64) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *ad* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.ads();
/// # }
/// ```
pub struct AdMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AdMethods<'a, S> {}

impl<'a, S> AdMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one ad by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Ad ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AdGetCall<'a, S> {
        AdGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new ad.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Ad, profile_id: i64) -> AdInsertCall<'a, S> {
        AdInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of ads, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AdListCall<'a, S> {
        AdListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _type_: Default::default(),
            _ssl_required: Default::default(),
            _ssl_compliant: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _size_ids: Default::default(),
            _search_string: Default::default(),
            _remarketing_list_ids: Default::default(),
            _placement_ids: Default::default(),
            _page_token: Default::default(),
            _overridden_event_tag_id: Default::default(),
            _max_results: Default::default(),
            _landing_page_ids: Default::default(),
            _ids: Default::default(),
            _dynamic_click_tracker: Default::default(),
            _creative_optimization_configuration_ids: Default::default(),
            _creative_ids: Default::default(),
            _compatibility: Default::default(),
            _campaign_ids: Default::default(),
            _audience_segment_ids: Default::default(),
            _archived: Default::default(),
            _advertiser_id: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing ad. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Ad ID.
    pub fn patch(&self, request: Ad, profile_id: i64, id: i64) -> AdPatchCall<'a, S> {
        AdPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing ad.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Ad, profile_id: i64) -> AdUpdateCall<'a, S> {
        AdUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *advertiserGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.advertiser_groups();
/// # }
/// ```
pub struct AdvertiserGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AdvertiserGroupMethods<'a, S> {}

impl<'a, S> AdvertiserGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing advertiser group.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Advertiser group ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> AdvertiserGroupDeleteCall<'a, S> {
        AdvertiserGroupDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one advertiser group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Advertiser group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AdvertiserGroupGetCall<'a, S> {
        AdvertiserGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new advertiser group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: AdvertiserGroup, profile_id: i64) -> AdvertiserGroupInsertCall<'a, S> {
        AdvertiserGroupInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of advertiser groups, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AdvertiserGroupListCall<'a, S> {
        AdvertiserGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser group. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - AdvertiserGroup ID.
    pub fn patch(&self, request: AdvertiserGroup, profile_id: i64, id: i64) -> AdvertiserGroupPatchCall<'a, S> {
        AdvertiserGroupPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: AdvertiserGroup, profile_id: i64) -> AdvertiserGroupUpdateCall<'a, S> {
        AdvertiserGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *advertiserLandingPage* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.advertiser_landing_pages();
/// # }
/// ```
pub struct AdvertiserLandingPageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AdvertiserLandingPageMethods<'a, S> {}

impl<'a, S> AdvertiserLandingPageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one landing page by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Landing page ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AdvertiserLandingPageGetCall<'a, S> {
        AdvertiserLandingPageGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new landing page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: LandingPage, profile_id: i64) -> AdvertiserLandingPageInsertCall<'a, S> {
        AdvertiserLandingPageInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of landing pages.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AdvertiserLandingPageListCall<'a, S> {
        AdvertiserLandingPageListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _subaccount_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _campaign_ids: Default::default(),
            _archived: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser landing page. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - LandingPage ID.
    pub fn patch(&self, request: LandingPage, profile_id: i64, id: i64) -> AdvertiserLandingPagePatchCall<'a, S> {
        AdvertiserLandingPagePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing landing page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: LandingPage, profile_id: i64) -> AdvertiserLandingPageUpdateCall<'a, S> {
        AdvertiserLandingPageUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *advertiser* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.advertisers();
/// # }
/// ```
pub struct AdvertiserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for AdvertiserMethods<'a, S> {}

impl<'a, S> AdvertiserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one advertiser by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Advertiser ID.
    pub fn get(&self, profile_id: i64, id: i64) -> AdvertiserGetCall<'a, S> {
        AdvertiserGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new advertiser.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Advertiser, profile_id: i64) -> AdvertiserInsertCall<'a, S> {
        AdvertiserInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of advertisers, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> AdvertiserListCall<'a, S> {
        AdvertiserListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _subaccount_id: Default::default(),
            _status: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _only_parent: Default::default(),
            _max_results: Default::default(),
            _include_advertisers_without_groups_only: Default::default(),
            _ids: Default::default(),
            _floodlight_configuration_ids: Default::default(),
            _advertiser_group_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Advertiser ID.
    pub fn patch(&self, request: Advertiser, profile_id: i64, id: i64) -> AdvertiserPatchCall<'a, S> {
        AdvertiserPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing advertiser.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Advertiser, profile_id: i64) -> AdvertiserUpdateCall<'a, S> {
        AdvertiserUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *browser* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.browsers();
/// # }
/// ```
pub struct BrowserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for BrowserMethods<'a, S> {}

impl<'a, S> BrowserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of browsers.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> BrowserListCall<'a, S> {
        BrowserListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *campaignCreativeAssociation* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.campaign_creative_associations();
/// # }
/// ```
pub struct CampaignCreativeAssociationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CampaignCreativeAssociationMethods<'a, S> {}

impl<'a, S> CampaignCreativeAssociationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates a creative with the specified campaign. This method creates a default ad with dimensions matching the creative in the campaign if such a default ad does not exist already.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `campaignId` - Campaign ID in this association.
    pub fn insert(&self, request: CampaignCreativeAssociation, profile_id: i64, campaign_id: i64) -> CampaignCreativeAssociationInsertCall<'a, S> {
        CampaignCreativeAssociationInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _campaign_id: campaign_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of creative IDs associated with the specified campaign. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `campaignId` - Campaign ID in this association.
    pub fn list(&self, profile_id: i64, campaign_id: i64) -> CampaignCreativeAssociationListCall<'a, S> {
        CampaignCreativeAssociationListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _campaign_id: campaign_id,
            _sort_order: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *campaign* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.campaigns();
/// # }
/// ```
pub struct CampaignMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CampaignMethods<'a, S> {}

impl<'a, S> CampaignMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one campaign by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Campaign ID.
    pub fn get(&self, profile_id: i64, id: i64) -> CampaignGetCall<'a, S> {
        CampaignGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new campaign.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Campaign, profile_id: i64) -> CampaignInsertCall<'a, S> {
        CampaignInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of campaigns, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CampaignListCall<'a, S> {
        CampaignListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _subaccount_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _overridden_event_tag_id: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _excluded_ids: Default::default(),
            _at_least_one_optimization_activity: Default::default(),
            _archived: Default::default(),
            _advertiser_ids: Default::default(),
            _advertiser_group_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing campaign. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Campaign ID.
    pub fn patch(&self, request: Campaign, profile_id: i64, id: i64) -> CampaignPatchCall<'a, S> {
        CampaignPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing campaign.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Campaign, profile_id: i64) -> CampaignUpdateCall<'a, S> {
        CampaignUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *changeLog* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.change_logs();
/// # }
/// ```
pub struct ChangeLogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ChangeLogMethods<'a, S> {}

impl<'a, S> ChangeLogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one change log by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Change log ID.
    pub fn get(&self, profile_id: i64, id: i64) -> ChangeLogGetCall<'a, S> {
        ChangeLogGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of change logs. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> ChangeLogListCall<'a, S> {
        ChangeLogListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _user_profile_ids: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _object_type: Default::default(),
            _object_ids: Default::default(),
            _min_change_time: Default::default(),
            _max_results: Default::default(),
            _max_change_time: Default::default(),
            _ids: Default::default(),
            _action: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *city* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.cities();
/// # }
/// ```
pub struct CityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CityMethods<'a, S> {}

impl<'a, S> CityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of cities, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CityListCall<'a, S> {
        CityListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _region_dart_ids: Default::default(),
            _name_prefix: Default::default(),
            _dart_ids: Default::default(),
            _country_dart_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *connectionType* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.connection_types();
/// # }
/// ```
pub struct ConnectionTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ConnectionTypeMethods<'a, S> {}

impl<'a, S> ConnectionTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one connection type by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Connection type ID.
    pub fn get(&self, profile_id: i64, id: i64) -> ConnectionTypeGetCall<'a, S> {
        ConnectionTypeGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of connection types.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> ConnectionTypeListCall<'a, S> {
        ConnectionTypeListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *contentCategory* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.content_categories();
/// # }
/// ```
pub struct ContentCategoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ContentCategoryMethods<'a, S> {}

impl<'a, S> ContentCategoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing content category.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Content category ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> ContentCategoryDeleteCall<'a, S> {
        ContentCategoryDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one content category by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Content category ID.
    pub fn get(&self, profile_id: i64, id: i64) -> ContentCategoryGetCall<'a, S> {
        ContentCategoryGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new content category.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: ContentCategory, profile_id: i64) -> ContentCategoryInsertCall<'a, S> {
        ContentCategoryInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of content categories, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> ContentCategoryListCall<'a, S> {
        ContentCategoryListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing content category. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - ContentCategory ID.
    pub fn patch(&self, request: ContentCategory, profile_id: i64, id: i64) -> ContentCategoryPatchCall<'a, S> {
        ContentCategoryPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing content category.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: ContentCategory, profile_id: i64) -> ContentCategoryUpdateCall<'a, S> {
        ContentCategoryUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *conversion* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batchinsert(...)` and `batchupdate(...)`
/// // to build up your call.
/// let rb = hub.conversions();
/// # }
/// ```
pub struct ConversionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ConversionMethods<'a, S> {}

impl<'a, S> ConversionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts conversions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn batchinsert(&self, request: ConversionsBatchInsertRequest, profile_id: i64) -> ConversionBatchinsertCall<'a, S> {
        ConversionBatchinsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates existing conversions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn batchupdate(&self, request: ConversionsBatchUpdateRequest, profile_id: i64) -> ConversionBatchupdateCall<'a, S> {
        ConversionBatchupdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *country* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.countries();
/// # }
/// ```
pub struct CountryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CountryMethods<'a, S> {}

impl<'a, S> CountryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one country by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `dartId` - Country DART ID.
    pub fn get(&self, profile_id: i64, dart_id: i64) -> CountryGetCall<'a, S> {
        CountryGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _dart_id: dart_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of countries.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CountryListCall<'a, S> {
        CountryListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creativeAsset* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.creative_assets();
/// # }
/// ```
pub struct CreativeAssetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeAssetMethods<'a, S> {}

impl<'a, S> CreativeAssetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative asset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `advertiserId` - Advertiser ID of this creative. This is a required field.
    pub fn insert(&self, request: CreativeAssetMetadata, profile_id: i64, advertiser_id: i64) -> CreativeAssetInsertCall<'a, S> {
        CreativeAssetInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creativeFieldValue* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.creative_field_values();
/// # }
/// ```
pub struct CreativeFieldValueMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeFieldValueMethods<'a, S> {}

impl<'a, S> CreativeFieldValueMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing creative field value.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - Creative field ID for this creative field value.
    /// * `id` - Creative Field Value ID
    pub fn delete(&self, profile_id: i64, creative_field_id: i64, id: i64) -> CreativeFieldValueDeleteCall<'a, S> {
        CreativeFieldValueDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one creative field value by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - Creative field ID for this creative field value.
    /// * `id` - Creative Field Value ID
    pub fn get(&self, profile_id: i64, creative_field_id: i64, id: i64) -> CreativeFieldValueGetCall<'a, S> {
        CreativeFieldValueGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative field value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - Creative field ID for this creative field value.
    pub fn insert(&self, request: CreativeFieldValue, profile_id: i64, creative_field_id: i64) -> CreativeFieldValueInsertCall<'a, S> {
        CreativeFieldValueInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of creative field values, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - Creative field ID for this creative field value.
    pub fn list(&self, profile_id: i64, creative_field_id: i64) -> CreativeFieldValueListCall<'a, S> {
        CreativeFieldValueListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative field value. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - CreativeField ID.
    /// * `id` - CreativeFieldValue ID.
    pub fn patch(&self, request: CreativeFieldValue, profile_id: i64, creative_field_id: i64, id: i64) -> CreativeFieldValuePatchCall<'a, S> {
        CreativeFieldValuePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative field value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `creativeFieldId` - Creative field ID for this creative field value.
    pub fn update(&self, request: CreativeFieldValue, profile_id: i64, creative_field_id: i64) -> CreativeFieldValueUpdateCall<'a, S> {
        CreativeFieldValueUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _creative_field_id: creative_field_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creativeField* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.creative_fields();
/// # }
/// ```
pub struct CreativeFieldMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeFieldMethods<'a, S> {}

impl<'a, S> CreativeFieldMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing creative field.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Creative Field ID
    pub fn delete(&self, profile_id: i64, id: i64) -> CreativeFieldDeleteCall<'a, S> {
        CreativeFieldDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one creative field by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Creative Field ID
    pub fn get(&self, profile_id: i64, id: i64) -> CreativeFieldGetCall<'a, S> {
        CreativeFieldGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: CreativeField, profile_id: i64) -> CreativeFieldInsertCall<'a, S> {
        CreativeFieldInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of creative fields, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CreativeFieldListCall<'a, S> {
        CreativeFieldListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative field. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - CreativeField ID.
    pub fn patch(&self, request: CreativeField, profile_id: i64, id: i64) -> CreativeFieldPatchCall<'a, S> {
        CreativeFieldPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative field.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: CreativeField, profile_id: i64) -> CreativeFieldUpdateCall<'a, S> {
        CreativeFieldUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creativeGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.creative_groups();
/// # }
/// ```
pub struct CreativeGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeGroupMethods<'a, S> {}

impl<'a, S> CreativeGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one creative group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Creative group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> CreativeGroupGetCall<'a, S> {
        CreativeGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: CreativeGroup, profile_id: i64) -> CreativeGroupInsertCall<'a, S> {
        CreativeGroupInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of creative groups, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CreativeGroupListCall<'a, S> {
        CreativeGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _group_number: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative group. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - CreativeGroup ID.
    pub fn patch(&self, request: CreativeGroup, profile_id: i64, id: i64) -> CreativeGroupPatchCall<'a, S> {
        CreativeGroupPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: CreativeGroup, profile_id: i64) -> CreativeGroupUpdateCall<'a, S> {
        CreativeGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *creative* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.creatives();
/// # }
/// ```
pub struct CreativeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for CreativeMethods<'a, S> {}

impl<'a, S> CreativeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one creative by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Creative ID.
    pub fn get(&self, profile_id: i64, id: i64) -> CreativeGetCall<'a, S> {
        CreativeGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Creative, profile_id: i64) -> CreativeInsertCall<'a, S> {
        CreativeInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of creatives, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> CreativeListCall<'a, S> {
        CreativeListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _types: Default::default(),
            _studio_creative_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _size_ids: Default::default(),
            _search_string: Default::default(),
            _rendering_ids: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _creative_field_ids: Default::default(),
            _companion_creative_ids: Default::default(),
            _campaign_id: Default::default(),
            _archived: Default::default(),
            _advertiser_id: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Creative ID.
    pub fn patch(&self, request: Creative, profile_id: i64, id: i64) -> CreativePatchCall<'a, S> {
        CreativePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing creative.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Creative, profile_id: i64) -> CreativeUpdateCall<'a, S> {
        CreativeUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dimensionValue* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`
/// // to build up your call.
/// let rb = hub.dimension_values();
/// # }
/// ```
pub struct DimensionValueMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for DimensionValueMethods<'a, S> {}

impl<'a, S> DimensionValueMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves list of report dimension values for a list of filters.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    pub fn query(&self, request: DimensionValueRequest, profile_id: i64) -> DimensionValueQueryCall<'a, S> {
        DimensionValueQueryCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *directorySite* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.directory_sites();
/// # }
/// ```
pub struct DirectorySiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for DirectorySiteMethods<'a, S> {}

impl<'a, S> DirectorySiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one directory site by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Directory site ID.
    pub fn get(&self, profile_id: i64, id: i64) -> DirectorySiteGetCall<'a, S> {
        DirectorySiteGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new directory site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: DirectorySite, profile_id: i64) -> DirectorySiteInsertCall<'a, S> {
        DirectorySiteInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of directory sites, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> DirectorySiteListCall<'a, S> {
        DirectorySiteListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _dfp_network_code: Default::default(),
            _active: Default::default(),
            _accepts_publisher_paid_placements: Default::default(),
            _accepts_interstitial_placements: Default::default(),
            _accepts_in_stream_video_placements: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *dynamicTargetingKey* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.dynamic_targeting_keys();
/// # }
/// ```
pub struct DynamicTargetingKeyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for DynamicTargetingKeyMethods<'a, S> {}

impl<'a, S> DynamicTargetingKeyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing dynamic targeting key.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `objectId` - ID of the object of this dynamic targeting key. This is a required field.
    /// * `name` - Name of this dynamic targeting key. This is a required field. Must be less than 256 characters long and cannot contain commas. All characters are converted to lowercase.
    /// * `objectType` - Type of the object of this dynamic targeting key. This is a required field.
    pub fn delete(&self, profile_id: i64, object_id: i64, name: &str, object_type: &DynamicTargetingKeyObjectTypeEnum) -> DynamicTargetingKeyDeleteCall<'a, S> {
        DynamicTargetingKeyDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _object_id: object_id,
            _name: name.to_string(),
            _object_type: object_type.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new dynamic targeting key. Keys must be created at the advertiser level before being assigned to the advertiser's ads, creatives, or placements. There is a maximum of 1000 keys per advertiser, out of which a maximum of 20 keys can be assigned per ad, creative, or placement.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: DynamicTargetingKey, profile_id: i64) -> DynamicTargetingKeyInsertCall<'a, S> {
        DynamicTargetingKeyInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of dynamic targeting keys.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> DynamicTargetingKeyListCall<'a, S> {
        DynamicTargetingKeyListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _object_type: Default::default(),
            _object_id: Default::default(),
            _names: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *eventTag* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.event_tags();
/// # }
/// ```
pub struct EventTagMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for EventTagMethods<'a, S> {}

impl<'a, S> EventTagMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing event tag.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Event tag ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> EventTagDeleteCall<'a, S> {
        EventTagDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one event tag by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Event tag ID.
    pub fn get(&self, profile_id: i64, id: i64) -> EventTagGetCall<'a, S> {
        EventTagGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new event tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: EventTag, profile_id: i64) -> EventTagInsertCall<'a, S> {
        EventTagInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of event tags, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> EventTagListCall<'a, S> {
        EventTagListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _ids: Default::default(),
            _event_tag_types: Default::default(),
            _enabled: Default::default(),
            _definitions_only: Default::default(),
            _campaign_id: Default::default(),
            _advertiser_id: Default::default(),
            _ad_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing event tag. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - EventTag ID.
    pub fn patch(&self, request: EventTag, profile_id: i64, id: i64) -> EventTagPatchCall<'a, S> {
        EventTagPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing event tag.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: EventTag, profile_id: i64) -> EventTagUpdateCall<'a, S> {
        EventTagUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *file* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.files();
/// # }
/// ```
pub struct FileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for FileMethods<'a, S> {}

impl<'a, S> FileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report file by its report ID and file ID. This method supports media download.
    /// 
    /// # Arguments
    ///
    /// * `reportId` - The ID of the report.
    /// * `fileId` - The ID of the report file.
    pub fn get(&self, report_id: i64, file_id: i64) -> FileGetCall<'a, S> {
        FileGetCall {
            hub: self.hub,
            _report_id: report_id,
            _file_id: file_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists files for a user profile.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    pub fn list(&self, profile_id: i64) -> FileListCall<'a, S> {
        FileListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _scope: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *floodlightActivity* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `generatetag(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.floodlight_activities();
/// # }
/// ```
pub struct FloodlightActivityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for FloodlightActivityMethods<'a, S> {}

impl<'a, S> FloodlightActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing floodlight activity.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Floodlight activity ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> FloodlightActivityDeleteCall<'a, S> {
        FloodlightActivityDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a tag for a floodlight activity.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn generatetag(&self, profile_id: i64) -> FloodlightActivityGeneratetagCall<'a, S> {
        FloodlightActivityGeneratetagCall {
            hub: self.hub,
            _profile_id: profile_id,
            _floodlight_activity_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one floodlight activity by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Floodlight activity ID.
    pub fn get(&self, profile_id: i64, id: i64) -> FloodlightActivityGetCall<'a, S> {
        FloodlightActivityGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new floodlight activity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: FloodlightActivity, profile_id: i64) -> FloodlightActivityInsertCall<'a, S> {
        FloodlightActivityInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of floodlight activities, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> FloodlightActivityListCall<'a, S> {
        FloodlightActivityListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _tag_string: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _floodlight_configuration_id: Default::default(),
            _floodlight_activity_group_type: Default::default(),
            _floodlight_activity_group_tag_string: Default::default(),
            _floodlight_activity_group_name: Default::default(),
            _floodlight_activity_group_ids: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight activity. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - FloodlightActivity ID.
    pub fn patch(&self, request: FloodlightActivity, profile_id: i64, id: i64) -> FloodlightActivityPatchCall<'a, S> {
        FloodlightActivityPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight activity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: FloodlightActivity, profile_id: i64) -> FloodlightActivityUpdateCall<'a, S> {
        FloodlightActivityUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *floodlightActivityGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.floodlight_activity_groups();
/// # }
/// ```
pub struct FloodlightActivityGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for FloodlightActivityGroupMethods<'a, S> {}

impl<'a, S> FloodlightActivityGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one floodlight activity group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Floodlight activity Group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> FloodlightActivityGroupGetCall<'a, S> {
        FloodlightActivityGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new floodlight activity group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: FloodlightActivityGroup, profile_id: i64) -> FloodlightActivityGroupInsertCall<'a, S> {
        FloodlightActivityGroupInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of floodlight activity groups, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> FloodlightActivityGroupListCall<'a, S> {
        FloodlightActivityGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _type_: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _floodlight_configuration_id: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight activity group. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - FloodlightActivityGroup ID.
    pub fn patch(&self, request: FloodlightActivityGroup, profile_id: i64, id: i64) -> FloodlightActivityGroupPatchCall<'a, S> {
        FloodlightActivityGroupPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight activity group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: FloodlightActivityGroup, profile_id: i64) -> FloodlightActivityGroupUpdateCall<'a, S> {
        FloodlightActivityGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *floodlightConfiguration* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.floodlight_configurations();
/// # }
/// ```
pub struct FloodlightConfigurationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for FloodlightConfigurationMethods<'a, S> {}

impl<'a, S> FloodlightConfigurationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one floodlight configuration by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Floodlight configuration ID.
    pub fn get(&self, profile_id: i64, id: i64) -> FloodlightConfigurationGetCall<'a, S> {
        FloodlightConfigurationGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of floodlight configurations, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> FloodlightConfigurationListCall<'a, S> {
        FloodlightConfigurationListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight configuration. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - FloodlightConfiguration ID.
    pub fn patch(&self, request: FloodlightConfiguration, profile_id: i64, id: i64) -> FloodlightConfigurationPatchCall<'a, S> {
        FloodlightConfigurationPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing floodlight configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: FloodlightConfiguration, profile_id: i64) -> FloodlightConfigurationUpdateCall<'a, S> {
        FloodlightConfigurationUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *inventoryItem* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.inventory_items();
/// # }
/// ```
pub struct InventoryItemMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for InventoryItemMethods<'a, S> {}

impl<'a, S> InventoryItemMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one inventory item by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for order documents.
    /// * `id` - Inventory item ID.
    pub fn get(&self, profile_id: i64, project_id: i64, id: i64) -> InventoryItemGetCall<'a, S> {
        InventoryItemGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of inventory items, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for order documents.
    pub fn list(&self, profile_id: i64, project_id: i64) -> InventoryItemListCall<'a, S> {
        InventoryItemListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _type_: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _site_id: Default::default(),
            _page_token: Default::default(),
            _order_id: Default::default(),
            _max_results: Default::default(),
            _in_plan: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *language* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.languages();
/// # }
/// ```
pub struct LanguageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for LanguageMethods<'a, S> {}

impl<'a, S> LanguageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of languages.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> LanguageListCall<'a, S> {
        LanguageListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metro* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.metros();
/// # }
/// ```
pub struct MetroMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for MetroMethods<'a, S> {}

impl<'a, S> MetroMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of metros.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> MetroListCall<'a, S> {
        MetroListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mobileApp* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.mobile_apps();
/// # }
/// ```
pub struct MobileAppMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for MobileAppMethods<'a, S> {}

impl<'a, S> MobileAppMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one mobile app by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Mobile app ID.
    pub fn get(&self, profile_id: i64, id: &str) -> MobileAppGetCall<'a, S> {
        MobileAppGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves list of available mobile apps.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> MobileAppListCall<'a, S> {
        MobileAppListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _directories: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *mobileCarrier* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.mobile_carriers();
/// # }
/// ```
pub struct MobileCarrierMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for MobileCarrierMethods<'a, S> {}

impl<'a, S> MobileCarrierMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one mobile carrier by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Mobile carrier ID.
    pub fn get(&self, profile_id: i64, id: i64) -> MobileCarrierGetCall<'a, S> {
        MobileCarrierGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of mobile carriers.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> MobileCarrierListCall<'a, S> {
        MobileCarrierListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operatingSystemVersion* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operating_system_versions();
/// # }
/// ```
pub struct OperatingSystemVersionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for OperatingSystemVersionMethods<'a, S> {}

impl<'a, S> OperatingSystemVersionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one operating system version by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Operating system version ID.
    pub fn get(&self, profile_id: i64, id: i64) -> OperatingSystemVersionGetCall<'a, S> {
        OperatingSystemVersionGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of operating system versions.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> OperatingSystemVersionListCall<'a, S> {
        OperatingSystemVersionListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operatingSystem* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.operating_systems();
/// # }
/// ```
pub struct OperatingSystemMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for OperatingSystemMethods<'a, S> {}

impl<'a, S> OperatingSystemMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one operating system by DART ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `dartId` - Operating system DART ID.
    pub fn get(&self, profile_id: i64, dart_id: i64) -> OperatingSystemGetCall<'a, S> {
        OperatingSystemGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _dart_id: dart_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of operating systems.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> OperatingSystemListCall<'a, S> {
        OperatingSystemListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderDocument* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.order_documents();
/// # }
/// ```
pub struct OrderDocumentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for OrderDocumentMethods<'a, S> {}

impl<'a, S> OrderDocumentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one order document by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for order documents.
    /// * `id` - Order document ID.
    pub fn get(&self, profile_id: i64, project_id: i64, id: i64) -> OrderDocumentGetCall<'a, S> {
        OrderDocumentGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of order documents, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for order documents.
    pub fn list(&self, profile_id: i64, project_id: i64) -> OrderDocumentListCall<'a, S> {
        OrderDocumentListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _site_id: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _order_id: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _approved: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one order by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for orders.
    /// * `id` - Order ID.
    pub fn get(&self, profile_id: i64, project_id: i64, id: i64) -> OrderGetCall<'a, S> {
        OrderGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of orders, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `projectId` - Project ID for orders.
    pub fn list(&self, profile_id: i64, project_id: i64) -> OrderListCall<'a, S> {
        OrderListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _project_id: project_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _site_id: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *placementGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.placement_groups();
/// # }
/// ```
pub struct PlacementGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for PlacementGroupMethods<'a, S> {}

impl<'a, S> PlacementGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one placement group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Placement group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> PlacementGroupGetCall<'a, S> {
        PlacementGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new placement group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: PlacementGroup, profile_id: i64) -> PlacementGroupInsertCall<'a, S> {
        PlacementGroupInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of placement groups, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> PlacementGroupListCall<'a, S> {
        PlacementGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _site_ids: Default::default(),
            _search_string: Default::default(),
            _pricing_types: Default::default(),
            _placement_strategy_ids: Default::default(),
            _placement_group_type: Default::default(),
            _page_token: Default::default(),
            _min_start_date: Default::default(),
            _min_end_date: Default::default(),
            _max_start_date: Default::default(),
            _max_results: Default::default(),
            _max_end_date: Default::default(),
            _ids: Default::default(),
            _directory_site_ids: Default::default(),
            _content_category_ids: Default::default(),
            _campaign_ids: Default::default(),
            _archived: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement group. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - PlacementGroup ID.
    pub fn patch(&self, request: PlacementGroup, profile_id: i64, id: i64) -> PlacementGroupPatchCall<'a, S> {
        PlacementGroupPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: PlacementGroup, profile_id: i64) -> PlacementGroupUpdateCall<'a, S> {
        PlacementGroupUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *placementStrategy* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.placement_strategies();
/// # }
/// ```
pub struct PlacementStrategyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for PlacementStrategyMethods<'a, S> {}

impl<'a, S> PlacementStrategyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing placement strategy.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Placement strategy ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> PlacementStrategyDeleteCall<'a, S> {
        PlacementStrategyDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one placement strategy by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Placement strategy ID.
    pub fn get(&self, profile_id: i64, id: i64) -> PlacementStrategyGetCall<'a, S> {
        PlacementStrategyGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new placement strategy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: PlacementStrategy, profile_id: i64) -> PlacementStrategyInsertCall<'a, S> {
        PlacementStrategyInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of placement strategies, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> PlacementStrategyListCall<'a, S> {
        PlacementStrategyListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement strategy. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - PlacementStrategy ID.
    pub fn patch(&self, request: PlacementStrategy, profile_id: i64, id: i64) -> PlacementStrategyPatchCall<'a, S> {
        PlacementStrategyPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement strategy.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: PlacementStrategy, profile_id: i64) -> PlacementStrategyUpdateCall<'a, S> {
        PlacementStrategyUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *placement* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generatetags(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.placements();
/// # }
/// ```
pub struct PlacementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for PlacementMethods<'a, S> {}

impl<'a, S> PlacementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates tags for a placement.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn generatetags(&self, profile_id: i64) -> PlacementGeneratetagCall<'a, S> {
        PlacementGeneratetagCall {
            hub: self.hub,
            _profile_id: profile_id,
            _tag_formats: Default::default(),
            _placement_ids: Default::default(),
            _campaign_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one placement by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Placement ID.
    pub fn get(&self, profile_id: i64, id: i64) -> PlacementGetCall<'a, S> {
        PlacementGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new placement.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Placement, profile_id: i64) -> PlacementInsertCall<'a, S> {
        PlacementInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of placements, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> PlacementListCall<'a, S> {
        PlacementListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _size_ids: Default::default(),
            _site_ids: Default::default(),
            _search_string: Default::default(),
            _pricing_types: Default::default(),
            _placement_strategy_ids: Default::default(),
            _payment_source: Default::default(),
            _page_token: Default::default(),
            _min_start_date: Default::default(),
            _min_end_date: Default::default(),
            _max_start_date: Default::default(),
            _max_results: Default::default(),
            _max_end_date: Default::default(),
            _ids: Default::default(),
            _group_ids: Default::default(),
            _directory_site_ids: Default::default(),
            _content_category_ids: Default::default(),
            _compatibilities: Default::default(),
            _campaign_ids: Default::default(),
            _archived: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Placement ID.
    pub fn patch(&self, request: Placement, profile_id: i64, id: i64) -> PlacementPatchCall<'a, S> {
        PlacementPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing placement.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Placement, profile_id: i64) -> PlacementUpdateCall<'a, S> {
        PlacementUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *platformType* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.platform_types();
/// # }
/// ```
pub struct PlatformTypeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for PlatformTypeMethods<'a, S> {}

impl<'a, S> PlatformTypeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one platform type by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Platform type ID.
    pub fn get(&self, profile_id: i64, id: i64) -> PlatformTypeGetCall<'a, S> {
        PlatformTypeGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of platform types.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> PlatformTypeListCall<'a, S> {
        PlatformTypeListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *postalCode* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.postal_codes();
/// # }
/// ```
pub struct PostalCodeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for PostalCodeMethods<'a, S> {}

impl<'a, S> PostalCodeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one postal code by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `code` - Postal code ID.
    pub fn get(&self, profile_id: i64, code: &str) -> PostalCodeGetCall<'a, S> {
        PostalCodeGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _code: code.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of postal codes.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> PostalCodeListCall<'a, S> {
        PostalCodeListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one project by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Project ID.
    pub fn get(&self, profile_id: i64, id: i64) -> ProjectGetCall<'a, S> {
        ProjectGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of projects, possibly filtered. This method supports paging .
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> ProjectListCall<'a, S> {
        ProjectListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _advertiser_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *region* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.regions();
/// # }
/// ```
pub struct RegionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for RegionMethods<'a, S> {}

impl<'a, S> RegionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of regions.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> RegionListCall<'a, S> {
        RegionListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *remarketingListShare* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.remarketing_list_shares();
/// # }
/// ```
pub struct RemarketingListShareMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for RemarketingListShareMethods<'a, S> {}

impl<'a, S> RemarketingListShareMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one remarketing list share by remarketing list ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `remarketingListId` - Remarketing list ID.
    pub fn get(&self, profile_id: i64, remarketing_list_id: i64) -> RemarketingListShareGetCall<'a, S> {
        RemarketingListShareGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _remarketing_list_id: remarketing_list_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing list share. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - RemarketingList ID.
    pub fn patch(&self, request: RemarketingListShare, profile_id: i64, id: i64) -> RemarketingListSharePatchCall<'a, S> {
        RemarketingListSharePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing list share.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: RemarketingListShare, profile_id: i64) -> RemarketingListShareUpdateCall<'a, S> {
        RemarketingListShareUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *remarketingList* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.remarketing_lists();
/// # }
/// ```
pub struct RemarketingListMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for RemarketingListMethods<'a, S> {}

impl<'a, S> RemarketingListMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one remarketing list by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Remarketing list ID.
    pub fn get(&self, profile_id: i64, id: i64) -> RemarketingListGetCall<'a, S> {
        RemarketingListGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new remarketing list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: RemarketingList, profile_id: i64) -> RemarketingListInsertCall<'a, S> {
        RemarketingListInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of remarketing lists, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `advertiserId` - Select only remarketing lists owned by this advertiser.
    pub fn list(&self, profile_id: i64, advertiser_id: i64) -> RemarketingListListCall<'a, S> {
        RemarketingListListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _advertiser_id: advertiser_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _page_token: Default::default(),
            _name: Default::default(),
            _max_results: Default::default(),
            _floodlight_activity_id: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing list. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - RemarketingList ID.
    pub fn patch(&self, request: RemarketingList, profile_id: i64, id: i64) -> RemarketingListPatchCall<'a, S> {
        RemarketingListPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing remarketing list.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: RemarketingList, profile_id: i64) -> RemarketingListUpdateCall<'a, S> {
        RemarketingListUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `compatible_fields_query(...)`, `delete(...)`, `files_get(...)`, `files_list(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `run(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the fields that are compatible to be selected in the respective sections of a report criteria, given the fields already selected in the input report and user permissions.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    pub fn compatible_fields_query(&self, request: Report, profile_id: i64) -> ReportCompatibleFieldQueryCall<'a, S> {
        ReportCompatibleFieldQueryCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report file by its report ID and file ID. This method supports media download.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the report.
    /// * `fileId` - The ID of the report file.
    pub fn files_get(&self, profile_id: i64, report_id: i64, file_id: i64) -> ReportFileGetCall<'a, S> {
        ReportFileGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _report_id: report_id,
            _file_id: file_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists files for a report.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the parent report.
    pub fn files_list(&self, profile_id: i64, report_id: i64) -> ReportFileListCall<'a, S> {
        ReportFileListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _report_id: report_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a report by its ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the report.
    pub fn delete(&self, profile_id: i64, report_id: i64) -> ReportDeleteCall<'a, S> {
        ReportDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _report_id: report_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a report by its ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the report.
    pub fn get(&self, profile_id: i64, report_id: i64) -> ReportGetCall<'a, S> {
        ReportGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _report_id: report_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a report.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    pub fn insert(&self, request: Report, profile_id: i64) -> ReportInsertCall<'a, S> {
        ReportInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves list of reports.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    pub fn list(&self, profile_id: i64) -> ReportListCall<'a, S> {
        ReportListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _scope: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing report. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - The DFA user profile ID.
    /// * `reportId` - The ID of the report.
    pub fn patch(&self, request: Report, profile_id: i64, report_id: i64) -> ReportPatchCall<'a, S> {
        ReportPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _report_id: report_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a report.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the report.
    pub fn run(&self, profile_id: i64, report_id: i64) -> ReportRunCall<'a, S> {
        ReportRunCall {
            hub: self.hub,
            _profile_id: profile_id,
            _report_id: report_id,
            _synchronous: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a report.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - The Campaign Manager 360 user profile ID.
    /// * `reportId` - The ID of the report.
    pub fn update(&self, request: Report, profile_id: i64, report_id: i64) -> ReportUpdateCall<'a, S> {
        ReportUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _report_id: report_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *site* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.sites();
/// # }
/// ```
pub struct SiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for SiteMethods<'a, S> {}

impl<'a, S> SiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one site by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Site ID.
    pub fn get(&self, profile_id: i64, id: i64) -> SiteGetCall<'a, S> {
        SiteGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Site, profile_id: i64) -> SiteInsertCall<'a, S> {
        SiteInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of sites, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> SiteListCall<'a, S> {
        SiteListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _unmapped_site: Default::default(),
            _subaccount_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _directory_site_ids: Default::default(),
            _campaign_ids: Default::default(),
            _approved: Default::default(),
            _ad_words_site: Default::default(),
            _accepts_publisher_paid_placements: Default::default(),
            _accepts_interstitial_placements: Default::default(),
            _accepts_in_stream_video_placements: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing site. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Site ID.
    pub fn patch(&self, request: Site, profile_id: i64, id: i64) -> SitePatchCall<'a, S> {
        SitePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing site.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Site, profile_id: i64) -> SiteUpdateCall<'a, S> {
        SiteUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *size* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.sizes();
/// # }
/// ```
pub struct SizeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for SizeMethods<'a, S> {}

impl<'a, S> SizeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one size by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Size ID.
    pub fn get(&self, profile_id: i64, id: i64) -> SizeGetCall<'a, S> {
        SizeGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new size.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Size, profile_id: i64) -> SizeInsertCall<'a, S> {
        SizeInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of sizes, possibly filtered. Retrieved sizes are globally unique and may include values not currently in use by your account. Due to this, the list of sizes returned by this method may differ from the list seen in the Trafficking UI.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> SizeListCall<'a, S> {
        SizeListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _width: Default::default(),
            _ids: Default::default(),
            _iab_standard: Default::default(),
            _height: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subaccount* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.subaccounts();
/// # }
/// ```
pub struct SubaccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for SubaccountMethods<'a, S> {}

impl<'a, S> SubaccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one subaccount by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Subaccount ID.
    pub fn get(&self, profile_id: i64, id: i64) -> SubaccountGetCall<'a, S> {
        SubaccountGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new subaccount.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: Subaccount, profile_id: i64) -> SubaccountInsertCall<'a, S> {
        SubaccountInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of subaccounts, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> SubaccountListCall<'a, S> {
        SubaccountListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subaccount. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Subaccount ID.
    pub fn patch(&self, request: Subaccount, profile_id: i64, id: i64) -> SubaccountPatchCall<'a, S> {
        SubaccountPatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing subaccount.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: Subaccount, profile_id: i64) -> SubaccountUpdateCall<'a, S> {
        SubaccountUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetableRemarketingList* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.targetable_remarketing_lists();
/// # }
/// ```
pub struct TargetableRemarketingListMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for TargetableRemarketingListMethods<'a, S> {}

impl<'a, S> TargetableRemarketingListMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one remarketing list by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Remarketing list ID.
    pub fn get(&self, profile_id: i64, id: i64) -> TargetableRemarketingListGetCall<'a, S> {
        TargetableRemarketingListGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of targetable remarketing lists, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `advertiserId` - Select only targetable remarketing lists targetable by these advertisers.
    pub fn list(&self, profile_id: i64, advertiser_id: i64) -> TargetableRemarketingListListCall<'a, S> {
        TargetableRemarketingListListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _advertiser_id: advertiser_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _page_token: Default::default(),
            _name: Default::default(),
            _max_results: Default::default(),
            _active: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *targetingTemplate* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.targeting_templates();
/// # }
/// ```
pub struct TargetingTemplateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for TargetingTemplateMethods<'a, S> {}

impl<'a, S> TargetingTemplateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one targeting template by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Targeting template ID.
    pub fn get(&self, profile_id: i64, id: i64) -> TargetingTemplateGetCall<'a, S> {
        TargetingTemplateGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new targeting template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: TargetingTemplate, profile_id: i64) -> TargetingTemplateInsertCall<'a, S> {
        TargetingTemplateInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of targeting templates, optionally filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> TargetingTemplateListCall<'a, S> {
        TargetingTemplateListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _advertiser_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing targeting template. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - TargetingTemplate ID.
    pub fn patch(&self, request: TargetingTemplate, profile_id: i64, id: i64) -> TargetingTemplatePatchCall<'a, S> {
        TargetingTemplatePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing targeting template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: TargetingTemplate, profile_id: i64) -> TargetingTemplateUpdateCall<'a, S> {
        TargetingTemplateUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userProfile* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.user_profiles();
/// # }
/// ```
pub struct UserProfileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for UserProfileMethods<'a, S> {}

impl<'a, S> UserProfileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one user profile by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - The user profile ID.
    pub fn get(&self, profile_id: i64) -> UserProfileGetCall<'a, S> {
        UserProfileGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves list of user profiles for a user.
    pub fn list(&self) -> UserProfileListCall<'a, S> {
        UserProfileListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userRolePermissionGroup* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.user_role_permission_groups();
/// # }
/// ```
pub struct UserRolePermissionGroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for UserRolePermissionGroupMethods<'a, S> {}

impl<'a, S> UserRolePermissionGroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one user role permission group by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - User role permission group ID.
    pub fn get(&self, profile_id: i64, id: i64) -> UserRolePermissionGroupGetCall<'a, S> {
        UserRolePermissionGroupGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of all supported user role permission groups.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> UserRolePermissionGroupListCall<'a, S> {
        UserRolePermissionGroupListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userRolePermission* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.user_role_permissions();
/// # }
/// ```
pub struct UserRolePermissionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for UserRolePermissionMethods<'a, S> {}

impl<'a, S> UserRolePermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one user role permission by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - User role permission ID.
    pub fn get(&self, profile_id: i64, id: i64) -> UserRolePermissionGetCall<'a, S> {
        UserRolePermissionGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a list of user role permissions, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> UserRolePermissionListCall<'a, S> {
        UserRolePermissionListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userRole* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.user_roles();
/// # }
/// ```
pub struct UserRoleMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for UserRoleMethods<'a, S> {}

impl<'a, S> UserRoleMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing user role.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - User role ID.
    pub fn delete(&self, profile_id: i64, id: i64) -> UserRoleDeleteCall<'a, S> {
        UserRoleDeleteCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one user role by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - User role ID.
    pub fn get(&self, profile_id: i64, id: i64) -> UserRoleGetCall<'a, S> {
        UserRoleGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new user role.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn insert(&self, request: UserRole, profile_id: i64) -> UserRoleInsertCall<'a, S> {
        UserRoleInsertCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of user roles, possibly filtered. This method supports paging.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> UserRoleListCall<'a, S> {
        UserRoleListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _subaccount_id: Default::default(),
            _sort_order: Default::default(),
            _sort_field: Default::default(),
            _search_string: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _ids: Default::default(),
            _account_user_role_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing user role. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - UserRole ID.
    pub fn patch(&self, request: UserRole, profile_id: i64, id: i64) -> UserRolePatchCall<'a, S> {
        UserRolePatchCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing user role.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `profileId` - User profile ID associated with this request.
    pub fn update(&self, request: UserRole, profile_id: i64) -> UserRoleUpdateCall<'a, S> {
        UserRoleUpdateCall {
            hub: self.hub,
            _request: request,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *videoFormat* resources.
/// It is not used directly, but through the [`Dfareporting`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d3 as dfareporting3d3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d3::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.video_formats();
/// # }
/// ```
pub struct VideoFormatMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Dfareporting<S>,
}

impl<'a, S> client::MethodsBuilder for VideoFormatMethods<'a, S> {}

impl<'a, S> VideoFormatMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one video format by ID.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    /// * `id` - Video format ID.
    pub fn get(&self, profile_id: i64, id: i32) -> VideoFormatGetCall<'a, S> {
        VideoFormatGetCall {
            hub: self.hub,
            _profile_id: profile_id,
            _id: id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists available video formats.
    /// 
    /// # Arguments
    ///
    /// * `profileId` - User profile ID associated with this request.
    pub fn list(&self, profile_id: i64) -> VideoFormatListCall<'a, S> {
        VideoFormatListCall {
            hub: self.hub,
            _profile_id: profile_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



