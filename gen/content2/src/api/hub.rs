use super::*;

/// Central instance to access all ShoppingContent related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2 as content2;
/// use content2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use content2::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.orders().list(89)
///              .add_statuses(&Default::default())
///              .placed_date_start("eos")
///              .placed_date_end("dolor")
///              .page_token("ea")
///              .order_by("ipsum")
///              .max_results(13)
///              .acknowledged(true)
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
pub struct ShoppingContent<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for ShoppingContent<S> {}

impl<'a, S> ShoppingContent<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> ShoppingContent<S> {
        ShoppingContent {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://shoppingcontent.googleapis.com/content/v2/".to_string(),
            _root_url: "https://shoppingcontent.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, S> {
        AccountMethods { hub: &self }
    }
    pub fn accountstatuses(&'a self) -> AccountstatusMethods<'a, S> {
        AccountstatusMethods { hub: &self }
    }
    pub fn accounttax(&'a self) -> AccounttaxMethods<'a, S> {
        AccounttaxMethods { hub: &self }
    }
    pub fn datafeeds(&'a self) -> DatafeedMethods<'a, S> {
        DatafeedMethods { hub: &self }
    }
    pub fn datafeedstatuses(&'a self) -> DatafeedstatusMethods<'a, S> {
        DatafeedstatusMethods { hub: &self }
    }
    pub fn inventory(&'a self) -> InventoryMethods<'a, S> {
        InventoryMethods { hub: &self }
    }
    pub fn liasettings(&'a self) -> LiasettingMethods<'a, S> {
        LiasettingMethods { hub: &self }
    }
    pub fn orderinvoices(&'a self) -> OrderinvoiceMethods<'a, S> {
        OrderinvoiceMethods { hub: &self }
    }
    pub fn orderreports(&'a self) -> OrderreportMethods<'a, S> {
        OrderreportMethods { hub: &self }
    }
    pub fn orderreturns(&'a self) -> OrderreturnMethods<'a, S> {
        OrderreturnMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, S> {
        OrderMethods { hub: &self }
    }
    pub fn pos(&'a self) -> PoMethods<'a, S> {
        PoMethods { hub: &self }
    }
    pub fn products(&'a self) -> ProductMethods<'a, S> {
        ProductMethods { hub: &self }
    }
    pub fn productstatuses(&'a self) -> ProductstatusMethods<'a, S> {
        ProductstatusMethods { hub: &self }
    }
    pub fn shippingsettings(&'a self) -> ShippingsettingMethods<'a, S> {
        ShippingsettingMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://shoppingcontent.googleapis.com/content/v2/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://shoppingcontent.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
