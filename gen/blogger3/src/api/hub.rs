use super::*;

/// Central instance to access all Blogger related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// use blogger3::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.posts().list("blogId")
///              .view("gubergren")
///              .add_status("rebum.")
///              .start_date("est")
///              .sort_option("ipsum")
///              .page_token("ipsum")
///              .order_by("est")
///              .max_results(39)
///              .labels("ea")
///              .fetch_images(false)
///              .fetch_bodies(true)
///              .end_date("eos")
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
pub struct Blogger<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Blogger<S> {}

impl<'a, S> Blogger<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Blogger<S> {
        Blogger {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://blogger.googleapis.com/".to_string(),
            _root_url: "https://blogger.googleapis.com/".to_string(),
        }
    }

    pub fn blog_user_infos(&'a self) -> BlogUserInfoMethods<'a, S> {
        BlogUserInfoMethods { hub: &self }
    }
    pub fn blogs(&'a self) -> BlogMethods<'a, S> {
        BlogMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, S> {
        CommentMethods { hub: &self }
    }
    pub fn page_views(&'a self) -> PageViewMethods<'a, S> {
        PageViewMethods { hub: &self }
    }
    pub fn pages(&'a self) -> PageMethods<'a, S> {
        PageMethods { hub: &self }
    }
    pub fn post_user_infos(&'a self) -> PostUserInfoMethods<'a, S> {
        PostUserInfoMethods { hub: &self }
    }
    pub fn posts(&'a self) -> PostMethods<'a, S> {
        PostMethods { hub: &self }
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
    /// It defaults to `https://blogger.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://blogger.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
