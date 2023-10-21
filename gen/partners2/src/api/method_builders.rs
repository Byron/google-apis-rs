use super::*;
/// A builder providing access to all methods supported on *userEvent* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.user_events();
/// # }
/// ```
pub struct UserEventMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for UserEventMethods<'a, S> {}

impl<'a, S> UserEventMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a user event.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogUserEventRequest) -> UserEventLogCall<'a, S> {
        UserEventLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *clientMessage* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `log(...)`
/// // to build up your call.
/// let rb = hub.client_messages();
/// # }
/// ```
pub struct ClientMessageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for ClientMessageMethods<'a, S> {}

impl<'a, S> ClientMessageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Logs a generic message from the client, such as
    /// `Failed to render component`, `Profile page is running slow`,
    /// `More than 500 users have accessed this result.`, etc.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn log(&self, request: LogMessageRequest) -> ClientMessageLogCall<'a, S> {
        ClientMessageLogCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *lead* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.leads();
/// # }
/// ```
pub struct LeadMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for LeadMethods<'a, S> {}

impl<'a, S> LeadMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists advertiser leads for a user's associated company.
    /// Should only be called within the context of an authorized logged in user.
    pub fn list(&self) -> LeadListCall<'a, S> {
        LeadListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *offer* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `history_list(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.offers();
/// # }
/// ```
pub struct OfferMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for OfferMethods<'a, S> {}

impl<'a, S> OfferMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Historical Offers for the current user (or user's entire company)
    pub fn history_list(&self) -> OfferHistoryListCall<'a, S> {
        OfferHistoryListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _entire_company: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the Offers available for the current user
    pub fn list(&self) -> OfferListCall<'a, S> {
        OfferListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *analytic* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.analytics();
/// # }
/// ```
pub struct AnalyticMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for AnalyticMethods<'a, S> {}

impl<'a, S> AnalyticMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists analytics data for a user's associated company.
    /// Should only be called within the context of an authorized logged in user.
    pub fn list(&self) -> AnalyticListCall<'a, S> {
        AnalyticListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *userState* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.user_states();
/// # }
/// ```
pub struct UserStateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for UserStateMethods<'a, S> {}

impl<'a, S> UserStateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists states for current user.
    pub fn list(&self) -> UserStateListCall<'a, S> {
        UserStateListCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_partnersstatus(...)`, `update_companies(...)` and `update_leads(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified lead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_leads(&self, request: Lead) -> MethodUpdateLeadCall<'a, S> {
        MethodUpdateLeadCall {
            hub: self.hub,
            _request: request,
            _update_mask: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update company.
    /// Should only be called within the context of an authorized logged in user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_companies(&self, request: Company) -> MethodUpdateCompanyCall<'a, S> {
        MethodUpdateCompanyCall {
            hub: self.hub,
            _request: request,
            _update_mask: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets Partners Status of the logged in user's agency.
    /// Should only be called if the logged in user is the admin of the agency.
    pub fn get_partnersstatus(&self) -> MethodGetPartnersstatuCall<'a, S> {
        MethodGetPartnersstatuCall {
            hub: self.hub,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *company* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `leads_create(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.companies();
/// # }
/// ```
pub struct CompanyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for CompanyMethods<'a, S> {}

impl<'a, S> CompanyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an advertiser lead for the given company ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `companyId` - The ID of the company to contact.
    pub fn leads_create(&self, request: CreateLeadRequest, company_id: &str) -> CompanyLeadCreateCall<'a, S> {
        CompanyLeadCreateCall {
            hub: self.hub,
            _request: request,
            _company_id: company_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a company.
    /// 
    /// # Arguments
    ///
    /// * `companyId` - The ID of the company to retrieve.
    pub fn get(&self, company_id: &str) -> CompanyGetCall<'a, S> {
        CompanyGetCall {
            hub: self.hub,
            _company_id: company_id.to_string(),
            _view: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _order_by: Default::default(),
            _currency_code: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists companies.
    pub fn list(&self) -> CompanyListCall<'a, S> {
        CompanyListCall {
            hub: self.hub,
            _website_url: Default::default(),
            _view: Default::default(),
            _specializations: Default::default(),
            _services: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _min_monthly_budget_units: Default::default(),
            _min_monthly_budget_nanos: Default::default(),
            _min_monthly_budget_currency_code: Default::default(),
            _max_monthly_budget_units: Default::default(),
            _max_monthly_budget_nanos: Default::default(),
            _max_monthly_budget_currency_code: Default::default(),
            _language_codes: Default::default(),
            _industries: Default::default(),
            _gps_motivations: Default::default(),
            _company_name: Default::default(),
            _address: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`Partners`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create_company_relation(...)`, `delete_company_relation(...)`, `get(...)` and `update_profile(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a user's profile. A user can only update their own profile and
    /// should only be called within the context of a logged in user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_profile(&self, request: UserProfile) -> UserUpdateProfileCall<'a, S> {
        UserUpdateProfileCall {
            hub: self.hub,
            _request: request,
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a user's company relation. Affiliates the user to a company.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The ID of the user. Can be set to <code>me</code> to mean
    ///              the currently authenticated user.
    pub fn create_company_relation(&self, request: CompanyRelation, user_id: &str) -> UserCreateCompanyRelationCall<'a, S> {
        UserCreateCompanyRelationCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a user's company relation. Unaffiliaites the user from a company.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The ID of the user. Can be set to <code>me</code> to mean
    ///              the currently authenticated user.
    pub fn delete_company_relation(&self, user_id: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        UserDeleteCompanyRelationCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - Identifier of the user. Can be set to <code>me</code> to mean the currently
    ///              authenticated user.
    pub fn get(&self, user_id: &str) -> UserGetCall<'a, S> {
        UserGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _user_view: Default::default(),
            _request_metadata_user_overrides_user_id: Default::default(),
            _request_metadata_user_overrides_ip_address: Default::default(),
            _request_metadata_traffic_source_traffic_sub_id: Default::default(),
            _request_metadata_traffic_source_traffic_source_id: Default::default(),
            _request_metadata_partners_session_id: Default::default(),
            _request_metadata_locale: Default::default(),
            _request_metadata_experiment_ids: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



