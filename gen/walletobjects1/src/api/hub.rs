use super::*;

/// Central instance to access all Walletobjects related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// use walletobjects1::api::AddMessageRequest;
/// use walletobjects1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = AddMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.eventticketclass().addmessage(req, "resourceId")
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
pub struct Walletobjects<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Walletobjects<S> {}

impl<'a, S> Walletobjects<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Walletobjects<S> {
        Walletobjects {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://walletobjects.googleapis.com/".to_string(),
            _root_url: "https://walletobjects.googleapis.com/".to_string(),
        }
    }

    pub fn eventticketclass(&'a self) -> EventticketclasMethods<'a, S> {
        EventticketclasMethods { hub: &self }
    }
    pub fn eventticketobject(&'a self) -> EventticketobjectMethods<'a, S> {
        EventticketobjectMethods { hub: &self }
    }
    pub fn flightclass(&'a self) -> FlightclasMethods<'a, S> {
        FlightclasMethods { hub: &self }
    }
    pub fn flightobject(&'a self) -> FlightobjectMethods<'a, S> {
        FlightobjectMethods { hub: &self }
    }
    pub fn genericclass(&'a self) -> GenericclasMethods<'a, S> {
        GenericclasMethods { hub: &self }
    }
    pub fn genericobject(&'a self) -> GenericobjectMethods<'a, S> {
        GenericobjectMethods { hub: &self }
    }
    pub fn giftcardclass(&'a self) -> GiftcardclasMethods<'a, S> {
        GiftcardclasMethods { hub: &self }
    }
    pub fn giftcardobject(&'a self) -> GiftcardobjectMethods<'a, S> {
        GiftcardobjectMethods { hub: &self }
    }
    pub fn issuer(&'a self) -> IssuerMethods<'a, S> {
        IssuerMethods { hub: &self }
    }
    pub fn jwt(&'a self) -> JwtMethods<'a, S> {
        JwtMethods { hub: &self }
    }
    pub fn loyaltyclass(&'a self) -> LoyaltyclasMethods<'a, S> {
        LoyaltyclasMethods { hub: &self }
    }
    pub fn loyaltyobject(&'a self) -> LoyaltyobjectMethods<'a, S> {
        LoyaltyobjectMethods { hub: &self }
    }
    pub fn media(&'a self) -> MediaMethods<'a, S> {
        MediaMethods { hub: &self }
    }
    pub fn offerclass(&'a self) -> OfferclasMethods<'a, S> {
        OfferclasMethods { hub: &self }
    }
    pub fn offerobject(&'a self) -> OfferobjectMethods<'a, S> {
        OfferobjectMethods { hub: &self }
    }
    pub fn permissions(&'a self) -> PermissionMethods<'a, S> {
        PermissionMethods { hub: &self }
    }
    pub fn smarttap(&'a self) -> SmarttapMethods<'a, S> {
        SmarttapMethods { hub: &self }
    }
    pub fn transitclass(&'a self) -> TransitclasMethods<'a, S> {
        TransitclasMethods { hub: &self }
    }
    pub fn transitobject(&'a self) -> TransitobjectMethods<'a, S> {
        TransitobjectMethods { hub: &self }
    }
    pub fn walletobjects(&'a self) -> WalletobjectMethods<'a, S> {
        WalletobjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://walletobjects.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://walletobjects.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
