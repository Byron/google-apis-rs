use super::*;

/// Central instance to access all DisplayVideo related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_displayvideo1 as displayvideo1;
/// use displayvideo1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use displayvideo1::{DisplayVideo, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = DisplayVideo::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.advertisers().campaigns_targeting_types_assigned_targeting_options_list(-55, -62, &Default::default())
///              .page_token("Lorem")
///              .page_size(-12)
///              .order_by("eos")
///              .filter("dolor")
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
pub struct DisplayVideo<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for DisplayVideo<S> {}

impl<'a, S> DisplayVideo<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> DisplayVideo<S> {
        DisplayVideo {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://displayvideo.googleapis.com/".to_string(),
            _root_url: "https://displayvideo.googleapis.com/".to_string(),
        }
    }

    pub fn advertisers(&'a self) -> AdvertiserMethods<'a, S> {
        AdvertiserMethods { hub: &self }
    }
    pub fn combined_audiences(&'a self) -> CombinedAudienceMethods<'a, S> {
        CombinedAudienceMethods { hub: &self }
    }
    pub fn custom_bidding_algorithms(&'a self) -> CustomBiddingAlgorithmMethods<'a, S> {
        CustomBiddingAlgorithmMethods { hub: &self }
    }
    pub fn custom_lists(&'a self) -> CustomListMethods<'a, S> {
        CustomListMethods { hub: &self }
    }
    pub fn first_and_third_party_audiences(&'a self) -> FirstAndThirdPartyAudienceMethods<'a, S> {
        FirstAndThirdPartyAudienceMethods { hub: &self }
    }
    pub fn floodlight_groups(&'a self) -> FloodlightGroupMethods<'a, S> {
        FloodlightGroupMethods { hub: &self }
    }
    pub fn google_audiences(&'a self) -> GoogleAudienceMethods<'a, S> {
        GoogleAudienceMethods { hub: &self }
    }
    pub fn guaranteed_orders(&'a self) -> GuaranteedOrderMethods<'a, S> {
        GuaranteedOrderMethods { hub: &self }
    }
    pub fn inventory_source_groups(&'a self) -> InventorySourceGroupMethods<'a, S> {
        InventorySourceGroupMethods { hub: &self }
    }
    pub fn inventory_sources(&'a self) -> InventorySourceMethods<'a, S> {
        InventorySourceMethods { hub: &self }
    }
    pub fn media(&'a self) -> MediaMethods<'a, S> {
        MediaMethods { hub: &self }
    }
    pub fn partners(&'a self) -> PartnerMethods<'a, S> {
        PartnerMethods { hub: &self }
    }
    pub fn sdfdownloadtasks(&'a self) -> SdfdownloadtaskMethods<'a, S> {
        SdfdownloadtaskMethods { hub: &self }
    }
    pub fn targeting_types(&'a self) -> TargetingTypeMethods<'a, S> {
        TargetingTypeMethods { hub: &self }
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
    /// It defaults to `https://displayvideo.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://displayvideo.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
