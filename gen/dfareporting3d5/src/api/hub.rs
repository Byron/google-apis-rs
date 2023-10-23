use super::*;

/// Central instance to access all Dfareporting related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_dfareporting3d5 as dfareporting3d5;
/// use dfareporting3d5::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use dfareporting3d5::{Dfareporting, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Dfareporting::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reports().files_list(-2, -59)
///              .sort_order(&Default::default())
///              .sort_field(&Default::default())
///              .page_token("amet.")
///              .max_results(-20)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Dfareporting<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Dfareporting<S> {}

impl<'a, S> Dfareporting<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Dfareporting<S> {
        Dfareporting {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://dfareporting.googleapis.com/dfareporting/v3.5/".to_string(),
            _root_url: "https://dfareporting.googleapis.com/".to_string(),
        }
    }

    pub fn account_active_ad_summaries(&'a self) -> AccountActiveAdSummaryMethods<'a, S> {
        AccountActiveAdSummaryMethods { hub: &self }
    }
    pub fn account_permission_groups(&'a self) -> AccountPermissionGroupMethods<'a, S> {
        AccountPermissionGroupMethods { hub: &self }
    }
    pub fn account_permissions(&'a self) -> AccountPermissionMethods<'a, S> {
        AccountPermissionMethods { hub: &self }
    }
    pub fn account_user_profiles(&'a self) -> AccountUserProfileMethods<'a, S> {
        AccountUserProfileMethods { hub: &self }
    }
    pub fn accounts(&'a self) -> AccountMethods<'a, S> {
        AccountMethods { hub: &self }
    }
    pub fn ads(&'a self) -> AdMethods<'a, S> {
        AdMethods { hub: &self }
    }
    pub fn advertiser_groups(&'a self) -> AdvertiserGroupMethods<'a, S> {
        AdvertiserGroupMethods { hub: &self }
    }
    pub fn advertiser_landing_pages(&'a self) -> AdvertiserLandingPageMethods<'a, S> {
        AdvertiserLandingPageMethods { hub: &self }
    }
    pub fn advertisers(&'a self) -> AdvertiserMethods<'a, S> {
        AdvertiserMethods { hub: &self }
    }
    pub fn browsers(&'a self) -> BrowserMethods<'a, S> {
        BrowserMethods { hub: &self }
    }
    pub fn campaign_creative_associations(&'a self) -> CampaignCreativeAssociationMethods<'a, S> {
        CampaignCreativeAssociationMethods { hub: &self }
    }
    pub fn campaigns(&'a self) -> CampaignMethods<'a, S> {
        CampaignMethods { hub: &self }
    }
    pub fn change_logs(&'a self) -> ChangeLogMethods<'a, S> {
        ChangeLogMethods { hub: &self }
    }
    pub fn cities(&'a self) -> CityMethods<'a, S> {
        CityMethods { hub: &self }
    }
    pub fn connection_types(&'a self) -> ConnectionTypeMethods<'a, S> {
        ConnectionTypeMethods { hub: &self }
    }
    pub fn content_categories(&'a self) -> ContentCategoryMethods<'a, S> {
        ContentCategoryMethods { hub: &self }
    }
    pub fn conversions(&'a self) -> ConversionMethods<'a, S> {
        ConversionMethods { hub: &self }
    }
    pub fn countries(&'a self) -> CountryMethods<'a, S> {
        CountryMethods { hub: &self }
    }
    pub fn creative_assets(&'a self) -> CreativeAssetMethods<'a, S> {
        CreativeAssetMethods { hub: &self }
    }
    pub fn creative_field_values(&'a self) -> CreativeFieldValueMethods<'a, S> {
        CreativeFieldValueMethods { hub: &self }
    }
    pub fn creative_fields(&'a self) -> CreativeFieldMethods<'a, S> {
        CreativeFieldMethods { hub: &self }
    }
    pub fn creative_groups(&'a self) -> CreativeGroupMethods<'a, S> {
        CreativeGroupMethods { hub: &self }
    }
    pub fn creatives(&'a self) -> CreativeMethods<'a, S> {
        CreativeMethods { hub: &self }
    }
    pub fn dimension_values(&'a self) -> DimensionValueMethods<'a, S> {
        DimensionValueMethods { hub: &self }
    }
    pub fn directory_sites(&'a self) -> DirectorySiteMethods<'a, S> {
        DirectorySiteMethods { hub: &self }
    }
    pub fn dynamic_targeting_keys(&'a self) -> DynamicTargetingKeyMethods<'a, S> {
        DynamicTargetingKeyMethods { hub: &self }
    }
    pub fn event_tags(&'a self) -> EventTagMethods<'a, S> {
        EventTagMethods { hub: &self }
    }
    pub fn files(&'a self) -> FileMethods<'a, S> {
        FileMethods { hub: &self }
    }
    pub fn floodlight_activities(&'a self) -> FloodlightActivityMethods<'a, S> {
        FloodlightActivityMethods { hub: &self }
    }
    pub fn floodlight_activity_groups(&'a self) -> FloodlightActivityGroupMethods<'a, S> {
        FloodlightActivityGroupMethods { hub: &self }
    }
    pub fn floodlight_configurations(&'a self) -> FloodlightConfigurationMethods<'a, S> {
        FloodlightConfigurationMethods { hub: &self }
    }
    pub fn inventory_items(&'a self) -> InventoryItemMethods<'a, S> {
        InventoryItemMethods { hub: &self }
    }
    pub fn languages(&'a self) -> LanguageMethods<'a, S> {
        LanguageMethods { hub: &self }
    }
    pub fn metros(&'a self) -> MetroMethods<'a, S> {
        MetroMethods { hub: &self }
    }
    pub fn mobile_apps(&'a self) -> MobileAppMethods<'a, S> {
        MobileAppMethods { hub: &self }
    }
    pub fn mobile_carriers(&'a self) -> MobileCarrierMethods<'a, S> {
        MobileCarrierMethods { hub: &self }
    }
    pub fn operating_system_versions(&'a self) -> OperatingSystemVersionMethods<'a, S> {
        OperatingSystemVersionMethods { hub: &self }
    }
    pub fn operating_systems(&'a self) -> OperatingSystemMethods<'a, S> {
        OperatingSystemMethods { hub: &self }
    }
    pub fn order_documents(&'a self) -> OrderDocumentMethods<'a, S> {
        OrderDocumentMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, S> {
        OrderMethods { hub: &self }
    }
    pub fn placement_groups(&'a self) -> PlacementGroupMethods<'a, S> {
        PlacementGroupMethods { hub: &self }
    }
    pub fn placement_strategies(&'a self) -> PlacementStrategyMethods<'a, S> {
        PlacementStrategyMethods { hub: &self }
    }
    pub fn placements(&'a self) -> PlacementMethods<'a, S> {
        PlacementMethods { hub: &self }
    }
    pub fn platform_types(&'a self) -> PlatformTypeMethods<'a, S> {
        PlatformTypeMethods { hub: &self }
    }
    pub fn postal_codes(&'a self) -> PostalCodeMethods<'a, S> {
        PostalCodeMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }
    pub fn regions(&'a self) -> RegionMethods<'a, S> {
        RegionMethods { hub: &self }
    }
    pub fn remarketing_list_shares(&'a self) -> RemarketingListShareMethods<'a, S> {
        RemarketingListShareMethods { hub: &self }
    }
    pub fn remarketing_lists(&'a self) -> RemarketingListMethods<'a, S> {
        RemarketingListMethods { hub: &self }
    }
    pub fn reports(&'a self) -> ReportMethods<'a, S> {
        ReportMethods { hub: &self }
    }
    pub fn sites(&'a self) -> SiteMethods<'a, S> {
        SiteMethods { hub: &self }
    }
    pub fn sizes(&'a self) -> SizeMethods<'a, S> {
        SizeMethods { hub: &self }
    }
    pub fn subaccounts(&'a self) -> SubaccountMethods<'a, S> {
        SubaccountMethods { hub: &self }
    }
    pub fn targetable_remarketing_lists(&'a self) -> TargetableRemarketingListMethods<'a, S> {
        TargetableRemarketingListMethods { hub: &self }
    }
    pub fn targeting_templates(&'a self) -> TargetingTemplateMethods<'a, S> {
        TargetingTemplateMethods { hub: &self }
    }
    pub fn user_profiles(&'a self) -> UserProfileMethods<'a, S> {
        UserProfileMethods { hub: &self }
    }
    pub fn user_role_permission_groups(&'a self) -> UserRolePermissionGroupMethods<'a, S> {
        UserRolePermissionGroupMethods { hub: &self }
    }
    pub fn user_role_permissions(&'a self) -> UserRolePermissionMethods<'a, S> {
        UserRolePermissionMethods { hub: &self }
    }
    pub fn user_roles(&'a self) -> UserRoleMethods<'a, S> {
        UserRoleMethods { hub: &self }
    }
    pub fn video_formats(&'a self) -> VideoFormatMethods<'a, S> {
        VideoFormatMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://dfareporting.googleapis.com/dfareporting/v3.5/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://dfareporting.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
