use super::*;

/// Central instance to access all Partners related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_partners2 as partners2;
/// use partners2::api::CompanyRelation;
/// use partners2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CompanyRelation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().create_company_relation(req, "userId")
///              .request_metadata_user_overrides_user_id("dolor")
///              .request_metadata_user_overrides_ip_address("ea")
///              .request_metadata_traffic_source_traffic_sub_id("ipsum")
///              .request_metadata_traffic_source_traffic_source_id("invidunt")
///              .request_metadata_partners_session_id("amet")
///              .request_metadata_locale("duo")
///              .add_request_metadata_experiment_ids("ipsum")
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
pub struct Partners<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Partners<S> {}

impl<'a, S> Partners<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Partners<S> {
        Partners {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://partners.googleapis.com/".to_string(),
            _root_url: "https://partners.googleapis.com/".to_string(),
        }
    }

    pub fn analytics(&'a self) -> AnalyticMethods<'a, S> {
        AnalyticMethods { hub: &self }
    }
    pub fn client_messages(&'a self) -> ClientMessageMethods<'a, S> {
        ClientMessageMethods { hub: &self }
    }
    pub fn companies(&'a self) -> CompanyMethods<'a, S> {
        CompanyMethods { hub: &self }
    }
    pub fn leads(&'a self) -> LeadMethods<'a, S> {
        LeadMethods { hub: &self }
    }
    pub fn methods(&'a self) -> MethodMethods<'a, S> {
        MethodMethods { hub: &self }
    }
    pub fn offers(&'a self) -> OfferMethods<'a, S> {
        OfferMethods { hub: &self }
    }
    pub fn user_events(&'a self) -> UserEventMethods<'a, S> {
        UserEventMethods { hub: &self }
    }
    pub fn user_states(&'a self) -> UserStateMethods<'a, S> {
        UserStateMethods { hub: &self }
    }
    pub fn users(&'a self) -> UserMethods<'a, S> {
        UserMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://partners.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://partners.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
