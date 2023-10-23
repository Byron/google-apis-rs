use super::*;

/// Central instance to access all DriveHub related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_drive2 as drive2;
/// use drive2::api::File;
/// use drive2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use drive2::{DriveHub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = DriveHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = File::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.files().patch(req, "fileId")
///              .use_content_as_indexable_text(false)
///              .update_viewed_date(false)
///              .timed_text_track_name("diam")
///              .timed_text_language("dolor")
///              .supports_team_drives(false)
///              .supports_all_drives(false)
///              .set_modified_date(false)
///              .remove_parents("vero")
///              .pinned(false)
///              .ocr_language("Stet")
///              .ocr(false)
///              .new_revision(true)
///              .modified_date_behavior(&Default::default())
///              .include_permissions_for_view("Lorem")
///              .include_labels("diam")
///              .enforce_single_parent(true)
///              .convert(false)
///              .add_parents("accusam")
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
pub struct DriveHub<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for DriveHub<S> {}

impl<'a, S> DriveHub<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> DriveHub<S> {
        DriveHub {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://www.googleapis.com/drive/v2/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn about(&'a self) -> AboutMethods<'a, S> {
        AboutMethods { hub: &self }
    }
    pub fn apps(&'a self) -> AppMethods<'a, S> {
        AppMethods { hub: &self }
    }
    pub fn changes(&'a self) -> ChangeMethods<'a, S> {
        ChangeMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, S> {
        ChannelMethods { hub: &self }
    }
    pub fn children(&'a self) -> ChildMethods<'a, S> {
        ChildMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, S> {
        CommentMethods { hub: &self }
    }
    pub fn drives(&'a self) -> DriveMethods<'a, S> {
        DriveMethods { hub: &self }
    }
    pub fn files(&'a self) -> FileMethods<'a, S> {
        FileMethods { hub: &self }
    }
    pub fn parents(&'a self) -> ParentMethods<'a, S> {
        ParentMethods { hub: &self }
    }
    pub fn permissions(&'a self) -> PermissionMethods<'a, S> {
        PermissionMethods { hub: &self }
    }
    pub fn properties(&'a self) -> PropertyMethods<'a, S> {
        PropertyMethods { hub: &self }
    }
    pub fn replies(&'a self) -> ReplyMethods<'a, S> {
        ReplyMethods { hub: &self }
    }
    pub fn revisions(&'a self) -> RevisionMethods<'a, S> {
        RevisionMethods { hub: &self }
    }
    pub fn teamdrives(&'a self) -> TeamdriveMethods<'a, S> {
        TeamdriveMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/drive/v2/`.
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
