use super::*;

/// Central instance to access all CustomSearchAPI related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_customsearch1 as customsearch1;
/// use customsearch1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list()
///              .start(86)
///              .sort("dolor")
///              .site_search_filter("duo")
///              .site_search("vero")
///              .search_type("vero")
///              .safe("invidunt")
///              .rights("Stet")
///              .related_site("vero")
///              .q("elitr")
///              .or_terms("Lorem")
///              .num(-29)
///              .lr("no")
///              .low_range("ipsum")
///              .link_site("accusam")
///              .img_type("takimata")
///              .img_size("consetetur")
///              .img_dominant_color("voluptua.")
///              .img_color_type("et")
///              .hq("erat")
///              .hl("consetetur")
///              .high_range("amet.")
///              .googlehost("sed")
///              .gl("takimata")
///              .filter("dolores")
///              .file_type("gubergren")
///              .exclude_terms("et")
///              .exact_terms("accusam")
///              .date_restrict("voluptua.")
///              .cx("dolore")
///              .cr("dolore")
///              .c2coff("dolore")
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
pub struct CustomSearchAPI<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for CustomSearchAPI<S> {}

impl<'a, S> CustomSearchAPI<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> CustomSearchAPI<S> {
        CustomSearchAPI {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://customsearch.googleapis.com/".to_string(),
            _root_url: "https://customsearch.googleapis.com/".to_string(),
        }
    }

    pub fn cse(&'a self) -> CseMethods<'a, S> {
        CseMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
