use super::*;

/// Central instance to access all YouTube related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// use youtube3::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().list(&vec!["duo".into()])
///              .video_category_id("ipsum")
///              .region_code("sed")
///              .page_token("ut")
///              .on_behalf_of_content_owner("gubergren")
///              .my_rating(&Default::default())
///              .max_width(-16)
///              .max_results(44)
///              .max_height(-50)
///              .locale("ipsum")
///              .add_id("est")
///              .hl("gubergren")
///              .chart(&Default::default())
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
pub struct YouTube<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, S> client::Hub for YouTube<S> {}

impl<'a, S> YouTube<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> YouTube<S> {
        YouTube {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://youtube.googleapis.com/".to_string(),
            _root_url: "https://youtube.googleapis.com/".to_string(),
        }
    }

    pub fn abuse_reports(&'a self) -> AbuseReportMethods<'a, S> {
        AbuseReportMethods { hub: &self }
    }
    pub fn activities(&'a self) -> ActivityMethods<'a, S> {
        ActivityMethods { hub: &self }
    }
    pub fn captions(&'a self) -> CaptionMethods<'a, S> {
        CaptionMethods { hub: &self }
    }
    pub fn channel_banners(&'a self) -> ChannelBannerMethods<'a, S> {
        ChannelBannerMethods { hub: &self }
    }
    pub fn channel_sections(&'a self) -> ChannelSectionMethods<'a, S> {
        ChannelSectionMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, S> {
        ChannelMethods { hub: &self }
    }
    pub fn comment_threads(&'a self) -> CommentThreadMethods<'a, S> {
        CommentThreadMethods { hub: &self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, S> {
        CommentMethods { hub: &self }
    }
    pub fn i18n_languages(&'a self) -> I18nLanguageMethods<'a, S> {
        I18nLanguageMethods { hub: &self }
    }
    pub fn i18n_regions(&'a self) -> I18nRegionMethods<'a, S> {
        I18nRegionMethods { hub: &self }
    }
    pub fn live_broadcasts(&'a self) -> LiveBroadcastMethods<'a, S> {
        LiveBroadcastMethods { hub: &self }
    }
    pub fn live_chat_bans(&'a self) -> LiveChatBanMethods<'a, S> {
        LiveChatBanMethods { hub: &self }
    }
    pub fn live_chat_messages(&'a self) -> LiveChatMessageMethods<'a, S> {
        LiveChatMessageMethods { hub: &self }
    }
    pub fn live_chat_moderators(&'a self) -> LiveChatModeratorMethods<'a, S> {
        LiveChatModeratorMethods { hub: &self }
    }
    pub fn live_streams(&'a self) -> LiveStreamMethods<'a, S> {
        LiveStreamMethods { hub: &self }
    }
    pub fn members(&'a self) -> MemberMethods<'a, S> {
        MemberMethods { hub: &self }
    }
    pub fn memberships_levels(&'a self) -> MembershipsLevelMethods<'a, S> {
        MembershipsLevelMethods { hub: &self }
    }
    pub fn playlist_items(&'a self) -> PlaylistItemMethods<'a, S> {
        PlaylistItemMethods { hub: &self }
    }
    pub fn playlists(&'a self) -> PlaylistMethods<'a, S> {
        PlaylistMethods { hub: &self }
    }
    pub fn search(&'a self) -> SearchMethods<'a, S> {
        SearchMethods { hub: &self }
    }
    pub fn subscriptions(&'a self) -> SubscriptionMethods<'a, S> {
        SubscriptionMethods { hub: &self }
    }
    pub fn super_chat_events(&'a self) -> SuperChatEventMethods<'a, S> {
        SuperChatEventMethods { hub: &self }
    }
    pub fn tests(&'a self) -> TestMethods<'a, S> {
        TestMethods { hub: &self }
    }
    pub fn third_party_links(&'a self) -> ThirdPartyLinkMethods<'a, S> {
        ThirdPartyLinkMethods { hub: &self }
    }
    pub fn thumbnails(&'a self) -> ThumbnailMethods<'a, S> {
        ThumbnailMethods { hub: &self }
    }
    pub fn video_abuse_report_reasons(&'a self) -> VideoAbuseReportReasonMethods<'a, S> {
        VideoAbuseReportReasonMethods { hub: &self }
    }
    pub fn video_categories(&'a self) -> VideoCategoryMethods<'a, S> {
        VideoCategoryMethods { hub: &self }
    }
    pub fn videos(&'a self) -> VideoMethods<'a, S> {
        VideoMethods { hub: &self }
    }
    pub fn watermarks(&'a self) -> WatermarkMethods<'a, S> {
        WatermarkMethods { hub: &self }
    }
    pub fn youtube(&'a self) -> YoutubeMethods<'a, S> {
        YoutubeMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://youtube.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://youtube.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}
