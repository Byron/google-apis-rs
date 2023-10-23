use super::*;

/// Central instance to access all AdExchangeBuyer related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adexchangebuyer1d4 as adexchangebuyer1d4;
/// use adexchangebuyer1d4::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use adexchangebuyer1d4::{AdExchangeBuyer, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = AdExchangeBuyer::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.creatives().list()
///              .page_token("amet.")
///              .open_auction_status_filter(&Default::default())
///              .max_results(42)
///              .deals_status_filter(&Default::default())
///              .add_buyer_creative_id("amet.")
///              .add_account_id(-20)
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
pub struct AdExchangeBuyer<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for AdExchangeBuyer<S> {}

impl<'a, S> AdExchangeBuyer<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> AdExchangeBuyer<S> {
        AdExchangeBuyer {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://www.googleapis.com/adexchangebuyer/v1.4/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, S> {
        AccountMethods { hub: &self }
    }
    pub fn billing_info(&'a self) -> BillingInfoMethods<'a, S> {
        BillingInfoMethods { hub: &self }
    }
    pub fn budget(&'a self) -> BudgetMethods<'a, S> {
        BudgetMethods { hub: &self }
    }
    pub fn creatives(&'a self) -> CreativeMethods<'a, S> {
        CreativeMethods { hub: &self }
    }
    pub fn marketplacedeals(&'a self) -> MarketplacedealMethods<'a, S> {
        MarketplacedealMethods { hub: &self }
    }
    pub fn marketplacenotes(&'a self) -> MarketplacenoteMethods<'a, S> {
        MarketplacenoteMethods { hub: &self }
    }
    pub fn marketplaceprivateauction(&'a self) -> MarketplaceprivateauctionMethods<'a, S> {
        MarketplaceprivateauctionMethods { hub: &self }
    }
    pub fn performance_report(&'a self) -> PerformanceReportMethods<'a, S> {
        PerformanceReportMethods { hub: &self }
    }
    pub fn pretargeting_config(&'a self) -> PretargetingConfigMethods<'a, S> {
        PretargetingConfigMethods { hub: &self }
    }
    pub fn products(&'a self) -> ProductMethods<'a, S> {
        ProductMethods { hub: &self }
    }
    pub fn proposals(&'a self) -> ProposalMethods<'a, S> {
        ProposalMethods { hub: &self }
    }
    pub fn pubprofiles(&'a self) -> PubprofileMethods<'a, S> {
        PubprofileMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/adexchangebuyer/v1.4/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
