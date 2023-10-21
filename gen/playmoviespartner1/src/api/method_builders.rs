use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`PlayMovies`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playmoviespartner1 as playmoviespartner1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `avails_get(...)`, `avails_list(...)`, `orders_get(...)`, `orders_list(...)`, `store_infos_country_get(...)` and `store_infos_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Orders owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn orders_list(&self, account_id: &str) -> AccountOrderListCall<'a, S> {
        AccountOrderListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _studio_names: Default::default(),
            _status: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _custom_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Order given its id.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `orderId` - REQUIRED. Order ID.
    pub fn orders_get(&self, account_id: &str, order_id: &str) -> AccountOrderGetCall<'a, S> {
        AccountOrderGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List Avails owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn avails_list(&self, account_id: &str) -> AccountAvailListCall<'a, S> {
        AccountAvailListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _title: Default::default(),
            _territories: Default::default(),
            _studio_names: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alt_ids: Default::default(),
            _alt_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get an Avail given its avail group id and avail id.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `availId` - REQUIRED. Avail ID.
    pub fn avails_get(&self, account_id: &str, avail_id: &str) -> AccountAvailGetCall<'a, S> {
        AccountAvailGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _avail_id: avail_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a StoreInfo given its video id and country.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _Get methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    /// * `videoId` - REQUIRED. Video ID.
    /// * `country` - REQUIRED. Edit country.
    pub fn store_infos_country_get(&self, account_id: &str, video_id: &str, country: &str) -> AccountStoreInfoCountryGetCall<'a, S> {
        AccountStoreInfoCountryGetCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_id: video_id.to_string(),
            _country: country.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List StoreInfos owned or managed by the partner.
    /// 
    /// See _Authentication and Authorization rules_ and
    /// _List methods rules_ for more information about this method.
    /// 
    /// # Arguments
    ///
    /// * `accountId` - REQUIRED. See _General rules_ for more information about this field.
    pub fn store_infos_list(&self, account_id: &str) -> AccountStoreInfoListCall<'a, S> {
        AccountStoreInfoListCall {
            hub: self.hub,
            _account_id: account_id.to_string(),
            _video_ids: Default::default(),
            _video_id: Default::default(),
            _studio_names: Default::default(),
            _season_ids: Default::default(),
            _pph_names: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _mids: Default::default(),
            _countries: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



