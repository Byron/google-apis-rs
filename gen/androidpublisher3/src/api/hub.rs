use super::*;

/// Central instance to access all AndroidPublisher related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidpublisher3 as androidpublisher3;
/// use androidpublisher3::api::Subscription;
/// use androidpublisher3::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use androidpublisher3::{AndroidPublisher, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = AndroidPublisher::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Subscription::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.monetization().subscriptions_patch(req, "packageName", "productId")
///              .update_mask(&Default::default())
///              .regions_version_version("amet.")
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
pub struct AndroidPublisher<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for AndroidPublisher<S> {}

impl<'a, S> AndroidPublisher<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> AndroidPublisher<S> {
        AndroidPublisher {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://androidpublisher.googleapis.com/".to_string(),
            _root_url: "https://androidpublisher.googleapis.com/".to_string(),
        }
    }

    pub fn applications(&'a self) -> ApplicationMethods<'a, S> {
        ApplicationMethods { hub: &self }
    }
    pub fn edits(&'a self) -> EditMethods<'a, S> {
        EditMethods { hub: &self }
    }
    pub fn generatedapks(&'a self) -> GeneratedapkMethods<'a, S> {
        GeneratedapkMethods { hub: &self }
    }
    pub fn grants(&'a self) -> GrantMethods<'a, S> {
        GrantMethods { hub: &self }
    }
    pub fn inappproducts(&'a self) -> InappproductMethods<'a, S> {
        InappproductMethods { hub: &self }
    }
    pub fn internalappsharingartifacts(&'a self) -> InternalappsharingartifactMethods<'a, S> {
        InternalappsharingartifactMethods { hub: &self }
    }
    pub fn monetization(&'a self) -> MonetizationMethods<'a, S> {
        MonetizationMethods { hub: &self }
    }
    pub fn orders(&'a self) -> OrderMethods<'a, S> {
        OrderMethods { hub: &self }
    }
    pub fn purchases(&'a self) -> PurchaseMethods<'a, S> {
        PurchaseMethods { hub: &self }
    }
    pub fn reviews(&'a self) -> ReviewMethods<'a, S> {
        ReviewMethods { hub: &self }
    }
    pub fn systemapks(&'a self) -> SystemapkMethods<'a, S> {
        SystemapkMethods { hub: &self }
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
    /// It defaults to `https://androidpublisher.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://androidpublisher.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
