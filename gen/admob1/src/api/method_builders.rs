use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`AdMob`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_admob1 as admob1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use admob1::{AdMob, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AdMob::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `ad_units_list(...)`, `apps_list(...)`, `get(...)`, `list(...)`, `mediation_report_generate(...)` and `network_report_generate(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AdMob<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the ad units under the specified AdMob account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the account to list ad units for. Example: accounts/pub-9876543210987654
    pub fn ad_units_list(&self, parent: &str) -> AccountAdUnitListCall<'a, S> {
        AccountAdUnitListCall {
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
    /// List the apps under the specified AdMob account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the account to list apps for. Example: accounts/pub-9876543210987654
    pub fn apps_list(&self, parent: &str) -> AccountAppListCall<'a, S> {
        AccountAppListCall {
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
    /// Generates an AdMob mediation report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
    pub fn mediation_report_generate(&self, request: GenerateMediationReportRequest, parent: &str) -> AccountMediationReportGenerateCall<'a, S> {
        AccountMediationReportGenerateCall {
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
    /// Generates an AdMob Network report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
    pub fn network_report_generate(&self, request: GenerateNetworkReportRequest, parent: &str) -> AccountNetworkReportGenerateCall<'a, S> {
        AccountNetworkReportGenerateCall {
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
    /// Gets information about the specified AdMob publisher account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Resource name of the publisher account to retrieve. Example: accounts/pub-9876543210987654
    pub fn get(&self, name: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the AdMob publisher account that was most recently signed in to from the AdMob UI. For more information, see https://support.google.com/admob/answer/10243672.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



