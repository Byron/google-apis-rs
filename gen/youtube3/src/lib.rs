// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *YouTube* crate version *0.1.1+20150309*, where *20150309* is the exact revision of the *youtube:v3* schema built by the [mako](http://www.makotemplates.org/) code generator *v0.1.1*.
//! 
//! Everything else about the *YouTube* *v3* API can be found at the
//! [official documentation site](https://developers.google.com/youtube/v3).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/youtube3).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.YouTube.html) ... 
//! 
//! * [activities](struct.Activity.html)
//!  * [*insert*](struct.ActivityInsertCall.html) and [*list*](struct.ActivityListCall.html)
//! * channel banners
//!  * [*insert*](struct.ChannelBannerInsertCall.html)
//! * [channel sections](struct.ChannelSection.html)
//!  * [*delete*](struct.ChannelSectionDeleteCall.html), [*insert*](struct.ChannelSectionInsertCall.html), [*list*](struct.ChannelSectionListCall.html) and [*update*](struct.ChannelSectionUpdateCall.html)
//! * [channels](struct.Channel.html)
//!  * [*list*](struct.ChannelListCall.html) and [*update*](struct.ChannelUpdateCall.html)
//! * [guide categories](struct.GuideCategory.html)
//!  * [*list*](struct.GuideCategoryListCall.html)
//! * [i18n languages](struct.I18nLanguage.html)
//!  * [*list*](struct.I18nLanguageListCall.html)
//! * [i18n regions](struct.I18nRegion.html)
//!  * [*list*](struct.I18nRegionListCall.html)
//! * [live broadcasts](struct.LiveBroadcast.html)
//!  * [*bind*](struct.LiveBroadcastBindCall.html), [*control*](struct.LiveBroadcastControlCall.html), [*delete*](struct.LiveBroadcastDeleteCall.html), [*insert*](struct.LiveBroadcastInsertCall.html), [*list*](struct.LiveBroadcastListCall.html), [*transition*](struct.LiveBroadcastTransitionCall.html) and [*update*](struct.LiveBroadcastUpdateCall.html)
//! * [live streams](struct.LiveStream.html)
//!  * [*delete*](struct.LiveStreamDeleteCall.html), [*insert*](struct.LiveStreamInsertCall.html), [*list*](struct.LiveStreamListCall.html) and [*update*](struct.LiveStreamUpdateCall.html)
//! * [playlist items](struct.PlaylistItem.html)
//!  * [*delete*](struct.PlaylistItemDeleteCall.html), [*insert*](struct.PlaylistItemInsertCall.html), [*list*](struct.PlaylistItemListCall.html) and [*update*](struct.PlaylistItemUpdateCall.html)
//! * [playlists](struct.Playlist.html)
//!  * [*delete*](struct.PlaylistDeleteCall.html), [*insert*](struct.PlaylistInsertCall.html), [*list*](struct.PlaylistListCall.html) and [*update*](struct.PlaylistUpdateCall.html)
//! * search
//!  * [*list*](struct.SearchListCall.html)
//! * [subscriptions](struct.Subscription.html)
//!  * [*delete*](struct.SubscriptionDeleteCall.html), [*insert*](struct.SubscriptionInsertCall.html) and [*list*](struct.SubscriptionListCall.html)
//! * [thumbnails](struct.Thumbnail.html)
//!  * [*set*](struct.ThumbnailSetCall.html)
//! * [video categories](struct.VideoCategory.html)
//!  * [*list*](struct.VideoCategoryListCall.html)
//! * [videos](struct.Video.html)
//!  * [*delete*](struct.VideoDeleteCall.html), [*get rating*](struct.VideoGetRatingCall.html), [*insert*](struct.VideoInsertCall.html), [*list*](struct.VideoListCall.html), [*rate*](struct.VideoRateCall.html) and [*update*](struct.VideoUpdateCall.html)
//! * watermarks
//!  * [*set*](struct.WatermarkSetCall.html) and [*unset*](struct.WatermarkUnsetCall.html)
//! 
//! 
//! Upload supported by ...
//! 
//! * [*set watermarks*](struct.WatermarkSetCall.html)
//! * [*insert channel banners*](struct.ChannelBannerInsertCall.html)
//! * [*set thumbnails*](struct.ThumbnailSetCall.html)
//! * [*insert videos*](struct.VideoInsertCall.html)
//! 
//! Subscription supported by ...
//! 
//! * [*list playlist items*](struct.PlaylistItemListCall.html)
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.YouTube.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.live_broadcasts().control(...).doit()
//! let r = hub.live_broadcasts().insert(...).doit()
//! let r = hub.live_broadcasts().list(...).doit()
//! let r = hub.live_broadcasts().transition(...).doit()
//! let r = hub.live_broadcasts().update(...).doit()
//! let r = hub.live_broadcasts().delete(...).doit()
//! let r = hub.live_broadcasts().bind(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-youtube3 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "google-youtube3" as youtube3;
//! use youtube3::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use youtube3::YouTube;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = YouTube::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.live_broadcasts().list("part")
//!              .page_token("justo")
//!              .on_behalf_of_content_owner_channel("amet.")
//!              .on_behalf_of_content_owner("erat")
//!              .mine(true)
//!              .max_results(92)
//!              .id("nonumy")
//!              .broadcast_status("dolores")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
//!         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
//!         Error::MissingToken => println!("OAuth2: Missing Token"),
//!         Error::Cancelled => println!("Operation canceled by user"),
//!         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
//!         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
//!         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
//!         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
//!     },
//!     Ok(_) => println!("Success (value doesn't print)"),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are borrowed
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder, Resource, JsonServerError};


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View private information of your YouTube channel relevant during the audit process with a YouTube partner
    PartnerChannelAudit,

    /// View your YouTube account
    Readonly,

    /// Manage your YouTube account
    Full,

    /// Manage your YouTube account
    ForceSsl,

    /// View and manage your assets and associated content on YouTube
    Partner,

    /// Manage your YouTube videos
    Upload,
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            Scope::PartnerChannelAudit => "https://www.googleapis.com/auth/youtubepartner-channel-audit",
            Scope::Readonly => "https://www.googleapis.com/auth/youtube.readonly",
            Scope::Full => "https://www.googleapis.com/auth/youtube",
            Scope::ForceSsl => "https://www.googleapis.com/auth/youtube.force-ssl",
            Scope::Partner => "https://www.googleapis.com/auth/youtubepartner",
            Scope::Upload => "https://www.googleapis.com/auth/youtube.upload",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all YouTube related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// use youtube3::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().list("part")
///              .page_token("sadipscing")
///              .on_behalf_of_content_owner_channel("aliquyam")
///              .on_behalf_of_content_owner("ea")
///              .mine(false)
///              .max_results(80)
///              .id("justo")
///              .broadcast_status("et")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         Error::HttpError(err) => println!("HTTPERROR: {:?}", err),
///         Error::MissingAPIKey => println!("Auth: Missing API Key - used if there are no scopes"),
///         Error::MissingToken => println!("OAuth2: Missing Token"),
///         Error::Cancelled => println!("Operation canceled by user"),
///         Error::UploadSizeLimitExceeded(size, max_size) => println!("Upload size too big: {} of {}", size, max_size),
///         Error::Failure(_) => println!("General Failure (hyper::client::Response doesn't print)"),
///         Error::FieldClash(clashed_field) => println!("You added custom parameter which is part of builder: {:?}", clashed_field),
///         Error::JsonDecodeError(err) => println!("Couldn't understand server reply - maybe API needs update: {:?}", err),
///     },
///     Ok(_) => println!("Success (value doesn't print)"),
/// }
/// # }
/// ```
pub struct YouTube<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for YouTube<C, NC, A> {}

impl<'a, C, NC, A> YouTube<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> YouTube<C, NC, A> {
        YouTube {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/0.1.1".to_string(),
            _m: PhantomData
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C, NC, A> {
        ActivityMethods { hub: &self }
    }
    pub fn channel_banners(&'a self) -> ChannelBannerMethods<'a, C, NC, A> {
        ChannelBannerMethods { hub: &self }
    }
    pub fn channel_sections(&'a self) -> ChannelSectionMethods<'a, C, NC, A> {
        ChannelSectionMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a, C, NC, A> {
        ChannelMethods { hub: &self }
    }
    pub fn guide_categories(&'a self) -> GuideCategoryMethods<'a, C, NC, A> {
        GuideCategoryMethods { hub: &self }
    }
    pub fn i18n_languages(&'a self) -> I18nLanguageMethods<'a, C, NC, A> {
        I18nLanguageMethods { hub: &self }
    }
    pub fn i18n_regions(&'a self) -> I18nRegionMethods<'a, C, NC, A> {
        I18nRegionMethods { hub: &self }
    }
    pub fn live_broadcasts(&'a self) -> LiveBroadcastMethods<'a, C, NC, A> {
        LiveBroadcastMethods { hub: &self }
    }
    pub fn live_streams(&'a self) -> LiveStreamMethods<'a, C, NC, A> {
        LiveStreamMethods { hub: &self }
    }
    pub fn playlist_items(&'a self) -> PlaylistItemMethods<'a, C, NC, A> {
        PlaylistItemMethods { hub: &self }
    }
    pub fn playlists(&'a self) -> PlaylistMethods<'a, C, NC, A> {
        PlaylistMethods { hub: &self }
    }
    pub fn search(&'a self) -> SearchMethods<'a, C, NC, A> {
        SearchMethods { hub: &self }
    }
    pub fn subscriptions(&'a self) -> SubscriptionMethods<'a, C, NC, A> {
        SubscriptionMethods { hub: &self }
    }
    pub fn thumbnails(&'a self) -> ThumbnailMethods<'a, C, NC, A> {
        ThumbnailMethods { hub: &self }
    }
    pub fn video_categories(&'a self) -> VideoCategoryMethods<'a, C, NC, A> {
        VideoCategoryMethods { hub: &self }
    }
    pub fn videos(&'a self) -> VideoMethods<'a, C, NC, A> {
        VideoMethods { hub: &self }
    }
    pub fn watermarks(&'a self) -> WatermarkMethods<'a, C, NC, A> {
        WatermarkMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/0.1.1`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subscriptions](struct.SubscriptionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SubscriptionListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#subscriptionListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of subscriptions that match the request criteria.
    pub items: Vec<Subscription>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for SubscriptionListResponse {}


/// The auditDetails object encapsulates channel data that is relevant for YouTube Partners during the audit process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelAuditDetails {
    /// Whether or not the channel has any copyright strikes.
    #[serde(alias="copyrightStrikesGoodStanding")]
    pub copyright_strikes_good_standing: bool,
    /// Whether or not the channel respects the community guidelines.
    #[serde(alias="communityGuidelinesGoodStanding")]
    pub community_guidelines_good_standing: bool,
    /// Whether or not the channel has any unresolved claims.
    #[serde(alias="contentIdClaimsGoodStanding")]
    pub content_id_claims_good_standing: bool,
    /// Describes the general state of the channel. This field will always show if there are any issues whatsoever with the channel. Currently this field represents the result of the logical and operation over the community guidelines good standing, the copyright strikes good standing and the content ID claims good standing, but this may change in the future.
    #[serde(alias="overallGoodStanding")]
    pub overall_good_standing: bool,
}

impl Part for ChannelAuditDetails {}


/// Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetails {
    /// The uploaded video file's combined (video and audio) bitrate in bits per second.
    #[serde(alias="bitrateBps")]
    pub bitrate_bps: String,
    /// The uploaded video file's container format.
    pub container: String,
    /// Geographic coordinates that identify the place where the uploaded video was recorded. Coordinates are defined using WGS 84.
    #[serde(alias="recordingLocation")]
    pub recording_location: GeoPoint,
    /// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
    #[serde(alias="fileType")]
    pub file_type: String,
    /// The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported:  
    /// - Date only: YYYY-MM-DD 
    /// - Naive time: YYYY-MM-DDTHH:MM:SS 
    /// - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM
    #[serde(alias="creationTime")]
    pub creation_time: String,
    /// The length of the uploaded video in milliseconds.
    #[serde(alias="durationMs")]
    pub duration_ms: String,
    /// The uploaded file's name. This field is present whether a video file or another type of file was uploaded.
    #[serde(alias="fileName")]
    pub file_name: String,
    /// The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.
    #[serde(alias="fileSize")]
    pub file_size: String,
    /// A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.
    #[serde(alias="videoStreams")]
    pub video_streams: Vec<VideoFileDetailsVideoStream>,
    /// A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.
    #[serde(alias="audioStreams")]
    pub audio_streams: Vec<VideoFileDetailsAudioStream>,
}

impl Part for VideoFileDetails {}


/// Playlist localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistLocalization {
    /// The localized strings for playlist's description.
    pub description: String,
    /// The localized strings for playlist's title.
    pub title: String,
}

impl Part for PlaylistLocalization {}


/// Information about a resource that received a comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsComment {
    /// The resourceId object contains information that identifies the resource associated with the comment.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsComment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list playlist items](struct.PlaylistItemListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlaylistItemListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItemListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of playlist items that match the request criteria.
    pub items: Vec<PlaylistItem>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for PlaylistItemListResponse {}


/// A pair Property / Value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyValue {
    /// A property.
    pub property: String,
    /// The property's value.
    pub value: String,
}

impl Part for PropertyValue {}


/// Describes a temporal position of a visual widget inside a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoTiming {
    /// Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds.
    #[serde(alias="offsetMs")]
    pub offset_ms: String,
    /// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
    #[serde(alias="type")]
    pub type_: String,
    /// Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.
    #[serde(alias="durationMs")]
    pub duration_ms: String,
}

impl Part for InvideoTiming {}


/// Basic details about a playlist, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistSnippet {
    /// The playlist's description.
    pub description: String,
    /// The playlist's title.
    pub title: String,
    /// The ID that YouTube uses to uniquely identify the channel that published the playlist.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The date and time that the playlist was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// Keyword tags associated with the playlist.
    pub tags: Vec<String>,
    /// The channel title of the channel that the video belongs to.
    #[serde(alias="channelTitle")]
    pub channel_title: String,
    /// The language of the playlist's default title and description.
    #[serde(alias="defaultLanguage")]
    pub default_language: String,
    /// Localized title and description, read-only.
    pub localized: PlaylistLocalization,
    /// A map of thumbnail images associated with the playlist. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
}

impl Part for PlaylistSnippet {}


/// Information about a resource that received a positive (like) rating.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsLike {
    /// The resourceId object contains information that identifies the rated resource.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsLike {}


/// A live stream describes a live ingestion point.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete live streams](struct.LiveStreamDeleteCall.html) (none)
/// * [update live streams](struct.LiveStreamUpdateCall.html) (request|response)
/// * [list live streams](struct.LiveStreamListCall.html) (none)
/// * [insert live streams](struct.LiveStreamInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStream {
    /// The status object contains information about live stream's status.
    pub status: Option<LiveStreamStatus>,
    /// The snippet object contains basic details about the stream, including its channel, title, and description.
    pub snippet: Option<LiveStreamSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStream".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The content_details object contains information about the stream, including the closed captions ingestion URL.
    #[serde(alias="contentDetails")]
    pub content_details: Option<LiveStreamContentDetails>,
    /// The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube.
    pub cdn: Option<CdnSettings>,
    /// The ID that YouTube assigns to uniquely identify the stream.
    pub id: Option<String>,
}

impl RequestValue for LiveStream {}
impl Resource for LiveStream {}
impl ResponseResult for LiveStream {}

impl ToParts for LiveStream {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.cdn.is_some() { r = r + "cdn,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set thumbnails](struct.ThumbnailSetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ThumbnailSetResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// A list of thumbnails.
    pub items: Vec<ThumbnailDetails>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#thumbnailSetResponse".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
}

impl ResponseResult for ThumbnailSetResponse {}


/// Information about the uploaded video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsUpload {
    /// The ID that YouTube uses to uniquely identify the uploaded video.
    #[serde(alias="videoId")]
    pub video_id: String,
}

impl Part for ActivityContentDetailsUpload {}


/// Branding properties for the channel view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSettings {
    /// Specifies the channel description.
    pub description: String,
    /// Specifies the channel title.
    pub title: String,
    /// Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible.
    #[serde(alias="moderateComments")]
    pub moderate_comments: bool,
    /// Whether the tab to browse the videos should be displayed.
    #[serde(alias="showBrowseView")]
    pub show_browse_view: bool,
    /// Title for the featured channels tab.
    #[serde(alias="featuredChannelsTitle")]
    pub featured_channels_title: String,
    /// no description provided
    #[serde(alias="defaultLanguage")]
    pub default_language: String,
    /// The trailer of the channel, for users that are not subscribers.
    #[serde(alias="unsubscribedTrailer")]
    pub unsubscribed_trailer: String,
    /// The list of featured channels.
    #[serde(alias="featuredChannelsUrls")]
    pub featured_channels_urls: Vec<String>,
    /// A prominent color that can be rendered on this channel page.
    #[serde(alias="profileColor")]
    pub profile_color: String,
    /// Which content tab users should see when viewing the channel.
    #[serde(alias="defaultTab")]
    pub default_tab: String,
    /// Lists keywords associated with the channel, comma-separated.
    pub keywords: String,
    /// Whether related channels should be proposed.
    #[serde(alias="showRelatedChannels")]
    pub show_related_channels: bool,
    /// The ID for a Google Analytics account to track and measure traffic to the channels.
    #[serde(alias="trackingAnalyticsAccountId")]
    pub tracking_analytics_account_id: String,
}

impl Part for ChannelSettings {}


/// Basic details about a search result, including title, description and thumbnails of the item referenced by the search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchResultSnippet {
    /// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
    #[serde(alias="liveBroadcastContent")]
    pub live_broadcast_content: String,
    /// A description of the search result.
    pub description: String,
    /// The title of the search result.
    pub title: String,
    /// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
    /// The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The creation date and time of the resource that the search result identifies. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// The title of the channel that published the resource that the search result identifies.
    #[serde(alias="channelTitle")]
    pub channel_title: String,
}

impl Part for SearchResultSnippet {}


/// Describes information necessary for ingesting an RTMP or an HTTP stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IngestionInfo {
    /// The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL.
    #[serde(alias="backupIngestionAddress")]
    pub backup_ingestion_address: String,
    /// The HTTP or RTMP stream name that YouTube assigns to the video stream.
    #[serde(alias="streamName")]
    pub stream_name: String,
    /// The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL.
    /// 
    /// Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format:
    /// 
    /// STREAM_URL/STREAM_NAME
    #[serde(alias="ingestionAddress")]
    pub ingestion_address: String,
}

impl Part for IngestionInfo {}


/// Brief description of the live stream cdn settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CdnSettings {
    /// The format of the video stream that you are sending to Youtube.
    pub format: String,
    /// The ingestionInfo object contains information that YouTube provides that you need to transmit your RTMP or HTTP stream to YouTube.
    #[serde(alias="ingestionInfo")]
    pub ingestion_info: IngestionInfo,
    /// The method or protocol used to transmit the video stream.
    #[serde(alias="ingestionType")]
    pub ingestion_type: String,
}

impl Part for CdnSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get rating videos](struct.VideoGetRatingCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoGetRatingResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// A list of ratings that match the request criteria.
    pub items: Vec<VideoRating>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoGetRatingResponse".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
}

impl ResponseResult for VideoGetRatingResponse {}


/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoCategorySnippet {
    /// no description provided
    pub assignable: bool,
    /// The YouTube channel that created the video category.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The video category's title.
    pub title: String,
}

impl Part for VideoCategorySnippet {}


/// Details about a resource which was added to a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsChannelItem {
    /// The resourceId object contains information that identifies the resource that was added to the channel.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsChannelItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastSnippet {
    /// The date and time that the broadcast actually ended. This information is only available once the broadcast's state is complete. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="actualEndTime")]
    pub actual_end_time: String,
    /// The broadcast's description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource.
    pub description: String,
    /// The broadcast's title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource.
    pub title: String,
    /// The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The date and time that the broadcast was added to YouTube's live broadcast schedule. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// The date and time that the broadcast is scheduled to start. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="scheduledStartTime")]
    pub scheduled_start_time: String,
    /// The date and time that the broadcast actually started. This information is only available once the broadcast's state is live. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="actualStartTime")]
    pub actual_start_time: String,
    /// The date and time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="scheduledEndTime")]
    pub scheduled_end_time: String,
    /// A map of thumbnail images associated with the broadcast. For each nested object in this object, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
}

impl Part for LiveBroadcastSnippet {}


/// Basic details about a subscription, including title, description and thumbnails of the subscribed item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionSnippet {
    /// The subscription's details.
    pub description: String,
    /// The subscription's title.
    pub title: String,
    /// The id object contains information about the channel that the user subscribed to.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
    /// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
    /// The ID that YouTube uses to uniquely identify the subscriber's channel.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The date and time that the subscription was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// Channel title for the channel that the subscription belongs to.
    #[serde(alias="channelTitle")]
    pub channel_title: String,
}

impl Part for SubscriptionSnippet {}


/// Details about a channelsection, including playlists and channels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionContentDetails {
    /// The channel ids for type multiple_channels.
    pub channels: Vec<String>,
    /// The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed.
    pub playlists: Vec<String>,
}

impl Part for ChannelSectionContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n regions](struct.I18nRegionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nRegionListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource.
    pub items: Vec<I18nRegion>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegionListResponse".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
}

impl ResponseResult for I18nRegionListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list live streams](struct.LiveStreamListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LiveStreamListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStreamListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of live streams that match the request criteria.
    pub items: Vec<LiveStream>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for LiveStreamListResponse {}


/// Detailed settings of a stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamContentDetails {
    /// Indicates whether the stream is reusable, which means that it can be bound to multiple broadcasts. It is common for broadcasters to reuse the same stream for many different broadcasts if those broadcasts occur at different times.
    /// 
    /// If you set this value to false, then the stream will not be reusable, which means that it can only be bound to one broadcast. Non-reusable streams differ from reusable streams in the following ways:  
    /// - A non-reusable stream can only be bound to one broadcast. 
    /// - A non-reusable stream might be deleted by an automated process after the broadcast ends. 
    /// - The  liveStreams.list method does not list non-reusable streams if you call the method and set the mine parameter to true. The only way to use that method to retrieve the resource for a non-reusable stream is to use the id parameter to identify the stream.
    #[serde(alias="isReusable")]
    pub is_reusable: bool,
    /// The ingestion URL where the closed captions of this stream are sent.
    #[serde(alias="closedCaptionsIngestionUrl")]
    pub closed_captions_ingestion_url: String,
}

impl Part for LiveStreamContentDetails {}


/// Basic details about an i18n language, such as language code and human-readable name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nLanguageSnippet {
    /// The human-readable name of the language in the language itself.
    pub name: String,
    /// A short BCP-47 code that uniquely identifies a language.
    pub hl: String,
}

impl Part for I18nLanguageSnippet {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set watermarks](struct.WatermarkSetCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize)]
pub struct InvideoBranding {
    /// no description provided
    #[serde(alias="targetChannelId")]
    pub target_channel_id: Option<String>,
    /// no description provided
    pub position: Option<InvideoPosition>,
    /// no description provided
    #[serde(alias="imageUrl")]
    pub image_url: Option<String>,
    /// no description provided
    pub timing: Option<InvideoTiming>,
    /// no description provided
    #[serde(alias="imageBytes")]
    pub image_bytes: Option<String>,
}

impl RequestValue for InvideoBranding {}


/// Information about the playlist item's privacy status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemStatus {
    /// This resource's privacy status.
    #[serde(alias="privacyStatus")]
    pub privacy_status: String,
}

impl Part for PlaylistItemStatus {}


/// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelConversionPing {
    /// Defines the context of the ping.
    pub context: String,
    /// The url (without the schema) that the player shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D cview%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping.
    #[serde(alias="conversionUrl")]
    pub conversion_url: String,
}

impl Part for ChannelConversionPing {}


/// Project specific details about the content of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProjectDetails {
    /// A list of project tags associated with the video during the upload.
    pub tags: Vec<String>,
}

impl Part for VideoProjectDetails {}


/// A playlistItem resource identifies another resource, such as a video, that is included in a playlist. In addition, the playlistItem  resource contains details about the included resource that pertain specifically to how that resource is used in that playlist.
/// 
/// YouTube uses playlists to identify special collections of videos for a channel, such as:  
/// - uploaded videos 
/// - favorite videos 
/// - positively rated (liked) videos 
/// - watch history 
/// - watch later  To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company's videos, playlists, and other YouTube information.
/// 
/// You can retrieve the playlist IDs for each of these lists from the  channel resource  for a given channel. You can then use the   playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the   playlistItems.insert and   playlistItems.delete methods. For example, if a user gives a positive rating to a video, you would insert that video into the liked videos playlist for that user's channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert playlist items](struct.PlaylistItemInsertCall.html) (request|response)
/// * [update playlist items](struct.PlaylistItemUpdateCall.html) (request|response)
/// * [list playlist items](struct.PlaylistItemListCall.html) (none)
/// * [delete playlist items](struct.PlaylistItemDeleteCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItem {
    /// The status object contains information about the playlist item's privacy status.
    pub status: Option<PlaylistItemStatus>,
    /// The snippet object contains basic details about the playlist item, such as its title and position in the playlist.
    pub snippet: Option<PlaylistItemSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItem".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The contentDetails object is included in the resource if the included item is a YouTube video. The object contains additional information about the video.
    #[serde(alias="contentDetails")]
    pub content_details: Option<PlaylistItemContentDetails>,
    /// The ID that YouTube uses to uniquely identify the playlist item.
    pub id: Option<String>,
}

impl RequestValue for PlaylistItem {}
impl Resource for PlaylistItem {}
impl ResponseResult for PlaylistItem {}

impl ToParts for PlaylistItem {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list guide categories](struct.GuideCategoryListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GuideCategoryListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#guideCategoryListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of categories that can be associated with YouTube channels. In this map, the category ID is the map key, and its value is the corresponding guideCategory resource.
    pub items: Vec<GuideCategory>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for GuideCategoryListResponse {}


/// Localized versions of certain video properties (e.g. title).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoLocalization {
    /// Localized version of the video's description.
    pub description: String,
    /// Localized version of the video's title.
    pub title: String,
}

impl Part for VideoLocalization {}


/// Basic details about a channel section, including title, style and position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionSnippet {
    /// The style of the channel section.
    pub style: String,
    /// Localized title, read-only.
    pub localized: ChannelSectionLocalization,
    /// The channel section's title for multiple_playlists and multiple_channels.
    pub title: String,
    /// The position of the channel section in the channel.
    pub position: u32,
    /// The ID that YouTube uses to uniquely identify the channel that published the channel section.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The type of the channel section.
    #[serde(alias="type")]
    pub type_: String,
    /// The language of the channel section's default title and description.
    #[serde(alias="defaultLanguage")]
    pub default_language: String,
}

impl Part for ChannelSectionSnippet {}


/// Details about the content of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentDetails {
    /// no description provided
    #[serde(alias="relatedPlaylists")]
    pub related_playlists: ChannelContentDetailsRelatedPlaylists,
    /// The googlePlusUserId object identifies the Google+ profile ID associated with this channel.
    #[serde(alias="googlePlusUserId")]
    pub google_plus_user_id: String,
}

impl Part for ChannelContentDetails {}


/// Stub token pagination template to suppress results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct TokenPagination;

impl Part for TokenPagination {}


/// A i18nRegion resource identifies a region where YouTube is available.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n regions](struct.I18nRegionListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nRegion {
    /// The snippet object contains basic details about the i18n region, such as region code and human-readable name.
    pub snippet: Option<I18nRegionSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegion".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the i18n region.
    pub id: Option<String>,
}

impl Resource for I18nRegion {}

impl ToParts for I18nRegion {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// Internal representation of thumbnails for a YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThumbnailDetails {
    /// The default image for this resource.
    pub default: Thumbnail,
    /// The high quality image for this resource.
    pub high: Thumbnail,
    /// The medium quality image for this resource.
    pub medium: Thumbnail,
    /// The maximum resolution quality image for this resource.
    pub maxres: Thumbnail,
    /// The standard quality image for this resource.
    pub standard: Thumbnail,
}

impl Part for ThumbnailDetails {}


/// Details about monetization of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoMonetizationDetails {
    /// The value of access indicates whether the video can be monetized or not.
    pub access: AccessPolicy,
}

impl Part for VideoMonetizationDetails {}


/// Information that identifies the recommended resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsRecommendation {
    /// The resourceId object contains information that identifies the recommended resource.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
    /// The reason that the resource is recommended to the user.
    pub reason: String,
    /// The seedResourceId object contains information about the resource that caused the recommendation.
    #[serde(alias="seedResourceId")]
    pub seed_resource_id: ResourceId,
}

impl Part for ActivityContentDetailsRecommendation {}


/// Recording information associated with the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoRecordingDetails {
    /// The date and time when the video was recorded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.
    #[serde(alias="recordingDate")]
    pub recording_date: String,
    /// The text description of the location where the video was recorded.
    #[serde(alias="locationDescription")]
    pub location_description: String,
    /// The geolocation information associated with the video.
    pub location: GeoPoint,
}

impl Part for VideoRecordingDetails {}


/// Information about a channel that a user subscribed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsSubscription {
    /// The resourceId object contains information that identifies the resource that the user subscribed to.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsSubscription {}


/// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelConversionPings {
    /// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
    pub pings: Vec<ChannelConversionPing>,
}

impl Part for ChannelConversionPings {}


/// Details about the content of an activity: the video that was shared, the channel that was subscribed to, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetails {
    /// The comment object contains information about a resource that received a comment. This property is only present if the snippet.type is comment.
    pub comment: ActivityContentDetailsComment,
    /// The playlistItem object contains information about a new playlist item. This property is only present if the snippet.type is playlistItem.
    #[serde(alias="playlistItem")]
    pub playlist_item: ActivityContentDetailsPlaylistItem,
    /// The like object contains information about a resource that received a positive (like) rating. This property is only present if the snippet.type is like.
    pub like: ActivityContentDetailsLike,
    /// The promotedItem object contains details about a resource which is being promoted. This property is only present if the snippet.type is promotedItem.
    #[serde(alias="promotedItem")]
    pub promoted_item: ActivityContentDetailsPromotedItem,
    /// The recommendation object contains information about a recommended resource. This property is only present if the snippet.type is recommendation.
    pub recommendation: ActivityContentDetailsRecommendation,
    /// The favorite object contains information about a video that was marked as a favorite video. This property is only present if the snippet.type is favorite.
    pub favorite: ActivityContentDetailsFavorite,
    /// The upload object contains information about the uploaded video. This property is only present if the snippet.type is upload.
    pub upload: ActivityContentDetailsUpload,
    /// The social object contains details about a social network post. This property is only present if the snippet.type is social.
    pub social: ActivityContentDetailsSocial,
    /// The channelItem object contains details about a resource which was added to a channel. This property is only present if the snippet.type is channelItem.
    #[serde(alias="channelItem")]
    pub channel_item: ActivityContentDetailsChannelItem,
    /// The bulletin object contains details about a channel bulletin post. This object is only present if the snippet.type is bulletin.
    pub bulletin: ActivityContentDetailsBulletin,
    /// The subscription object contains information about a channel that a user subscribed to. This property is only present if the snippet.type is subscription.
    pub subscription: ActivityContentDetailsSubscription,
}

impl Part for ActivityContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list playlists](struct.PlaylistListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PlaylistListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of playlists that match the request criteria.
    pub items: Vec<Playlist>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for PlaylistListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemContentDetails {
    /// A user-generated note for this item.
    pub note: String,
    /// The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.
    #[serde(alias="startAt")]
    pub start_at: String,
    /// The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.
    #[serde(alias="endAt")]
    pub end_at: String,
    /// The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request.
    #[serde(alias="videoId")]
    pub video_id: String,
}

impl Part for PlaylistItemContentDetails {}


/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentOwnerDetails {
    /// The ID of the content owner linked to the channel.
    #[serde(alias="contentOwner")]
    pub content_owner: String,
    /// The date and time of when the channel was linked to the content owner. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="timeLinked")]
    pub time_linked: String,
}

impl Part for ChannelContentOwnerDetails {}


/// Describes processing status and progress and availability of some other Video resource parts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProcessingDetails {
    /// This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.
    #[serde(alias="fileDetailsAvailability")]
    pub file_details_availability: String,
    /// This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(alias="editorSuggestionsAvailability")]
    pub editor_suggestions_availability: String,
    /// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
    #[serde(alias="processingStatus")]
    pub processing_status: String,
    /// This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(alias="processingIssuesAvailability")]
    pub processing_issues_availability: String,
    /// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
    #[serde(alias="processingFailureReason")]
    pub processing_failure_reason: String,
    /// This value indicates whether thumbnail images have been generated for the video.
    #[serde(alias="thumbnailsAvailability")]
    pub thumbnails_availability: String,
    /// The processingProgress object contains information about the progress YouTube has made in processing the video. The values are really only relevant if the video's processing status is processing.
    #[serde(alias="processingProgress")]
    pub processing_progress: VideoProcessingDetailsProcessingProgress,
    /// This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(alias="tagSuggestionsAvailability")]
    pub tag_suggestions_availability: String,
}

impl Part for VideoProcessingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastStatus {
    /// The broadcast's recording status.
    #[serde(alias="recordingStatus")]
    pub recording_status: String,
    /// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
    #[serde(alias="privacyStatus")]
    pub privacy_status: String,
    /// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
    #[serde(alias="lifeCycleStatus")]
    pub life_cycle_status: String,
    /// Priority of the live broadcast event (internal state).
    #[serde(alias="liveBroadcastPriority")]
    pub live_broadcast_priority: String,
}

impl Part for LiveBroadcastStatus {}


/// Details about the content to witch a subscription refers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionContentDetails {
    /// The number of new items in the subscription since its content was last read.
    #[serde(alias="newItemCount")]
    pub new_item_count: u32,
    /// The type of activity this subscription is for (only uploads, everything).
    #[serde(alias="activityType")]
    pub activity_type: String,
    /// The approximate number of items that the subscription points to.
    #[serde(alias="totalItemCount")]
    pub total_item_count: u32,
}

impl Part for SubscriptionContentDetails {}


/// A video resource represents a YouTube video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rate videos](struct.VideoRateCall.html) (none)
/// * [get rating videos](struct.VideoGetRatingCall.html) (none)
/// * [list videos](struct.VideoListCall.html) (none)
/// * [insert videos](struct.VideoInsertCall.html) (request|response)
/// * [update videos](struct.VideoUpdateCall.html) (request|response)
/// * [delete videos](struct.VideoDeleteCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    /// The status object contains information about the video's uploading, processing, and privacy statuses.
    pub status: Option<VideoStatus>,
    /// The topicDetails object encapsulates information about Freebase topics associated with the video.
    #[serde(alias="topicDetails")]
    pub topic_details: Option<VideoTopicDetails>,
    /// The monetizationDetails object encapsulates information about the monetization status of the video.
    #[serde(alias="monetizationDetails")]
    pub monetization_details: Option<VideoMonetizationDetails>,
    /// The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner.
    pub suggestions: Option<VideoSuggestions>,
    /// Age restriction details related to a video.
    #[serde(alias="ageGating")]
    pub age_gating: Option<VideoAgeGating>,
    /// The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner.
    #[serde(alias="fileDetails")]
    pub file_details: Option<VideoFileDetails>,
    /// The player object contains information that you would use to play the video in an embedded player.
    pub player: Option<VideoPlayer>,
    /// The ID that YouTube uses to uniquely identify the video.
    pub id: Option<String>,
    /// List with all localizations.
    pub localizations: Option<HashMap<String, VideoLocalization>>,
    /// The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast.
    #[serde(alias="liveStreamingDetails")]
    pub live_streaming_details: Option<VideoLiveStreamingDetails>,
    /// The snippet object contains basic details about the video, such as its title, description, and category.
    pub snippet: Option<VideoSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#video".
    pub kind: Option<String>,
    /// The statistics object contains statistics about the video.
    pub statistics: Option<VideoStatistics>,
    /// The projectDetails object contains information about the project specific video metadata.
    #[serde(alias="projectDetails")]
    pub project_details: Option<VideoProjectDetails>,
    /// The conversionPings object encapsulates information about url pings that need to be respected by the App in different video contexts.
    #[serde(alias="conversionPings")]
    pub conversion_pings: Option<VideoConversionPings>,
    /// The processingProgress object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video.
    /// 
    /// The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner.
    #[serde(alias="processingDetails")]
    pub processing_details: Option<VideoProcessingDetails>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The contentDetails object contains information about the video content, including the length of the video and its aspect ratio.
    #[serde(alias="contentDetails")]
    pub content_details: Option<VideoContentDetails>,
    /// The recordingDetails object encapsulates information about the location, date and address where the video was recorded.
    #[serde(alias="recordingDetails")]
    pub recording_details: Option<VideoRecordingDetails>,
}

impl RequestValue for Video {}
impl Resource for Video {}
impl ResponseResult for Video {}

impl ToParts for Video {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.topic_details.is_some() { r = r + "topicDetails,"; }
        if self.monetization_details.is_some() { r = r + "monetizationDetails,"; }
        if self.suggestions.is_some() { r = r + "suggestions,"; }
        if self.age_gating.is_some() { r = r + "ageGating,"; }
        if self.file_details.is_some() { r = r + "fileDetails,"; }
        if self.player.is_some() { r = r + "player,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        if self.live_streaming_details.is_some() { r = r + "liveStreamingDetails,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.statistics.is_some() { r = r + "statistics,"; }
        if self.project_details.is_some() { r = r + "projectDetails,"; }
        if self.conversion_pings.is_some() { r = r + "conversionPings,"; }
        if self.processing_details.is_some() { r = r + "processingDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.recording_details.is_some() { r = r + "recordingDetails,"; }
        r.pop();
        r
    }
}

/// Geographical coordinates of a point, in WGS84.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoPoint {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Altitude above the reference ellipsoid, in meters.
    pub altitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
}

impl Part for GeoPoint {}


/// Branding properties of a YouTube channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelBrandingSettings {
    /// Branding properties for branding images.
    pub image: ImageSettings,
    /// Branding properties for the watch page.
    pub watch: WatchSettings,
    /// Branding properties for the channel view.
    pub channel: ChannelSettings,
    /// Additional experimental branding properties.
    pub hints: Vec<PropertyValue>,
}

impl Part for ChannelBrandingSettings {}


/// Player to be used for a video playback.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoPlayer {
    /// An <iframe> tag that embeds a player that will play the video.
    #[serde(alias="embedHtml")]
    pub embed_html: String,
}

impl Part for VideoPlayer {}


/// Basic details about a channel, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSnippet {
    /// The date and time that the channel was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// The description of the channel.
    pub description: String,
    /// The channel's title.
    pub title: String,
    /// Localized title and description, read-only.
    pub localized: ChannelLocalization,
    /// The language of the channel's default title and description.
    #[serde(alias="defaultLanguage")]
    pub default_language: String,
    /// A map of thumbnail images associated with the channel. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
}

impl Part for ChannelSnippet {}


/// Branding properties for the watch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchSettings {
    /// The background color for the video watch page's branded area.
    #[serde(alias="textColor")]
    pub text_color: String,
    /// An ID that uniquely identifies a playlist that displays next to the video player.
    #[serde(alias="featuredPlaylistId")]
    pub featured_playlist_id: String,
    /// The text color for the video watch page's branded area.
    #[serde(alias="backgroundColor")]
    pub background_color: String,
}

impl Part for WatchSettings {}


/// ChannelSection localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionLocalization {
    /// The localized strings for channel section's title.
    pub title: String,
}

impl Part for ChannelSectionLocalization {}


/// DEPRECATED Region restriction of the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContentDetailsRegionRestriction {
    /// A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.
    pub allowed: Vec<String>,
    /// A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.
    pub blocked: Vec<String>,
}

impl Part for VideoContentDetailsRegionRestriction {}


/// Details about the content of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContentDetails {
    /// The value of definition indicates whether the video is available in high definition or only in standard definition.
    pub definition: String,
    /// The countryRestriction object contains information about the countries where a video is (or is not) viewable.
    #[serde(alias="countryRestriction")]
    pub country_restriction: AccessPolicy,
    /// Specifies the ratings that the video received under various rating schemes.
    #[serde(alias="contentRating")]
    pub content_rating: ContentRating,
    /// The value of captions indicates whether the video has captions or not.
    pub caption: String,
    /// The regionRestriction object contains information about the countries where a video is (or is not) viewable. The object will contain either the contentDetails.regionRestriction.allowed property or the contentDetails.regionRestriction.blocked property.
    #[serde(alias="regionRestriction")]
    pub region_restriction: VideoContentDetailsRegionRestriction,
    /// The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.
    pub duration: String,
    /// The value of is_license_content indicates whether the video is licensed content.
    #[serde(alias="licensedContent")]
    pub licensed_content: bool,
    /// The value of dimension indicates whether the video is available in 3D or in 2D.
    pub dimension: String,
}

impl Part for VideoContentDetails {}


/// Describes a single promoted item id. It is a union of various possible types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotedItemId {
    /// If the promoted item represents a website, this field represents the url pointing to the website. This field will be present only if type has the value website.
    #[serde(alias="websiteUrl")]
    pub website_url: String,
    /// If type is recentUpload, this field identifies the channel from which to take the recent upload. If missing, the channel is assumed to be the same channel for which the invideoPromotion is set.
    #[serde(alias="recentlyUploadedBy")]
    pub recently_uploaded_by: String,
    /// Describes the type of the promoted item.
    #[serde(alias="type")]
    pub type_: String,
    /// If the promoted item represents a video, this field represents the unique YouTube ID identifying it. This field will be present only if type has the value video.
    #[serde(alias="videoId")]
    pub video_id: String,
}

impl Part for PromotedItemId {}


/// A subscription resource contains information about a YouTube user subscription. A subscription notifies a user when new videos are added to a channel or when another user takes one of several actions on YouTube, such as uploading a video, rating a video, or commenting on a video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert subscriptions](struct.SubscriptionInsertCall.html) (request|response)
/// * [list subscriptions](struct.SubscriptionListCall.html) (none)
/// * [delete subscriptions](struct.SubscriptionDeleteCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// The snippet object contains basic details about the subscription, including its title and the channel that the user subscribed to.
    pub snippet: Option<SubscriptionSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#subscription".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The contentDetails object contains basic statistics about the subscription.
    #[serde(alias="contentDetails")]
    pub content_details: Option<SubscriptionContentDetails>,
    /// The subscriberSnippet object contains basic details about the sbuscriber.
    #[serde(alias="subscriberSnippet")]
    pub subscriber_snippet: Option<SubscriptionSubscriberSnippet>,
    /// The ID that YouTube uses to uniquely identify the subscription.
    pub id: Option<String>,
}

impl RequestValue for Subscription {}
impl Resource for Subscription {}
impl ResponseResult for Subscription {}

impl ToParts for Subscription {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.subscriber_snippet.is_some() { r = r + "subscriberSnippet,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// Basic details about an i18n region, such as region code and human-readable name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nRegionSnippet {
    /// The region code as a 2-letter ISO country code.
    pub gl: String,
    /// The human-readable name of the region.
    pub name: String,
}

impl Part for I18nRegionSnippet {}


/// Information about a new playlist item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsPlaylistItem {
    /// The resourceId object contains information about the resource that was added to the playlist.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
    /// The value that YouTube uses to uniquely identify the playlist.
    #[serde(alias="playlistId")]
    pub playlist_id: String,
    /// ID of the item within the playlist.
    #[serde(alias="playlistItemId")]
    pub playlist_item_id: String,
}

impl Part for ActivityContentDetailsPlaylistItem {}


/// Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoPosition {
    /// Describes in which corner of the video the visual widget will appear.
    #[serde(alias="cornerPosition")]
    pub corner_position: String,
    /// Defines the position type.
    #[serde(alias="type")]
    pub type_: String,
}

impl Part for InvideoPosition {}


/// A playlist resource represents a YouTube playlist. A playlist is a collection of videos that can be viewed sequentially and shared with other users. A playlist can contain up to 200 videos, and YouTube does not limit the number of playlists that each user creates. By default, playlists are publicly visible to other users, but playlists can be public or private.
/// 
/// YouTube also uses playlists to identify special collections of videos for a channel, such as:  
/// - uploaded videos 
/// - favorite videos 
/// - positively rated (liked) videos 
/// - watch history 
/// - watch later  To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company's videos, playlists, and other YouTube information. You can retrieve the playlist IDs for each of these lists from the  channel resource for a given channel.
/// 
/// You can then use the   playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the   playlistItems.insert and   playlistItems.delete methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert playlists](struct.PlaylistInsertCall.html) (request|response)
/// * [delete playlists](struct.PlaylistDeleteCall.html) (none)
/// * [list playlists](struct.PlaylistListCall.html) (none)
/// * [update playlists](struct.PlaylistUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
    /// The status object contains status information for the playlist.
    pub status: Option<PlaylistStatus>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".
    pub kind: Option<String>,
    /// The contentDetails object contains information like video count.
    #[serde(alias="contentDetails")]
    pub content_details: Option<PlaylistContentDetails>,
    /// The snippet object contains basic details about the playlist, such as its title and description.
    pub snippet: Option<PlaylistSnippet>,
    /// The player object contains information that you would use to play the playlist in an embedded player.
    pub player: Option<PlaylistPlayer>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the playlist.
    pub id: Option<String>,
    /// Localizations for different languages
    pub localizations: Option<HashMap<String, PlaylistLocalization>>,
}

impl RequestValue for Playlist {}
impl Resource for Playlist {}
impl ResponseResult for Playlist {}

impl ToParts for Playlist {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.player.is_some() { r = r + "player,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        r.pop();
        r
    }
}

/// Basic details about a guide category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GuideCategorySnippet {
    /// no description provided
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// Description of the guide category.
    pub title: String,
}

impl Part for GuideCategorySnippet {}


/// Basic details about a video, including title, description, uploader, thumbnails and category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSnippet {
    /// The video's description.
    pub description: String,
    /// A list of keyword tags associated with the video. Tags may contain spaces. This field is only visible to the video's uploader.
    pub tags: Vec<String>,
    /// The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The language of the videos's default snippet.
    #[serde(alias="defaultLanguage")]
    pub default_language: String,
    /// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
    #[serde(alias="liveBroadcastContent")]
    pub live_broadcast_content: String,
    /// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
    /// The video's title.
    pub title: String,
    /// The YouTube video category associated with the video.
    #[serde(alias="categoryId")]
    pub category_id: String,
    /// Localized snippet selected with the hl parameter. If no such localization exists, this field is populated with the default snippet. (Read-only)
    pub localized: VideoLocalization,
    /// Channel title for the channel that the video belongs to.
    #[serde(alias="channelTitle")]
    pub channel_title: String,
}

impl Part for VideoSnippet {}


/// Describes an invideo promotion campaign consisting of multiple promoted items. A campaign belongs to a single channel_id.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoPromotion {
    /// The default temporal position within the video where the promoted item will be displayed. Can be overriden by more specific timing in the item.
    #[serde(alias="defaultTiming")]
    pub default_timing: InvideoTiming,
    /// List of promoted items in decreasing priority.
    pub items: Vec<PromotedItem>,
    /// Indicates whether the channel's promotional campaign uses "smart timing." This feature attempts to show promotions at a point in the video when they are more likely to be clicked and less likely to disrupt the viewing experience. This feature also picks up a single promotion to show on each video.
    #[serde(alias="useSmartTiming")]
    pub use_smart_timing: bool,
    /// The spatial position within the video where the promoted item will be displayed.
    pub position: InvideoPosition,
}

impl Part for InvideoPromotion {}


/// Describes a single promoted item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotedItem {
    /// The temporal position within the video where the promoted item will be displayed. If present, it overrides the default timing.
    pub timing: InvideoTiming,
    /// If true, the content owner's name will be used when displaying the promotion. This field can only be set when the update is made on behalf of the content owner.
    #[serde(alias="promotedByContentOwner")]
    pub promoted_by_content_owner: bool,
    /// A custom message to display for this promotion. This field is currently ignored unless the promoted item is a website.
    #[serde(alias="customMessage")]
    pub custom_message: String,
    /// Identifies the promoted item.
    pub id: PromotedItemId,
}

impl Part for PromotedItem {}


/// Detailed settings of a broadcast.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastContentDetails {
    /// This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast's status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint's eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers.
    #[serde(alias="startWithSlate")]
    pub start_with_slate: bool,
    /// This value uniquely identifies the live stream bound to the broadcast.
    #[serde(alias="boundStreamId")]
    pub bound_stream_id: String,
    /// This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video.
    #[serde(alias="enableEmbed")]
    pub enable_embed: bool,
    /// This setting indicates whether closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API.
    #[serde(alias="enableClosedCaptions")]
    pub enable_closed_captions: bool,
    /// This setting indicates whether YouTube should enable content encryption for the broadcast.
    #[serde(alias="enableContentEncryption")]
    pub enable_content_encryption: bool,
    /// Automatically start recording after the event goes live. The default value for this property is true.
    /// 
    /// 
    /// 
    /// Important: You must also set the enableDvr property's value to true if you want the playback to be available immediately after the broadcast ends. If you set this property's value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback.
    #[serde(alias="recordFromStart")]
    pub record_from_start: bool,
    /// This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true.
    /// 
    /// 
    /// 
    /// Important: You must set the value to true and also set the enableArchive property's value to true if you want to make playback available immediately after the broadcast ends.
    #[serde(alias="enableDvr")]
    pub enable_dvr: bool,
    /// The monitorStream object contains information about the monitor stream, which the broadcaster can use to review the event content before the broadcast stream is shown publicly.
    #[serde(alias="monitorStream")]
    pub monitor_stream: MonitorStreamInfo,
}

impl Part for LiveBroadcastContentDetails {}


/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoStatus {
    /// The video's license.
    pub license: String,
    /// This value indicates if the video can be embedded on another website.
    pub embeddable: bool,
    /// The video's privacy status.
    #[serde(alias="privacyStatus")]
    pub privacy_status: String,
    /// The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishAt")]
    pub publish_at: String,
    /// This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled.
    #[serde(alias="publicStatsViewable")]
    pub public_stats_viewable: bool,
    /// The status of the uploaded video.
    #[serde(alias="uploadStatus")]
    pub upload_status: String,
    /// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
    #[serde(alias="rejectionReason")]
    pub rejection_reason: String,
    /// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
    #[serde(alias="failureReason")]
    pub failure_reason: String,
}

impl Part for VideoStatus {}


/// A guideCategory resource identifies a category that YouTube algorithmically assigns based on a channel's content or other indicators, such as the channel's popularity. The list is similar to video categories, with the difference being that a video's uploader can assign a video category but only YouTube can assign a channel category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct GuideCategory {
    /// The snippet object contains basic details about the category, such as its title.
    pub snippet: GuideCategorySnippet,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#guideCategory".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The ID that YouTube uses to uniquely identify the guide category.
    pub id: String,
}

impl Part for GuideCategory {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channel sections](struct.ChannelSectionListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ChannelSectionListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// A list of ChannelSections that match the request criteria.
    pub items: Vec<ChannelSection>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSectionListResponse".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
}

impl ResponseResult for ChannelSectionListResponse {}


/// Settings and Info of the monitor stream
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitorStreamInfo {
    /// If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay.
    #[serde(alias="broadcastStreamDelayMs")]
    pub broadcast_stream_delay_ms: u32,
    /// HTML code that embeds a player that plays the monitor stream.
    #[serde(alias="embedHtml")]
    pub embed_html: String,
    /// This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster's consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints.
    /// 
    /// You need to set this value to true if you intend to have a broadcast delay for your event.
    /// 
    /// Note: This property cannot be updated once the broadcast is in the testing or live state.
    #[serde(alias="enableMonitorStream")]
    pub enable_monitor_stream: bool,
}

impl Part for MonitorStreamInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n languages](struct.I18nLanguageListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nLanguageListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource.
    pub items: Vec<I18nLanguage>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguageListResponse".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
}

impl ResponseResult for I18nLanguageListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedProperty {
    /// no description provided
    pub default: String,
    /// The language of the default property.
    #[serde(alias="defaultLanguage")]
    pub default_language: LanguageTag,
    /// no description provided
    pub localized: Vec<LocalizedString>,
}

impl Part for LocalizedProperty {}


/// A liveBroadcast resource represents an event that will be streamed, via live video, on YouTube.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [control live broadcasts](struct.LiveBroadcastControlCall.html) (response)
/// * [insert live broadcasts](struct.LiveBroadcastInsertCall.html) (request|response)
/// * [list live broadcasts](struct.LiveBroadcastListCall.html) (none)
/// * [transition live broadcasts](struct.LiveBroadcastTransitionCall.html) (response)
/// * [update live broadcasts](struct.LiveBroadcastUpdateCall.html) (request|response)
/// * [delete live broadcasts](struct.LiveBroadcastDeleteCall.html) (none)
/// * [bind live broadcasts](struct.LiveBroadcastBindCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcast {
    /// The status object contains information about the event's status.
    pub status: Option<LiveBroadcastStatus>,
    /// The snippet object contains basic details about the event, including its title, description, start time, and end time.
    pub snippet: Option<LiveBroadcastSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcast".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The contentDetails object contains information about the event's video content, such as whether the content can be shown in an embedded video player or if it will be archived and therefore available for viewing after the event has concluded.
    #[serde(alias="contentDetails")]
    pub content_details: Option<LiveBroadcastContentDetails>,
    /// The ID that YouTube assigns to uniquely identify the broadcast.
    pub id: Option<String>,
}

impl RequestValue for LiveBroadcast {}
impl Resource for LiveBroadcast {}
impl ResponseResult for LiveBroadcast {}

impl ToParts for LiveBroadcast {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// Information about a video stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetailsVideoStream {
    /// The video stream's bitrate, in bits per second.
    #[serde(alias="bitrateBps")]
    pub bitrate_bps: String,
    /// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
    pub vendor: String,
    /// The video codec that the stream uses.
    pub codec: String,
    /// The encoded video content's width in pixels. You can calculate the video's encoding aspect ratio as width_pixels / height_pixels.
    #[serde(alias="widthPixels")]
    pub width_pixels: u32,
    /// The encoded video content's height in pixels.
    #[serde(alias="heightPixels")]
    pub height_pixels: u32,
    /// The video content's display aspect ratio, which specifies the aspect ratio in which the video should be displayed.
    #[serde(alias="aspectRatio")]
    pub aspect_ratio: f64,
    /// The amount that YouTube needs to rotate the original source content to properly display the video.
    pub rotation: String,
    /// The video stream's frame rate, in frames per second.
    #[serde(alias="frameRateFps")]
    pub frame_rate_fps: f64,
}

impl Part for VideoFileDetailsVideoStream {}


/// A thumbnail is an image representing a YouTube resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set thumbnails](struct.ThumbnailSetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    /// The thumbnail image's URL.
    pub url: Option<String>,
    /// (Optional) Width of the thumbnail image.
    pub width: Option<u32>,
    /// (Optional) Height of the thumbnail image.
    pub height: Option<u32>,
}

impl Resource for Thumbnail {}


/// A channel resource contains information about a YouTube channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channels](struct.ChannelListCall.html) (none)
/// * [update channels](struct.ChannelUpdateCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The status object encapsulates information about the privacy status of the channel.
    pub status: Option<ChannelStatus>,
    /// The invideoPromotion object encapsulates information about promotion campaign associated with the channel.
    #[serde(alias="invideoPromotion")]
    pub invideo_promotion: Option<InvideoPromotion>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channel".
    pub kind: Option<String>,
    /// The statistics object encapsulates statistics for the channel.
    pub statistics: Option<ChannelStatistics>,
    /// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
    #[serde(alias="contentOwnerDetails")]
    pub content_owner_details: Option<ChannelContentOwnerDetails>,
    /// The topicDetails object encapsulates information about Freebase topics associated with the channel.
    #[serde(alias="topicDetails")]
    pub topic_details: Option<ChannelTopicDetails>,
    /// The contentDetails object encapsulates information about the channel's content.
    #[serde(alias="contentDetails")]
    pub content_details: Option<ChannelContentDetails>,
    /// The brandingSettings object encapsulates information about the branding of the channel.
    #[serde(alias="brandingSettings")]
    pub branding_settings: Option<ChannelBrandingSettings>,
    /// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
    #[serde(alias="conversionPings")]
    pub conversion_pings: Option<ChannelConversionPings>,
    /// The snippet object contains basic details about the channel, such as its title, description, and thumbnail images.
    pub snippet: Option<ChannelSnippet>,
    /// The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process.
    #[serde(alias="auditDetails")]
    pub audit_details: Option<ChannelAuditDetails>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the channel.
    pub id: Option<String>,
    /// Localizations for different languages
    pub localizations: Option<HashMap<String, ChannelLocalization>>,
}

impl RequestValue for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}

impl ToParts for Channel {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.status.is_some() { r = r + "status,"; }
        if self.invideo_promotion.is_some() { r = r + "invideoPromotion,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.statistics.is_some() { r = r + "statistics,"; }
        if self.content_owner_details.is_some() { r = r + "contentOwnerDetails,"; }
        if self.topic_details.is_some() { r = r + "topicDetails,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.branding_settings.is_some() { r = r + "brandingSettings,"; }
        if self.conversion_pings.is_some() { r = r + "conversionPings,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.audit_details.is_some() { r = r + "auditDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        r.pop();
        r
    }
}

/// Statistics about a channel: number of subscribers, number of videos in the channel, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelStatistics {
    /// The number of comments for the channel.
    #[serde(alias="commentCount")]
    pub comment_count: i64,
    /// The number of subscribers that the channel has.
    #[serde(alias="subscriberCount")]
    pub subscriber_count: i64,
    /// The number of videos uploaded to the channel.
    #[serde(alias="videoCount")]
    pub video_count: i64,
    /// Whether or not the number of subscribers is shown for this user.
    #[serde(alias="hiddenSubscriberCount")]
    pub hidden_subscriber_count: bool,
    /// The number of times the channel has been viewed.
    #[serde(alias="viewCount")]
    pub view_count: i64,
}

impl Part for ChannelStatistics {}


/// Details about a social network post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsSocial {
    /// The resourceId object encapsulates information that identifies the resource associated with a social network post.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
    /// An image of the post's author.
    #[serde(alias="imageUrl")]
    pub image_url: String,
    /// The name of the social network.
    #[serde(alias="type")]
    pub type_: String,
    /// The URL of the social network post.
    #[serde(alias="referenceUrl")]
    pub reference_url: String,
    /// The author of the social network post.
    pub author: String,
}

impl Part for ActivityContentDetailsSocial {}


/// Channel localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelLocalization {
    /// The localized strings for channel's description.
    pub description: String,
    /// The localized strings for channel's title, read-only.
    pub title: String,
}

impl Part for ChannelLocalization {}


/// A resource id is a generic reference that points to another YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceId {
    /// The type of the API resource.
    pub kind: String,
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
    #[serde(alias="playlistId")]
    pub playlist_id: String,
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.
    #[serde(alias="videoId")]
    pub video_id: String,
}

impl Part for ResourceId {}


/// A search result contains information about a YouTube video, channel, or playlist that matches the search parameters specified in an API request. While a search result points to a uniquely identifiable resource, like a video, it does not have its own persistent data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchResult {
    /// The snippet object contains basic details about a search result, such as its title or description. For example, if the search result is a video, then the title will be the video's title and the description will be the video's description.
    pub snippet: SearchResultSnippet,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#searchResult".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The id object contains information that can be used to uniquely identify the resource that matches the search request.
    pub id: ResourceId,
}

impl Part for SearchResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list video categories](struct.VideoCategoryListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoCategoryListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource.
    pub items: Vec<VideoCategory>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for VideoCategoryListResponse {}


/// Basic details about an activity, including title, description, thumbnails, activity type and group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivitySnippet {
    /// A map of thumbnail images associated with the resource that is primarily associated with the activity. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
    /// The title of the resource primarily associated with the activity.
    pub title: String,
    /// The ID that YouTube uses to uniquely identify the channel associated with the activity.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// Channel title for the channel responsible for this activity
    #[serde(alias="channelTitle")]
    pub channel_title: String,
    /// The type of activity that the resource describes.
    #[serde(alias="type")]
    pub type_: String,
    /// The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user's activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value.
    #[serde(alias="groupId")]
    pub group_id: String,
    /// The description of the resource primarily associated with the activity.
    pub description: String,
}

impl Part for ActivitySnippet {}


/// Video processing progress and completion time estimate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProcessingDetailsProcessingProgress {
    /// An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.
    #[serde(alias="timeLeftMs")]
    pub time_left_ms: String,
    /// The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating:
    /// 100 * parts_processed / parts_total
    /// 
    /// Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.
    #[serde(alias="partsProcessed")]
    pub parts_processed: String,
    /// An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.
    #[serde(alias="partsTotal")]
    pub parts_total: String,
}

impl Part for VideoProcessingDetailsProcessingProgress {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list search](struct.SearchListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct SearchListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#searchListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of results that match the search criteria.
    pub items: Vec<SearchResult>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for SearchListResponse {}


/// Freebase topic information related to the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelTopicDetails {
    /// A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.
    #[serde(alias="topicIds")]
    pub topic_ids: Vec<String>,
}

impl Part for ChannelTopicDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list videos](struct.VideoListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of videos that match the request criteria.
    pub items: Vec<Video>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for VideoListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageTag {
    /// no description provided
    pub value: String,
}

impl Part for LanguageTag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistStatus {
    /// The playlist's privacy status.
    #[serde(alias="privacyStatus")]
    pub privacy_status: String,
}

impl Part for PlaylistStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoRating {
    /// no description provided
    pub rating: String,
    /// no description provided
    #[serde(alias="videoId")]
    pub video_id: String,
}

impl Part for VideoRating {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamSnippet {
    /// The ID that YouTube uses to uniquely identify the channel that is transmitting the stream.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The stream's description. The value cannot be longer than 10000 characters.
    pub description: String,
    /// The date and time that the stream was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// The stream's title. The value must be between 1 and 128 characters long.
    pub title: String,
}

impl Part for LiveStreamSnippet {}


/// JSON template for the status part of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelStatus {
    /// Privacy status of the channel.
    #[serde(alias="privacyStatus")]
    pub privacy_status: String,
    /// If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity.
    #[serde(alias="isLinked")]
    pub is_linked: bool,
    /// The long uploads status of this channel. See
    #[serde(alias="longUploadsStatus")]
    pub long_uploads_status: String,
}

impl Part for ChannelStatus {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channels](struct.ChannelListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ChannelListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of channels that match the request criteria.
    pub items: Vec<Channel>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for ChannelListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete channel sections](struct.ChannelSectionDeleteCall.html) (none)
/// * [update channel sections](struct.ChannelSectionUpdateCall.html) (request|response)
/// * [list channel sections](struct.ChannelSectionListCall.html) (none)
/// * [insert channel sections](struct.ChannelSectionInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSection {
    /// The snippet object contains basic details about the channel section, such as its type, style and title.
    pub snippet: Option<ChannelSectionSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The contentDetails object contains details about the channel section content, such as a list of playlists or channels featured in the section.
    #[serde(alias="contentDetails")]
    pub content_details: Option<ChannelSectionContentDetails>,
    /// The ID that YouTube uses to uniquely identify the channel section.
    pub id: Option<String>,
    /// Localizations for different languages
    pub localizations: Option<HashMap<String, ChannelSectionLocalization>>,
}

impl RequestValue for ChannelSection {}
impl Resource for ChannelSection {}
impl ResponseResult for ChannelSection {}

impl ToParts for ChannelSection {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list live broadcasts](struct.LiveBroadcastListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct LiveBroadcastListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcastListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of broadcasts that match the request criteria.
    pub items: Vec<LiveBroadcast>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for LiveBroadcastListResponse {}


/// Brief description of the live stream status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamStatus {
    /// no description provided
    #[serde(alias="streamStatus")]
    pub stream_status: String,
}

impl Part for LiveStreamStatus {}


/// Details about the live streaming metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoLiveStreamingDetails {
    /// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
    #[serde(alias="concurrentViewers")]
    pub concurrent_viewers: String,
    /// The time that the broadcast is scheduled to begin. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="scheduledStartTime")]
    pub scheduled_start_time: String,
    /// The time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.
    #[serde(alias="scheduledEndTime")]
    pub scheduled_end_time: String,
    /// The time that the broadcast actually started. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast begins.
    #[serde(alias="actualStartTime")]
    pub actual_start_time: String,
    /// The time that the broadcast actually ended. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast is over.
    #[serde(alias="actualEndTime")]
    pub actual_end_time: String,
}

impl Part for VideoLiveStreamingDetails {}


/// Ratings schemes. The country-specific ratings are mostly for movies and shows. NEXT_ID: 65
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentRating {
    /// Internal YouTube rating.
    #[serde(alias="ytRating")]
    pub yt_rating: String,
    /// Rating system for French Canadian TV - Regie du cinema
    #[serde(alias="catvfrRating")]
    pub catvfr_rating: String,
    /// Rating system in India - Central Board of Film Certification
    #[serde(alias="cbfcRating")]
    pub cbfc_rating: String,
    /// Rating system for Thailand - Board of Filmand Video Censors
    #[serde(alias="bfvcRating")]
    pub bfvc_rating: String,
    /// Rating system for Austria - Bundesministeriums f�r Unterricht, Kunst und Kultur!
    #[serde(alias="bmukkRating")]
    pub bmukk_rating: String,
    /// Rating system for Switzerland - Switzerland Rating System
    #[serde(alias="chfilmRating")]
    pub chfilm_rating: String,
    /// Rating system for Taiwan - Ministry of Culture - Tawan
    #[serde(alias="moctwRating")]
    pub moctw_rating: String,
    /// Rating system for Canadian TV - Canadian TV Classification System
    #[serde(alias="catvRating")]
    pub catv_rating: String,
    /// Rating system for Peru - Peru Rating System
    #[serde(alias="pefilmRating")]
    pub pefilm_rating: String,
    /// no description provided
    #[serde(alias="djctqRatingReasons")]
    pub djctq_rating_reasons: Vec<String>,
    /// Rating system for Argentina - Instituto Nacional de Cine y Artes Audiovisuales
    #[serde(alias="incaaRating")]
    pub incaa_rating: String,
    /// Rating system for Israel - Israel Rating System
    #[serde(alias="ilfilmRating")]
    pub ilfilm_rating: String,
    /// Rating system for Luxembourg - Commission de surveillance de la classification des films
    #[serde(alias="cscfRating")]
    pub cscf_rating: String,
    /// Rating system in Germany - Voluntary Self Regulation of the Movie Industry
    #[serde(alias="fskRating")]
    pub fsk_rating: String,
    /// Rating system in South Korea - Korea Media Rating Board
    #[serde(alias="kmrbRating")]
    pub kmrb_rating: String,
    /// Rating system in Brazil - Department of Justice, Rating, Titles and Qualification
    #[serde(alias="djctqRating")]
    pub djctq_rating: String,
    /// Rating system for Hong kong - Office for Film, Newspaper and Article Administration
    #[serde(alias="fcoRating")]
    pub fco_rating: String,
    /// Rating system for Norway - Medietilsynet
    #[serde(alias="medietilsynetRating")]
    pub medietilsynet_rating: String,
    /// Rating system for Greece - Greece Rating System
    #[serde(alias="grfilmRating")]
    pub grfilm_rating: String,
    /// Rating system for Chile - Consejo de Calificaci�n Cinematogr�fica
    #[serde(alias="cccRating")]
    pub ccc_rating: String,
    /// Rating system for Ireland - Raidi� Teilif�s �ireann
    #[serde(alias="rteRating")]
    pub rte_rating: String,
    /// Rating system in France - French Minister of Culture
    #[serde(alias="fmocRating")]
    pub fmoc_rating: String,
    /// Rating system in Japan - Eiga Rinri Kanri Iinkai
    #[serde(alias="eirinRating")]
    pub eirin_rating: String,
    /// Rating system for Portugal - Comiss�o de Classifica��o de Espect�culos
    #[serde(alias="cceRating")]
    pub cce_rating: String,
    /// Rating system for Latvia - National Film Center of Latvia
    #[serde(alias="nkclvRating")]
    pub nkclv_rating: String,
    /// Rating system for South africa - Film & Publication Board
    #[serde(alias="fpbRating")]
    pub fpb_rating: String,
    /// Rating system for Iceland - SMAIS
    #[serde(alias="smaisRating")]
    pub smais_rating: String,
    /// Canadian Home Video Rating System
    #[serde(alias="chvrsRating")]
    pub chvrs_rating: String,
    /// Rating system for Italy - Autorit� per le Garanzie nelle Comunicazioni
    #[serde(alias="agcomRating")]
    pub agcom_rating: String,
    /// Rating system for Colombia - MoC
    #[serde(alias="mocRating")]
    pub moc_rating: String,
    /// Rating system for Hungary - Rating Committee of the National Office of Film
    #[serde(alias="rcnofRating")]
    pub rcnof_rating: String,
    /// Rating system for Malaysia - Film Censorship Board of Malaysia
    #[serde(alias="fcbmRating")]
    pub fcbm_rating: String,
    /// Rating system for Netherlands - Nederlands Instituut voor de Classificatie van Audiovisuele Media
    #[serde(alias="kijkwijzerRating")]
    pub kijkwijzer_rating: String,
    /// Rating system for Singapore - Media Development Authority
    #[serde(alias="mdaRating")]
    pub mda_rating: String,
    /// Rating system for Nigeria - National Film and Video Censors Board
    #[serde(alias="nfvcbRating")]
    pub nfvcb_rating: String,
    /// Rating system for Venezuela - SiBCI
    #[serde(alias="resorteviolenciaRating")]
    pub resorteviolencia_rating: String,
    /// Rating system for France - Conseil sup�rieur de l?audiovisuel
    #[serde(alias="csaRating")]
    pub csa_rating: String,
    /// Rating system in New Zealand - Office of Film and Literature Classification
    #[serde(alias="oflcRating")]
    pub oflc_rating: String,
    /// TV Parental Guidelines rating of the content.
    #[serde(alias="tvpgRating")]
    pub tvpg_rating: String,
    /// Rating system for Bulgaria - National Film Centre
    #[serde(alias="nfrcRating")]
    pub nfrc_rating: String,
    /// Rating system for Malta - Film Age-Classification Board
    #[serde(alias="mccaaRating")]
    pub mccaa_rating: String,
    /// Rating system in Mexico - General Directorate of Radio, Television and Cinematography
    #[serde(alias="rtcRating")]
    pub rtc_rating: String,
    /// Rating system in Italy - Ministero dei Beni e delle Attivita Culturali e del Turismo
    #[serde(alias="mibacRating")]
    pub mibac_rating: String,
    /// British Board of Film Classification
    #[serde(alias="bbfcRating")]
    pub bbfc_rating: String,
    /// Rating system for Egypt - Egypt Rating System
    #[serde(alias="egfilmRating")]
    pub egfilm_rating: String,
    /// Rating system for Belgium - Belgium Rating System
    #[serde(alias="cicfRating")]
    pub cicf_rating: String,
    /// Rating system for Poland - National Broadcasting Council
    #[serde(alias="nbcplRating")]
    pub nbcpl_rating: String,
    /// Rating system for Maldives - National Bureau of Classification
    #[serde(alias="nbcRating")]
    pub nbc_rating: String,
    /// Motion Picture Association of America rating for the content.
    #[serde(alias="mpaaRating")]
    pub mpaa_rating: String,
    /// Rating system in Ireland - Irish Film Classification Office
    #[serde(alias="ifcoRating")]
    pub ifco_rating: String,
    /// Rating system in Australia - Australian Classification Board
    #[serde(alias="acbRating")]
    pub acb_rating: String,
    /// Rating system for Estonia - Estonia Rating System
    #[serde(alias="eefilmRating")]
    pub eefilm_rating: String,
    /// Rating system for Czech republic - Czech republic Rating System
    #[serde(alias="czfilmRating")]
    pub czfilm_rating: String,
    /// Rating system for Indonesia - Lembaga Sensor Film
    #[serde(alias="lsfRating")]
    pub lsf_rating: String,
    /// Rating system in Russia
    #[serde(alias="russiaRating")]
    pub russia_rating: String,
    /// Rating system for Kenya - Kenya Film Classification Board
    #[serde(alias="kfcbRating")]
    pub kfcb_rating: String,
    /// Rating system for Philippines - MOVIE AND TELEVISION REVIEW AND CLASSIFICATION BOARD
    #[serde(alias="mtrcbRating")]
    pub mtrcb_rating: String,
    /// Rating system for Chile - Asociaci�n Nacional de Televisi�n
    #[serde(alias="anatelRating")]
    pub anatel_rating: String,
    /// Rating system for Sweden - Statens medier�d (National Media Council)
    #[serde(alias="smsaRating")]
    pub smsa_rating: String,
    /// Rating system for Romania - CONSILIUL NATIONAL AL AUDIOVIZUALULUI - CNA
    #[serde(alias="cnaRating")]
    pub cna_rating: String,
    /// Rating system in Spain - Instituto de Cinematografia y de las Artes Audiovisuales
    #[serde(alias="icaaRating")]
    pub icaa_rating: String,
    /// Rating system for Denmark - The Media Council for Children and Young People
    #[serde(alias="mccypRating")]
    pub mccyp_rating: String,
    /// Rating system for Slovakia - Slovakia Rating System
    #[serde(alias="skfilmRating")]
    pub skfilm_rating: String,
    /// Rating system for Finland - Finnish Centre for Media Education and Audiovisual Media
    #[serde(alias="mekuRating")]
    pub meku_rating: String,
}

impl Part for ContentRating {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](struct.ActivityListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct ActivityListResponse {
    /// Serialized EventId of the request which produced this response.
    #[serde(alias="eventId")]
    pub event_id: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(alias="nextPageToken")]
    pub next_page_token: String,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#activityListResponse".
    pub kind: String,
    /// The visitorId identifies the visitor.
    #[serde(alias="visitorId")]
    pub visitor_id: String,
    /// A list of activities, or events, that match the request criteria.
    pub items: Vec<Activity>,
    /// no description provided
    #[serde(alias="tokenPagination")]
    pub token_pagination: TokenPagination,
    /// Etag of this resource.
    pub etag: String,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(alias="prevPageToken")]
    pub prev_page_token: String,
    /// no description provided
    #[serde(alias="pageInfo")]
    pub page_info: PageInfo,
}

impl ResponseResult for ActivityListResponse {}


/// An activity resource contains information about an action that a particular channel, or user, has taken on YouTube.The actions reported in activity feeds include rating a video, sharing a video, marking a video as a favorite, commenting on a video, uploading a video, and so forth. Each activity resource identifies the type of action, the channel associated with the action, and the resource(s) associated with the action, such as the video that was rated or uploaded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert activities](struct.ActivityInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The snippet object contains basic details about the activity, including the activity's type and group ID.
    pub snippet: Option<ActivitySnippet>,
    /// The contentDetails object contains information about the content associated with the activity. For example, if the snippet.type value is videoRated, then the contentDetails object's content identifies the rated video.
    #[serde(alias="contentDetails")]
    pub content_details: Option<ActivityContentDetails>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#activity".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the activity.
    pub id: Option<String>,
}

impl RequestValue for Activity {}
impl ResponseResult for Activity {}

impl ToParts for Activity {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// Basic details about a subscription's subscriber including title, description, channel ID and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionSubscriberSnippet {
    /// The channel ID of the subscriber.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The description of the subscriber.
    pub description: String,
    /// Thumbnails for this subscriber.
    pub thumbnails: ThumbnailDetails,
    /// The title of the subscriber.
    pub title: String,
}

impl Part for SubscriptionSubscriberSnippet {}


/// Branding properties for images associated with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageSettings {
    /// Banner image. TV size medium resolution (1280x720).
    #[serde(alias="bannerTvMediumImageUrl")]
    pub banner_tv_medium_image_url: String,
    /// The image map script for the large banner image.
    #[serde(alias="largeBrandedBannerImageImapScript")]
    pub large_branded_banner_image_imap_script: LocalizedProperty,
    /// Banner image. Mobile size (640x175).
    #[serde(alias="bannerMobileImageUrl")]
    pub banner_mobile_image_url: String,
    /// The URL for the 640px by 70px banner image that appears below the video player in the default view of the video watch page.
    #[serde(alias="smallBrandedBannerImageUrl")]
    pub small_branded_banner_image_url: LocalizedProperty,
    /// Banner image. Tablet size high resolution (2276x377).
    #[serde(alias="bannerTabletHdImageUrl")]
    pub banner_tablet_hd_image_url: String,
    /// Banner image. Tablet size low resolution (1138x188).
    #[serde(alias="bannerTabletLowImageUrl")]
    pub banner_tablet_low_image_url: String,
    /// Banner image. Mobile size medium/high resolution (960x263).
    #[serde(alias="bannerMobileMediumHdImageUrl")]
    pub banner_mobile_medium_hd_image_url: String,
    /// The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.
    #[serde(alias="trackingImageUrl")]
    pub tracking_image_url: String,
    /// Banner image. Mobile size high resolution (1440x395).
    #[serde(alias="bannerMobileExtraHdImageUrl")]
    pub banner_mobile_extra_hd_image_url: String,
    /// Banner image. Tablet size (1707x283).
    #[serde(alias="bannerTabletImageUrl")]
    pub banner_tablet_image_url: String,
    /// Banner image. Mobile size low resolution (320x88).
    #[serde(alias="bannerMobileLowImageUrl")]
    pub banner_mobile_low_image_url: String,
    /// Banner image. TV size extra high resolution (2120x1192).
    #[serde(alias="bannerTvImageUrl")]
    pub banner_tv_image_url: String,
    /// Banner image. TV size low resolution (854x480).
    #[serde(alias="bannerTvLowImageUrl")]
    pub banner_tv_low_image_url: String,
    /// Banner image. Tablet size extra high resolution (2560x424).
    #[serde(alias="bannerTabletExtraHdImageUrl")]
    pub banner_tablet_extra_hd_image_url: String,
    /// The URL for the 854px by 70px image that appears below the video player in the expanded video view of the video watch page.
    #[serde(alias="largeBrandedBannerImageUrl")]
    pub large_branded_banner_image_url: LocalizedProperty,
    /// Banner image. TV size high resolution (1920x1080).
    #[serde(alias="bannerTvHighImageUrl")]
    pub banner_tv_high_image_url: String,
    /// The URL for the background image shown on the video watch page. The image should be 1200px by 615px, with a maximum file size of 128k.
    #[serde(alias="backgroundImageUrl")]
    pub background_image_url: LocalizedProperty,
    /// The image map script for the small banner image.
    #[serde(alias="smallBrandedBannerImageImapScript")]
    pub small_branded_banner_image_imap_script: LocalizedProperty,
    /// Banner image. Desktop size (1060x175).
    #[serde(alias="bannerImageUrl")]
    pub banner_image_url: String,
    /// Banner image. Mobile size high resolution (1280x360).
    #[serde(alias="bannerMobileHdImageUrl")]
    pub banner_mobile_hd_image_url: String,
    /// This is used only in update requests; if it's set, we use this URL to generate all of the above banner URLs.
    #[serde(alias="bannerExternalUrl")]
    pub banner_external_url: String,
    /// The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels.
    #[serde(alias="watchIconImageUrl")]
    pub watch_icon_image_url: String,
}

impl Part for ImageSettings {}


/// Details about a resource which is being promoted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsPromotedItem {
    /// The type of call-to-action, a message to the user indicating action that can be taken.
    #[serde(alias="ctaType")]
    pub cta_type: String,
    /// The URL the client should fetch to request a promoted item.
    #[serde(alias="adTag")]
    pub ad_tag: String,
    /// The URL the client should direct the user to, if the user chooses to visit the advertiser's website.
    #[serde(alias="destinationUrl")]
    pub destination_url: String,
    /// The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown.
    #[serde(alias="forecastingUrl")]
    pub forecasting_url: Vec<String>,
    /// The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item.
    #[serde(alias="impressionUrl")]
    pub impression_url: Vec<String>,
    /// The URL the client should ping to indicate that the user was shown this promoted item.
    #[serde(alias="creativeViewUrl")]
    pub creative_view_url: String,
    /// The ID that YouTube uses to uniquely identify the promoted video.
    #[serde(alias="videoId")]
    pub video_id: String,
    /// The text description to accompany the promoted item.
    #[serde(alias="descriptionText")]
    pub description_text: String,
    /// The custom call-to-action button text. If specified, it will override the default button text for the cta_type.
    #[serde(alias="customCtaButtonText")]
    pub custom_cta_button_text: String,
    /// The URL the client should ping to indicate that the user clicked through on this promoted item.
    #[serde(alias="clickTrackingUrl")]
    pub click_tracking_url: String,
}

impl Part for ActivityContentDetailsPromotedItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoConversionPings {
    /// Pings that the app shall fire for a video (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
    pub pings: Vec<VideoConversionPing>,
}

impl Part for VideoConversionPings {}


/// Details about a channel bulletin post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsBulletin {
    /// The resourceId object contains information that identifies the resource associated with a bulletin post.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsBulletin {}


/// An i18nLanguage resource identifies a UI language currently supported by YouTube.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n languages](struct.I18nLanguageListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct I18nLanguage {
    /// The snippet object contains basic details about the i18n language, such as language code and human-readable name.
    pub snippet: Option<I18nLanguageSnippet>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguage".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the i18n language.
    pub id: Option<String>,
}

impl Resource for I18nLanguage {}

impl ToParts for I18nLanguage {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    /// no description provided
    pub language: String,
    /// no description provided
    pub value: String,
}

impl Part for LocalizedString {}


/// Information about an audio stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetailsAudioStream {
    /// The audio stream's bitrate, in bits per second.
    #[serde(alias="bitrateBps")]
    pub bitrate_bps: String,
    /// The audio codec that the stream uses.
    pub codec: String,
    /// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
    pub vendor: String,
    /// The number of audio channels that the stream contains.
    #[serde(alias="channelCount")]
    pub channel_count: u32,
}

impl Part for VideoFileDetailsAudioStream {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAgeGating {
    /// Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present.
    pub restricted: bool,
    /// Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.
    #[serde(alias="alcoholContent")]
    pub alcohol_content: bool,
    /// Video game rating, if any.
    #[serde(alias="videoGameRating")]
    pub video_game_rating: String,
}

impl Part for VideoAgeGating {}


/// Freebase topic information related to the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoTopicDetails {
    /// A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.
    #[serde(alias="topicIds")]
    pub topic_ids: Vec<String>,
    /// Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.
    #[serde(alias="relevantTopicIds")]
    pub relevant_topic_ids: Vec<String>,
}

impl Part for VideoTopicDetails {}


/// Statistics about the video, such as the number of times the video was viewed or liked.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoStatistics {
    /// The number of comments for the video.
    #[serde(alias="commentCount")]
    pub comment_count: i64,
    /// The number of times the video has been viewed.
    #[serde(alias="viewCount")]
    pub view_count: i64,
    /// The number of users who currently have the video marked as a favorite video.
    #[serde(alias="favoriteCount")]
    pub favorite_count: i64,
    /// The number of users who have indicated that they disliked the video by giving it a negative rating.
    #[serde(alias="dislikeCount")]
    pub dislike_count: i64,
    /// The number of users who have indicated that they liked the video by giving it a positive rating.
    #[serde(alias="likeCount")]
    pub like_count: i64,
}

impl Part for VideoStatistics {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoConversionPing {
    /// Defines the context of the ping.
    pub context: String,
    /// The url (without the schema) that the app shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D like%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA%3Bytvid%3DUrIaJUvIQDg&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping.
    #[serde(alias="conversionUrl")]
    pub conversion_url: String,
}

impl Part for VideoConversionPing {}


/// A videoCategory resource identifies a category that has been or could be associated with uploaded videos.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct VideoCategory {
    /// The snippet object contains basic details about the video category, including its title.
    pub snippet: VideoCategorySnippet,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategory".
    pub kind: String,
    /// Etag of this resource.
    pub etag: String,
    /// The ID that YouTube uses to uniquely identify the video category.
    pub id: String,
}

impl Part for VideoCategory {}


/// Basic details about a playlist, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemSnippet {
    /// The ID that YouTube uses to uniquely identify the user that added the item to the playlist.
    #[serde(alias="channelId")]
    pub channel_id: String,
    /// The item's description.
    pub description: String,
    /// The item's title.
    pub title: String,
    /// The id object contains information that can be used to uniquely identify the resource that is included in the playlist as the playlist item.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
    /// The ID that YouTube uses to uniquely identify the playlist that the playlist item is in.
    #[serde(alias="playlistId")]
    pub playlist_id: String,
    /// The date and time that the item was added to the playlist. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    #[serde(alias="publishedAt")]
    pub published_at: String,
    /// Channel title for the channel that the playlist item belongs to.
    #[serde(alias="channelTitle")]
    pub channel_title: String,
    /// The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.
    pub position: u32,
    /// A map of thumbnail images associated with the playlist item. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    pub thumbnails: ThumbnailDetails,
}

impl Part for PlaylistItemSnippet {}


/// Information about a video that was marked as a favorite video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsFavorite {
    /// The resourceId object contains information that identifies the resource that was marked as a favorite.
    #[serde(alias="resourceId")]
    pub resource_id: ResourceId,
}

impl Part for ActivityContentDetailsFavorite {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistPlayer {
    /// An <iframe> tag that embeds a player that will play the playlist.
    #[serde(alias="embedHtml")]
    pub embed_html: String,
}

impl Part for PlaylistPlayer {}


/// A single tag suggestion with it's relevance information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSuggestionsTagSuggestion {
    /// A set of video categories for which the tag is relevant. You can use this information to display appropriate tag suggestions based on the video category that the video uploader associates with the video. By default, tag suggestions are relevant for all categories if there are no restricts defined for the keyword.
    #[serde(alias="categoryRestricts")]
    pub category_restricts: Vec<String>,
    /// The keyword tag suggested for the video.
    pub tag: String,
}

impl Part for VideoSuggestionsTagSuggestion {}


/// Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSuggestions {
    /// A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.
    #[serde(alias="processingErrors")]
    pub processing_errors: Vec<String>,
    /// A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.
    #[serde(alias="tagSuggestions")]
    pub tag_suggestions: Vec<VideoSuggestionsTagSuggestion>,
    /// A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
    #[serde(alias="editorSuggestions")]
    pub editor_suggestions: Vec<String>,
    /// A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
    #[serde(alias="processingWarnings")]
    pub processing_warnings: Vec<String>,
    /// A list of suggestions that may improve YouTube's ability to process the video.
    #[serde(alias="processingHints")]
    pub processing_hints: Vec<String>,
}

impl Part for VideoSuggestions {}


/// Rights management policy for YouTube resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// A list of region codes that identify countries where the default policy do not apply.
    pub exception: Vec<String>,
    /// The value of allowed indicates whether the access to the policy is allowed or denied by default.
    pub allowed: bool,
}

impl Part for AccessPolicy {}


/// A channel banner returned as the response to a channel_banner.insert call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert channel banners](struct.ChannelBannerInsertCall.html) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelBannerResource {
    /// The URL of this banner image.
    pub url: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelBannerResource".
    pub kind: Option<String>,
    /// Etag of this resource.
    pub etag: Option<String>,
}

impl RequestValue for ChannelBannerResource {}
impl ResponseResult for ChannelBannerResource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistContentDetails {
    /// The number of videos in the playlist.
    #[serde(alias="itemCount")]
    pub item_count: u32,
}

impl Part for PlaylistContentDetails {}


/// Paging details for lists of resources, including total number of items available and number of resources returned in a single page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Deserialize)]
pub struct PageInfo {
    /// The number of results included in the API response.
    #[serde(alias="resultsPerPage")]
    pub results_per_page: i32,
    /// The total number of results in the result set.
    #[serde(alias="totalResults")]
    pub total_results: i32,
}

impl Part for PageInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentDetailsRelatedPlaylists {
    /// The ID of the playlist that contains the channel"s uploaded videos. Use the  videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos.
    pub uploads: String,
    /// The ID of the playlist that contains the channel"s watch history. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
    #[serde(alias="watchHistory")]
    pub watch_history: String,
    /// The ID of the playlist that contains the channel"s liked videos. Use the   playlistItems.insert and  playlistItems.delete to add or remove items from that list.
    pub likes: String,
    /// The ID of the playlist that contains the channel"s favorite videos. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
    pub favorites: String,
    /// The ID of the playlist that contains the channel"s watch later playlist. Use the playlistItems.insert and  playlistItems.delete to add or remove items from that list.
    #[serde(alias="watchLater")]
    pub watch_later: String,
}

impl NestedType for ChannelContentDetailsRelatedPlaylists {}
impl Part for ChannelContentDetailsRelatedPlaylists {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *i18nLanguage* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_languages();
/// # }
/// ```
pub struct I18nLanguageMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for I18nLanguageMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nLanguageMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of supported languages.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more i18nLanguage resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    pub fn list(&self, part: &str) -> I18nLanguageListCall<'a, C, NC, A> {
        I18nLanguageListCall {
            hub: self.hub,
            _part: part.to_string(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channelBanner* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.channel_banners();
/// # }
/// ```
pub struct ChannelBannerMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ChannelBannerMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelBannerMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a channel banner image to YouTube. This method represents the first two steps in a three-step process to update the banner image for a channel:
    /// 
    /// - Call the channelBanners.insert method to upload the binary image data to YouTube. The image must have a 16:9 aspect ratio and be at least 2120x1192 pixels.
    /// - Extract the url property's value from the response that the API returns for step 1.
    /// - Call the channels.update method to update the channel's branding settings. Set the brandingSettings.image.bannerExternalUrl property's value to the URL obtained in step 2.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &ChannelBannerResource) -> ChannelBannerInsertCall<'a, C, NC, A> {
        ChannelBannerInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channelSection* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channel_sections();
/// # }
/// ```
pub struct ChannelSectionMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ChannelSectionMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns channelSection resources that match the API request criteria.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> ChannelSectionListCall<'a, C, NC, A> {
        ChannelSectionListCall {
            hub: self.hub,
            _part: part.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _id: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a channelSection for the authenticated user's channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &ChannelSection) -> ChannelSectionInsertCall<'a, C, NC, A> {
        ChannelSectionInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a channelSection.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube channelSection ID for the resource that is being deleted. In a channelSection resource, the id property specifies the YouTube channelSection ID.
    pub fn delete(&self, id: &str) -> ChannelSectionDeleteCall<'a, C, NC, A> {
        ChannelSectionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a channelSection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &ChannelSection) -> ChannelSectionUpdateCall<'a, C, NC, A> {
        ChannelSectionUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *guideCategory* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.guide_categories();
/// # }
/// ```
pub struct GuideCategoryMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for GuideCategoryMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> GuideCategoryMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of categories that can be associated with YouTube channels.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more guideCategory resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a guideCategory resource, the snippet property contains other properties, such as the category's title. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> GuideCategoryListCall<'a, C, NC, A> {
        GuideCategoryListCall {
            hub: self.hub,
            _part: part.to_string(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *playlist* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlists();
/// # }
/// ```
pub struct PlaylistMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for PlaylistMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a playlist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &Playlist) -> PlaylistInsertCall<'a, C, NC, A> {
        PlaylistInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, status, and contentDetails.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn list(&self, part: &str) -> PlaylistListCall<'a, C, NC, A> {
        PlaylistListCall {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a playlist.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube playlist ID for the playlist that is being deleted. In a playlist resource, the id property specifies the playlist's ID.
    pub fn delete(&self, id: &str) -> PlaylistDeleteCall<'a, C, NC, A> {
        PlaylistDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a playlist. For example, you could change a playlist's title, description, or privacy status.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &Playlist) -> PlaylistUpdateCall<'a, C, NC, A> {
        PlaylistUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *thumbnail* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `set(...)`
/// // to build up your call.
/// let rb = hub.thumbnails();
/// # }
/// ```
pub struct ThumbnailMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ThumbnailMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ThumbnailMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a custom video thumbnail to YouTube and sets it for a video.
    /// 
    /// # Arguments
    ///
    /// * `videoId` - The videoId parameter specifies a YouTube video ID for which the custom video thumbnail is being provided.
    pub fn set(&self, video_id: &str) -> ThumbnailSetCall<'a, C, NC, A> {
        ThumbnailSetCall {
            hub: self.hub,
            _video_id: video_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *video* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get_rating(...)`, `insert(...)`, `list(...)`, `rate(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.videos();
/// # }
/// ```
pub struct VideoMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for VideoMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of videos that match the API request parameters.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more video resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, fileDetails, liveStreamingDetails, localizations, player, processingDetails, recordingDetails, statistics, status, suggestions, and topicDetails.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn list(&self, part: &str) -> VideoListCall<'a, C, NC, A> {
        VideoListCall {
            hub: self.hub,
            _part: part.to_string(),
            _video_category_id: Default::default(),
            _region_code: Default::default(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_rating: Default::default(),
            _max_results: Default::default(),
            _locale: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _chart: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a like or dislike rating to a video or remove a rating from a video.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube video ID of the video that is being rated or having its rating removed.
    /// * `rating` - Specifies the rating to record.
    pub fn rate(&self, id: &str, rating: &str) -> VideoRateCall<'a, C, NC, A> {
        VideoRateCall {
            hub: self.hub,
            _id: id.to_string(),
            _rating: rating.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the ratings that the authorized user gave to a list of specified videos.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) for which you are retrieving rating data. In a video resource, the id property specifies the video's ID.
    pub fn get_rating(&self, id: &str) -> VideoGetRatingCall<'a, C, NC, A> {
        VideoGetRatingCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a YouTube video.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube video ID for the resource that is being deleted. In a video resource, the id property specifies the video's ID.
    pub fn delete(&self, id: &str) -> VideoDeleteCall<'a, C, NC, A> {
        VideoDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a video's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &Video) -> VideoUpdateCall<'a, C, NC, A> {
        VideoUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a video to YouTube and optionally sets the video's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &Video) -> VideoInsertCall<'a, C, NC, A> {
        VideoInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _stabilize: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _notify_subscribers: Default::default(),
            _auto_levels: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subscription* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.subscriptions();
/// # }
/// ```
pub struct SubscriptionMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for SubscriptionMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a subscription for the authenticated user's channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &Subscription) -> SubscriptionInsertCall<'a, C, NC, A> {
        SubscriptionInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns subscription resources that match the API request criteria.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> SubscriptionListCall<'a, C, NC, A> {
        SubscriptionListCall {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _order: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_subscribers: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _for_channel_id: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscription.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube subscription ID for the resource that is being deleted. In a subscription resource, the id property specifies the YouTube subscription ID.
    pub fn delete(&self, id: &str) -> SubscriptionDeleteCall<'a, C, NC, A> {
        SubscriptionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *search* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.search();
/// # }
/// ```
pub struct SearchMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for SearchMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> SearchMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a search result, the snippet property contains other properties that identify the result's title, description, and so forth. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> SearchListCall<'a, C, NC, A> {
        SearchListCall {
            hub: self.hub,
            _part: part.to_string(),
            _video_type: Default::default(),
            _video_syndicated: Default::default(),
            _video_license: Default::default(),
            _video_embeddable: Default::default(),
            _video_duration: Default::default(),
            _video_dimension: Default::default(),
            _video_definition: Default::default(),
            _video_category_id: Default::default(),
            _video_caption: Default::default(),
            _type_: Default::default(),
            _topic_id: Default::default(),
            _safe_search: Default::default(),
            _relevance_language: Default::default(),
            _related_to_video_id: Default::default(),
            _region_code: Default::default(),
            _q: Default::default(),
            _published_before: Default::default(),
            _published_after: Default::default(),
            _page_token: Default::default(),
            _order: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _max_results: Default::default(),
            _location_radius: Default::default(),
            _location: Default::default(),
            _for_mine: Default::default(),
            _for_content_owner: Default::default(),
            _event_type: Default::default(),
            _channel_type: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *i18nRegion* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_regions();
/// # }
/// ```
pub struct I18nRegionMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for I18nRegionMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nRegionMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of supported regions.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more i18nRegion resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    pub fn list(&self, part: &str) -> I18nRegionListCall<'a, C, NC, A> {
        I18nRegionListCall {
            hub: self.hub,
            _part: part.to_string(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveStream* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_streams();
/// # }
/// ```
pub struct LiveStreamMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for LiveStreamMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &LiveStream) -> LiveStreamUpdateCall<'a, C, NC, A> {
        LiveStreamUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a video stream.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube live stream ID for the resource that is being deleted.
    pub fn delete(&self, id: &str) -> LiveStreamDeleteCall<'a, C, NC, A> {
        LiveStreamDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of video streams that match the API request parameters.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status.
    pub fn list(&self, part: &str) -> LiveStreamListCall<'a, C, NC, A> {
        LiveStreamListCall {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &LiveStream) -> LiveStreamInsertCall<'a, C, NC, A> {
        LiveStreamInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ChannelMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &Channel) -> ChannelUpdateCall<'a, C, NC, A> {
        ChannelUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of zero or more channel resources that match the request criteria.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, statistics, topicDetails, and invideoPromotion.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set part=contentDetails, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> ChannelListCall<'a, C, NC, A> {
        ChannelListCall {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_subscribers: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _managed_by_me: Default::default(),
            _id: Default::default(),
            _for_username: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *playlistItem* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlist_items();
/// # }
/// ```
pub struct PlaylistItemMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for PlaylistItemMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a playlist item.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube playlist item ID for the playlist item that is being deleted. In a playlistItem resource, the id property specifies the playlist item's ID.
    pub fn delete(&self, id: &str) -> PlaylistItemDeleteCall<'a, C, NC, A> {
        PlaylistItemDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn list(&self, part: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        PlaylistItemListCall {
            hub: self.hub,
            _part: part.to_string(),
            _video_id: Default::default(),
            _playlist_id: Default::default(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a resource to a playlist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &PlaylistItem) -> PlaylistItemInsertCall<'a, C, NC, A> {
        PlaylistItemInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a playlist item. For example, you could update the item's position in the playlist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &PlaylistItem) -> PlaylistItemUpdateCall<'a, C, NC, A> {
        PlaylistItemUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *watermark* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `set(...)` and `unset(...)`
/// // to build up your call.
/// let rb = hub.watermarks();
/// # }
/// ```
pub struct WatermarkMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for WatermarkMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a watermark image to YouTube and sets it for a channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `channelId` - The channelId parameter specifies a YouTube channel ID for which the watermark is being provided.
    pub fn set(&self, request: &InvideoBranding, channel_id: &str) -> WatermarkSetCall<'a, C, NC, A> {
        WatermarkSetCall {
            hub: self.hub,
            _request: request.clone(),
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a watermark.
    /// 
    /// # Arguments
    ///
    /// * `channelId` - The channelId parameter specifies a YouTube channel ID for which the watermark is being unset.
    pub fn unset(&self, channel_id: &str) -> WatermarkUnsetCall<'a, C, NC, A> {
        WatermarkUnsetCall {
            hub: self.hub,
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveBroadcast* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bind(...)`, `control(...)`, `delete(...)`, `insert(...)`, `list(...)`, `transition(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_broadcasts();
/// # }
/// ```
pub struct LiveBroadcastMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for LiveBroadcastMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Controls the settings for a slate that can be displayed in the broadcast stream.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube live broadcast ID that uniquely identifies the broadcast in which the slate is being updated.
    /// * `part` - The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn control(&self, id: &str, part: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        LiveBroadcastControlCall {
            hub: self.hub,
            _id: id.to_string(),
            _part: part.to_string(),
            _walltime: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _offset_time_ms: Default::default(),
            _display_slate: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource's contentDetails object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: &LiveBroadcast) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        LiveBroadcastUpdateCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a broadcast.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &LiveBroadcast) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        LiveBroadcastInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the unique ID of the broadcast that is being bound to a video stream.
    /// * `part` - The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn bind(&self, id: &str, part: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        LiveBroadcastBindCall {
            hub: self.hub,
            _id: id.to_string(),
            _part: part.to_string(),
            _stream_id: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of YouTube broadcasts that match the API request parameters.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn list(&self, part: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        LiveBroadcastListCall {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _broadcast_status: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a broadcast.
    /// 
    /// # Arguments
    ///
    /// * `id` - The id parameter specifies the YouTube live broadcast ID for the resource that is being deleted.
    pub fn delete(&self, id: &str) -> LiveBroadcastDeleteCall<'a, C, NC, A> {
        LiveBroadcastDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast's status to testing, YouTube starts to transmit video to that broadcast's monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active.
    /// 
    /// # Arguments
    ///
    /// * `broadcastStatus` - The broadcastStatus parameter identifies the state to which the broadcast is changing. Note that to transition a broadcast to either the testing or live state, the status.streamStatus must be active for the stream that the broadcast is bound to.
    /// * `id` - The id parameter specifies the unique ID of the broadcast that is transitioning to another status.
    /// * `part` - The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn transition(&self, broadcast_status: &str, id: &str, part: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        LiveBroadcastTransitionCall {
            hub: self.hub,
            _broadcast_status: broadcast_status.to_string(),
            _id: id.to_string(),
            _part: part.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *videoCategory* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.video_categories();
/// # }
/// ```
pub struct VideoCategoryMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for VideoCategoryMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoCategoryMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of categories that can be associated with YouTube videos.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet.
    pub fn list(&self, part: &str) -> VideoCategoryListCall<'a, C, NC, A> {
        VideoCategoryListCall {
            hub: self.hub,
            _part: part.to_string(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the `YouTube` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate "yup-oauth2" as oauth2;
/// extern crate "google-youtube3" as youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> MethodsBuilder for ActivityMethods<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityMethods<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user's subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.
    /// 
    /// # Arguments
    ///
    /// * `part` - The part parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    ///            If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &str) -> ActivityListCall<'a, C, NC, A> {
        ActivityListCall {
            hub: self.hub,
            _part: part.to_string(),
            _region_code: Default::default(),
            _published_before: Default::default(),
            _published_after: Default::default(),
            _page_token: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _home: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel's behalf.)
    /// 
    /// Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API's videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: &Activity) -> ActivityInsertCall<'a, C, NC, A> {
        ActivityInsertCall {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns a list of supported languages.
///
/// A builder for the *list* method supported by a *i18nLanguage* resource.
/// It is not used directly, but through a `I18nLanguageMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.i18n_languages().list("part")
///              .hl("diam")
///              .doit();
/// # }
/// ```
pub struct I18nLanguageListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for I18nLanguageListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nLanguageListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, I18nLanguageListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.i18nLanguages.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        for &field in ["alt", "part", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/i18nLanguages".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// 
    /// The part parameter specifies a comma-separated list of one or more i18nLanguage resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    pub fn part(mut self, new_value: &str) -> I18nLanguageListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.
    pub fn hl(mut self, new_value: &str) -> I18nLanguageListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> I18nLanguageListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> I18nLanguageListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> I18nLanguageListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Uploads a channel banner image to YouTube. This method represents the first two steps in a three-step process to update the banner image for a channel:
/// 
/// - Call the channelBanners.insert method to upload the binary image data to YouTube. The image must have a 16:9 aspect ratio and be at least 2120x1192 pixels.
/// - Extract the url property's value from the response that the API returns for step 1.
/// - Call the channels.update method to update the channel's branding settings. Set the brandingSettings.image.bannerExternalUrl property's value to the URL obtained in step 2.
///
/// A builder for the *insert* method supported by a *channelBanner* resource.
/// It is not used directly, but through a `ChannelBannerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::ChannelBannerResource;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ChannelBannerResource = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_banners().insert(&req)
///              .on_behalf_of_content_owner("ipsum")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct ChannelBannerInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelBannerResource,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelBannerInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelBannerInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, ChannelBannerResource)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channelBanners.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = if protocol == "simple" {
                "https://www.googleapis.com/upload/youtube/v3/channelBanners/insert".to_string()
            } else if protocol == "resumable" {
                "https://www.googleapis.com/resumable/upload/youtube/v3/channelBanners/insert".to_string()
            } else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let mut response = hyper::client::Response::new(Box::new(cmn::DummyNetworkStream));
                    match response {
                        Ok(ref mut res) => {
                            res.status = hyper::status::StatusCode::Ok;
                            res.headers.set(Location(upload_url.as_ref().unwrap().clone()))
                        }
                        _ => unreachable!(),
                    }
                    response
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 6291456 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 6291456))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone())
                        .header(content_type)
                        .body(&mut body_reader);
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 6291456 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 6291456))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(url);
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 6MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, ChannelBannerResource)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL 
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 6MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, ChannelBannerResource)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &ChannelBannerResource) -> ChannelBannerInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelBannerInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelBannerInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelBannerInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelBannerInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns channelSection resources that match the API request criteria.
///
/// A builder for the *list* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_sections().list("part")
///              .on_behalf_of_content_owner("et")
///              .mine(false)
///              .id("aliquyam")
///              .channel_id("sea")
///              .doit();
/// # }
/// ```
pub struct ChannelSectionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _id: Option<String>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelSectionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ChannelSectionListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channelSections.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._channel_id {
            params.push(("channelId", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner", "mine", "id", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/channelSections".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> ChannelSectionListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's channelSections.
    pub fn mine(mut self, new_value: bool) -> ChannelSectionListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channelSection ID(s) for the resource(s) that are being retrieved. In a channelSection resource, the id property specifies the YouTube channelSection ID.
    pub fn id(mut self, new_value: &str) -> ChannelSectionListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a YouTube channel ID. The API will only return that channel's channelSections.
    pub fn channel_id(mut self, new_value: &str) -> ChannelSectionListCall<'a, C, NC, A> {
        self._channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelSectionListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelSectionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Adds a channelSection for the authenticated user's channel.
///
/// A builder for the *insert* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::ChannelSection;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ChannelSection = Default::default();
/// req.snippet = Default::default(); // is ChannelSectionSnippet
/// req.content_details = Default::default(); // is ChannelSectionContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_sections().insert(&req)
///              .on_behalf_of_content_owner_channel("Lorem")
///              .on_behalf_of_content_owner("eos")
///              .doit();
/// # }
/// ```
pub struct ChannelSectionInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelSection,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelSectionInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ChannelSection)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channelSections.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/channelSections".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    pub fn request(mut self, new_value: &ChannelSection) -> ChannelSectionInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and contentDetails.
    pub fn part(mut self, new_value: &str) -> ChannelSectionInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> ChannelSectionInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelSectionInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelSectionInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a channelSection.
///
/// A builder for the *delete* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_sections().delete("id")
///              .on_behalf_of_content_owner("sadipscing")
///              .doit();
/// # }
/// ```
pub struct ChannelSectionDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelSectionDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channelSections.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/channelSections".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube channelSection ID for the resource that is being deleted. In a channelSection resource, the id property specifies the YouTube channelSection ID.
    pub fn id(mut self, new_value: &str) -> ChannelSectionDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelSectionDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelSectionDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Update a channelSection.
///
/// A builder for the *update* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::ChannelSection;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: ChannelSection = Default::default();
/// req.snippet = Default::default(); // is ChannelSectionSnippet
/// req.content_details = Default::default(); // is ChannelSectionContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_sections().update(&req)
///              .on_behalf_of_content_owner("dolor")
///              .doit();
/// # }
/// ```
pub struct ChannelSectionUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelSection,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelSectionUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ChannelSection)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channelSections.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/channelSections".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    pub fn request(mut self, new_value: &ChannelSection) -> ChannelSectionUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and contentDetails.
    pub fn part(mut self, new_value: &str) -> ChannelSectionUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelSectionUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelSectionUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of categories that can be associated with YouTube channels.
///
/// A builder for the *list* method supported by a *guideCategory* resource.
/// It is not used directly, but through a `GuideCategoryMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.guide_categories().list("part")
///              .region_code("elitr")
///              .id("amet")
///              .hl("no")
///              .doit();
/// # }
/// ```
pub struct GuideCategoryListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _region_code: Option<String>,
    _id: Option<String>,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for GuideCategoryListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> GuideCategoryListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, GuideCategoryListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.guideCategories.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._region_code {
            params.push(("regionCode", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        for &field in ["alt", "part", "regionCode", "id", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/guideCategories".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// 
    /// The part parameter specifies a comma-separated list of one or more guideCategory resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a guideCategory resource, the snippet property contains other properties, such as the category's title. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> GuideCategoryListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return the list of guide categories available in the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.
    pub fn region_code(mut self, new_value: &str) -> GuideCategoryListCall<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channel category ID(s) for the resource(s) that are being retrieved. In a guideCategory resource, the id property specifies the YouTube channel category ID.
    pub fn id(mut self, new_value: &str) -> GuideCategoryListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that will be used for text values in the API response.
    pub fn hl(mut self, new_value: &str) -> GuideCategoryListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> GuideCategoryListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> GuideCategoryListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> GuideCategoryListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a playlist.
///
/// A builder for the *insert* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Playlist;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Playlist = Default::default();
/// req.status = Default::default(); // is PlaylistStatus
/// req.snippet = Default::default(); // is PlaylistSnippet
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlists().insert(&req)
///              .on_behalf_of_content_owner_channel("labore")
///              .on_behalf_of_content_owner("eirmod")
///              .doit();
/// # }
/// ```
pub struct PlaylistInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Playlist,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Playlist)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlists.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlists".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &Playlist) -> PlaylistInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and status.
    pub fn part(mut self, new_value: &str) -> PlaylistInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> PlaylistInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs.
///
/// A builder for the *list* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *status*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlists().list("part")
///              .page_token("invidunt")
///              .on_behalf_of_content_owner_channel("aliquyam")
///              .on_behalf_of_content_owner("accusam")
///              .mine(false)
///              .max_results(92)
///              .id("et")
///              .channel_id("duo")
///              .doit();
/// # }
/// ```
pub struct PlaylistListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlaylistListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlists.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._channel_id {
            params.push(("channelId", value.to_string()));
        }
        for &field in ["alt", "part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlists".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *status*
    /// * *contentDetails*
    /// 
    /// The part parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, status, and contentDetails.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn part(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return playlists owned by the authenticated user.
    pub fn mine(mut self, new_value: bool) -> PlaylistListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> PlaylistListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube playlist ID(s) for the resource(s) that are being retrieved. In a playlist resource, the id property specifies the playlist's YouTube playlist ID.
    pub fn id(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// This value indicates that the API should only return the specified channel's playlists.
    pub fn channel_id(mut self, new_value: &str) -> PlaylistListCall<'a, C, NC, A> {
        self._channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a playlist.
///
/// A builder for the *delete* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlists().delete("id")
///              .on_behalf_of_content_owner("eirmod")
///              .doit();
/// # }
/// ```
pub struct PlaylistDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlists.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/playlists".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube playlist ID for the playlist that is being deleted. In a playlist resource, the id property specifies the playlist's ID.
    pub fn id(mut self, new_value: &str) -> PlaylistDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Modifies a playlist. For example, you could change a playlist's title, description, or privacy status.
///
/// A builder for the *update* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Playlist;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Playlist = Default::default();
/// req.status = Default::default(); // is PlaylistStatus
/// req.snippet = Default::default(); // is PlaylistSnippet
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlists().update(&req)
///              .on_behalf_of_content_owner("sanctus")
///              .doit();
/// # }
/// ```
pub struct PlaylistUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Playlist,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Playlist)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlists.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlists".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &Playlist) -> PlaylistUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and status.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a playlist's privacy setting is contained in the status part. As such, if your request is updating a private playlist, and the request's part parameter value includes the status part, the playlist's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the playlist will revert to the default privacy setting.
    pub fn part(mut self, new_value: &str) -> PlaylistUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Uploads a custom video thumbnail to YouTube and sets it for a video.
///
/// A builder for the *set* method supported by a *thumbnail* resource.
/// It is not used directly, but through a `ThumbnailMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.thumbnails().set("videoId")
///              .on_behalf_of_content_owner("amet")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct ThumbnailSetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _video_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ThumbnailSetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ThumbnailSetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, ThumbnailSetResponse)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.thumbnails.set", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("videoId", self._video_id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "videoId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = if protocol == "simple" {
                "https://www.googleapis.com/upload/youtube/v3/thumbnails/set".to_string()
            } else if protocol == "resumable" {
                "https://www.googleapis.com/resumable/upload/youtube/v3/thumbnails/set".to_string()
            } else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }


        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let mut response = hyper::client::Response::new(Box::new(cmn::DummyNetworkStream));
                    match response {
                        Ok(ref mut res) => {
                            res.status = hyper::status::StatusCode::Ok;
                            res.headers.set(Location(upload_url.as_ref().unwrap().clone()))
                        }
                        _ => unreachable!(),
                    }
                    response
                } else {
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone());
                    if protocol == "simple" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                    reader.seek(io::SeekFrom::Start(0)).unwrap();
                    if size > 2097152 {
                    	return Err(Error::UploadSizeLimitExceeded(size, 2097152))
                    }
                        req = req.header(ContentType(reader_mime_type.clone()))
                                 .header(ContentLength(size))
                                 .body(&mut reader);
                    }
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 2097152 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 2097152))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(url);
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 2MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, ThumbnailSetResponse)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL 
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 2MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, ThumbnailSetResponse)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    /// Sets the *video id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The videoId parameter specifies a YouTube video ID for which the custom video thumbnail is being provided.
    pub fn video_id(mut self, new_value: &str) -> ThumbnailSetCall<'a, C, NC, A> {
        self._video_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ThumbnailSetCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ThumbnailSetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ThumbnailSetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ThumbnailSetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of videos that match the API request parameters.
///
/// A builder for the *list* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *fileDetails*
/// * *liveStreamingDetails*
/// * *localizations*
/// * *player*
/// * *processingDetails*
/// * *recordingDetails*
/// * *statistics*
/// * *status*
/// * *suggestions*
/// * *topicDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().list("part")
///              .video_category_id("consetetur")
///              .region_code("ut")
///              .page_token("ea")
///              .on_behalf_of_content_owner("sed")
///              .my_rating("dolor")
///              .max_results(53)
///              .locale("dolor")
///              .id("et")
///              .hl("consetetur")
///              .chart("amet.")
///              .doit();
/// # }
/// ```
pub struct VideoListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _video_category_id: Option<String>,
    _region_code: Option<String>,
    _page_token: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _my_rating: Option<String>,
    _max_results: Option<u32>,
    _locale: Option<String>,
    _id: Option<String>,
    _hl: Option<String>,
    _chart: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VideoListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((13 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._video_category_id {
            params.push(("videoCategoryId", value.to_string()));
        }
        if let Some(value) = self._region_code {
            params.push(("regionCode", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._my_rating {
            params.push(("myRating", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._locale {
            params.push(("locale", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        if let Some(value) = self._chart {
            params.push(("chart", value.to_string()));
        }
        for &field in ["alt", "part", "videoCategoryId", "regionCode", "pageToken", "onBehalfOfContentOwner", "myRating", "maxResults", "locale", "id", "hl", "chart"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/videos".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *fileDetails*
    /// * *liveStreamingDetails*
    /// * *localizations*
    /// * *player*
    /// * *processingDetails*
    /// * *recordingDetails*
    /// * *statistics*
    /// * *status*
    /// * *suggestions*
    /// * *topicDetails*
    /// 
    /// The part parameter specifies a comma-separated list of one or more video resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, fileDetails, liveStreamingDetails, localizations, player, processingDetails, recordingDetails, statistics, status, suggestions, and topicDetails.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn part(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video category id* query property to the given value.
    ///
    /// 
    /// The videoCategoryId parameter identifies the video category for which the chart should be retrieved. This parameter can only be used in conjunction with the chart parameter. By default, charts are not restricted to a particular category.
    pub fn video_category_id(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._video_category_id = Some(new_value.to_string());
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to select a video chart available in the specified region. This parameter can only be used in conjunction with the chart parameter. The parameter value is an ISO 3166-1 alpha-2 country code.
    pub fn region_code(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    /// 
    /// Note: This parameter is supported for use in conjunction with the myRating parameter, but it is not supported for use in conjunction with the id parameter.
    pub fn page_token(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my rating* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to like or dislike to instruct the API to only return videos liked or disliked by the authenticated user.
    pub fn my_rating(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._my_rating = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    /// 
    /// Note: This parameter is supported for use in conjunction with the myRating parameter, but it is not supported for use in conjunction with the id parameter.
    pub fn max_results(mut self, new_value: u32) -> VideoListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// DEPRECATED
    pub fn locale(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) that are being retrieved. In a video resource, the id property specifies the video's ID.
    pub fn id(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter instructs the API to return a localized version of the video details. If localized text is nor available for the requested language, the localizations object in the API response will contain the requested information in the default language instead. The parameter value is a BCP-47 language code. Your application can determine whether the requested localization was returned by checking the value of the snippet.localized.language property in the API response.
    pub fn hl(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *chart* query property to the given value.
    ///
    /// 
    /// The chart parameter identifies the chart that you want to retrieve.
    pub fn chart(mut self, new_value: &str) -> VideoListCall<'a, C, NC, A> {
        self._chart = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Add a like or dislike rating to a video or remove a rating from a video.
///
/// A builder for the *rate* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().rate("id", "rating")
///              .on_behalf_of_content_owner("gubergren")
///              .doit();
/// # }
/// ```
pub struct VideoRateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _rating: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoRateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoRateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.rate", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        params.push(("rating", self._rating.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "rating", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/videos/rate".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube video ID of the video that is being rated or having its rating removed.
    pub fn id(mut self, new_value: &str) -> VideoRateCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *rating* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Specifies the rating to record.
    pub fn rating(mut self, new_value: &str) -> VideoRateCall<'a, C, NC, A> {
        self._rating = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoRateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoRateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoRateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoRateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Retrieves the ratings that the authorized user gave to a list of specified videos.
///
/// A builder for the *getRating* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().get_rating("id")
///              .on_behalf_of_content_owner("sit")
///              .doit();
/// # }
/// ```
pub struct VideoGetRatingCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoGetRatingCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoGetRatingCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VideoGetRatingResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.getRating", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/videos/getRating".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) for which you are retrieving rating data. In a video resource, the id property specifies the video's ID.
    pub fn id(mut self, new_value: &str) -> VideoGetRatingCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoGetRatingCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoGetRatingCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoGetRatingCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoGetRatingCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a YouTube video.
///
/// A builder for the *delete* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().delete("id")
///              .on_behalf_of_content_owner("diam")
///              .doit();
/// # }
/// ```
pub struct VideoDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/videos".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube video ID for the resource that is being deleted. In a video resource, the id property specifies the video's ID.
    pub fn id(mut self, new_value: &str) -> VideoDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a video's metadata.
///
/// A builder for the *update* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
/// * *fileDetails*
/// * *liveStreamingDetails*
/// * *localizations*
/// * *player*
/// * *processingDetails*
/// * *recordingDetails*
/// * *statistics*
/// * *status*
/// * *suggestions*
/// * *topicDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Video;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Video = Default::default();
/// req.status = Default::default(); // is VideoStatus
/// req.topic_details = Default::default(); // is VideoTopicDetails
/// req.suggestions = Default::default(); // is VideoSuggestions
/// req.file_details = Default::default(); // is VideoFileDetails
/// req.player = Default::default(); // is VideoPlayer
/// req.localizations = Default::default(); // is HashMap<String, VideoLocalization>
/// req.live_streaming_details = Default::default(); // is VideoLiveStreamingDetails
/// req.snippet = Default::default(); // is VideoSnippet
/// req.statistics = Default::default(); // is VideoStatistics
/// req.processing_details = Default::default(); // is VideoProcessingDetails
/// req.content_details = Default::default(); // is VideoContentDetails
/// req.recording_details = Default::default(); // is VideoRecordingDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().update(&req)
///              .on_behalf_of_content_owner("rebum.")
///              .doit();
/// # }
/// ```
pub struct VideoUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Video,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Video)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/videos".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *fileDetails*
    /// * *liveStreamingDetails*
    /// * *localizations*
    /// * *player*
    /// * *processingDetails*
    /// * *recordingDetails*
    /// * *statistics*
    /// * *status*
    /// * *suggestions*
    /// * *topicDetails*
    /// 
    pub fn request(mut self, new_value: &Video) -> VideoUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *fileDetails*
    /// * *liveStreamingDetails*
    /// * *localizations*
    /// * *player*
    /// * *processingDetails*
    /// * *recordingDetails*
    /// * *statistics*
    /// * *status*
    /// * *suggestions*
    /// * *topicDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet, contentDetails, fileDetails, liveStreamingDetails, localizations, player, processingDetails, recordingDetails, statistics, status, suggestions, and topicDetails.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a video's privacy setting is contained in the status part. As such, if your request is updating a private video, and the request's part parameter value includes the status part, the video's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the video will revert to the default privacy setting.
    /// 
    /// In addition, not all of those parts contain properties that can be set when setting or updating a video's metadata. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response.
    pub fn part(mut self, new_value: &str) -> VideoUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Uploads a video to YouTube and optionally sets the video's metadata.
///
/// A builder for the *insert* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
/// * *fileDetails*
/// * *liveStreamingDetails*
/// * *localizations*
/// * *player*
/// * *processingDetails*
/// * *recordingDetails*
/// * *statistics*
/// * *status*
/// * *suggestions*
/// * *topicDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.upload*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Video;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Video = Default::default();
/// req.status = Default::default(); // is VideoStatus
/// req.topic_details = Default::default(); // is VideoTopicDetails
/// req.suggestions = Default::default(); // is VideoSuggestions
/// req.file_details = Default::default(); // is VideoFileDetails
/// req.player = Default::default(); // is VideoPlayer
/// req.localizations = Default::default(); // is HashMap<String, VideoLocalization>
/// req.live_streaming_details = Default::default(); // is VideoLiveStreamingDetails
/// req.snippet = Default::default(); // is VideoSnippet
/// req.statistics = Default::default(); // is VideoStatistics
/// req.processing_details = Default::default(); // is VideoProcessingDetails
/// req.content_details = Default::default(); // is VideoContentDetails
/// req.recording_details = Default::default(); // is VideoRecordingDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().insert(&req)
///              .stabilize(true)
///              .on_behalf_of_content_owner_channel("sadipscing")
///              .on_behalf_of_content_owner("vero")
///              .notify_subscribers(false)
///              .auto_levels(false)
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct VideoInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Video,
    _part: String,
    _stabilize: Option<bool>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _notify_subscribers: Option<bool>,
    _auto_levels: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<(hyper::client::Response, Video)>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videos.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._stabilize {
            params.push(("stabilize", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._notify_subscribers {
            params.push(("notifySubscribers", value.to_string()));
        }
        if let Some(value) = self._auto_levels {
            params.push(("autoLevels", value.to_string()));
        }
        for &field in ["alt", "part", "stabilize", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "notifySubscribers", "autoLevels"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = if protocol == "simple" {
                "https://www.googleapis.com/upload/youtube/v3/videos".to_string()
            } else if protocol == "resumable" {
                "https://www.googleapis.com/resumable/upload/youtube/v3/videos".to_string()
            } else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let mut response = hyper::client::Response::new(Box::new(cmn::DummyNetworkStream));
                    match response {
                        Ok(ref mut res) => {
                            res.status = hyper::status::StatusCode::Ok;
                            res.headers.set(Location(upload_url.as_ref().unwrap().clone()))
                        }
                        _ => unreachable!(),
                    }
                    response
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 68719476736 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 68719476736))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone())
                        .header(content_type)
                        .body(&mut body_reader);
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 68719476736 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 68719476736))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(url);
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 64GB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream' and 'video/*'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, Video)>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL 
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 64GB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream' and 'video/*'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<(hyper::client::Response, Video)>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *fileDetails*
    /// * *liveStreamingDetails*
    /// * *localizations*
    /// * *player*
    /// * *processingDetails*
    /// * *recordingDetails*
    /// * *statistics*
    /// * *status*
    /// * *suggestions*
    /// * *topicDetails*
    /// 
    pub fn request(mut self, new_value: &Video) -> VideoInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *fileDetails*
    /// * *liveStreamingDetails*
    /// * *localizations*
    /// * *player*
    /// * *processingDetails*
    /// * *recordingDetails*
    /// * *statistics*
    /// * *status*
    /// * *suggestions*
    /// * *topicDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet, contentDetails, fileDetails, liveStreamingDetails, localizations, player, processingDetails, recordingDetails, statistics, status, suggestions, and topicDetails. However, not all of those parts contain properties that can be set when setting or updating a video's metadata. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response.
    pub fn part(mut self, new_value: &str) -> VideoInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *stabilize* query property to the given value.
    ///
    /// 
    /// The stabilize parameter indicates whether YouTube should adjust the video to remove shaky camera motions.
    pub fn stabilize(mut self, new_value: bool) -> VideoInsertCall<'a, C, NC, A> {
        self._stabilize = Some(new_value);
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> VideoInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *notify subscribers* query property to the given value.
    ///
    /// 
    /// The notifySubscribers parameter indicates whether YouTube should send notification to subscribers about the inserted video.
    pub fn notify_subscribers(mut self, new_value: bool) -> VideoInsertCall<'a, C, NC, A> {
        self._notify_subscribers = Some(new_value);
        self
    }
    /// Sets the *auto levels* query property to the given value.
    ///
    /// 
    /// The autoLevels parameter indicates whether YouTube should automatically enhance the video's lighting and color.
    pub fn auto_levels(mut self, new_value: bool) -> VideoInsertCall<'a, C, NC, A> {
        self._auto_levels = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Adds a subscription for the authenticated user's channel.
///
/// A builder for the *insert* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Subscription;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Subscription = Default::default();
/// req.snippet = Default::default(); // is SubscriptionSnippet
/// req.content_details = Default::default(); // is SubscriptionContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.subscriptions().insert(&req)
///              .doit();
/// # }
/// ```
pub struct SubscriptionInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Subscription,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SubscriptionInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Subscription)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.subscriptions.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["alt", "part"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/subscriptions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    pub fn request(mut self, new_value: &Subscription) -> SubscriptionInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and contentDetails.
    pub fn part(mut self, new_value: &str) -> SubscriptionInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SubscriptionInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SubscriptionInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns subscription resources that match the API request criteria.
///
/// A builder for the *list* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.subscriptions().list("part")
///              .page_token("dolore")
///              .order("duo")
///              .on_behalf_of_content_owner_channel("aliquyam")
///              .on_behalf_of_content_owner("Lorem")
///              .my_subscribers(true)
///              .mine(true)
///              .max_results(56)
///              .id("takimata")
///              .for_channel_id("nonumy")
///              .channel_id("kasd")
///              .doit();
/// # }
/// ```
pub struct SubscriptionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _order: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _my_subscribers: Option<bool>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _for_channel_id: Option<String>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SubscriptionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SubscriptionListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.subscriptions.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((13 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order {
            params.push(("order", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._my_subscribers {
            params.push(("mySubscribers", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._for_channel_id {
            params.push(("forChannelId", value.to_string()));
        }
        if let Some(value) = self._channel_id {
            params.push(("channelId", value.to_string()));
        }
        for &field in ["alt", "part", "pageToken", "order", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mySubscribers", "mine", "maxResults", "id", "forChannelId", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/subscriptions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *order* query property to the given value.
    ///
    /// 
    /// The order parameter specifies the method that will be used to sort resources in the API response.
    pub fn order(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._order = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my subscribers* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the subscribers of the authenticated user.
    pub fn my_subscribers(mut self, new_value: bool) -> SubscriptionListCall<'a, C, NC, A> {
        self._my_subscribers = Some(new_value);
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's subscriptions.
    pub fn mine(mut self, new_value: bool) -> SubscriptionListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> SubscriptionListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube subscription ID(s) for the resource(s) that are being retrieved. In a subscription resource, the id property specifies the YouTube subscription ID.
    pub fn id(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *for channel id* query property to the given value.
    ///
    /// 
    /// The forChannelId parameter specifies a comma-separated list of channel IDs. The API response will then only contain subscriptions matching those channels.
    pub fn for_channel_id(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._for_channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a YouTube channel ID. The API will only return that channel's subscriptions.
    pub fn channel_id(mut self, new_value: &str) -> SubscriptionListCall<'a, C, NC, A> {
        self._channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SubscriptionListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SubscriptionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a subscription.
///
/// A builder for the *delete* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.subscriptions().delete("id")
///              .doit();
/// # }
/// ```
pub struct SubscriptionDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SubscriptionDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.subscriptions.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/subscriptions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube subscription ID for the resource that is being deleted. In a subscription resource, the id property specifies the YouTube subscription ID.
    pub fn id(mut self, new_value: &str) -> SubscriptionDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SubscriptionDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SubscriptionDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.
///
/// A builder for the *list* method supported by a *search* resource.
/// It is not used directly, but through a `SearchMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.search().list("part")
///              .video_type("At")
///              .video_syndicated("labore")
///              .video_license("invidunt")
///              .video_embeddable("ea")
///              .video_duration("sadipscing")
///              .video_dimension("rebum.")
///              .video_definition("dolore")
///              .video_category_id("nonumy")
///              .video_caption("sed")
///              .type_("aliquyam")
///              .topic_id("sit")
///              .safe_search("eirmod")
///              .relevance_language("consetetur")
///              .related_to_video_id("labore")
///              .region_code("sed")
///              .q("ea")
///              .published_before("gubergren")
///              .published_after("aliquyam")
///              .page_token("eos")
///              .order("tempor")
///              .on_behalf_of_content_owner("sea")
///              .max_results(16)
///              .location_radius("ipsum")
///              .location("aliquyam")
///              .for_mine(false)
///              .for_content_owner(false)
///              .event_type("diam")
///              .channel_type("ut")
///              .channel_id("justo")
///              .doit();
/// # }
/// ```
pub struct SearchListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _video_type: Option<String>,
    _video_syndicated: Option<String>,
    _video_license: Option<String>,
    _video_embeddable: Option<String>,
    _video_duration: Option<String>,
    _video_dimension: Option<String>,
    _video_definition: Option<String>,
    _video_category_id: Option<String>,
    _video_caption: Option<String>,
    _type_: Option<String>,
    _topic_id: Option<String>,
    _safe_search: Option<String>,
    _relevance_language: Option<String>,
    _related_to_video_id: Option<String>,
    _region_code: Option<String>,
    _q: Option<String>,
    _published_before: Option<String>,
    _published_after: Option<String>,
    _page_token: Option<String>,
    _order: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _max_results: Option<u32>,
    _location_radius: Option<String>,
    _location: Option<String>,
    _for_mine: Option<bool>,
    _for_content_owner: Option<bool>,
    _event_type: Option<String>,
    _channel_type: Option<String>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for SearchListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> SearchListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.search.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((32 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._video_type {
            params.push(("videoType", value.to_string()));
        }
        if let Some(value) = self._video_syndicated {
            params.push(("videoSyndicated", value.to_string()));
        }
        if let Some(value) = self._video_license {
            params.push(("videoLicense", value.to_string()));
        }
        if let Some(value) = self._video_embeddable {
            params.push(("videoEmbeddable", value.to_string()));
        }
        if let Some(value) = self._video_duration {
            params.push(("videoDuration", value.to_string()));
        }
        if let Some(value) = self._video_dimension {
            params.push(("videoDimension", value.to_string()));
        }
        if let Some(value) = self._video_definition {
            params.push(("videoDefinition", value.to_string()));
        }
        if let Some(value) = self._video_category_id {
            params.push(("videoCategoryId", value.to_string()));
        }
        if let Some(value) = self._video_caption {
            params.push(("videoCaption", value.to_string()));
        }
        if let Some(value) = self._type_ {
            params.push(("type", value.to_string()));
        }
        if let Some(value) = self._topic_id {
            params.push(("topicId", value.to_string()));
        }
        if let Some(value) = self._safe_search {
            params.push(("safeSearch", value.to_string()));
        }
        if let Some(value) = self._relevance_language {
            params.push(("relevanceLanguage", value.to_string()));
        }
        if let Some(value) = self._related_to_video_id {
            params.push(("relatedToVideoId", value.to_string()));
        }
        if let Some(value) = self._region_code {
            params.push(("regionCode", value.to_string()));
        }
        if let Some(value) = self._q {
            params.push(("q", value.to_string()));
        }
        if let Some(value) = self._published_before {
            params.push(("publishedBefore", value.to_string()));
        }
        if let Some(value) = self._published_after {
            params.push(("publishedAfter", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._order {
            params.push(("order", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._location_radius {
            params.push(("locationRadius", value.to_string()));
        }
        if let Some(value) = self._location {
            params.push(("location", value.to_string()));
        }
        if let Some(value) = self._for_mine {
            params.push(("forMine", value.to_string()));
        }
        if let Some(value) = self._for_content_owner {
            params.push(("forContentOwner", value.to_string()));
        }
        if let Some(value) = self._event_type {
            params.push(("eventType", value.to_string()));
        }
        if let Some(value) = self._channel_type {
            params.push(("channelType", value.to_string()));
        }
        if let Some(value) = self._channel_id {
            params.push(("channelId", value.to_string()));
        }
        for &field in ["alt", "part", "videoType", "videoSyndicated", "videoLicense", "videoEmbeddable", "videoDuration", "videoDimension", "videoDefinition", "videoCategoryId", "videoCaption", "type", "topicId", "safeSearch", "relevanceLanguage", "relatedToVideoId", "regionCode", "q", "publishedBefore", "publishedAfter", "pageToken", "order", "onBehalfOfContentOwner", "maxResults", "locationRadius", "location", "forMine", "forContentOwner", "eventType", "channelType", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/search".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// 
    /// The part parameter specifies a comma-separated list of one or more search resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a search result, the snippet property contains other properties that identify the result's title, description, and so forth. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video type* query property to the given value.
    ///
    /// 
    /// The videoType parameter lets you restrict a search to a particular type of videos.
    pub fn video_type(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_type = Some(new_value.to_string());
        self
    }
    /// Sets the *video syndicated* query property to the given value.
    ///
    /// 
    /// The videoSyndicated parameter lets you to restrict a search to only videos that can be played outside youtube.com.
    pub fn video_syndicated(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_syndicated = Some(new_value.to_string());
        self
    }
    /// Sets the *video license* query property to the given value.
    ///
    /// 
    /// The videoLicense parameter filters search results to only include videos with a particular license. YouTube lets video uploaders choose to attach either the Creative Commons license or the standard YouTube license to each of their videos.
    pub fn video_license(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_license = Some(new_value.to_string());
        self
    }
    /// Sets the *video embeddable* query property to the given value.
    ///
    /// 
    /// The videoEmbeddable parameter lets you to restrict a search to only videos that can be embedded into a webpage.
    pub fn video_embeddable(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_embeddable = Some(new_value.to_string());
        self
    }
    /// Sets the *video duration* query property to the given value.
    ///
    /// 
    /// The videoDuration parameter filters video search results based on their duration.
    pub fn video_duration(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_duration = Some(new_value.to_string());
        self
    }
    /// Sets the *video dimension* query property to the given value.
    ///
    /// 
    /// The videoDimension parameter lets you restrict a search to only retrieve 2D or 3D videos.
    pub fn video_dimension(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_dimension = Some(new_value.to_string());
        self
    }
    /// Sets the *video definition* query property to the given value.
    ///
    /// 
    /// The videoDefinition parameter lets you restrict a search to only include either high definition (HD) or standard definition (SD) videos. HD videos are available for playback in at least 720p, though higher resolutions, like 1080p, might also be available.
    pub fn video_definition(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_definition = Some(new_value.to_string());
        self
    }
    /// Sets the *video category id* query property to the given value.
    ///
    /// 
    /// The videoCategoryId parameter filters video search results based on their category.
    pub fn video_category_id(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_category_id = Some(new_value.to_string());
        self
    }
    /// Sets the *video caption* query property to the given value.
    ///
    /// 
    /// The videoCaption parameter indicates whether the API should filter video search results based on whether they have captions.
    pub fn video_caption(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._video_caption = Some(new_value.to_string());
        self
    }
    /// Sets the *type* query property to the given value.
    ///
    /// 
    /// The type parameter restricts a search query to only retrieve a particular type of resource. The value is a comma-separated list of resource types.
    pub fn type_(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Sets the *topic id* query property to the given value.
    ///
    /// 
    /// The topicId parameter indicates that the API response should only contain resources associated with the specified topic. The value identifies a Freebase topic ID.
    pub fn topic_id(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._topic_id = Some(new_value.to_string());
        self
    }
    /// Sets the *safe search* query property to the given value.
    ///
    /// 
    /// The safeSearch parameter indicates whether the search results should include restricted content as well as standard content.
    pub fn safe_search(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._safe_search = Some(new_value.to_string());
        self
    }
    /// Sets the *relevance language* query property to the given value.
    ///
    /// 
    /// The relevanceLanguage parameter instructs the API to return search results that are most relevant to the specified language. The parameter value is typically an ISO 639-1 two-letter language code. However, you should use the values zh-Hans for simplified Chinese and zh-Hant for traditional Chinese. Please note that results in other languages will still be returned if they are highly relevant to the search query term.
    pub fn relevance_language(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._relevance_language = Some(new_value.to_string());
        self
    }
    /// Sets the *related to video id* query property to the given value.
    ///
    /// 
    /// The relatedToVideoId parameter retrieves a list of videos that are related to the video that the parameter value identifies. The parameter value must be set to a YouTube video ID and, if you are using this parameter, the type parameter must be set to video.
    pub fn related_to_video_id(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._related_to_video_id = Some(new_value.to_string());
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return search results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.
    pub fn region_code(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *q* query property to the given value.
    ///
    /// 
    /// The q parameter specifies the query term to search for.
    pub fn q(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Sets the *published before* query property to the given value.
    ///
    /// 
    /// The publishedBefore parameter indicates that the API response should only contain resources created before the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).
    pub fn published_before(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._published_before = Some(new_value.to_string());
        self
    }
    /// Sets the *published after* query property to the given value.
    ///
    /// 
    /// The publishedAfter parameter indicates that the API response should only contain resources created after the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).
    pub fn published_after(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._published_after = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *order* query property to the given value.
    ///
    /// 
    /// The order parameter specifies the method that will be used to order resources in the API response.
    pub fn order(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._order = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> SearchListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *location radius* query property to the given value.
    ///
    /// 
    /// The locationRadius, in conjunction with the location parameter, defines a geographic area. If the geographic coordinates associated with a video fall within that area, then the video may be included in search results. This parameter value must be a floating point number followed by a measurement unit. Valid measurement units are m, km, ft, and mi. For example, valid parameter values include 1500m, 5km, 10000ft, and 0.75mi. The API does not support locationRadius parameter values larger than 1000 kilometers.
    pub fn location_radius(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._location_radius = Some(new_value.to_string());
        self
    }
    /// Sets the *location* query property to the given value.
    ///
    /// 
    /// The location parameter restricts a search to videos that have a geographical location specified in their metadata. The value is a string that specifies geographic latitude/longitude coordinates e.g. (37.42307,-122.08427)
    pub fn location(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._location = Some(new_value.to_string());
        self
    }
    /// Sets the *for mine* query property to the given value.
    ///
    /// 
    /// The forMine parameter restricts the search to only retrieve videos owned by the authenticated user. If you set this parameter to true, then the type parameter's value must also be set to video.
    pub fn for_mine(mut self, new_value: bool) -> SearchListCall<'a, C, NC, A> {
        self._for_mine = Some(new_value);
        self
    }
    /// Sets the *for content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The forContentOwner parameter restricts the search to only retrieve resources owned by the content owner specified by the onBehalfOfContentOwner parameter. The user must be authenticated using a CMS account linked to the specified content owner and onBehalfOfContentOwner must be provided.
    pub fn for_content_owner(mut self, new_value: bool) -> SearchListCall<'a, C, NC, A> {
        self._for_content_owner = Some(new_value);
        self
    }
    /// Sets the *event type* query property to the given value.
    ///
    /// 
    /// The eventType parameter restricts a search to broadcast events.
    pub fn event_type(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._event_type = Some(new_value.to_string());
        self
    }
    /// Sets the *channel type* query property to the given value.
    ///
    /// 
    /// The channelType parameter lets you restrict a search to a particular type of channel.
    pub fn channel_type(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._channel_type = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter indicates that the API response should only contain resources created by the channel
    pub fn channel_id(mut self, new_value: &str) -> SearchListCall<'a, C, NC, A> {
        self._channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SearchListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> SearchListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> SearchListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of supported regions.
///
/// A builder for the *list* method supported by a *i18nRegion* resource.
/// It is not used directly, but through a `I18nRegionMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.i18n_regions().list("part")
///              .hl("amet")
///              .doit();
/// # }
/// ```
pub struct I18nRegionListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for I18nRegionListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nRegionListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, I18nRegionListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.i18nRegions.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        for &field in ["alt", "part", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/i18nRegions".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// 
    /// The part parameter specifies a comma-separated list of one or more i18nRegion resource properties that the API response will include. The part names that you can include in the parameter value are id and snippet.
    pub fn part(mut self, new_value: &str) -> I18nRegionListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.
    pub fn hl(mut self, new_value: &str) -> I18nRegionListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> I18nRegionListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> I18nRegionListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> I18nRegionListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.
///
/// A builder for the *update* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *cdn*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::LiveStream;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LiveStream = Default::default();
/// req.status = Default::default(); // is LiveStreamStatus
/// req.snippet = Default::default(); // is LiveStreamSnippet
/// req.cdn = Default::default(); // is CdnSettings
/// req.id = Some("accusam".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().update(&req)
///              .on_behalf_of_content_owner_channel("clita")
///              .on_behalf_of_content_owner("diam")
///              .doit();
/// # }
/// ```
pub struct LiveStreamUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveStream,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveStreamUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveStream)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveStreams.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveStreams".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *cdn*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &LiveStream) -> LiveStreamUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *cdn*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part properties that you can include in the parameter value are id, snippet, cdn, and status.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. If the request body does not specify a value for a mutable property, the existing value for that property will be removed.
    pub fn part(mut self, new_value: &str) -> LiveStreamUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveStreamUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveStreamUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a video stream.
///
/// A builder for the *delete* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().delete("id")
///              .on_behalf_of_content_owner_channel("est")
///              .on_behalf_of_content_owner("clita")
///              .doit();
/// # }
/// ```
pub struct LiveStreamDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveStreamDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveStreams.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/liveStreams".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live stream ID for the resource that is being deleted.
    pub fn id(mut self, new_value: &str) -> LiveStreamDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveStreamDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveStreamDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of video streams that match the API request parameters.
///
/// A builder for the *list* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *cdn*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().list("part")
///              .page_token("ut")
///              .on_behalf_of_content_owner_channel("dolores")
///              .on_behalf_of_content_owner("eos")
///              .mine(false)
///              .max_results(82)
///              .id("sed")
///              .doit();
/// # }
/// ```
pub struct LiveStreamListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveStreamListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveStreamListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveStreams.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        for &field in ["alt", "part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveStreams".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *cdn*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status.
    pub fn part(mut self, new_value: &str) -> LiveStreamListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> LiveStreamListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// The mine parameter can be used to instruct the API to only return streams owned by the authenticated user. Set the parameter value to true to only retrieve your own streams.
    pub fn mine(mut self, new_value: bool) -> LiveStreamListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set. Acceptable values are 0 to 50, inclusive. The default value is 5.
    pub fn max_results(mut self, new_value: u32) -> LiveStreamListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of YouTube stream IDs that identify the streams being retrieved. In a liveStream resource, the id property specifies the stream's ID.
    pub fn id(mut self, new_value: &str) -> LiveStreamListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveStreamListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveStreamListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience.
///
/// A builder for the *insert* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *cdn*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::LiveStream;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LiveStream = Default::default();
/// req.status = Default::default(); // is LiveStreamStatus
/// req.snippet = Default::default(); // is LiveStreamSnippet
/// req.cdn = Default::default(); // is CdnSettings
/// req.id = Some("aliquyam".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().insert(&req)
///              .on_behalf_of_content_owner_channel("ea")
///              .on_behalf_of_content_owner("ea")
///              .doit();
/// # }
/// ```
pub struct LiveStreamInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveStream,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveStreamInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveStream)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveStreams.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveStreams".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *cdn*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &LiveStream) -> LiveStreamInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *cdn*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part properties that you can include in the parameter value are id, snippet, cdn, and status.
    pub fn part(mut self, new_value: &str) -> LiveStreamInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveStreamInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveStreamInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a channel's metadata.
///
/// A builder for the *update* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *invideoPromotion*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Channel;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Channel = Default::default();
/// req.invideo_promotion = Default::default(); // is InvideoPromotion
/// req.id = Some("et".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().update(&req)
///              .on_behalf_of_content_owner("dolor")
///              .doit();
/// # }
/// ```
pub struct ChannelUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Channel,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channels.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/channels".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *invideoPromotion*
    /// 
    pub fn request(mut self, new_value: &Channel) -> ChannelUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *invideoPromotion*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are id and invideoPromotion.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies.
    pub fn part(mut self, new_value: &str) -> ChannelUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a collection of zero or more channel resources that match the request criteria.
///
/// A builder for the *list* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *statistics*
/// * *topicDetails*
/// * *invideoPromotion*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
/// * *https://www.googleapis.com/auth/youtubepartner-channel-audit*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().list("part")
///              .page_token("kasd")
///              .on_behalf_of_content_owner("invidunt")
///              .my_subscribers(true)
///              .mine(true)
///              .max_results(38)
///              .managed_by_me(true)
///              .id("eirmod")
///              .for_username("At")
///              .category_id("consetetur")
///              .doit();
/// # }
/// ```
pub struct ChannelListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _my_subscribers: Option<bool>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _managed_by_me: Option<bool>,
    _id: Option<String>,
    _for_username: Option<String>,
    _category_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ChannelListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ChannelListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.channels.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((12 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._my_subscribers {
            params.push(("mySubscribers", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._managed_by_me {
            params.push(("managedByMe", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._for_username {
            params.push(("forUsername", value.to_string()));
        }
        if let Some(value) = self._category_id {
            params.push(("categoryId", value.to_string()));
        }
        for &field in ["alt", "part", "pageToken", "onBehalfOfContentOwner", "mySubscribers", "mine", "maxResults", "managedByMe", "id", "forUsername", "categoryId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/channels".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *statistics*
    /// * *topicDetails*
    /// * *invideoPromotion*
    /// 
    /// The part parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, statistics, topicDetails, and invideoPromotion.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set part=contentDetails, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my subscribers* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a list of channels that subscribed to the authenticated user's channel.
    pub fn my_subscribers(mut self, new_value: bool) -> ChannelListCall<'a, C, NC, A> {
        self._my_subscribers = Some(new_value);
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return channels owned by the authenticated user.
    pub fn mine(mut self, new_value: bool) -> ChannelListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> ChannelListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *managed by me* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return channels managed by the content owner that the onBehalfOfContentOwner parameter specifies. The user must be authenticated as a CMS account linked to the specified content owner and onBehalfOfContentOwner must be provided.
    pub fn managed_by_me(mut self, new_value: bool) -> ChannelListCall<'a, C, NC, A> {
        self._managed_by_me = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channel ID(s) for the resource(s) that are being retrieved. In a channel resource, the id property specifies the channel's YouTube channel ID.
    pub fn id(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *for username* query property to the given value.
    ///
    /// 
    /// The forUsername parameter specifies a YouTube username, thereby requesting the channel associated with that username.
    pub fn for_username(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._for_username = Some(new_value.to_string());
        self
    }
    /// Sets the *category id* query property to the given value.
    ///
    /// 
    /// The categoryId parameter specifies a YouTube guide category, thereby requesting YouTube channels associated with that category.
    pub fn category_id(mut self, new_value: &str) -> ChannelListCall<'a, C, NC, A> {
        self._category_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ChannelListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a playlist item.
///
/// A builder for the *delete* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlist_items().delete("id")
///              .doit();
/// # }
/// ```
pub struct PlaylistItemDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistItemDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlistItems.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((2 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        for &field in ["id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube playlist item ID for the playlist item that is being deleted. In a playlistItem resource, the id property specifies the playlist item's ID.
    pub fn id(mut self, new_value: &str) -> PlaylistItemDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistItemDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistItemDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs.
///
/// A builder for the *list* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlist_items().list("part")
///              .video_id("sit")
///              .playlist_id("takimata")
///              .page_token("elitr")
///              .on_behalf_of_content_owner("nonumy")
///              .max_results(86)
///              .id("Lorem")
///              .doit();
/// # }
/// ```
pub struct PlaylistItemListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _video_id: Option<String>,
    _playlist_id: Option<String>,
    _page_token: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistItemListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlaylistItemListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlistItems.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._video_id {
            params.push(("videoId", value.to_string()));
        }
        if let Some(value) = self._playlist_id {
            params.push(("playlistId", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        for &field in ["alt", "part", "videoId", "playlistId", "pageToken", "onBehalfOfContentOwner", "maxResults", "id"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set part=snippet, the API response will contain all of those properties.
    pub fn part(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video id* query property to the given value.
    ///
    /// 
    /// The videoId parameter specifies that the request should return only the playlist items that contain the specified video.
    pub fn video_id(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// Sets the *playlist id* query property to the given value.
    ///
    /// 
    /// The playlistId parameter specifies the unique ID of the playlist for which you want to retrieve playlist items. Note that even though this is an optional parameter, every request to retrieve playlist items must specify a value for either the id parameter or the playlistId parameter.
    pub fn playlist_id(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._playlist_id = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> PlaylistItemListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of one or more unique playlist item IDs.
    pub fn id(mut self, new_value: &str) -> PlaylistItemListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistItemListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistItemListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Adds a resource to a playlist.
///
/// A builder for the *insert* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::PlaylistItem;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PlaylistItem = Default::default();
/// req.status = Default::default(); // is PlaylistItemStatus
/// req.snippet = Default::default(); // is PlaylistItemSnippet
/// req.content_details = Default::default(); // is PlaylistItemContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlist_items().insert(&req)
///              .on_behalf_of_content_owner("Lorem")
///              .doit();
/// # }
/// ```
pub struct PlaylistItemInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: PlaylistItem,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistItemInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlaylistItem)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlistItems.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &PlaylistItem) -> PlaylistItemInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> PlaylistItemInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistItemInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistItemInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistItemInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Modifies a playlist item. For example, you could update the item's position in the playlist.
///
/// A builder for the *update* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::PlaylistItem;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: PlaylistItem = Default::default();
/// req.status = Default::default(); // is PlaylistItemStatus
/// req.snippet = Default::default(); // is PlaylistItemSnippet
/// req.content_details = Default::default(); // is PlaylistItemContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.playlist_items().update(&req)
///              .doit();
/// # }
/// ```
pub struct PlaylistItemUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: PlaylistItem,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for PlaylistItemUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, PlaylistItem)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.playlistItems.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["alt", "part"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/playlistItems".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &PlaylistItem) -> PlaylistItemUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet, contentDetails, and status.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a playlist item can specify a start time and end time, which identify the times portion of the video that should play when users watch the video in the playlist. If your request is updating a playlist item that sets these values, and the request's part parameter value includes the contentDetails part, the playlist item's start and end times will be updated to whatever value the request body specifies. If the request body does not specify values, the existing start and end times will be removed and replaced with the default settings.
    pub fn part(mut self, new_value: &str) -> PlaylistItemUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> PlaylistItemUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> PlaylistItemUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Uploads a watermark image to YouTube and sets it for a channel.
///
/// A builder for the *set* method supported by a *watermark* resource.
/// It is not used directly, but through a `WatermarkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::InvideoBranding;
/// use std::fs;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: InvideoBranding = Default::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.watermarks().set(&req, "channelId")
///              .on_behalf_of_content_owner("ut")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap());
/// # }
/// ```
pub struct WatermarkSetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: InvideoBranding,
    _channel_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for WatermarkSetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkSetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> Result<hyper::client::Response>
		where RS: ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.watermarks.set", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("channelId", self._channel_id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["channelId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = if protocol == "simple" {
                "https://www.googleapis.com/upload/youtube/v3/watermarks/set".to_string()
            } else if protocol == "resumable" {
                "https://www.googleapis.com/resumable/upload/youtube/v3/watermarks/set".to_string()
            } else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let mut response = hyper::client::Response::new(Box::new(cmn::DummyNetworkStream));
                    match response {
                        Ok(ref mut res) => {
                            res.status = hyper::status::StatusCode::Ok;
                            res.headers.set(Location(upload_url.as_ref().unwrap().clone()))
                        }
                        _ => unreachable!(),
                    }
                    response
                } else {
                    let mut mp_reader: MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 10485760 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 10485760))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut io::Read, ContentType(mime_type))
                        },
                        _ => (&mut request_value_reader as &mut io::Read, ContentType(json_mime_type.clone())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                        .header(UserAgent(self.hub._user_agent.clone()))
                        .header(auth_header.clone())
                        .header(content_type)
                        .body(&mut body_reader);
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req = req.header(cmn::XUploadContentType(reader_mime_type.clone()));
                    }
    
                    dlg.pre_request();
                    req.send()
                }
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 10485760 {
                        	return Err(Error::UploadSizeLimitExceeded(size, 10485760))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url = &res.headers.get::<Location>().expect("Location header is part of protocol").0;
                            if upload_url_from_server {
                                dlg.store_upload_url(url);
                            }

                            cmn::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: auth_header.clone(),
                                url: url,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload()
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status.is_success() {
                                    dlg.finished(false);
                                    return Err(Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 10MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> Result<hyper::client::Response>
                where RS: ReadSeek {
        self.doit(stream, mime_type, "simple")
    }
    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL 
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *max size*: 10MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> Result<hyper::client::Response>
                where RS: ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable")
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &InvideoBranding) -> WatermarkSetCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The channelId parameter specifies a YouTube channel ID for which the watermark is being provided.
    pub fn channel_id(mut self, new_value: &str) -> WatermarkSetCall<'a, C, NC, A> {
        self._channel_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> WatermarkSetCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> WatermarkSetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> WatermarkSetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> WatermarkSetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a watermark.
///
/// A builder for the *unset* method supported by a *watermark* resource.
/// It is not used directly, but through a `WatermarkMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.watermarks().unset("channelId")
///              .on_behalf_of_content_owner("amet.")
///              .doit();
/// # }
/// ```
pub struct WatermarkUnsetCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _channel_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for WatermarkUnsetCall<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkUnsetCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.watermarks.unset", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("channelId", self._channel_id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["channelId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/watermarks/unset".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *channel id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The channelId parameter specifies a YouTube channel ID for which the watermark is being unset.
    pub fn channel_id(mut self, new_value: &str) -> WatermarkUnsetCall<'a, C, NC, A> {
        self._channel_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> WatermarkUnsetCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> WatermarkUnsetCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> WatermarkUnsetCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> WatermarkUnsetCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Controls the settings for a slate that can be displayed in the broadcast stream.
///
/// A builder for the *control* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().control("id", "part")
///              .walltime("dolor")
///              .on_behalf_of_content_owner_channel("sea")
///              .on_behalf_of_content_owner("ut")
///              .offset_time_ms("eirmod")
///              .display_slate(true)
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastControlCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _part: String,
    _walltime: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _offset_time_ms: Option<String>,
    _display_slate: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastControlCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastControlCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcast)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.control", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((9 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._walltime {
            params.push(("walltime", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._offset_time_ms {
            params.push(("offsetTimeMs", value.to_string()));
        }
        if let Some(value) = self._display_slate {
            params.push(("displaySlate", value.to_string()));
        }
        for &field in ["alt", "id", "part", "walltime", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "offsetTimeMs", "displaySlate"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts/control".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live broadcast ID that uniquely identifies the broadcast in which the slate is being updated.
    pub fn id(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *walltime* query property to the given value.
    ///
    /// 
    /// The walltime parameter specifies the wall clock time at which the specified slate change will occur. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.
    pub fn walltime(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._walltime = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *offset time ms* query property to the given value.
    ///
    /// 
    /// The offsetTimeMs parameter specifies a positive time offset when the specified slate change will occur. The value is measured in milliseconds from the beginning of the broadcast's monitor stream, which is the time that the testing phase for the broadcast began. Even though it is specified in milliseconds, the value is actually an approximation, and YouTube completes the requested action as closely as possible to that time.
    /// 
    /// If you do not specify a value for this parameter, then YouTube performs the action as soon as possible. See the Getting started guide for more details.
    /// 
    /// Important: You should only specify a value for this parameter if your broadcast stream is delayed.
    pub fn offset_time_ms(mut self, new_value: &str) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._offset_time_ms = Some(new_value.to_string());
        self
    }
    /// Sets the *display slate* query property to the given value.
    ///
    /// 
    /// The displaySlate parameter specifies whether the slate is being enabled or disabled.
    pub fn display_slate(mut self, new_value: bool) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._display_slate = Some(new_value);
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastControlCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastControlCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastControlCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource's contentDetails object.
///
/// A builder for the *update* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::LiveBroadcast;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LiveBroadcast = Default::default();
/// req.status = Default::default(); // is LiveBroadcastStatus
/// req.snippet = Default::default(); // is LiveBroadcastSnippet
/// req.content_details = Default::default(); // is LiveBroadcastContentDetails
/// req.id = Some("voluptua.".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().update(&req)
///              .on_behalf_of_content_owner_channel("dolor")
///              .on_behalf_of_content_owner("et")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastUpdateCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveBroadcast,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastUpdateCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastUpdateCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcast)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.update", 
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &LiveBroadcast) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part properties that you can include in the parameter value are id, snippet, contentDetails, and status.
    /// 
    /// Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a broadcast's privacy status is defined in the status part. As such, if your request is updating a private or unlisted broadcast, and the request's part parameter value includes the status part, the broadcast's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the broadcast will revert to the default privacy setting.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastUpdateCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastUpdateCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastUpdateCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Creates a broadcast.
///
/// A builder for the *insert* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::LiveBroadcast;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: LiveBroadcast = Default::default();
/// req.status = Default::default(); // is LiveBroadcastStatus
/// req.snippet = Default::default(); // is LiveBroadcastSnippet
/// req.content_details = Default::default(); // is LiveBroadcastContentDetails
/// req.id = Some("et".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().insert(&req)
///              .on_behalf_of_content_owner_channel("vero")
///              .on_behalf_of_content_owner("ut")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveBroadcast,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcast)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    pub fn request(mut self, new_value: &LiveBroadcast) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part properties that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream.
///
/// A builder for the *bind* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().bind("id", "part")
///              .stream_id("ipsum")
///              .on_behalf_of_content_owner_channel("justo")
///              .on_behalf_of_content_owner("dolore")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastBindCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _part: String,
    _stream_id: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastBindCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastBindCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcast)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.bind", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._stream_id {
            params.push(("streamId", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "id", "part", "streamId", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts/bind".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the unique ID of the broadcast that is being bound to a video stream.
    pub fn id(mut self, new_value: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *stream id* query property to the given value.
    ///
    /// 
    /// The streamId parameter specifies the unique ID of the video stream that is being bound to a broadcast. If this parameter is omitted, the API will remove any existing binding between the broadcast and a video stream.
    pub fn stream_id(mut self, new_value: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._stream_id = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastBindCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastBindCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastBindCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of YouTube broadcasts that match the API request parameters.
///
/// A builder for the *list* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().list("part")
///              .page_token("dolor")
///              .on_behalf_of_content_owner_channel("takimata")
///              .on_behalf_of_content_owner("et")
///              .mine(false)
///              .max_results(17)
///              .id("sed")
///              .broadcast_status("no")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _broadcast_status: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcastListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._broadcast_status {
            params.push(("broadcastStatus", value.to_string()));
        }
        for &field in ["alt", "part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id", "broadcastStatus"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// The mine parameter can be used to instruct the API to only return broadcasts owned by the authenticated user. Set the parameter value to true to only retrieve your own broadcasts.
    pub fn mine(mut self, new_value: bool) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of YouTube broadcast IDs that identify the broadcasts being retrieved. In a liveBroadcast resource, the id property specifies the broadcast's ID.
    pub fn id(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *broadcast status* query property to the given value.
    ///
    /// 
    /// The broadcastStatus parameter filters the API response to only include broadcasts with the specified status.
    pub fn broadcast_status(mut self, new_value: &str) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._broadcast_status = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Deletes a broadcast.
///
/// A builder for the *delete* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().delete("id")
///              .on_behalf_of_content_owner_channel("rebum.")
///              .on_behalf_of_content_owner("labore")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastDeleteCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastDeleteCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastDeleteCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<hyper::client::Response> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.delete", 
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("id", self._id.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live broadcast ID for the resource that is being deleted.
    pub fn id(mut self, new_value: &str) -> LiveBroadcastDeleteCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastDeleteCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastDeleteCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastDeleteCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastDeleteCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast's status to testing, YouTube starts to transmit video to that broadcast's monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active.
///
/// A builder for the *transition* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
/// * *status*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().transition("broadcastStatus", "id", "part")
///              .on_behalf_of_content_owner_channel("sea")
///              .on_behalf_of_content_owner("elitr")
///              .doit();
/// # }
/// ```
pub struct LiveBroadcastTransitionCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _broadcast_status: String,
    _id: String,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for LiveBroadcastTransitionCall<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastTransitionCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, LiveBroadcast)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.liveBroadcasts.transition", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("broadcastStatus", self._broadcast_status.to_string()));
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._on_behalf_of_content_owner_channel {
            params.push(("onBehalfOfContentOwnerChannel", value.to_string()));
        }
        if let Some(value) = self._on_behalf_of_content_owner {
            params.push(("onBehalfOfContentOwner", value.to_string()));
        }
        for &field in ["alt", "broadcastStatus", "id", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/liveBroadcasts/transition".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *broadcast status* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The broadcastStatus parameter identifies the state to which the broadcast is changing. Note that to transition a broadcast to either the testing or live state, the status.streamStatus must be active for the stream that the broadcast is bound to.
    pub fn broadcast_status(mut self, new_value: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._broadcast_status = new_value.to_string();
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the unique ID of the broadcast that is transitioning to another status.
    pub fn id(mut self, new_value: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// * *status*
    /// 
    /// The part parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn part(mut self, new_value: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner channel* query property to the given value.
    ///
    /// 
    /// This parameter can only be used in a properly authorized request. Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwnerChannel parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies.
    /// 
    /// This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastTransitionCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> LiveBroadcastTransitionCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> LiveBroadcastTransitionCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of categories that can be associated with YouTube videos.
///
/// A builder for the *list* method supported by a *videoCategory* resource.
/// It is not used directly, but through a `VideoCategoryMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.video_categories().list("part")
///              .region_code("sea")
///              .id("consetetur")
///              .hl("diam")
///              .doit();
/// # }
/// ```
pub struct VideoCategoryListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _region_code: Option<String>,
    _id: Option<String>,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for VideoCategoryListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoCategoryListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, VideoCategoryListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.videoCategories.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((6 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._region_code {
            params.push(("regionCode", value.to_string()));
        }
        if let Some(value) = self._id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = self._hl {
            params.push(("hl", value.to_string()));
        }
        for &field in ["alt", "part", "regionCode", "id", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/videoCategories".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// 
    /// The part parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet.
    pub fn part(mut self, new_value: &str) -> VideoCategoryListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return the list of video categories available in the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.
    pub fn region_code(mut self, new_value: &str) -> VideoCategoryListCall<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of video category IDs for the resources that you are retrieving.
    pub fn id(mut self, new_value: &str) -> VideoCategoryListCall<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.
    pub fn hl(mut self, new_value: &str) -> VideoCategoryListCall<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoCategoryListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> VideoCategoryListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> VideoCategoryListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user's subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// **Settable Parts**
/// 
/// * *id*
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("part")
///              .region_code("dolores")
///              .published_before("consetetur")
///              .published_after("dolor")
///              .page_token("aliquyam")
///              .mine(false)
///              .max_results(85)
///              .home(true)
///              .channel_id("Stet")
///              .doit();
/// # }
/// ```
pub struct ActivityListCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _region_code: Option<String>,
    _published_before: Option<String>,
    _published_after: Option<String>,
    _page_token: Option<String>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _home: Option<bool>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ActivityListCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityListCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ActivityListResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.activities.list", 
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((11 + self._additional_params.len()));
        params.push(("part", self._part.to_string()));
        if let Some(value) = self._region_code {
            params.push(("regionCode", value.to_string()));
        }
        if let Some(value) = self._published_before {
            params.push(("publishedBefore", value.to_string()));
        }
        if let Some(value) = self._published_after {
            params.push(("publishedAfter", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._mine {
            params.push(("mine", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._home {
            params.push(("home", value.to_string()));
        }
        if let Some(value) = self._channel_id {
            params.push(("channelId", value.to_string()));
        }
        for &field in ["alt", "part", "regionCode", "publishedBefore", "publishedAfter", "pageToken", "mine", "maxResults", "home", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/activities".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Readonly.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }



        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *part* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *id*
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails.
    /// 
    /// If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set part=snippet, the API response will also contain all of those nested properties.
    pub fn part(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code. YouTube uses this value when the authorized user's previous activity on YouTube does not provide enough information to generate the activity feed.
    pub fn region_code(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *published before* query property to the given value.
    ///
    /// 
    /// The publishedBefore parameter specifies the date and time before which an activity must have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be excluded from the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    pub fn published_before(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._published_before = Some(new_value.to_string());
        self
    }
    /// Sets the *published after* query property to the given value.
    ///
    /// 
    /// The publishedAfter parameter specifies the earliest date and time that an activity could have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be included in the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
    pub fn published_after(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._published_after = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's activities.
    pub fn mine(mut self, new_value: bool) -> ActivityListCall<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    pub fn max_results(mut self, new_value: u32) -> ActivityListCall<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *home* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve the activity feed that displays on the YouTube home page for the currently authenticated user.
    pub fn home(mut self, new_value: bool) -> ActivityListCall<'a, C, NC, A> {
        self._home = Some(new_value);
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a unique YouTube channel ID. The API will then return a list of that channel's activities.
    pub fn channel_id(mut self, new_value: &str) -> ActivityListCall<'a, C, NC, A> {
        self._channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityListCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivityListCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


/// Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel's behalf.)
/// 
/// Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API's videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.
///
/// A builder for the *insert* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethods` instance.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for at least one of the following scopes to make a valid call, possibly depending on *parts*:
/// 
/// * *https://www.googleapis.com/auth/youtube*
/// * *https://www.googleapis.com/auth/youtube.force-ssl*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "google-youtube3" as youtube3;
/// use youtube3::Activity;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::new(),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req: Activity = Default::default();
/// req.snippet = Default::default(); // is ActivitySnippet
/// req.content_details = Default::default(); // is ActivityContentDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().insert(&req)
///              .doit();
/// # }
/// ```
pub struct ActivityInsertCall<'a, C, NC, A>
    where C: 'a, NC: 'a, A: 'a {

    hub: &'a YouTube<C, NC, A>,
    _request: Activity,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, NC, A> CallBuilder for ActivityInsertCall<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityInsertCall<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Activity)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "youtube.activities.insert", 
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["alt", "part"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = "https://www.googleapis.com/youtube/v3/activities".to_string();
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Full.as_slice().to_string(), ());
        }

        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader = io::Cursor::new(json::to_vec(&self._request));
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let mut token = self.hub.auth.borrow_mut().token(self._scopes.keys());
            if token.is_none() {
                token = dlg.token();
            }
            if token.is_none() {
                dlg.finished(false);
                return Err(Error::MissingToken)
            }
            let auth_header = Authorization(oauth2::Scheme { token_type: oauth2::TokenType::Bearer,
                                                             access_token: token.unwrap().access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.as_slice())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res, json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return Err(Error::Failure(res))
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    pub fn request(mut self, new_value: &Activity) -> ActivityInsertCall<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *part* query property to the given value.
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    ///
    /// **Settable Parts**
    /// 
    /// * *snippet*
    /// * *contentDetails*
    /// 
    /// The part parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
    /// 
    /// The part names that you can include in the parameter value are snippet and contentDetails.
    pub fn part(mut self, new_value: &str) -> ActivityInsertCall<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *delegate* property to the given value.
    ///
    /// 
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityInsertCall<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    /// 
    /// # Additional Parameters
    ///
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for the response.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityInsertCall<'a, C, NC, A>
                                                        where T: Str {
        self._additional_params.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ActivityInsertCall<'a, C, NC, A> 
                                                        where T: Str {
        self._scopes.insert(scope.as_slice().to_string(), ());
        self
    }
}


