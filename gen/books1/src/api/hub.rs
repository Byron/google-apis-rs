use super::*;

/// Central instance to access all Books related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_books1 as books1;
/// use books1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use books1::{Books, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Books::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.promooffer().accept()
///              .volume_id("eos")
///              .serial("dolor")
///              .product("ea")
///              .offer_id("ipsum")
///              .model("invidunt")
///              .manufacturer("amet")
///              .device("duo")
///              .android_id("ipsum")
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
pub struct Books<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Books<S> {}

impl<'a, S> Books<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Books<S> {
        Books {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://books.googleapis.com/".to_string(),
            _root_url: "https://books.googleapis.com/".to_string(),
        }
    }

    pub fn bookshelves(&'a self) -> BookshelfMethods<'a, S> {
        BookshelfMethods { hub: &self }
    }
    pub fn cloudloading(&'a self) -> CloudloadingMethods<'a, S> {
        CloudloadingMethods { hub: &self }
    }
    pub fn dictionary(&'a self) -> DictionaryMethods<'a, S> {
        DictionaryMethods { hub: &self }
    }
    pub fn familysharing(&'a self) -> FamilysharingMethods<'a, S> {
        FamilysharingMethods { hub: &self }
    }
    pub fn layers(&'a self) -> LayerMethods<'a, S> {
        LayerMethods { hub: &self }
    }
    pub fn myconfig(&'a self) -> MyconfigMethods<'a, S> {
        MyconfigMethods { hub: &self }
    }
    pub fn mylibrary(&'a self) -> MylibraryMethods<'a, S> {
        MylibraryMethods { hub: &self }
    }
    pub fn notification(&'a self) -> NotificationMethods<'a, S> {
        NotificationMethods { hub: &self }
    }
    pub fn onboarding(&'a self) -> OnboardingMethods<'a, S> {
        OnboardingMethods { hub: &self }
    }
    pub fn personalizedstream(&'a self) -> PersonalizedstreamMethods<'a, S> {
        PersonalizedstreamMethods { hub: &self }
    }
    pub fn promooffer(&'a self) -> PromoofferMethods<'a, S> {
        PromoofferMethods { hub: &self }
    }
    pub fn series(&'a self) -> SeriesMethods<'a, S> {
        SeriesMethods { hub: &self }
    }
    pub fn volumes(&'a self) -> VolumeMethods<'a, S> {
        VolumeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://books.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://books.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
