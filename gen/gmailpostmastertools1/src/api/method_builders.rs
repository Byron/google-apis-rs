use super::*;
/// A builder providing access to all methods supported on *domain* resources.
/// It is not used directly, but through the [`PostmasterTools`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gmailpostmastertools1 as gmailpostmastertools1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gmailpostmastertools1::{PostmasterTools, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PostmasterTools::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `traffic_stats_get(...)` and `traffic_stats_list(...)`
/// // to build up your call.
/// let rb = hub.domains();
/// # }
/// ```
pub struct DomainMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PostmasterTools<S>,
}

impl<'a, S> client::MethodsBuilder for DomainMethods<'a, S> {}

impl<'a, S> DomainMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get traffic statistics for a domain on a specific date. Returns PERMISSION_DENIED if user does not have permission to access TrafficStats for the domain.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the traffic statistics to get. E.g., domains/mymail.mydomain.com/trafficStats/20160807.
    pub fn traffic_stats_get(&self, name: &str) -> DomainTrafficStatGetCall<'a, S> {
        DomainTrafficStatGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List traffic statistics for all available days. Returns PERMISSION_DENIED if user does not have permission to access TrafficStats for the domain.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The resource name of the domain whose traffic statistics we'd like to list. It should have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name.
    pub fn traffic_stats_list(&self, parent: &str) -> DomainTrafficStatListCall<'a, S> {
        DomainTrafficStatListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _start_date_year: Default::default(),
            _start_date_month: Default::default(),
            _start_date_day: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _end_date_year: Default::default(),
            _end_date_month: Default::default(),
            _end_date_day: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a specific domain registered by the client. Returns NOT_FOUND if the domain does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the domain. It should have the form `domains/{domain_name}`, where domain_name is the fully qualified domain name.
    pub fn get(&self, name: &str) -> DomainGetCall<'a, S> {
        DomainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the domains that have been registered by the client. The order of domains in the response is unspecified and non-deterministic. Newly created domains will not necessarily be added to the end of this list.
    pub fn list(&self) -> DomainListCall<'a, S> {
        DomainListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



