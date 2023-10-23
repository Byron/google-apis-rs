use super::*;

/// Central instance to access all Storage related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// use storage1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use storage1::{Storage, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("sed")
///              .source_generation(-70)
///              .rewrite_token("sed")
///              .projection(&Default::default())
///              .max_bytes_rewritten_per_call(-61)
///              .if_source_metageneration_not_match(-15)
///              .if_source_metageneration_match(-13)
///              .if_source_generation_not_match(-24)
///              .if_source_generation_match(-43)
///              .if_metageneration_not_match(-24)
///              .if_metageneration_match(-68)
///              .if_generation_not_match(-76)
///              .if_generation_match(-31)
///              .destination_predefined_acl(&Default::default())
///              .destination_kms_key_name("sed")
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
pub struct Storage<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for Storage<S> {}

impl<'a, S> Storage<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Storage<S> {
        Storage {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://storage.googleapis.com/storage/v1/".to_string(),
            _root_url: "https://storage.googleapis.com/".to_string(),
        }
    }

    pub fn bucket_access_controls(&'a self) -> BucketAccessControlMethods<'a, S> {
        BucketAccessControlMethods { hub: &self }
    }
    pub fn buckets(&'a self) -> BucketMethods<'a, S> {
        BucketMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, S> {
        ChannelMethods { hub: &self }
    }
    pub fn default_object_access_controls(&'a self) -> DefaultObjectAccessControlMethods<'a, S> {
        DefaultObjectAccessControlMethods { hub: &self }
    }
    pub fn notifications(&'a self) -> NotificationMethods<'a, S> {
        NotificationMethods { hub: &self }
    }
    pub fn object_access_controls(&'a self) -> ObjectAccessControlMethods<'a, S> {
        ObjectAccessControlMethods { hub: &self }
    }
    pub fn objects(&'a self) -> ObjectMethods<'a, S> {
        ObjectMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://storage.googleapis.com/storage/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://storage.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
