// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! # Features
//! 
//! Handle the following *Resources* with ease ... 
//! 
//! * [activities](struct.Activity.html) ([*insert*](struct.ActivityInsertMethodBuilder.html) and [*list*](struct.ActivityListMethodBuilder.html))
//! * channel banners ([*insert*](struct.ChannelBannerInsertMethodBuilder.html))
//! * [channel sections](struct.ChannelSection.html) ([*delete*](struct.ChannelSectionDeleteMethodBuilder.html), [*insert*](struct.ChannelSectionInsertMethodBuilder.html), [*list*](struct.ChannelSectionListMethodBuilder.html) and [*update*](struct.ChannelSectionUpdateMethodBuilder.html))
//! * [channels](struct.Channel.html) ([*list*](struct.ChannelListMethodBuilder.html) and [*update*](struct.ChannelUpdateMethodBuilder.html))
//! * [guide categories](struct.GuideCategory.html) ([*list*](struct.GuideCategoryListMethodBuilder.html))
//! * [i18n languages](struct.I18nLanguage.html) ([*list*](struct.I18nLanguageListMethodBuilder.html))
//! * [i18n regions](struct.I18nRegion.html) ([*list*](struct.I18nRegionListMethodBuilder.html))
//! * [live broadcasts](struct.LiveBroadcast.html) ([*bind*](struct.LiveBroadcastBindMethodBuilder.html), [*control*](struct.LiveBroadcastControlMethodBuilder.html), [*delete*](struct.LiveBroadcastDeleteMethodBuilder.html), [*insert*](struct.LiveBroadcastInsertMethodBuilder.html), [*list*](struct.LiveBroadcastListMethodBuilder.html), [*transition*](struct.LiveBroadcastTransitionMethodBuilder.html) and [*update*](struct.LiveBroadcastUpdateMethodBuilder.html))
//! * [live streams](struct.LiveStream.html) ([*delete*](struct.LiveStreamDeleteMethodBuilder.html), [*insert*](struct.LiveStreamInsertMethodBuilder.html), [*list*](struct.LiveStreamListMethodBuilder.html) and [*update*](struct.LiveStreamUpdateMethodBuilder.html))
//! * [playlist items](struct.PlaylistItem.html) ([*delete*](struct.PlaylistItemDeleteMethodBuilder.html), [*insert*](struct.PlaylistItemInsertMethodBuilder.html), [*list*](struct.PlaylistItemListMethodBuilder.html) and [*update*](struct.PlaylistItemUpdateMethodBuilder.html))
//! * [playlists](struct.Playlist.html) ([*delete*](struct.PlaylistDeleteMethodBuilder.html), [*insert*](struct.PlaylistInsertMethodBuilder.html), [*list*](struct.PlaylistListMethodBuilder.html) and [*update*](struct.PlaylistUpdateMethodBuilder.html))
//! * search ([*list*](struct.SearchListMethodBuilder.html))
//! * [subscriptions](struct.Subscription.html) ([*delete*](struct.SubscriptionDeleteMethodBuilder.html), [*insert*](struct.SubscriptionInsertMethodBuilder.html) and [*list*](struct.SubscriptionListMethodBuilder.html))
//! * [thumbnails](struct.Thumbnail.html) ([*set*](struct.ThumbnailSetMethodBuilder.html))
//! * [video categories](struct.VideoCategory.html) ([*list*](struct.VideoCategoryListMethodBuilder.html))
//! * [videos](struct.Video.html) ([*delete*](struct.VideoDeleteMethodBuilder.html), [*getRating*](struct.VideoGetRatingMethodBuilder.html), [*insert*](struct.VideoInsertMethodBuilder.html), [*list*](struct.VideoListMethodBuilder.html), [*rate*](struct.VideoRateMethodBuilder.html) and [*update*](struct.VideoUpdateMethodBuilder.html))
//! * watermarks ([*set*](struct.WatermarkSetMethodBuilder.html) and [*unset*](struct.WatermarkUnsetMethodBuilder.html))
//! 
//! Everything else about the *YouTube* API can be found at the
//! [official documentation site](https://developers.google.com/youtube/v3).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **Hub**
//!     * a central object to maintain state and allow accessing all *Activities*
//! * **Resources**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **Parts**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **Activities**
//!     * operations to apply to *Resources*
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
//! # Usage (*TODO*)
//! 
//! ## Instantiating the Hub
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate "yup-oauth2" as oauth2;
//! extern crate "rustc-serialize" as rustc_serialize;
//! extern crate youtube3;
//! 
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! # use youtube3::YouTube;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and `client_secret`, 
//! // among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about what's going on
//! // You probably want to bring in your own `TokenStorage` to persist tokens and retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = YouTube::new(hyper::Client::new(), auth);
//! # }
//! ```
//! 
//! **TODO** Example calls - there should soon be a generator able to do that with proper inputs
//! 
//! ## Handling Errors
//! 
//! # Some details
//! 
//! ## About Customization/Callbacks
//! 
//! ## About parts
//! 
//! * Optionals needed for Json, otherwise I'd happily drop them
//! * explain that examples use all response parts, even though they are shown for request values
//! 
//! ## About builder arguments
//! 
//! * pods are copy
//! * strings are &str
//! * request values are borrowed
//! * additional parameters using `param()`
//! 
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 
#![feature(core,io)]

extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::default::Default;
use std::io::{Read, Seek};
use std::fs;

pub use cmn::{Hub, ReadSeek, ResourceMethodsBuilder, MethodBuilder, Resource, Part, ResponseResult, RequestValue,
              NestedType, Delegate, DefaultDelegate, Result};


// ##############
// UTILITIES ###
// ############

/// This macro is advertised in the documentation, which is why we deliver it as well
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
pub enum Scope {
    /// View private information of your YouTube channel relevant during the audit process with a YouTube partner
    PartnerChannelAudit,

    /// View your YouTube account
    Readonly,

    /// Manage your YouTube account
    Full,

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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and `client_secret`, 
/// // among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about what's going on
/// // You probably want to bring in your own `TokenStorage` to persist tokens and retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// # }
/// ```
/// 
pub struct YouTube<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for YouTube<C, NC, A> {}

impl<'a, C, NC, A> YouTube<C, NC, A>
    where  NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> YouTube<C, NC, A> {
        YouTube {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _m: PhantomData,
        }
    }

    pub fn activities(&'a self) -> ActivityMethodsBuilder<'a, C, NC, A> {
        ActivityMethodsBuilder { hub: &self }
    }
    pub fn channel_banners(&'a self) -> ChannelBannerMethodsBuilder<'a, C, NC, A> {
        ChannelBannerMethodsBuilder { hub: &self }
    }
    pub fn channel_sections(&'a self) -> ChannelSectionMethodsBuilder<'a, C, NC, A> {
        ChannelSectionMethodsBuilder { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethodsBuilder<'a, C, NC, A> {
        ChannelMethodsBuilder { hub: &self }
    }
    pub fn guide_categories(&'a self) -> GuideCategoryMethodsBuilder<'a, C, NC, A> {
        GuideCategoryMethodsBuilder { hub: &self }
    }
    pub fn i18n_languages(&'a self) -> I18nLanguageMethodsBuilder<'a, C, NC, A> {
        I18nLanguageMethodsBuilder { hub: &self }
    }
    pub fn i18n_regions(&'a self) -> I18nRegionMethodsBuilder<'a, C, NC, A> {
        I18nRegionMethodsBuilder { hub: &self }
    }
    pub fn live_broadcasts(&'a self) -> LiveBroadcastMethodsBuilder<'a, C, NC, A> {
        LiveBroadcastMethodsBuilder { hub: &self }
    }
    pub fn live_streams(&'a self) -> LiveStreamMethodsBuilder<'a, C, NC, A> {
        LiveStreamMethodsBuilder { hub: &self }
    }
    pub fn playlist_items(&'a self) -> PlaylistItemMethodsBuilder<'a, C, NC, A> {
        PlaylistItemMethodsBuilder { hub: &self }
    }
    pub fn playlists(&'a self) -> PlaylistMethodsBuilder<'a, C, NC, A> {
        PlaylistMethodsBuilder { hub: &self }
    }
    pub fn search(&'a self) -> SearchMethodsBuilder<'a, C, NC, A> {
        SearchMethodsBuilder { hub: &self }
    }
    pub fn subscriptions(&'a self) -> SubscriptionMethodsBuilder<'a, C, NC, A> {
        SubscriptionMethodsBuilder { hub: &self }
    }
    pub fn thumbnails(&'a self) -> ThumbnailMethodsBuilder<'a, C, NC, A> {
        ThumbnailMethodsBuilder { hub: &self }
    }
    pub fn video_categories(&'a self) -> VideoCategoryMethodsBuilder<'a, C, NC, A> {
        VideoCategoryMethodsBuilder { hub: &self }
    }
    pub fn videos(&'a self) -> VideoMethodsBuilder<'a, C, NC, A> {
        VideoMethodsBuilder { hub: &self }
    }
    pub fn watermarks(&'a self) -> WatermarkMethodsBuilder<'a, C, NC, A> {
        WatermarkMethodsBuilder { hub: &self }
    }
}


// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoConversionPings {
	/// Pings that the app shall fire for a video (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
	pub pings: Vec<VideoConversionPing>,
}

impl Part for VideoConversionPings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.SubscriptionListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#subscriptionListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of subscriptions that match the request criteria.
	pub items: Vec<Subscription>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for SubscriptionListResponse {}


/// Information about a resource that received a positive (like) rating.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsLike {
	/// The resourceId object contains information that identifies the rated resource.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsLike {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastSnippet {
	/// The date and time that the broadcast actually ended. This information is only available once the broadcast's state is complete. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub actual_end_time: Option<String>,
	/// The broadcast's description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource.
	pub description: Option<String>,
	/// The broadcast's title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource.
	pub title: Option<String>,
	/// The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast.
	pub channel_id: Option<String>,
	/// The date and time that the broadcast was added to YouTube's live broadcast schedule. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// The date and time that the broadcast is scheduled to start. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduled_start_time: Option<String>,
	/// The date and time that the broadcast actually started. This information is only available once the broadcast's state is live. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub actual_start_time: Option<String>,
	/// The date and time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduled_end_time: Option<String>,
	/// A map of thumbnail images associated with the broadcast. For each nested object in this object, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
}

impl Part for LiveBroadcastSnippet {}


/// Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoFileDetails {
	/// The uploaded video file's combined (video and audio) bitrate in bits per second.
	pub bitrate_bps: Option<String>,
	/// The uploaded video file's container format.
	pub container: Option<String>,
	/// Geographic coordinates that identify the place where the uploaded video was recorded. Coordinates are defined using WGS 84.
	pub recording_location: Option<GeoPoint>,
	/// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
	pub file_type: Option<String>,
	/// The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported:  
/// - Date only: YYYY-MM-DD 
/// - Naive time: YYYY-MM-DDTHH:MM:SS 
/// - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM
	pub creation_time: Option<String>,
	/// The length of the uploaded video in milliseconds.
	pub duration_ms: Option<String>,
	/// The uploaded file's name. This field is present whether a video file or another type of file was uploaded.
	pub file_name: Option<String>,
	/// The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.
	pub file_size: Option<String>,
	/// A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.
	pub video_streams: Vec<VideoFileDetailsVideoStream>,
	/// A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.
	pub audio_streams: Vec<VideoFileDetailsAudioStream>,
}

impl Part for VideoFileDetails {}


/// Playlist localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistLocalization {
	/// The localized strings for playlist's description.
	pub description: Option<String>,
	/// The localized strings for playlist's title.
	pub title: Option<String>,
}

impl Part for PlaylistLocalization {}


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
/// * [insert](struct.PlaylistInsertMethodBuilder.html) (request|response)
/// * [delete](struct.PlaylistDeleteMethodBuilder.html) (none)
/// * [list](struct.PlaylistListMethodBuilder.html) (none)
/// * [update](struct.PlaylistUpdateMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Playlist {
	/// The status object contains status information for the playlist.
	pub status: Option<PlaylistStatus>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".
	pub kind: Option<String>,
	/// The contentDetails object contains information like video count.
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
	pub localizations: HashMap<String, PlaylistLocalization>,
}

impl RequestValue for Playlist {}
impl Resource for Playlist {}
impl ResponseResult for Playlist {}

impl Playlist {
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
		if self.localizations.len() > 0 { r = r + "localizations,"; }
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
/// * [list](struct.PlaylistItemListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItemListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of playlist items that match the request criteria.
	pub items: Vec<PlaylistItem>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for PlaylistItemListResponse {}


/// A pair Property / Value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PropertyValue {
	/// A property.
	pub property: Option<String>,
	/// The property's value.
	pub value: Option<String>,
}

impl Part for PropertyValue {}


/// Describes a temporal position of a visual widget inside a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoTiming {
	/// Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds.
	pub offset_ms: Option<String>,
	/// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
	pub type_: Option<String>,
	/// Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.
	pub duration_ms: Option<String>,
}

impl Part for InvideoTiming {}


/// Basic details about a playlist, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistSnippet {
	/// The playlist's description.
	pub description: Option<String>,
	/// Keyword tags associated with the playlist.
	pub tags: Vec<String>,
	/// The ID that YouTube uses to uniquely identify the channel that published the playlist.
	pub channel_id: Option<String>,
	/// The date and time that the playlist was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// The channel title of the channel that the video belongs to.
	pub channel_title: Option<String>,
	/// The playlist's title.
	pub title: Option<String>,
	/// The language of the playlist's default title and description.
	pub default_language: Option<String>,
	/// Localized title and description, read-only.
	pub localized: Option<PlaylistLocalization>,
	/// A map of thumbnail images associated with the playlist. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
}

impl Part for PlaylistSnippet {}


/// The auditDetails object encapsulates channel data that is relevant for YouTube Partners during the audit process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelAuditDetails {
	/// Whether or not the channel has any copyright strikes.
	pub copyright_strikes_good_standing: Option<bool>,
	/// Whether or not the channel respects the community guidelines.
	pub community_guidelines_good_standing: Option<bool>,
	/// Whether or not the channel has any unresolved claims.
	pub content_id_claims_good_standing: Option<bool>,
	/// Describes the general state of the channel. This field will always show if there are any issues whatsoever with the channel. Currently this field represents the result of the logical and operation over the community guidelines good standing, the copyright strikes good standing and the content ID claims good standing, but this may change in the future.
	pub overall_good_standing: Option<bool>,
}

impl Part for ChannelAuditDetails {}


/// A live stream describes a live ingestion point.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete](struct.LiveStreamDeleteMethodBuilder.html) (none)
/// * [update](struct.LiveStreamUpdateMethodBuilder.html) (request|response)
/// * [list](struct.LiveStreamListMethodBuilder.html) (none)
/// * [insert](struct.LiveStreamInsertMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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
	pub content_details: Option<LiveStreamContentDetails>,
	/// The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube.
	pub cdn: Option<CdnSettings>,
	/// The ID that YouTube assigns to uniquely identify the stream.
	pub id: Option<String>,
}

impl RequestValue for LiveStream {}
impl Resource for LiveStream {}
impl ResponseResult for LiveStream {}

impl LiveStream {
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
/// * [set](struct.ThumbnailSetMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ThumbnailSetResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// A list of thumbnails.
	pub items: Vec<ThumbnailDetails>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#thumbnailSetResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
}

impl ResponseResult for ThumbnailSetResponse {}


/// Information about the uploaded video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsUpload {
	/// The ID that YouTube uses to uniquely identify the uploaded video.
	pub video_id: Option<String>,
}

impl Part for ActivityContentDetailsUpload {}


/// Branding properties for the channel view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSettings {
	/// Specifies the channel description.
	pub description: Option<String>,
	/// Specifies the channel title.
	pub title: Option<String>,
	/// Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible.
	pub moderate_comments: Option<bool>,
	/// Whether the tab to browse the videos should be displayed.
	pub show_browse_view: Option<bool>,
	/// Title for the featured channels tab.
	pub featured_channels_title: Option<String>,
	/// no description provided
	pub default_language: Option<String>,
	/// The trailer of the channel, for users that are not subscribers.
	pub unsubscribed_trailer: Option<String>,
	/// The list of featured channels.
	pub featured_channels_urls: Vec<String>,
	/// A prominent color that can be rendered on this channel page.
	pub profile_color: Option<String>,
	/// Which content tab users should see when viewing the channel.
	pub default_tab: Option<String>,
	/// Lists keywords associated with the channel, comma-separated.
	pub keywords: Option<String>,
	/// Whether related channels should be proposed.
	pub show_related_channels: Option<bool>,
	/// The ID for a Google Analytics account to track and measure traffic to the channels.
	pub tracking_analytics_account_id: Option<String>,
}

impl Part for ChannelSettings {}


/// Statistics about the video, such as the number of times the video was viewed or liked.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoStatistics {
	/// The number of comments for the video.
	pub comment_count: Option<i64>,
	/// The number of times the video has been viewed.
	pub view_count: Option<i64>,
	/// The number of users who currently have the video marked as a favorite video.
	pub favorite_count: Option<i64>,
	/// The number of users who have indicated that they disliked the video by giving it a negative rating.
	pub dislike_count: Option<i64>,
	/// The number of users who have indicated that they liked the video by giving it a positive rating.
	pub like_count: Option<i64>,
}

impl Part for VideoStatistics {}


/// Brief description of the live stream cdn settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct CdnSettings {
	/// The format of the video stream that you are sending to Youtube.
	pub format: Option<String>,
	/// The ingestionInfo object contains information that YouTube provides that you need to transmit your RTMP or HTTP stream to YouTube.
	pub ingestion_info: Option<IngestionInfo>,
	/// The method or protocol used to transmit the video stream.
	pub ingestion_type: Option<String>,
}

impl Part for CdnSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [getRating](struct.VideoGetRatingMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoGetRatingResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// A list of ratings that match the request criteria.
	pub items: Vec<VideoRating>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoGetRatingResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
}

impl ResponseResult for VideoGetRatingResponse {}


/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoCategorySnippet {
	/// no description provided
	pub assignable: Option<bool>,
	/// The YouTube channel that created the video category.
	pub channel_id: Option<String>,
	/// The video category's title.
	pub title: Option<String>,
}

impl Part for VideoCategorySnippet {}


/// Details about a resource which was added to a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsChannelItem {
	/// The resourceId object contains information that identifies the resource that was added to the channel.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsChannelItem {}


/// Basic details about an i18n language, such as language code and human-readable name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nLanguageSnippet {
	/// The human-readable name of the language in the language itself.
	pub name: Option<String>,
	/// A short BCP-47 code that uniquely identifies a language.
	pub hl: Option<String>,
}

impl Part for I18nLanguageSnippet {}


/// Basic details about a subscription, including title, description and thumbnails of the subscribed item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionSnippet {
	/// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The subscription's title.
	pub title: Option<String>,
	/// The id object contains information about the channel that the user subscribed to.
	pub resource_id: Option<ResourceId>,
	/// The subscription's details.
	pub description: Option<String>,
	/// The ID that YouTube uses to uniquely identify the subscriber's channel.
	pub channel_id: Option<String>,
	/// The date and time that the subscription was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// Channel title for the channel that the subscription belongs to.
	pub channel_title: Option<String>,
}

impl Part for SubscriptionSnippet {}


/// Details about a channelsection, including playlists and channels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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
/// * [list](struct.I18nRegionListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nRegionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource.
	pub items: Vec<I18nRegion>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegionListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
}

impl ResponseResult for I18nRegionListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.LiveStreamListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStreamListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of live streams that match the request criteria.
	pub items: Vec<LiveStream>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for LiveStreamListResponse {}


/// Describes a single promoted item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PromotedItem {
	/// The temporal position within the video where the promoted item will be displayed. If present, it overrides the default timing.
	pub timing: Option<InvideoTiming>,
	/// If true, the content owner's name will be used when displaying the promotion. This field can only be set when the update is made on behalf of the content owner.
	pub promoted_by_content_owner: Option<bool>,
	/// A custom message to display for this promotion. This field is currently ignored unless the promoted item is a website.
	pub custom_message: Option<String>,
	/// Identifies the promoted item.
	pub id: Option<PromotedItemId>,
}

impl Part for PromotedItem {}


/// Branding properties of a YouTube channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelBrandingSettings {
	/// Branding properties for branding images.
	pub image: Option<ImageSettings>,
	/// Branding properties for the watch page.
	pub watch: Option<WatchSettings>,
	/// Branding properties for the channel view.
	pub channel: Option<ChannelSettings>,
	/// Additional experimental branding properties.
	pub hints: Vec<PropertyValue>,
}

impl Part for ChannelBrandingSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.PlaylistListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of playlists that match the request criteria.
	pub items: Vec<Playlist>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for PlaylistListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set](struct.WatermarkSetMethodBuilder.html) (request)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoBranding {
	/// no description provided
	pub target_channel_id: Option<String>,
	/// no description provided
	pub position: Option<InvideoPosition>,
	/// no description provided
	pub image_url: Option<String>,
	/// no description provided
	pub timing: Option<InvideoTiming>,
	/// no description provided
	pub image_bytes: Option<String>,
}

impl RequestValue for InvideoBranding {}

impl InvideoBranding {
	/// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
	/// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
	/// the parts you want to see in the server response.
	fn to_parts(&self) -> String {
		let mut r = String::new();
		if self.target_channel_id.is_some() { r = r + "targetChannelId,"; }
		if self.position.is_some() { r = r + "position,"; }
		if self.image_url.is_some() { r = r + "imageUrl,"; }
		if self.timing.is_some() { r = r + "timing,"; }
		if self.image_bytes.is_some() { r = r + "imageBytes,"; }
		r.pop();
		r
	}
}

/// Information about the playlist item's privacy status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemStatus {
	/// This resource's privacy status.
	pub privacy_status: Option<String>,
}

impl Part for PlaylistItemStatus {}


/// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelConversionPing {
	/// Defines the context of the ping.
	pub context: Option<String>,
	/// The url (without the schema) that the player shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D cview%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping.
	pub conversion_url: Option<String>,
}

impl Part for ChannelConversionPing {}


/// Describes an invideo promotion campaign consisting of multiple promoted items. A campaign belongs to a single channel_id.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoPromotion {
	/// The default temporal position within the video where the promoted item will be displayed. Can be overriden by more specific timing in the item.
	pub default_timing: Option<InvideoTiming>,
	/// List of promoted items in decreasing priority.
	pub items: Vec<PromotedItem>,
	/// Indicates whether the channel's promotional campaign uses "smart timing." This feature attempts to show promotions at a point in the video when they are more likely to be clicked and less likely to disrupt the viewing experience. This feature also picks up a single promotion to show on each video.
	pub use_smart_timing: Option<bool>,
	/// The spatial position within the video where the promoted item will be displayed.
	pub position: Option<InvideoPosition>,
}

impl Part for InvideoPromotion {}


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
/// * [insert](struct.PlaylistItemInsertMethodBuilder.html) (request|response)
/// * [update](struct.PlaylistItemUpdateMethodBuilder.html) (request|response)
/// * [list](struct.PlaylistItemListMethodBuilder.html) (none)
/// * [delete](struct.PlaylistItemDeleteMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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
	pub content_details: Option<PlaylistItemContentDetails>,
	/// The ID that YouTube uses to uniquely identify the playlist item.
	pub id: Option<String>,
}

impl RequestValue for PlaylistItem {}
impl Resource for PlaylistItem {}
impl ResponseResult for PlaylistItem {}

impl PlaylistItem {
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
/// * [list](struct.GuideCategoryListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GuideCategoryListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#guideCategoryListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of categories that can be associated with YouTube channels. In this map, the category ID is the map key, and its value is the corresponding guideCategory resource.
	pub items: Vec<GuideCategory>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for GuideCategoryListResponse {}


/// Localized versions of certain video properties (e.g. title).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoLocalization {
	/// Localized version of the video's description.
	pub description: Option<String>,
	/// Localized version of the video's title.
	pub title: Option<String>,
}

impl Part for VideoLocalization {}


/// Basic details about a channel section, including title, style and position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSectionSnippet {
	/// The style of the channel section.
	pub style: Option<String>,
	/// Localized title, read-only.
	pub localized: Option<ChannelSectionLocalization>,
	/// The channel section's title for multiple_playlists and multiple_channels.
	pub title: Option<String>,
	/// The position of the channel section in the channel.
	pub position: Option<u32>,
	/// The ID that YouTube uses to uniquely identify the channel that published the channel section.
	pub channel_id: Option<String>,
	/// The type of the channel section.
	pub type_: Option<String>,
	/// The language of the channel section's default title and description.
	pub default_language: Option<String>,
}

impl Part for ChannelSectionSnippet {}


/// Details about the content of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelContentDetails {
	/// no description provided
	pub related_playlists: HashMap<String, ChannelContentDetailsRelatedPlaylists>,
	/// The googlePlusUserId object identifies the Google+ profile ID associated with this channel.
	pub google_plus_user_id: Option<String>,
}

impl Part for ChannelContentDetails {}


/// Stub token pagination template to suppress results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct TokenPagination;

impl Part for TokenPagination {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemContentDetails {
	/// A user-generated note for this item.
	pub note: Option<String>,
	/// The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.
	pub start_at: Option<String>,
	/// The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.
	pub end_at: Option<String>,
	/// The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request.
	pub video_id: Option<String>,
}

impl Part for PlaylistItemContentDetails {}


/// Internal representation of thumbnails for a YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ThumbnailDetails {
	/// The default image for this resource.
	pub default: Option<Thumbnail>,
	/// The high quality image for this resource.
	pub high: Option<Thumbnail>,
	/// The medium quality image for this resource.
	pub medium: Option<Thumbnail>,
	/// The maximum resolution quality image for this resource.
	pub maxres: Option<Thumbnail>,
	/// The standard quality image for this resource.
	pub standard: Option<Thumbnail>,
}

impl Part for ThumbnailDetails {}


/// Details about monetization of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoMonetizationDetails {
	/// The value of access indicates whether the video can be monetized or not.
	pub access: Option<AccessPolicy>,
}

impl Part for VideoMonetizationDetails {}


/// Information that identifies the recommended resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsRecommendation {
	/// The resourceId object contains information that identifies the recommended resource.
	pub resource_id: Option<ResourceId>,
	/// The reason that the resource is recommended to the user.
	pub reason: Option<String>,
	/// The seedResourceId object contains information about the resource that caused the recommendation.
	pub seed_resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsRecommendation {}


/// Recording information associated with the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoRecordingDetails {
	/// The date and time when the video was recorded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.
	pub recording_date: Option<String>,
	/// The text description of the location where the video was recorded.
	pub location_description: Option<String>,
	/// The geolocation information associated with the video.
	pub location: Option<GeoPoint>,
}

impl Part for VideoRecordingDetails {}


/// Information about a channel that a user subscribed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsSubscription {
	/// The resourceId object contains information that identifies the resource that the user subscribed to.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsSubscription {}


/// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelConversionPings {
	/// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
	pub pings: Vec<ChannelConversionPing>,
}

impl Part for ChannelConversionPings {}


/// Details about the content of an activity: the video that was shared, the channel that was subscribed to, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetails {
	/// The comment object contains information about a resource that received a comment. This property is only present if the snippet.type is comment.
	pub comment: Option<ActivityContentDetailsComment>,
	/// The playlistItem object contains information about a new playlist item. This property is only present if the snippet.type is playlistItem.
	pub playlist_item: Option<ActivityContentDetailsPlaylistItem>,
	/// The like object contains information about a resource that received a positive (like) rating. This property is only present if the snippet.type is like.
	pub like: Option<ActivityContentDetailsLike>,
	/// The promotedItem object contains details about a resource which is being promoted. This property is only present if the snippet.type is promotedItem.
	pub promoted_item: Option<ActivityContentDetailsPromotedItem>,
	/// The recommendation object contains information about a recommended resource. This property is only present if the snippet.type is recommendation.
	pub recommendation: Option<ActivityContentDetailsRecommendation>,
	/// The favorite object contains information about a video that was marked as a favorite video. This property is only present if the snippet.type is favorite.
	pub favorite: Option<ActivityContentDetailsFavorite>,
	/// The upload object contains information about the uploaded video. This property is only present if the snippet.type is upload.
	pub upload: Option<ActivityContentDetailsUpload>,
	/// The social object contains details about a social network post. This property is only present if the snippet.type is social.
	pub social: Option<ActivityContentDetailsSocial>,
	/// The channelItem object contains details about a resource which was added to a channel. This property is only present if the snippet.type is channelItem.
	pub channel_item: Option<ActivityContentDetailsChannelItem>,
	/// The bulletin object contains details about a channel bulletin post. This object is only present if the snippet.type is bulletin.
	pub bulletin: Option<ActivityContentDetailsBulletin>,
	/// The subscription object contains information about a channel that a user subscribed to. This property is only present if the snippet.type is subscription.
	pub subscription: Option<ActivityContentDetailsSubscription>,
}

impl Part for ActivityContentDetails {}


/// A i18nRegion resource identifies a region where YouTube is available.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.I18nRegionListMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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


/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelContentOwnerDetails {
	/// The ID of the content owner linked to the channel.
	pub content_owner: Option<String>,
	/// The date and time of when the channel was linked to the content owner. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub time_linked: Option<String>,
}

impl Part for ChannelContentOwnerDetails {}


/// Describes processing status and progress and availability of some other Video resource parts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoProcessingDetails {
	/// This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.
	pub file_details_availability: Option<String>,
	/// This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub editor_suggestions_availability: Option<String>,
	/// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
	pub processing_status: Option<String>,
	/// This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub processing_issues_availability: Option<String>,
	/// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
	pub processing_failure_reason: Option<String>,
	/// This value indicates whether thumbnail images have been generated for the video.
	pub thumbnails_availability: Option<String>,
	/// The processingProgress object contains information about the progress YouTube has made in processing the video. The values are really only relevant if the video's processing status is processing.
	pub processing_progress: Option<VideoProcessingDetailsProcessingProgress>,
	/// This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub tag_suggestions_availability: Option<String>,
}

impl Part for VideoProcessingDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastStatus {
	/// The broadcast's recording status.
	pub recording_status: Option<String>,
	/// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
	pub privacy_status: Option<String>,
	/// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
	pub life_cycle_status: Option<String>,
	/// Priority of the live broadcast event (internal state).
	pub live_broadcast_priority: Option<String>,
}

impl Part for LiveBroadcastStatus {}


/// Details about the content to witch a subscription refers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionContentDetails {
	/// The number of new items in the subscription since its content was last read.
	pub new_item_count: Option<u32>,
	/// The type of activity this subscription is for (only uploads, everything).
	pub activity_type: Option<String>,
	/// The approximate number of items that the subscription points to.
	pub total_item_count: Option<u32>,
}

impl Part for SubscriptionContentDetails {}


/// A video resource represents a YouTube video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rate](struct.VideoRateMethodBuilder.html) (none)
/// * [getRating](struct.VideoGetRatingMethodBuilder.html) (none)
/// * [list](struct.VideoListMethodBuilder.html) (none)
/// * [insert](struct.VideoInsertMethodBuilder.html) (request|response)
/// * [update](struct.VideoUpdateMethodBuilder.html) (request|response)
/// * [delete](struct.VideoDeleteMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Video {
	/// The status object contains information about the video's uploading, processing, and privacy statuses.
	pub status: Option<VideoStatus>,
	/// The topicDetails object encapsulates information about Freebase topics associated with the video.
	pub topic_details: Option<VideoTopicDetails>,
	/// The monetizationDetails object encapsulates information about the monetization status of the video.
	pub monetization_details: Option<VideoMonetizationDetails>,
	/// The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner.
	pub suggestions: Option<VideoSuggestions>,
	/// Age restriction details related to a video.
	pub age_gating: Option<VideoAgeGating>,
	/// The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner.
	pub file_details: Option<VideoFileDetails>,
	/// The player object contains information that you would use to play the video in an embedded player.
	pub player: Option<VideoPlayer>,
	/// The ID that YouTube uses to uniquely identify the video.
	pub id: Option<String>,
	/// List with all localizations.
	pub localizations: HashMap<String, VideoLocalization>,
	/// The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast.
	pub live_streaming_details: Option<VideoLiveStreamingDetails>,
	/// The processingProgress object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video.
/// 
/// The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner.
	pub processing_details: Option<VideoProcessingDetails>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#video".
	pub kind: Option<String>,
	/// The statistics object contains statistics about the video.
	pub statistics: Option<VideoStatistics>,
	/// The contentDetails object contains information about the video content, including the length of the video and its aspect ratio.
	pub content_details: Option<VideoContentDetails>,
	/// The conversionPings object encapsulates information about url pings that need to be respected by the App in different video contexts.
	pub conversion_pings: Option<VideoConversionPings>,
	/// The snippet object contains basic details about the video, such as its title, description, and category.
	pub snippet: Option<VideoSnippet>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The projectDetails object contains information about the project specific video metadata.
	pub project_details: Option<VideoProjectDetails>,
	/// The recordingDetails object encapsulates information about the location, date and address where the video was recorded.
	pub recording_details: Option<VideoRecordingDetails>,
}

impl RequestValue for Video {}
impl Resource for Video {}
impl ResponseResult for Video {}

impl Video {
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
		if self.localizations.len() > 0 { r = r + "localizations,"; }
		if self.live_streaming_details.is_some() { r = r + "liveStreamingDetails,"; }
		if self.processing_details.is_some() { r = r + "processingDetails,"; }
		if self.kind.is_some() { r = r + "kind,"; }
		if self.statistics.is_some() { r = r + "statistics,"; }
		if self.content_details.is_some() { r = r + "contentDetails,"; }
		if self.conversion_pings.is_some() { r = r + "conversionPings,"; }
		if self.snippet.is_some() { r = r + "snippet,"; }
		if self.etag.is_some() { r = r + "etag,"; }
		if self.project_details.is_some() { r = r + "projectDetails,"; }
		if self.recording_details.is_some() { r = r + "recordingDetails,"; }
		r.pop();
		r
	}
}

/// Geographical coordinates of a point, in WGS84.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GeoPoint {
	/// Latitude in degrees.
	pub latitude: Option<f64>,
	/// Altitude above the reference ellipsoid, in meters.
	pub altitude: Option<f64>,
	/// Longitude in degrees.
	pub longitude: Option<f64>,
}

impl Part for GeoPoint {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoAgeGating {
	/// Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present.
	pub restricted: Option<bool>,
	/// Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.
	pub alcohol_content: Option<bool>,
	/// Video game rating, if any.
	pub video_game_rating: Option<String>,
}

impl Part for VideoAgeGating {}


/// Player to be used for a video playback.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoPlayer {
	/// An <iframe> tag that embeds a player that will play the video.
	pub embed_html: Option<String>,
}

impl Part for VideoPlayer {}


/// Basic details about a channel, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSnippet {
	/// The date and time that the channel was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// A map of thumbnail images associated with the channel. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The channel's title.
	pub title: Option<String>,
	/// Localized title and description, read-only.
	pub localized: Option<ChannelLocalization>,
	/// The language of the channel's default title and description.
	pub default_language: Option<String>,
	/// The description of the channel.
	pub description: Option<String>,
}

impl Part for ChannelSnippet {}


/// Branding properties for the watch.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct WatchSettings {
	/// The background color for the video watch page's branded area.
	pub text_color: Option<String>,
	/// An ID that uniquely identifies a playlist that displays next to the video player.
	pub featured_playlist_id: Option<String>,
	/// The text color for the video watch page's branded area.
	pub background_color: Option<String>,
}

impl Part for WatchSettings {}


/// ChannelSection localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSectionLocalization {
	/// The localized strings for channel section's title.
	pub title: Option<String>,
}

impl Part for ChannelSectionLocalization {}


/// DEPRECATED Region restriction of the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoContentDetailsRegionRestriction {
	/// A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.
	pub allowed: Vec<String>,
	/// A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.
	pub blocked: Vec<String>,
}

impl Part for VideoContentDetailsRegionRestriction {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoRating {
	/// no description provided
	pub rating: Option<String>,
	/// no description provided
	pub video_id: Option<String>,
}

impl Part for VideoRating {}


/// Describes a single promoted item id. It is a union of various possible types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PromotedItemId {
	/// If the promoted item represents a website, this field represents the url pointing to the website. This field will be present only if type has the value website.
	pub website_url: Option<String>,
	/// If type is recentUpload, this field identifies the channel from which to take the recent upload. If missing, the channel is assumed to be the same channel for which the invideoPromotion is set.
	pub recently_uploaded_by: Option<String>,
	/// Describes the type of the promoted item.
	pub type_: Option<String>,
	/// If the promoted item represents a video, this field represents the unique YouTube ID identifying it. This field will be present only if type has the value video.
	pub video_id: Option<String>,
}

impl Part for PromotedItemId {}


/// A subscription resource contains information about a YouTube user subscription. A subscription notifies a user when new videos are added to a channel or when another user takes one of several actions on YouTube, such as uploading a video, rating a video, or commenting on a video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert](struct.SubscriptionInsertMethodBuilder.html) (request|response)
/// * [list](struct.SubscriptionListMethodBuilder.html) (none)
/// * [delete](struct.SubscriptionDeleteMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Subscription {
	/// The snippet object contains basic details about the subscription, including its title and the channel that the user subscribed to.
	pub snippet: Option<SubscriptionSnippet>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#subscription".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The contentDetails object contains basic statistics about the subscription.
	pub content_details: Option<SubscriptionContentDetails>,
	/// The subscriberSnippet object contains basic details about the sbuscriber.
	pub subscriber_snippet: Option<SubscriptionSubscriberSnippet>,
	/// The ID that YouTube uses to uniquely identify the subscription.
	pub id: Option<String>,
}

impl RequestValue for Subscription {}
impl Resource for Subscription {}
impl ResponseResult for Subscription {}

impl Subscription {
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
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nRegionSnippet {
	/// The region code as a 2-letter ISO country code.
	pub gl: Option<String>,
	/// The human-readable name of the region.
	pub name: Option<String>,
}

impl Part for I18nRegionSnippet {}


/// Information about a new playlist item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsPlaylistItem {
	/// The resourceId object contains information about the resource that was added to the playlist.
	pub resource_id: Option<ResourceId>,
	/// The value that YouTube uses to uniquely identify the playlist.
	pub playlist_id: Option<String>,
	/// ID of the item within the playlist.
	pub playlist_item_id: Option<String>,
}

impl Part for ActivityContentDetailsPlaylistItem {}


/// Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoPosition {
	/// Describes in which corner of the video the visual widget will appear.
	pub corner_position: Option<String>,
	/// Defines the position type.
	pub type_: Option<String>,
}

impl Part for InvideoPosition {}


/// Information about a resource that received a comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsComment {
	/// The resourceId object contains information that identifies the resource associated with the comment.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsComment {}


/// Basic details about a guide category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GuideCategorySnippet {
	/// no description provided
	pub channel_id: Option<String>,
	/// Description of the guide category.
	pub title: Option<String>,
}

impl Part for GuideCategorySnippet {}


/// Basic details about a video, including title, description, uploader, thumbnails and category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoSnippet {
	/// The video's description.
	pub description: Option<String>,
	/// A list of keyword tags associated with the video. Tags may contain spaces. This field is only visible to the video's uploader.
	pub tags: Vec<String>,
	/// The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.
	pub channel_id: Option<String>,
	/// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
	pub live_broadcast_content: Option<String>,
	/// The language of the videos's default snippet.
	pub default_language: Option<String>,
	/// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The video's title.
	pub title: Option<String>,
	/// The YouTube video category associated with the video.
	pub category_id: Option<String>,
	/// Localized snippet selected with the hl parameter. If no such localization exists, this field is populated with the default snippet. (Read-only)
	pub localized: Option<VideoLocalization>,
	/// Channel title for the channel that the video belongs to.
	pub channel_title: Option<String>,
}

impl Part for VideoSnippet {}


/// Project specific details about the content of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoProjectDetails {
	/// A list of project tags associated with the video during the upload.
	pub tags: Vec<String>,
}

impl Part for VideoProjectDetails {}


/// Detailed settings of a stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamContentDetails {
	/// Indicates whether the stream is reusable, which means that it can be bound to multiple broadcasts. It is common for broadcasters to reuse the same stream for many different broadcasts if those broadcasts occur at different times.
/// 
/// If you set this value to false, then the stream will not be reusable, which means that it can only be bound to one broadcast. Non-reusable streams differ from reusable streams in the following ways:  
/// - A non-reusable stream can only be bound to one broadcast. 
/// - A non-reusable stream might be deleted by an automated process after the broadcast ends. 
/// - The  liveStreams.list method does not list non-reusable streams if you call the method and set the mine parameter to true. The only way to use that method to retrieve the resource for a non-reusable stream is to use the id parameter to identify the stream.
	pub is_reusable: Option<bool>,
	/// The ingestion URL where the closed captions of this stream are sent.
	pub closed_captions_ingestion_url: Option<String>,
}

impl Part for LiveStreamContentDetails {}


/// Detailed settings of a broadcast.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastContentDetails {
	/// This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast's status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint's eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers.
	pub start_with_slate: Option<bool>,
	/// This value uniquely identifies the live stream bound to the broadcast.
	pub bound_stream_id: Option<String>,
	/// This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video.
	pub enable_embed: Option<bool>,
	/// This setting indicates whether closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API.
	pub enable_closed_captions: Option<bool>,
	/// This setting indicates whether YouTube should enable content encryption for the broadcast.
	pub enable_content_encryption: Option<bool>,
	/// Automatically start recording after the event goes live. The default value for this property is true.
/// 
/// 
/// 
/// Important: You must also set the enableDvr property's value to true if you want the playback to be available immediately after the broadcast ends. If you set this property's value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback.
	pub record_from_start: Option<bool>,
	/// This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true.
/// 
/// 
/// 
/// Important: You must set the value to true and also set the enableArchive property's value to true if you want to make playback available immediately after the broadcast ends.
	pub enable_dvr: Option<bool>,
	/// The monitorStream object contains information about the monitor stream, which the broadcaster can use to review the event content before the broadcast stream is shown publicly.
	pub monitor_stream: Option<MonitorStreamInfo>,
}

impl Part for LiveBroadcastContentDetails {}


/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoStatus {
	/// The video's license.
	pub license: Option<String>,
	/// This value indicates if the video can be embedded on another website.
	pub embeddable: Option<bool>,
	/// The video's privacy status.
	pub privacy_status: Option<String>,
	/// The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publish_at: Option<String>,
	/// This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled.
	pub public_stats_viewable: Option<bool>,
	/// The status of the uploaded video.
	pub upload_status: Option<String>,
	/// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
	pub rejection_reason: Option<String>,
	/// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
	pub failure_reason: Option<String>,
}

impl Part for VideoStatus {}


/// A guideCategory resource identifies a category that YouTube algorithmically assigns based on a channel's content or other indicators, such as the channel's popularity. The list is similar to video categories, with the difference being that a video's uploader can assign a video category but only YouTube can assign a channel category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GuideCategory {
	/// The snippet object contains basic details about the category, such as its title.
	pub snippet: Option<GuideCategorySnippet>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#guideCategory".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The ID that YouTube uses to uniquely identify the guide category.
	pub id: Option<String>,
}

impl Part for GuideCategory {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.ChannelSectionListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSectionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// A list of ChannelSections that match the request criteria.
	pub items: Vec<ChannelSection>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSectionListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
}

impl ResponseResult for ChannelSectionListResponse {}


/// Settings and Info of the monitor stream
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct MonitorStreamInfo {
	/// If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay.
	pub broadcast_stream_delay_ms: Option<u32>,
	/// HTML code that embeds a player that plays the monitor stream.
	pub embed_html: Option<String>,
	/// This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster's consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints.
/// 
/// You need to set this value to true if you intend to have a broadcast delay for your event.
/// 
/// Note: This property cannot be updated once the broadcast is in the testing or live state.
	pub enable_monitor_stream: Option<bool>,
}

impl Part for MonitorStreamInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.I18nLanguageListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nLanguageListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource.
	pub items: Vec<I18nLanguage>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguageListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
}

impl ResponseResult for I18nLanguageListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LocalizedProperty {
	/// no description provided
	pub default: Option<String>,
	/// The language of the default property.
	pub default_language: Option<LanguageTag>,
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
/// * [control](struct.LiveBroadcastControlMethodBuilder.html) (response)
/// * [insert](struct.LiveBroadcastInsertMethodBuilder.html) (request|response)
/// * [list](struct.LiveBroadcastListMethodBuilder.html) (none)
/// * [transition](struct.LiveBroadcastTransitionMethodBuilder.html) (response)
/// * [update](struct.LiveBroadcastUpdateMethodBuilder.html) (request|response)
/// * [delete](struct.LiveBroadcastDeleteMethodBuilder.html) (none)
/// * [bind](struct.LiveBroadcastBindMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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
	pub content_details: Option<LiveBroadcastContentDetails>,
	/// The ID that YouTube assigns to uniquely identify the broadcast.
	pub id: Option<String>,
}

impl RequestValue for LiveBroadcast {}
impl Resource for LiveBroadcast {}
impl ResponseResult for LiveBroadcast {}

impl LiveBroadcast {
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
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoFileDetailsVideoStream {
	/// The video stream's bitrate, in bits per second.
	pub bitrate_bps: Option<String>,
	/// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
	pub vendor: Option<String>,
	/// The video codec that the stream uses.
	pub codec: Option<String>,
	/// The encoded video content's width in pixels. You can calculate the video's encoding aspect ratio as width_pixels / height_pixels.
	pub width_pixels: Option<u32>,
	/// The encoded video content's height in pixels.
	pub height_pixels: Option<u32>,
	/// The video content's display aspect ratio, which specifies the aspect ratio in which the video should be displayed.
	pub aspect_ratio: Option<f64>,
	/// The amount that YouTube needs to rotate the original source content to properly display the video.
	pub rotation: Option<String>,
	/// The video stream's frame rate, in frames per second.
	pub frame_rate_fps: Option<f64>,
}

impl Part for VideoFileDetailsVideoStream {}


/// A thumbnail is an image representing a YouTube resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set](struct.ThumbnailSetMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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
/// * [list](struct.ChannelListMethodBuilder.html) (none)
/// * [update](struct.ChannelUpdateMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Channel {
	/// The status object encapsulates information about the privacy status of the channel.
	pub status: Option<ChannelStatus>,
	/// The invideoPromotion object encapsulates information about promotion campaign associated with the channel.
	pub invideo_promotion: Option<InvideoPromotion>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channel".
	pub kind: Option<String>,
	/// The statistics object encapsulates statistics for the channel.
	pub statistics: Option<ChannelStatistics>,
	/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
	pub content_owner_details: Option<ChannelContentOwnerDetails>,
	/// The topicDetails object encapsulates information about Freebase topics associated with the channel.
	pub topic_details: Option<ChannelTopicDetails>,
	/// The contentDetails object encapsulates information about the channel's content.
	pub content_details: Option<ChannelContentDetails>,
	/// The brandingSettings object encapsulates information about the branding of the channel.
	pub branding_settings: Option<ChannelBrandingSettings>,
	/// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
	pub conversion_pings: Option<ChannelConversionPings>,
	/// The snippet object contains basic details about the channel, such as its title, description, and thumbnail images.
	pub snippet: Option<ChannelSnippet>,
	/// The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process.
	pub audit_details: Option<ChannelAuditDetails>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The ID that YouTube uses to uniquely identify the channel.
	pub id: Option<String>,
	/// Localizations for different languages
	pub localizations: HashMap<String, ChannelLocalization>,
}

impl RequestValue for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}

impl Channel {
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
		if self.localizations.len() > 0 { r = r + "localizations,"; }
		r.pop();
		r
	}
}

/// Statistics about a channel: number of subscribers, number of videos in the channel, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelStatistics {
	/// The number of comments for the channel.
	pub comment_count: Option<i64>,
	/// The number of subscribers that the channel has.
	pub subscriber_count: Option<i64>,
	/// The number of videos uploaded to the channel.
	pub video_count: Option<i64>,
	/// Whether or not the number of subscribers is shown for this user.
	pub hidden_subscriber_count: Option<bool>,
	/// The number of times the channel has been viewed.
	pub view_count: Option<i64>,
}

impl Part for ChannelStatistics {}


/// Details about a social network post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsSocial {
	/// The resourceId object encapsulates information that identifies the resource associated with a social network post.
	pub resource_id: Option<ResourceId>,
	/// An image of the post's author.
	pub image_url: Option<String>,
	/// The name of the social network.
	pub type_: Option<String>,
	/// The URL of the social network post.
	pub reference_url: Option<String>,
	/// The author of the social network post.
	pub author: Option<String>,
}

impl Part for ActivityContentDetailsSocial {}


/// Channel localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelLocalization {
	/// The localized strings for channel's description.
	pub description: Option<String>,
	/// The localized strings for channel's title, read-only.
	pub title: Option<String>,
}

impl Part for ChannelLocalization {}


/// A resource id is a generic reference that points to another YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ResourceId {
	/// The type of the API resource.
	pub kind: Option<String>,
	/// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
	pub channel_id: Option<String>,
	/// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
	pub playlist_id: Option<String>,
	/// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.
	pub video_id: Option<String>,
}

impl Part for ResourceId {}


/// A search result contains information about a YouTube video, channel, or playlist that matches the search parameters specified in an API request. While a search result points to a uniquely identifiable resource, like a video, it does not have its own persistent data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SearchResult {
	/// The snippet object contains basic details about a search result, such as its title or description. For example, if the search result is a video, then the title will be the video's title and the description will be the video's description.
	pub snippet: Option<SearchResultSnippet>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#searchResult".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The id object contains information that can be used to uniquely identify the resource that matches the search request.
	pub id: Option<ResourceId>,
}

impl Part for SearchResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.VideoCategoryListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoCategoryListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource.
	pub items: Vec<VideoCategory>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for VideoCategoryListResponse {}


/// Basic details about an activity, including title, description, thumbnails, activity type and group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivitySnippet {
	/// A map of thumbnail images associated with the resource that is primarily associated with the activity. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The title of the resource primarily associated with the activity.
	pub title: Option<String>,
	/// The ID that YouTube uses to uniquely identify the channel associated with the activity.
	pub channel_id: Option<String>,
	/// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// Channel title for the channel responsible for this activity
	pub channel_title: Option<String>,
	/// The type of activity that the resource describes.
	pub type_: Option<String>,
	/// The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user's activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value.
	pub group_id: Option<String>,
	/// The description of the resource primarily associated with the activity.
	pub description: Option<String>,
}

impl Part for ActivitySnippet {}


/// Video processing progress and completion time estimate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoProcessingDetailsProcessingProgress {
	/// An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.
	pub time_left_ms: Option<String>,
	/// The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating:
/// 100 * parts_processed / parts_total
/// 
/// Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.
	pub parts_processed: Option<String>,
	/// An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.
	pub parts_total: Option<String>,
}

impl Part for VideoProcessingDetailsProcessingProgress {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.SearchListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SearchListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#searchListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of results that match the search criteria.
	pub items: Vec<SearchResult>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for SearchListResponse {}


/// Freebase topic information related to the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelTopicDetails {
	/// A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.
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
/// * [list](struct.VideoListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of videos that match the request criteria.
	pub items: Vec<Video>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for VideoListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LanguageTag {
	/// no description provided
	pub value: Option<String>,
}

impl Part for LanguageTag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistStatus {
	/// The playlist's privacy status.
	pub privacy_status: Option<String>,
}

impl Part for PlaylistStatus {}


/// Details about the content of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoContentDetails {
	/// The value of definition indicates whether the video is available in high definition or only in standard definition.
	pub definition: Option<String>,
	/// The countryRestriction object contains information about the countries where a video is (or is not) viewable.
	pub country_restriction: Option<AccessPolicy>,
	/// Specifies the ratings that the video received under various rating schemes.
	pub content_rating: Option<ContentRating>,
	/// The value of captions indicates whether the video has captions or not.
	pub caption: Option<String>,
	/// The regionRestriction object contains information about the countries where a video is (or is not) viewable. The object will contain either the contentDetails.regionRestriction.allowed property or the contentDetails.regionRestriction.blocked property.
	pub region_restriction: Option<VideoContentDetailsRegionRestriction>,
	/// The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.
	pub duration: Option<String>,
	/// The value of is_license_content indicates whether the video is licensed content.
	pub licensed_content: Option<bool>,
	/// The value of dimension indicates whether the video is available in 3D or in 2D.
	pub dimension: Option<String>,
}

impl Part for VideoContentDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamSnippet {
	/// The ID that YouTube uses to uniquely identify the channel that is transmitting the stream.
	pub channel_id: Option<String>,
	/// The stream's description. The value cannot be longer than 10000 characters.
	pub description: Option<String>,
	/// The date and time that the stream was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// The stream's title. The value must be between 1 and 128 characters long.
	pub title: Option<String>,
}

impl Part for LiveStreamSnippet {}


/// JSON template for the status part of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelStatus {
	/// Privacy status of the channel.
	pub privacy_status: Option<String>,
	/// If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity.
	pub is_linked: Option<bool>,
	/// The long uploads status of this channel. See
	pub long_uploads_status: Option<String>,
}

impl Part for ChannelStatus {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.ChannelListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channelListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of channels that match the request criteria.
	pub items: Vec<Channel>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for ChannelListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete](struct.ChannelSectionDeleteMethodBuilder.html) (none)
/// * [update](struct.ChannelSectionUpdateMethodBuilder.html) (request|response)
/// * [list](struct.ChannelSectionListMethodBuilder.html) (none)
/// * [insert](struct.ChannelSectionInsertMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSection {
	/// The snippet object contains basic details about the channel section, such as its type, style and title.
	pub snippet: Option<ChannelSectionSnippet>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The contentDetails object contains details about the channel section content, such as a list of playlists or channels featured in the section.
	pub content_details: Option<ChannelSectionContentDetails>,
	/// The ID that YouTube uses to uniquely identify the channel section.
	pub id: Option<String>,
	/// Localizations for different languages
	pub localizations: HashMap<String, ChannelSectionLocalization>,
}

impl RequestValue for ChannelSection {}
impl Resource for ChannelSection {}
impl ResponseResult for ChannelSection {}

impl ChannelSection {
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
		if self.localizations.len() > 0 { r = r + "localizations,"; }
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
/// * [list](struct.LiveBroadcastListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcastListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of broadcasts that match the request criteria.
	pub items: Vec<LiveBroadcast>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for LiveBroadcastListResponse {}


/// Brief description of the live stream status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamStatus {
	/// no description provided
	pub stream_status: Option<String>,
}

impl Part for LiveStreamStatus {}


/// Details about the live streaming metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoLiveStreamingDetails {
	/// The time that the broadcast is scheduled to begin. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduled_start_time: Option<String>,
	/// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
	pub concurrent_viewers: Option<String>,
	/// The time that the broadcast actually started. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast begins.
	pub actual_start_time: Option<String>,
	/// The time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.
	pub scheduled_end_time: Option<String>,
	/// The time that the broadcast actually ended. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast is over.
	pub actual_end_time: Option<String>,
}

impl Part for VideoLiveStreamingDetails {}


/// Ratings schemes. The country-specific ratings are mostly for movies and shows. NEXT_ID: 65
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ContentRating {
	/// Internal YouTube rating.
	pub yt_rating: Option<String>,
	/// Rating system for French Canadian TV - Regie du cinema
	pub catvfr_rating: Option<String>,
	/// Rating system in India - Central Board of Film Certification
	pub cbfc_rating: Option<String>,
	/// Rating system for Thailand - Board of Filmand Video Censors
	pub bfvc_rating: Option<String>,
	/// Rating system for Austria - Bundesministeriums f�r Unterricht, Kunst und Kultur!
	pub bmukk_rating: Option<String>,
	/// Rating system for Switzerland - Switzerland Rating System
	pub chfilm_rating: Option<String>,
	/// Rating system for Taiwan - Ministry of Culture - Tawan
	pub moctw_rating: Option<String>,
	/// Rating system for Canadian TV - Canadian TV Classification System
	pub catv_rating: Option<String>,
	/// Rating system for Peru - Peru Rating System
	pub pefilm_rating: Option<String>,
	/// no description provided
	pub djctq_rating_reasons: Vec<String>,
	/// Rating system for Argentina - Instituto Nacional de Cine y Artes Audiovisuales
	pub incaa_rating: Option<String>,
	/// Rating system for Israel - Israel Rating System
	pub ilfilm_rating: Option<String>,
	/// Rating system for Luxembourg - Commission de surveillance de la classification des films
	pub cscf_rating: Option<String>,
	/// Rating system in Germany - Voluntary Self Regulation of the Movie Industry
	pub fsk_rating: Option<String>,
	/// Rating system in South Korea - Korea Media Rating Board
	pub kmrb_rating: Option<String>,
	/// Rating system in Brazil - Department of Justice, Rating, Titles and Qualification
	pub djctq_rating: Option<String>,
	/// Rating system for Indonesia - Lembaga Sensor Film
	pub lsf_rating: Option<String>,
	/// Rating system for Hong kong - Office for Film, Newspaper and Article Administration
	pub fco_rating: Option<String>,
	/// Rating system for Norway - Medietilsynet
	pub medietilsynet_rating: Option<String>,
	/// Rating system for Greece - Greece Rating System
	pub grfilm_rating: Option<String>,
	/// Rating system for Chile - Consejo de Calificaci�n Cinematogr�fica
	pub ccc_rating: Option<String>,
	/// Rating system for Ireland - Raidi� Teilif�s �ireann
	pub rte_rating: Option<String>,
	/// Rating system in France - French Minister of Culture
	pub fmoc_rating: Option<String>,
	/// Rating system for Sweden - Statens medier�d (National Media Council)
	pub smsa_rating: Option<String>,
	/// Rating system for Portugal - Comiss�o de Classifica��o de Espect�culos
	pub cce_rating: Option<String>,
	/// Rating system for Latvia - National Film Center of Latvia
	pub nkclv_rating: Option<String>,
	/// Rating system for South africa - Film & Publication Board
	pub fpb_rating: Option<String>,
	/// Rating system for Iceland - SMAIS
	pub smais_rating: Option<String>,
	/// Canadian Home Video Rating System
	pub chvrs_rating: Option<String>,
	/// Rating system for Italy - Autorit� per le Garanzie nelle Comunicazioni
	pub agcom_rating: Option<String>,
	/// Rating system for Colombia - MoC
	pub moc_rating: Option<String>,
	/// Rating system for Hungary - Rating Committee of the National Office of Film
	pub rcnof_rating: Option<String>,
	/// Rating system for Malaysia - Film Censorship Board of Malaysia
	pub fcbm_rating: Option<String>,
	/// Rating system for Netherlands - Nederlands Instituut voor de Classificatie van Audiovisuele Media
	pub kijkwijzer_rating: Option<String>,
	/// Rating system for Singapore - Media Development Authority
	pub mda_rating: Option<String>,
	/// Rating system for Nigeria - National Film and Video Censors Board
	pub nfvcb_rating: Option<String>,
	/// Rating system for Venezuela - SiBCI
	pub resorteviolencia_rating: Option<String>,
	/// Rating system for France - Conseil sup�rieur de l?audiovisuel
	pub csa_rating: Option<String>,
	/// Rating system in New Zealand - Office of Film and Literature Classification
	pub oflc_rating: Option<String>,
	/// TV Parental Guidelines rating of the content.
	pub tvpg_rating: Option<String>,
	/// Rating system for Bulgaria - National Film Centre
	pub nfrc_rating: Option<String>,
	/// Rating system for Malta - Film Age-Classification Board
	pub mccaa_rating: Option<String>,
	/// Rating system in Mexico - General Directorate of Radio, Television and Cinematography
	pub rtc_rating: Option<String>,
	/// Rating system in Italy - Ministero dei Beni e delle Attivita Culturali e del Turismo
	pub mibac_rating: Option<String>,
	/// British Board of Film Classification
	pub bbfc_rating: Option<String>,
	/// Rating system for Egypt - Egypt Rating System
	pub egfilm_rating: Option<String>,
	/// Rating system for Belgium - Belgium Rating System
	pub cicf_rating: Option<String>,
	/// Rating system for Poland - National Broadcasting Council
	pub nbcpl_rating: Option<String>,
	/// Rating system for Maldives - National Bureau of Classification
	pub nbc_rating: Option<String>,
	/// Motion Picture Association of America rating for the content.
	pub mpaa_rating: Option<String>,
	/// Rating system in Ireland - Irish Film Classification Office
	pub ifco_rating: Option<String>,
	/// Rating system in Australia - Australian Classification Board
	pub acb_rating: Option<String>,
	/// Rating system for Estonia - Estonia Rating System
	pub eefilm_rating: Option<String>,
	/// Rating system for Czech republic - Czech republic Rating System
	pub czfilm_rating: Option<String>,
	/// Rating system for Kenya - Kenya Film Classification Board
	pub kfcb_rating: Option<String>,
	/// Rating system in Russia
	pub russia_rating: Option<String>,
	/// Rating system for Philippines - MOVIE AND TELEVISION REVIEW AND CLASSIFICATION BOARD
	pub mtrcb_rating: Option<String>,
	/// Rating system for Chile - Asociaci�n Nacional de Televisi�n
	pub anatel_rating: Option<String>,
	/// Rating system in Japan - Eiga Rinri Kanri Iinkai
	pub eirin_rating: Option<String>,
	/// Rating system for Romania - CONSILIUL NATIONAL AL AUDIOVIZUALULUI - CNA
	pub cna_rating: Option<String>,
	/// Rating system in Spain - Instituto de Cinematografia y de las Artes Audiovisuales
	pub icaa_rating: Option<String>,
	/// Rating system for Denmark - The Media Council for Children and Young People
	pub mccyp_rating: Option<String>,
	/// Rating system for Slovakia - Slovakia Rating System
	pub skfilm_rating: Option<String>,
	/// Rating system for Finland - Finnish Centre for Media Education and Audiovisual Media
	pub meku_rating: Option<String>,
}

impl Part for ContentRating {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.ActivityListMethodBuilder.html) (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityListResponse {
	/// Serialized EventId of the request which produced this response.
	pub event_id: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub next_page_token: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#activityListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitor_id: Option<String>,
	/// A list of activities, or events, that match the request criteria.
	pub items: Vec<Activity>,
	/// no description provided
	pub token_pagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prev_page_token: Option<String>,
	/// no description provided
	pub page_info: Option<PageInfo>,
}

impl ResponseResult for ActivityListResponse {}


/// An activity resource contains information about an action that a particular channel, or user, has taken on YouTube.The actions reported in activity feeds include rating a video, sharing a video, marking a video as a favorite, commenting on a video, uploading a video, and so forth. Each activity resource identifies the type of action, the channel associated with the action, and the resource(s) associated with the action, such as the video that was rated or uploaded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert](struct.ActivityInsertMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Activity {
	/// The snippet object contains basic details about the activity, including the activity's type and group ID.
	pub snippet: Option<ActivitySnippet>,
	/// The contentDetails object contains information about the content associated with the activity. For example, if the snippet.type value is videoRated, then the contentDetails object's content identifies the rated video.
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

impl Activity {
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
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionSubscriberSnippet {
	/// The channel ID of the subscriber.
	pub channel_id: Option<String>,
	/// The description of the subscriber.
	pub description: Option<String>,
	/// Thumbnails for this subscriber.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The title of the subscriber.
	pub title: Option<String>,
}

impl Part for SubscriptionSubscriberSnippet {}


/// Branding properties for images associated with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ImageSettings {
	/// Banner image. TV size medium resolution (1280x720).
	pub banner_tv_medium_image_url: Option<String>,
	/// Banner image. Tablet size low resolution (1138x188).
	pub banner_tablet_low_image_url: Option<String>,
	/// The image map script for the large banner image.
	pub large_branded_banner_image_imap_script: Option<LocalizedProperty>,
	/// Banner image. Mobile size (640x175).
	pub banner_mobile_image_url: Option<String>,
	/// The URL for the 640px by 70px banner image that appears below the video player in the default view of the video watch page.
	pub small_branded_banner_image_url: Option<LocalizedProperty>,
	/// Banner image. Tablet size high resolution (2276x377).
	pub banner_tablet_hd_image_url: Option<String>,
	/// Banner image. TV size high resolution (1920x1080).
	pub banner_tv_high_image_url: Option<String>,
	/// Banner image. Mobile size medium/high resolution (960x263).
	pub banner_mobile_medium_hd_image_url: Option<String>,
	/// The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.
	pub tracking_image_url: Option<String>,
	/// Banner image. Mobile size high resolution (1440x395).
	pub banner_mobile_extra_hd_image_url: Option<String>,
	/// Banner image. Tablet size (1707x283).
	pub banner_tablet_image_url: Option<String>,
	/// Banner image. Mobile size low resolution (320x88).
	pub banner_mobile_low_image_url: Option<String>,
	/// Banner image. TV size extra high resolution (2120x1192).
	pub banner_tv_image_url: Option<String>,
	/// Banner image. TV size low resolution (854x480).
	pub banner_tv_low_image_url: Option<String>,
	/// Banner image. Tablet size extra high resolution (2560x424).
	pub banner_tablet_extra_hd_image_url: Option<String>,
	/// The URL for the 854px by 70px image that appears below the video player in the expanded video view of the video watch page.
	pub large_branded_banner_image_url: Option<LocalizedProperty>,
	/// Banner image. Desktop size (1060x175).
	pub banner_image_url: Option<String>,
	/// The URL for the background image shown on the video watch page. The image should be 1200px by 615px, with a maximum file size of 128k.
	pub background_image_url: Option<LocalizedProperty>,
	/// The image map script for the small banner image.
	pub small_branded_banner_image_imap_script: Option<LocalizedProperty>,
	/// Banner image. Mobile size high resolution (1280x360).
	pub banner_mobile_hd_image_url: Option<String>,
	/// This is used only in update requests; if it's set, we use this URL to generate all of the above banner URLs.
	pub banner_external_url: Option<String>,
	/// The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels.
	pub watch_icon_image_url: Option<String>,
}

impl Part for ImageSettings {}


/// Details about a resource which is being promoted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsPromotedItem {
	/// The type of call-to-action, a message to the user indicating action that can be taken.
	pub cta_type: Option<String>,
	/// The URL the client should fetch to request a promoted item.
	pub ad_tag: Option<String>,
	/// The URL the client should direct the user to, if the user chooses to visit the advertiser's website.
	pub destination_url: Option<String>,
	/// The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown.
	pub forecasting_url: Vec<String>,
	/// The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item.
	pub impression_url: Vec<String>,
	/// The URL the client should ping to indicate that the user was shown this promoted item.
	pub creative_view_url: Option<String>,
	/// The ID that YouTube uses to uniquely identify the promoted video.
	pub video_id: Option<String>,
	/// The text description to accompany the promoted item.
	pub description_text: Option<String>,
	/// The custom call-to-action button text. If specified, it will override the default button text for the cta_type.
	pub custom_cta_button_text: Option<String>,
	/// The URL the client should ping to indicate that the user clicked through on this promoted item.
	pub click_tracking_url: Option<String>,
}

impl Part for ActivityContentDetailsPromotedItem {}


/// Rights management policy for YouTube resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct AccessPolicy {
	/// A list of region codes that identify countries where the default policy do not apply.
	pub exception: Vec<String>,
	/// The value of allowed indicates whether the access to the policy is allowed or denied by default.
	pub allowed: Option<bool>,
}

impl Part for AccessPolicy {}


/// Details about a channel bulletin post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsBulletin {
	/// The resourceId object contains information that identifies the resource associated with a bulletin post.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsBulletin {}


/// An i18nLanguage resource identifies a UI language currently supported by YouTube.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list](struct.I18nLanguageListMethodBuilder.html) (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LocalizedString {
	/// no description provided
	pub language: Option<String>,
	/// no description provided
	pub value: Option<String>,
}

impl Part for LocalizedString {}


/// Information about an audio stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoFileDetailsAudioStream {
	/// The audio stream's bitrate, in bits per second.
	pub bitrate_bps: Option<String>,
	/// The audio codec that the stream uses.
	pub codec: Option<String>,
	/// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
	pub vendor: Option<String>,
	/// The number of audio channels that the stream contains.
	pub channel_count: Option<u32>,
}

impl Part for VideoFileDetailsAudioStream {}


/// Freebase topic information related to the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoTopicDetails {
	/// A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.
	pub topic_ids: Vec<String>,
	/// Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.
	pub relevant_topic_ids: Vec<String>,
}

impl Part for VideoTopicDetails {}


/// Describes information necessary for ingesting an RTMP or an HTTP stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct IngestionInfo {
	/// The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL.
	pub backup_ingestion_address: Option<String>,
	/// The HTTP or RTMP stream name that YouTube assigns to the video stream.
	pub stream_name: Option<String>,
	/// The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL.
/// 
/// Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format:
/// 
/// STREAM_URL/STREAM_NAME
	pub ingestion_address: Option<String>,
}

impl Part for IngestionInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoConversionPing {
	/// Defines the context of the ping.
	pub context: Option<String>,
	/// The url (without the schema) that the app shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D like%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA%3Bytvid%3DUrIaJUvIQDg&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping.
	pub conversion_url: Option<String>,
}

impl Part for VideoConversionPing {}


/// A videoCategory resource identifies a category that has been or could be associated with uploaded videos.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoCategory {
	/// The snippet object contains basic details about the video category, including its title.
	pub snippet: Option<VideoCategorySnippet>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategory".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The ID that YouTube uses to uniquely identify the video category.
	pub id: Option<String>,
}

impl Part for VideoCategory {}


/// Basic details about a playlist, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemSnippet {
	/// The ID that YouTube uses to uniquely identify the user that added the item to the playlist.
	pub channel_id: Option<String>,
	/// The item's description.
	pub description: Option<String>,
	/// The item's title.
	pub title: Option<String>,
	/// The id object contains information that can be used to uniquely identify the resource that is included in the playlist as the playlist item.
	pub resource_id: Option<ResourceId>,
	/// The ID that YouTube uses to uniquely identify the playlist that the playlist item is in.
	pub playlist_id: Option<String>,
	/// The date and time that the item was added to the playlist. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// Channel title for the channel that the playlist item belongs to.
	pub channel_title: Option<String>,
	/// The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.
	pub position: Option<u32>,
	/// A map of thumbnail images associated with the playlist item. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
}

impl Part for PlaylistItemSnippet {}


/// Information about a video that was marked as a favorite video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsFavorite {
	/// The resourceId object contains information that identifies the resource that was marked as a favorite.
	pub resource_id: Option<ResourceId>,
}

impl Part for ActivityContentDetailsFavorite {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistPlayer {
	/// An <iframe> tag that embeds a player that will play the playlist.
	pub embed_html: Option<String>,
}

impl Part for PlaylistPlayer {}


/// A single tag suggestion with it's relevance information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoSuggestionsTagSuggestion {
	/// A set of video categories for which the tag is relevant. You can use this information to display appropriate tag suggestions based on the video category that the video uploader associates with the video. By default, tag suggestions are relevant for all categories if there are no restricts defined for the keyword.
	pub category_restricts: Vec<String>,
	/// The keyword tag suggested for the video.
	pub tag: Option<String>,
}

impl Part for VideoSuggestionsTagSuggestion {}


/// Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoSuggestions {
	/// A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.
	pub processing_errors: Vec<String>,
	/// A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.
	pub tag_suggestions: Vec<VideoSuggestionsTagSuggestion>,
	/// A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
	pub editor_suggestions: Vec<String>,
	/// A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
	pub processing_warnings: Vec<String>,
	/// A list of suggestions that may improve YouTube's ability to process the video.
	pub processing_hints: Vec<String>,
}

impl Part for VideoSuggestions {}


/// Basic details about a search result, including title, description and thumbnails of the item referenced by the search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SearchResultSnippet {
	/// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
	pub live_broadcast_content: Option<String>,
	/// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The title of the search result.
	pub title: Option<String>,
	/// A description of the search result.
	pub description: Option<String>,
	/// The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies.
	pub channel_id: Option<String>,
	/// The creation date and time of the resource that the search result identifies. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub published_at: Option<String>,
	/// The title of the channel that published the resource that the search result identifies.
	pub channel_title: Option<String>,
}

impl Part for SearchResultSnippet {}


/// A channel banner returned as the response to a channel_banner.insert call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert](struct.ChannelBannerInsertMethodBuilder.html) (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
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

impl ChannelBannerResource {
	/// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
	/// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
	/// the parts you want to see in the server response.
	fn to_parts(&self) -> String {
		let mut r = String::new();
		if self.url.is_some() { r = r + "url,"; }
		if self.kind.is_some() { r = r + "kind,"; }
		if self.etag.is_some() { r = r + "etag,"; }
		r.pop();
		r
	}
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistContentDetails {
	/// The number of videos in the playlist.
	pub item_count: Option<u32>,
}

impl Part for PlaylistContentDetails {}


/// Paging details for lists of resources, including total number of items available and number of resources returned in a single page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PageInfo {
	/// The number of results included in the API response.
	pub results_per_page: Option<i32>,
	/// The total number of results in the result set.
	pub total_results: Option<i32>,
}

impl Part for PageInfo {}



// ###################
// NESTED SCHEMAS ###
// #################
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelContentDetailsRelatedPlaylists {
	/// The ID of the playlist that contains the channel"s uploaded videos. Use the  videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos.
	pub uploads: Option<String>,
	/// The ID of the playlist that contains the channel"s watch history. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub watch_history: Option<String>,
	/// The ID of the playlist that contains the channel"s liked videos. Use the   playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub likes: Option<String>,
	/// The ID of the playlist that contains the channel"s favorite videos. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub favorites: Option<String>,
	/// The ID of the playlist that contains the channel"s watch later playlist. Use the playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub watch_later: Option<String>,
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_languages();
/// # }
/// ```
pub struct I18nLanguageMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for I18nLanguageMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nLanguageMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of supported languages.    
    pub fn list<>(&self, part: &str) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
        I18nLanguageListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.channel_banners();
/// # }
/// ```
pub struct ChannelBannerMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ChannelBannerMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelBannerMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a channel banner image to YouTube. This method represents the first two steps in a three-step process to update the banner image for a channel:
    /// 
    /// - Call the channelBanners.insert method to upload the binary image data to YouTube. The image must have a 16:9 aspect ratio and be at least 2120x1192 pixels.
    /// - Extract the url property's value from the response that the API returns for step 1.
    /// - Call the channels.update method to update the channel's branding settings. Set the brandingSettings.image.bannerExternalUrl property's value to the URL obtained in step 2.
    pub fn insert<>(&self, request: &ChannelBannerResource) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
        ChannelBannerInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channel_sections();
/// # }
/// ```
pub struct ChannelSectionMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ChannelSectionMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns channelSection resources that match the API request criteria.    
    pub fn list<>(&self, part: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        ChannelSectionListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _id: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a channelSection for the authenticated user's channel.    
    pub fn insert<>(&self, request: &ChannelSection) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        ChannelSectionInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a channelSection.    
    pub fn delete<>(&self, id: &str) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
        ChannelSectionDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a channelSection.    
    pub fn update<>(&self, request: &ChannelSection) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
        ChannelSectionUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.guide_categories();
/// # }
/// ```
pub struct GuideCategoryMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for GuideCategoryMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> GuideCategoryMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of categories that can be associated with YouTube channels.    
    pub fn list<>(&self, part: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        GuideCategoryListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlists();
/// # }
/// ```
pub struct PlaylistMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for PlaylistMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a playlist.    
    pub fn insert<>(&self, request: &Playlist) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
        PlaylistInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs.    
    pub fn list<>(&self, part: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        PlaylistListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a playlist.    
    pub fn delete<>(&self, id: &str) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
        PlaylistDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a playlist. For example, you could change a playlist's title, description, or privacy status.    
    pub fn update<>(&self, request: &Playlist) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
        PlaylistUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `set(...)`
/// // to build up your call.
/// let rb = hub.thumbnails();
/// # }
/// ```
pub struct ThumbnailMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ThumbnailMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ThumbnailMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a custom video thumbnail to YouTube and sets it for a video.    
    pub fn set<>(&self, video_id: &str) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
        ThumbnailSetMethodBuilder {
            hub: self.hub,
            _video_id: video_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `get_rating(...)`, `insert(...)`, `list(...)`, `rate(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.videos();
/// # }
/// ```
pub struct VideoMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for VideoMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of videos that match the API request parameters.    
    pub fn list<>(&self, part: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        VideoListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Add a like or dislike rating to a video or remove a rating from a video.    
    pub fn rate<>(&self, id: &str, rating: &str) -> VideoRateMethodBuilder<'a, C, NC, A> {
        VideoRateMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _rating: rating.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the ratings that the authorized user gave to a list of specified videos.    
    pub fn get_rating<>(&self, id: &str) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
        VideoGetRatingMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a YouTube video.    
    pub fn delete<>(&self, id: &str) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
        VideoDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a video's metadata.    
    pub fn update<>(&self, request: &Video) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
        VideoUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a video to YouTube and optionally sets the video's metadata.    
    pub fn insert<>(&self, request: &Video) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        VideoInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _stabilize: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _notify_subscribers: Default::default(),
            _auto_levels: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.subscriptions();
/// # }
/// ```
pub struct SubscriptionMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for SubscriptionMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a subscription for the authenticated user's channel.    
    pub fn insert<>(&self, request: &Subscription) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
        SubscriptionInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns subscription resources that match the API request criteria.    
    pub fn list<>(&self, part: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        SubscriptionListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a subscription.    
    pub fn delete<>(&self, id: &str) -> SubscriptionDeleteMethodBuilder<'a, C, NC, A> {
        SubscriptionDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.search();
/// # }
/// ```
pub struct SearchMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for SearchMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SearchMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.    
    pub fn list<>(&self, part: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        SearchListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_regions();
/// # }
/// ```
pub struct I18nRegionMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for I18nRegionMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nRegionMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of supported regions.    
    pub fn list<>(&self, part: &str) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
        I18nRegionListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_streams();
/// # }
/// ```
pub struct LiveStreamMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for LiveStreamMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.    
    pub fn update<>(&self, request: &LiveStream) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
        LiveStreamUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a video stream.    
    pub fn delete<>(&self, id: &str) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
        LiveStreamDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of video streams that match the API request parameters.    
    pub fn list<>(&self, part: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        LiveStreamListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience.    
    pub fn insert<>(&self, request: &LiveStream) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
        LiveStreamInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ChannelMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a channel's metadata.    
    pub fn update<>(&self, request: &Channel) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
        ChannelUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of zero or more channel resources that match the request criteria.    
    pub fn list<>(&self, part: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        ChannelListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlist_items();
/// # }
/// ```
pub struct PlaylistItemMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for PlaylistItemMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a playlist item.    
    pub fn delete<>(&self, id: &str) -> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {
        PlaylistItemDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs.    
    pub fn list<>(&self, part: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        PlaylistItemListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _video_id: Default::default(),
            _playlist_id: Default::default(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a resource to a playlist.    
    pub fn insert<>(&self, request: &PlaylistItem) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
        PlaylistItemInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies a playlist item. For example, you could update the item's position in the playlist.    
    pub fn update<>(&self, request: &PlaylistItem) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
        PlaylistItemUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `set(...)` and `unset(...)`
/// // to build up your call.
/// let rb = hub.watermarks();
/// # }
/// ```
pub struct WatermarkMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for WatermarkMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a watermark image to YouTube and sets it for a channel.    
    pub fn set<>(&self, request: &InvideoBranding, channel_id: &str) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        WatermarkSetMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a watermark.    
    pub fn unset<>(&self, channel_id: &str) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
        WatermarkUnsetMethodBuilder {
            hub: self.hub,
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `bind(...)`, `control(...)`, `delete(...)`, `insert(...)`, `list(...)`, `transition(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_broadcasts();
/// # }
/// ```
pub struct LiveBroadcastMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for LiveBroadcastMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Controls the settings for a slate that can be displayed in the broadcast stream.    
    pub fn control<>(&self, id: &str, part: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        LiveBroadcastControlMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _part: part.to_string(),
            _walltime: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _offset_time_ms: Default::default(),
            _display_slate: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource's contentDetails object.    
    pub fn update<>(&self, request: &LiveBroadcast) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
        LiveBroadcastUpdateMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a broadcast.    
    pub fn insert<>(&self, request: &LiveBroadcast) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
        LiveBroadcastInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream.    
    pub fn bind<>(&self, id: &str, part: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        LiveBroadcastBindMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _part: part.to_string(),
            _stream_id: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of YouTube broadcasts that match the API request parameters.    
    pub fn list<>(&self, part: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        LiveBroadcastListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a broadcast.    
    pub fn delete<>(&self, id: &str) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
        LiveBroadcastDeleteMethodBuilder {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast's status to testing, YouTube starts to transmit video to that broadcast's monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active.    
    pub fn transition<>(&self, broadcast_status: &str, id: &str, part: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        LiveBroadcastTransitionMethodBuilder {
            hub: self.hub,
            _broadcast_status: broadcast_status.to_string(),
            _id: id.to_string(),
            _part: part.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.video_categories();
/// # }
/// ```
pub struct VideoCategoryMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for VideoCategoryMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoCategoryMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of categories that can be associated with YouTube videos.    
    pub fn list<>(&self, part: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        VideoCategoryListMethodBuilder {
            hub: self.hub,
            _part: part.to_string(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
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
/// extern crate "rustc-serialize" as rustc_serialize;
/// extern crate youtube3;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use youtube3::YouTube;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::new(),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = YouTube::new(hyper::Client::new(), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
/// // like `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethodsBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ActivityMethodsBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityMethodsBuilder<'a, C, NC, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user's subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.    
    pub fn list<>(&self, part: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        ActivityListMethodBuilder {
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
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel's behalf.)
    /// 
    /// Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API's videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.
    pub fn insert<>(&self, request: &Activity) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
        ActivityInsertMethodBuilder {
            hub: self.hub,
            _request: request.clone(),
            _part: request.to_parts(),
            _delegate: Default::default(),
            _scope: Default::default(),
            _additional_params: Default::default()
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns a list of supported languages.
///
/// A builder for the *list* method supported by a *i18nLanguage* resource.
/// It is not used directly, but through a `I18nLanguageMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .hl("eos")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct I18nLanguageListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for I18nLanguageListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nLanguageListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<I18nLanguageListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._hl.is_some() {
            params.push(("hl", self._hl.unwrap().to_string()));
        }
        for &field in ["part", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: I18nLanguageListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.    
    pub fn hl(mut self, new_value: &str) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> I18nLanguageListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
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
/// It is not used directly, but through a `ChannelBannerMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::ChannelBannerResource;
/// # use std::fs;
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
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channel_banners().insert(&req)
///              .on_behalf_of_content_owner("Stet")
///              .scope(&Default::default())
///              .upload_resumable(fs::File::open("file.ext").unwrap(), 282, "application/octet-stream".parse().unwrap());
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelBannerInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelBannerResource,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelBannerInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelBannerInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<R, RS>(mut self, stream: Option<(R, u64, mime::Mime)>, resumeable_stream: Option<(RS, u64, mime::Mime)>) -> Result<ChannelBannerResource> where R: Read, RS: ReadSeek {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = if stream.is_some() {
                "https://www.googleapis.com/upload/youtube/v3/channelBanners/insert".to_string()
            } else if resumeable_stream.is_some() {
                "https://www.googleapis.com/resumable/upload/youtube/v3/channelBanners/insert".to_string()
            } else { 
                unreachable!() 
        };

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ChannelBannerResource = Default::default();




        Result::Success(response)
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 6MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<R>(mut self, stream: R, size: u64, mime_type: mime::Mime) -> Result<ChannelBannerResource>
                where R: Read {
        self.doit(Some((stream, size, mime_type)), None::<(fs::File, u64, mime::Mime)>, )
    }
    /// Upload media in a resumeable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// TODO: Write more about how delegation works in this particular case.
    ///
    /// * *max size*: 6MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(mut self, resumeable_stream: RS, size: u64, mime_type: mime::Mime) -> Result<ChannelBannerResource>
                where RS: ReadSeek {
        self.doit(None::<(fs::File, u64, mime::Mime)>, Some((resumeable_stream, size, mime_type)), )
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &ChannelBannerResource) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelBannerInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns channelSection resources that match the API request criteria.
///
/// A builder for the *list* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("sed")
///              .mine(false)
///              .id("ipsum")
///              .channel_id("eos")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelSectionListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _id: Option<String>,
    _channel_id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelSectionListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<ChannelSectionListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._channel_id.is_some() {
            params.push(("channelId", self._channel_id.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner", "mine", "id", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ChannelSectionListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's channelSections.    
    pub fn mine(mut self, new_value: bool) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channelSection ID(s) for the resource(s) that are being retrieved. In a channelSection resource, the id property specifies the YouTube channelSection ID.    
    pub fn id(mut self, new_value: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a YouTube channel ID. The API will only return that channel's channelSections.    
    pub fn channel_id(mut self, new_value: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelSectionListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Adds a channelSection for the authenticated user's channel.
///
/// A builder for the *insert* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::ChannelSection;
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
///              .on_behalf_of_content_owner_channel("duo")
///              .on_behalf_of_content_owner("sadipscing")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelSectionInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelSection,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelSectionInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<ChannelSection> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ChannelSection = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &ChannelSection) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelSectionInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a channelSection.
///
/// A builder for the *delete* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("consetetur")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelSectionDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube channelSection ID for the resource that is being deleted. In a channelSection resource, the id property specifies the YouTube channelSection ID.    
    pub fn id(mut self, new_value: &str) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelSectionDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Update a channelSection.
///
/// A builder for the *update* method supported by a *channelSection* resource.
/// It is not used directly, but through a `ChannelSectionMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::ChannelSection;
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
///              .on_behalf_of_content_owner("ea")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelSectionUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: ChannelSection,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<ChannelSection> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ChannelSection = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &ChannelSection) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelSectionUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of categories that can be associated with YouTube channels.
///
/// A builder for the *list* method supported by a *guideCategory* resource.
/// It is not used directly, but through a `GuideCategoryMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .region_code("sanctus")
///              .id("invidunt")
///              .hl("et")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct GuideCategoryListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _region_code: Option<String>,
    _id: Option<String>,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for GuideCategoryListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> GuideCategoryListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<GuideCategoryListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._region_code.is_some() {
            params.push(("regionCode", self._region_code.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._hl.is_some() {
            params.push(("hl", self._hl.unwrap().to_string()));
        }
        for &field in ["part", "regionCode", "id", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: GuideCategoryListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return the list of guide categories available in the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.    
    pub fn region_code(mut self, new_value: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channel category ID(s) for the resource(s) that are being retrieved. In a guideCategory resource, the id property specifies the YouTube channel category ID.    
    pub fn id(mut self, new_value: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that will be used for text values in the API response.    
    pub fn hl(mut self, new_value: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> GuideCategoryListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Creates a playlist.
///
/// A builder for the *insert* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Playlist;
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
///              .on_behalf_of_content_owner_channel("sit")
///              .on_behalf_of_content_owner("takimata")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Playlist,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Playlist> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Playlist = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Playlist) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a collection of playlists that match the API request parameters. For example, you can retrieve all playlists that the authenticated user owns, or you can retrieve one or more playlists by their unique IDs.
///
/// A builder for the *list* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .page_token("consetetur")
///              .on_behalf_of_content_owner_channel("elitr")
///              .on_behalf_of_content_owner("sed")
///              .mine(true)
///              .max_results(60)
///              .id("clita")
///              .channel_id("sed")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<PlaylistListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._channel_id.is_some() {
            params.push(("channelId", self._channel_id.unwrap().to_string()));
        }
        for &field in ["part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: PlaylistListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return playlists owned by the authenticated user.    
    pub fn mine(mut self, new_value: bool) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube playlist ID(s) for the resource(s) that are being retrieved. In a playlist resource, the id property specifies the playlist's YouTube playlist ID.    
    pub fn id(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// This value indicates that the API should only return the specified channel's playlists.    
    pub fn channel_id(mut self, new_value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a playlist.
///
/// A builder for the *delete* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("labore")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube playlist ID for the playlist that is being deleted. In a playlist resource, the id property specifies the playlist's ID.    
    pub fn id(mut self, new_value: &str) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Modifies a playlist. For example, you could change a playlist's title, description, or privacy status.
///
/// A builder for the *update* method supported by a *playlist* resource.
/// It is not used directly, but through a `PlaylistMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Playlist;
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
///              .on_behalf_of_content_owner("kasd")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Playlist,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Playlist> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Playlist = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Playlist) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Uploads a custom video thumbnail to YouTube and sets it for a video.
///
/// A builder for the *set* method supported by a *thumbnail* resource.
/// It is not used directly, but through a `ThumbnailMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use std::fs;
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
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.thumbnails().set("videoId")
///              .on_behalf_of_content_owner("kasd")
///              .scope(&Default::default())
///              .upload_resumable(fs::File::open("file.ext").unwrap(), 282, "application/octet-stream".parse().unwrap());
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ThumbnailSetMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _video_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ThumbnailSetMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ThumbnailSetMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<R, RS>(mut self, stream: Option<(R, u64, mime::Mime)>, resumeable_stream: Option<(RS, u64, mime::Mime)>) -> Result<ThumbnailSetResponse> where R: Read, RS: ReadSeek {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("videoId", self._video_id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["videoId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = if stream.is_some() {
                "https://www.googleapis.com/upload/youtube/v3/thumbnails/set".to_string()
            } else if resumeable_stream.is_some() {
                "https://www.googleapis.com/resumable/upload/youtube/v3/thumbnails/set".to_string()
            } else { 
                unreachable!() 
        };

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ThumbnailSetResponse = Default::default();




        Result::Success(response)
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 2MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<R>(mut self, stream: R, size: u64, mime_type: mime::Mime) -> Result<ThumbnailSetResponse>
                where R: Read {
        self.doit(Some((stream, size, mime_type)), None::<(fs::File, u64, mime::Mime)>, )
    }
    /// Upload media in a resumeable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// TODO: Write more about how delegation works in this particular case.
    ///
    /// * *max size*: 2MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(mut self, resumeable_stream: RS, size: u64, mime_type: mime::Mime) -> Result<ThumbnailSetResponse>
                where RS: ReadSeek {
        self.doit(None::<(fs::File, u64, mime::Mime)>, Some((resumeable_stream, size, mime_type)), )
    }

    /// Sets the *video id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The videoId parameter specifies a YouTube video ID for which the custom video thumbnail is being provided.    
    pub fn video_id(mut self, new_value: &str) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
        self._video_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.    
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ThumbnailSetMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of videos that match the API request parameters.
///
/// A builder for the *list* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .video_category_id("kasd")
///              .region_code("ea")
///              .page_token("rebum.")
///              .on_behalf_of_content_owner("dolor")
///              .my_rating("sadipscing")
///              .max_results(10)
///              .locale("sed")
///              .id("et")
///              .hl("gubergren")
///              .chart("diam")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<VideoListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._video_category_id.is_some() {
            params.push(("videoCategoryId", self._video_category_id.unwrap().to_string()));
        }
        if self._region_code.is_some() {
            params.push(("regionCode", self._region_code.unwrap().to_string()));
        }
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._my_rating.is_some() {
            params.push(("myRating", self._my_rating.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._locale.is_some() {
            params.push(("locale", self._locale.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._hl.is_some() {
            params.push(("hl", self._hl.unwrap().to_string()));
        }
        if self._chart.is_some() {
            params.push(("chart", self._chart.unwrap().to_string()));
        }
        for &field in ["part", "videoCategoryId", "regionCode", "pageToken", "onBehalfOfContentOwner", "myRating", "maxResults", "locale", "id", "hl", "chart"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: VideoListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video category id* query property to the given value.
    ///
    /// 
    /// The videoCategoryId parameter identifies the video category for which the chart should be retrieved. This parameter can only be used in conjunction with the chart parameter. By default, charts are not restricted to a particular category.    
    pub fn video_category_id(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._video_category_id = Some(new_value.to_string());
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to select a video chart available in the specified region. This parameter can only be used in conjunction with the chart parameter. The parameter value is an ISO 3166-1 alpha-2 country code.    
    pub fn region_code(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.
    /// 
    /// Note: This parameter is supported for use in conjunction with the myRating parameter, but it is not supported for use in conjunction with the id parameter.
    pub fn page_token(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my rating* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to like or dislike to instruct the API to only return videos liked or disliked by the authenticated user.    
    pub fn my_rating(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._my_rating = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.
    /// 
    /// Note: This parameter is supported for use in conjunction with the myRating parameter, but it is not supported for use in conjunction with the id parameter.
    pub fn max_results(mut self, new_value: u32) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *locale* query property to the given value.
    ///
    /// 
    /// DEPRECATED    
    pub fn locale(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) that are being retrieved. In a video resource, the id property specifies the video's ID.    
    pub fn id(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter instructs the API to return a localized version of the video details. If localized text is nor available for the requested language, the localizations object in the API response will contain the requested information in the default language instead. The parameter value is a BCP-47 language code. Your application can determine whether the requested localization was returned by checking the value of the snippet.localized.language property in the API response.    
    pub fn hl(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Sets the *chart* query property to the given value.
    ///
    /// 
    /// The chart parameter identifies the chart that you want to retrieve.    
    pub fn chart(mut self, new_value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Add a like or dislike rating to a video or remove a rating from a video.
///
/// A builder for the *rate* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("sed")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoRateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _rating: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoRateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoRateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        params.push(("rating", self._rating.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "rating", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube video ID of the video that is being rated or having its rating removed.    
    pub fn id(mut self, new_value: &str) -> VideoRateMethodBuilder<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *rating* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// Specifies the rating to record.    
    pub fn rating(mut self, new_value: &str) -> VideoRateMethodBuilder<'a, C, NC, A> {
        self._rating = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoRateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoRateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoRateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoRateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Retrieves the ratings that the authorized user gave to a list of specified videos.
///
/// A builder for the *getRating* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("aliquyam")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoGetRatingMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoGetRatingMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoGetRatingMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<VideoGetRatingResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: VideoGetRatingResponse = Default::default();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube video ID(s) for the resource(s) for which you are retrieving rating data. In a video resource, the id property specifies the video's ID.    
    pub fn id(mut self, new_value: &str) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoGetRatingMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a YouTube video.
///
/// A builder for the *delete* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("amet.")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube video ID for the resource that is being deleted. In a video resource, the id property specifies the video's ID.    
    pub fn id(mut self, new_value: &str) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
        self._id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Updates a video's metadata.
///
/// A builder for the *update* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Video;
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
/// req.processing_details = Default::default(); // is VideoProcessingDetails
/// req.statistics = Default::default(); // is VideoStatistics
/// req.content_details = Default::default(); // is VideoContentDetails
/// req.snippet = Default::default(); // is VideoSnippet
/// req.recording_details = Default::default(); // is VideoRecordingDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().update(&req)
///              .on_behalf_of_content_owner("clita")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Video,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Video> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Video = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Video) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Uploads a video to YouTube and optionally sets the video's metadata.
///
/// A builder for the *insert* method supported by a *video* resource.
/// It is not used directly, but through a `VideoMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Video;
/// # use std::fs;
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
/// req.processing_details = Default::default(); // is VideoProcessingDetails
/// req.statistics = Default::default(); // is VideoStatistics
/// req.content_details = Default::default(); // is VideoContentDetails
/// req.snippet = Default::default(); // is VideoSnippet
/// req.recording_details = Default::default(); // is VideoRecordingDetails
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.videos().insert(&req)
///              .stabilize(true)
///              .on_behalf_of_content_owner_channel("amet")
///              .on_behalf_of_content_owner("accusam")
///              .notify_subscribers(true)
///              .auto_levels(false)
///              .scope(&Default::default())
///              .upload_resumable(fs::File::open("file.ext").unwrap(), 282, "application/octet-stream".parse().unwrap());
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Video,
    _part: String,
    _stabilize: Option<bool>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _notify_subscribers: Option<bool>,
    _auto_levels: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<R, RS>(mut self, stream: Option<(R, u64, mime::Mime)>, resumeable_stream: Option<(RS, u64, mime::Mime)>) -> Result<Video> where R: Read, RS: ReadSeek {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._stabilize.is_some() {
            params.push(("stabilize", self._stabilize.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._notify_subscribers.is_some() {
            params.push(("notifySubscribers", self._notify_subscribers.unwrap().to_string()));
        }
        if self._auto_levels.is_some() {
            params.push(("autoLevels", self._auto_levels.unwrap().to_string()));
        }
        for &field in ["part", "stabilize", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "notifySubscribers", "autoLevels"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = if stream.is_some() {
                "https://www.googleapis.com/upload/youtube/v3/videos".to_string()
            } else if resumeable_stream.is_some() {
                "https://www.googleapis.com/resumable/upload/youtube/v3/videos".to_string()
            } else { 
                unreachable!() 
        };

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Video = Default::default();




        Result::Success(response)
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 64GB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream' and 'video/*'
    pub fn upload<R>(mut self, stream: R, size: u64, mime_type: mime::Mime) -> Result<Video>
                where R: Read {
        self.doit(Some((stream, size, mime_type)), None::<(fs::File, u64, mime::Mime)>, )
    }
    /// Upload media in a resumeable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// TODO: Write more about how delegation works in this particular case.
    ///
    /// * *max size*: 64GB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream' and 'video/*'
    pub fn upload_resumable<RS>(mut self, resumeable_stream: RS, size: u64, mime_type: mime::Mime) -> Result<Video>
                where RS: ReadSeek {
        self.doit(None::<(fs::File, u64, mime::Mime)>, Some((resumeable_stream, size, mime_type)), )
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
    pub fn request(mut self, new_value: &Video) -> VideoInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *stabilize* query property to the given value.
    ///
    /// 
    /// The stabilize parameter indicates whether YouTube should adjust the video to remove shaky camera motions.    
    pub fn stabilize(mut self, new_value: bool) -> VideoInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *notify subscribers* query property to the given value.
    ///
    /// 
    /// The notifySubscribers parameter indicates whether YouTube should send notification to subscribers about the inserted video.    
    pub fn notify_subscribers(mut self, new_value: bool) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._notify_subscribers = Some(new_value);
        self
    }
    /// Sets the *auto levels* query property to the given value.
    ///
    /// 
    /// The autoLevels parameter indicates whether YouTube should automatically enhance the video's lighting and color.    
    pub fn auto_levels(mut self, new_value: bool) -> VideoInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Adds a subscription for the authenticated user's channel.
///
/// A builder for the *insert* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Subscription;
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
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct SubscriptionInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Subscription,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for SubscriptionInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Subscription> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["part"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Subscription = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Subscription) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> SubscriptionInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns subscription resources that match the API request criteria.
///
/// A builder for the *list* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .page_token("est")
///              .order("sit")
///              .on_behalf_of_content_owner_channel("ipsum")
///              .on_behalf_of_content_owner("erat")
///              .my_subscribers(false)
///              .mine(true)
///              .max_results(40)
///              .id("voluptua.")
///              .for_channel_id("dolor")
///              .channel_id("amet")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct SubscriptionListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for SubscriptionListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<SubscriptionListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(13 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._order.is_some() {
            params.push(("order", self._order.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._my_subscribers.is_some() {
            params.push(("mySubscribers", self._my_subscribers.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._for_channel_id.is_some() {
            params.push(("forChannelId", self._for_channel_id.unwrap().to_string()));
        }
        if self._channel_id.is_some() {
            params.push(("channelId", self._channel_id.unwrap().to_string()));
        }
        for &field in ["part", "pageToken", "order", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mySubscribers", "mine", "maxResults", "id", "forChannelId", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: SubscriptionListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *order* query property to the given value.
    ///
    /// 
    /// The order parameter specifies the method that will be used to sort resources in the API response.    
    pub fn order(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my subscribers* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the subscribers of the authenticated user.    
    pub fn my_subscribers(mut self, new_value: bool) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._my_subscribers = Some(new_value);
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's subscriptions.    
    pub fn mine(mut self, new_value: bool) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube subscription ID(s) for the resource(s) that are being retrieved. In a subscription resource, the id property specifies the YouTube subscription ID.    
    pub fn id(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *for channel id* query property to the given value.
    ///
    /// 
    /// The forChannelId parameter specifies a comma-separated list of channel IDs. The API response will then only contain subscriptions matching those channels.    
    pub fn for_channel_id(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._for_channel_id = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a YouTube channel ID. The API will only return that channel's subscriptions.    
    pub fn channel_id(mut self, new_value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> SubscriptionListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a subscription.
///
/// A builder for the *delete* method supported by a *subscription* resource.
/// It is not used directly, but through a `SubscriptionMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct SubscriptionDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for SubscriptionDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SubscriptionDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        for &field in ["id"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube subscription ID for the resource that is being deleted. In a subscription resource, the id property specifies the YouTube subscription ID.    
    pub fn id(mut self, new_value: &str) -> SubscriptionDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SubscriptionDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> SubscriptionDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> SubscriptionDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a collection of search results that match the query parameters specified in the API request. By default, a search result set identifies matching video, channel, and playlist resources, but you can also configure queries to only retrieve a specific type of resource.
///
/// A builder for the *list* method supported by a *search* resource.
/// It is not used directly, but through a `SearchMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .video_type("dolores")
///              .video_syndicated("diam")
///              .video_license("Lorem")
///              .video_embeddable("kasd")
///              .video_duration("elitr")
///              .video_dimension("At")
///              .video_definition("sit")
///              .video_category_id("clita")
///              .video_caption("sanctus")
///              .type_("dolor")
///              .topic_id("diam")
///              .safe_search("voluptua.")
///              .relevance_language("diam")
///              .related_to_video_id("nonumy")
///              .region_code("dolor")
///              .q("nonumy")
///              .published_before("sit")
///              .published_after("sed")
///              .page_token("ipsum")
///              .order("sed")
///              .on_behalf_of_content_owner("At")
///              .max_results(17)
///              .location_radius("ea")
///              .location("ut")
///              .for_mine(true)
///              .for_content_owner(true)
///              .event_type("et")
///              .channel_type("ipsum")
///              .channel_id("eos")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct SearchListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for SearchListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> SearchListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<SearchListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(32 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._video_type.is_some() {
            params.push(("videoType", self._video_type.unwrap().to_string()));
        }
        if self._video_syndicated.is_some() {
            params.push(("videoSyndicated", self._video_syndicated.unwrap().to_string()));
        }
        if self._video_license.is_some() {
            params.push(("videoLicense", self._video_license.unwrap().to_string()));
        }
        if self._video_embeddable.is_some() {
            params.push(("videoEmbeddable", self._video_embeddable.unwrap().to_string()));
        }
        if self._video_duration.is_some() {
            params.push(("videoDuration", self._video_duration.unwrap().to_string()));
        }
        if self._video_dimension.is_some() {
            params.push(("videoDimension", self._video_dimension.unwrap().to_string()));
        }
        if self._video_definition.is_some() {
            params.push(("videoDefinition", self._video_definition.unwrap().to_string()));
        }
        if self._video_category_id.is_some() {
            params.push(("videoCategoryId", self._video_category_id.unwrap().to_string()));
        }
        if self._video_caption.is_some() {
            params.push(("videoCaption", self._video_caption.unwrap().to_string()));
        }
        if self._type_.is_some() {
            params.push(("type", self._type_.unwrap().to_string()));
        }
        if self._topic_id.is_some() {
            params.push(("topicId", self._topic_id.unwrap().to_string()));
        }
        if self._safe_search.is_some() {
            params.push(("safeSearch", self._safe_search.unwrap().to_string()));
        }
        if self._relevance_language.is_some() {
            params.push(("relevanceLanguage", self._relevance_language.unwrap().to_string()));
        }
        if self._related_to_video_id.is_some() {
            params.push(("relatedToVideoId", self._related_to_video_id.unwrap().to_string()));
        }
        if self._region_code.is_some() {
            params.push(("regionCode", self._region_code.unwrap().to_string()));
        }
        if self._q.is_some() {
            params.push(("q", self._q.unwrap().to_string()));
        }
        if self._published_before.is_some() {
            params.push(("publishedBefore", self._published_before.unwrap().to_string()));
        }
        if self._published_after.is_some() {
            params.push(("publishedAfter", self._published_after.unwrap().to_string()));
        }
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._order.is_some() {
            params.push(("order", self._order.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._location_radius.is_some() {
            params.push(("locationRadius", self._location_radius.unwrap().to_string()));
        }
        if self._location.is_some() {
            params.push(("location", self._location.unwrap().to_string()));
        }
        if self._for_mine.is_some() {
            params.push(("forMine", self._for_mine.unwrap().to_string()));
        }
        if self._for_content_owner.is_some() {
            params.push(("forContentOwner", self._for_content_owner.unwrap().to_string()));
        }
        if self._event_type.is_some() {
            params.push(("eventType", self._event_type.unwrap().to_string()));
        }
        if self._channel_type.is_some() {
            params.push(("channelType", self._channel_type.unwrap().to_string()));
        }
        if self._channel_id.is_some() {
            params.push(("channelId", self._channel_id.unwrap().to_string()));
        }
        for &field in ["part", "videoType", "videoSyndicated", "videoLicense", "videoEmbeddable", "videoDuration", "videoDimension", "videoDefinition", "videoCategoryId", "videoCaption", "type", "topicId", "safeSearch", "relevanceLanguage", "relatedToVideoId", "regionCode", "q", "publishedBefore", "publishedAfter", "pageToken", "order", "onBehalfOfContentOwner", "maxResults", "locationRadius", "location", "forMine", "forContentOwner", "eventType", "channelType", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: SearchListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video type* query property to the given value.
    ///
    /// 
    /// The videoType parameter lets you restrict a search to a particular type of videos.    
    pub fn video_type(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_type = Some(new_value.to_string());
        self
    }
    /// Sets the *video syndicated* query property to the given value.
    ///
    /// 
    /// The videoSyndicated parameter lets you to restrict a search to only videos that can be played outside youtube.com.    
    pub fn video_syndicated(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_syndicated = Some(new_value.to_string());
        self
    }
    /// Sets the *video license* query property to the given value.
    ///
    /// 
    /// The videoLicense parameter filters search results to only include videos with a particular license. YouTube lets video uploaders choose to attach either the Creative Commons license or the standard YouTube license to each of their videos.    
    pub fn video_license(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_license = Some(new_value.to_string());
        self
    }
    /// Sets the *video embeddable* query property to the given value.
    ///
    /// 
    /// The videoEmbeddable parameter lets you to restrict a search to only videos that can be embedded into a webpage.    
    pub fn video_embeddable(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_embeddable = Some(new_value.to_string());
        self
    }
    /// Sets the *video duration* query property to the given value.
    ///
    /// 
    /// The videoDuration parameter filters video search results based on their duration.    
    pub fn video_duration(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_duration = Some(new_value.to_string());
        self
    }
    /// Sets the *video dimension* query property to the given value.
    ///
    /// 
    /// The videoDimension parameter lets you restrict a search to only retrieve 2D or 3D videos.    
    pub fn video_dimension(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_dimension = Some(new_value.to_string());
        self
    }
    /// Sets the *video definition* query property to the given value.
    ///
    /// 
    /// The videoDefinition parameter lets you restrict a search to only include either high definition (HD) or standard definition (SD) videos. HD videos are available for playback in at least 720p, though higher resolutions, like 1080p, might also be available.    
    pub fn video_definition(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_definition = Some(new_value.to_string());
        self
    }
    /// Sets the *video category id* query property to the given value.
    ///
    /// 
    /// The videoCategoryId parameter filters video search results based on their category.    
    pub fn video_category_id(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_category_id = Some(new_value.to_string());
        self
    }
    /// Sets the *video caption* query property to the given value.
    ///
    /// 
    /// The videoCaption parameter indicates whether the API should filter video search results based on whether they have captions.    
    pub fn video_caption(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._video_caption = Some(new_value.to_string());
        self
    }
    /// Sets the *type* query property to the given value.
    ///
    /// 
    /// The type parameter restricts a search query to only retrieve a particular type of resource. The value is a comma-separated list of resource types.    
    pub fn type_(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Sets the *topic id* query property to the given value.
    ///
    /// 
    /// The topicId parameter indicates that the API response should only contain resources associated with the specified topic. The value identifies a Freebase topic ID.    
    pub fn topic_id(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._topic_id = Some(new_value.to_string());
        self
    }
    /// Sets the *safe search* query property to the given value.
    ///
    /// 
    /// The safeSearch parameter indicates whether the search results should include restricted content as well as standard content.    
    pub fn safe_search(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._safe_search = Some(new_value.to_string());
        self
    }
    /// Sets the *relevance language* query property to the given value.
    ///
    /// 
    /// The relevanceLanguage parameter instructs the API to return search results that are most relevant to the specified language. The parameter value is typically an ISO 639-1 two-letter language code. However, you should use the values zh-Hans for simplified Chinese and zh-Hant for traditional Chinese. Please note that results in other languages will still be returned if they are highly relevant to the search query term.    
    pub fn relevance_language(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._relevance_language = Some(new_value.to_string());
        self
    }
    /// Sets the *related to video id* query property to the given value.
    ///
    /// 
    /// The relatedToVideoId parameter retrieves a list of videos that are related to the video that the parameter value identifies. The parameter value must be set to a YouTube video ID and, if you are using this parameter, the type parameter must be set to video.    
    pub fn related_to_video_id(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._related_to_video_id = Some(new_value.to_string());
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return search results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.    
    pub fn region_code(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *q* query property to the given value.
    ///
    /// 
    /// The q parameter specifies the query term to search for.    
    pub fn q(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Sets the *published before* query property to the given value.
    ///
    /// 
    /// The publishedBefore parameter indicates that the API response should only contain resources created before the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).    
    pub fn published_before(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._published_before = Some(new_value.to_string());
        self
    }
    /// Sets the *published after* query property to the given value.
    ///
    /// 
    /// The publishedAfter parameter indicates that the API response should only contain resources created after the specified time. The value is an RFC 3339 formatted date-time value (1970-01-01T00:00:00Z).    
    pub fn published_after(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._published_after = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *order* query property to the given value.
    ///
    /// 
    /// The order parameter specifies the method that will be used to order resources in the API response.    
    pub fn order(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._order = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *location radius* query property to the given value.
    ///
    /// 
    /// The locationRadius, in conjunction with the location parameter, defines a geographic area. If the geographic coordinates associated with a video fall within that area, then the video may be included in search results. This parameter value must be a floating point number followed by a measurement unit. Valid measurement units are m, km, ft, and mi. For example, valid parameter values include 1500m, 5km, 10000ft, and 0.75mi. The API does not support locationRadius parameter values larger than 1000 kilometers.    
    pub fn location_radius(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._location_radius = Some(new_value.to_string());
        self
    }
    /// Sets the *location* query property to the given value.
    ///
    /// 
    /// The location parameter restricts a search to videos that have a geographical location specified in their metadata. The value is a string that specifies geographic latitude/longitude coordinates e.g. (37.42307,-122.08427)    
    pub fn location(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._location = Some(new_value.to_string());
        self
    }
    /// Sets the *for mine* query property to the given value.
    ///
    /// 
    /// The forMine parameter restricts the search to only retrieve videos owned by the authenticated user. If you set this parameter to true, then the type parameter's value must also be set to video.    
    pub fn for_mine(mut self, new_value: bool) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._for_mine = Some(new_value);
        self
    }
    /// Sets the *for content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The forContentOwner parameter restricts the search to only retrieve resources owned by the content owner specified by the onBehalfOfContentOwner parameter. The user must be authenticated using a CMS account linked to the specified content owner and onBehalfOfContentOwner must be provided.
    pub fn for_content_owner(mut self, new_value: bool) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._for_content_owner = Some(new_value);
        self
    }
    /// Sets the *event type* query property to the given value.
    ///
    /// 
    /// The eventType parameter restricts a search to broadcast events.    
    pub fn event_type(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._event_type = Some(new_value.to_string());
        self
    }
    /// Sets the *channel type* query property to the given value.
    ///
    /// 
    /// The channelType parameter lets you restrict a search to a particular type of channel.    
    pub fn channel_type(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._channel_type = Some(new_value.to_string());
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter indicates that the API response should only contain resources created by the channel    
    pub fn channel_id(mut self, new_value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> SearchListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of supported regions.
///
/// A builder for the *list* method supported by a *i18nRegion* resource.
/// It is not used directly, but through a `I18nRegionMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .hl("ut")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct I18nRegionListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for I18nRegionListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> I18nRegionListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<I18nRegionListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._hl.is_some() {
            params.push(("hl", self._hl.unwrap().to_string()));
        }
        for &field in ["part", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: I18nRegionListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.    
    pub fn hl(mut self, new_value: &str) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> I18nRegionListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Updates a video stream. If the properties that you want to change cannot be updated, then you need to create a new stream with the proper settings.
///
/// A builder for the *update* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::LiveStream;
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
/// req.id = Some("et".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().update(&req)
///              .on_behalf_of_content_owner_channel("elitr")
///              .on_behalf_of_content_owner("est")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveStreamUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveStream,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveStreamUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveStream> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveStream = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &LiveStream) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveStreamUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a video stream.
///
/// A builder for the *delete* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner_channel("justo")
///              .on_behalf_of_content_owner("et")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveStreamDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveStreamDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live stream ID for the resource that is being deleted.    
    pub fn id(mut self, new_value: &str) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveStreamDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of video streams that match the API request parameters.
///
/// A builder for the *list* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .page_token("et")
///              .on_behalf_of_content_owner_channel("gubergren")
///              .on_behalf_of_content_owner("est")
///              .mine(true)
///              .max_results(78)
///              .id("invidunt")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveStreamListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _page_token: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _mine: Option<bool>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveStreamListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveStreamListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        for &field in ["part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveStreamListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// The mine parameter can be used to instruct the API to only return streams owned by the authenticated user. Set the parameter value to true to only retrieve your own streams.    
    pub fn mine(mut self, new_value: bool) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set. Acceptable values are 0 to 50, inclusive. The default value is 5.    
    pub fn max_results(mut self, new_value: u32) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of YouTube stream IDs that identify the streams being retrieved. In a liveStream resource, the id property specifies the stream's ID.    
    pub fn id(mut self, new_value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveStreamListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Creates a video stream. The stream enables you to send your video to YouTube, which can then broadcast the video to your audience.
///
/// A builder for the *insert* method supported by a *liveStream* resource.
/// It is not used directly, but through a `LiveStreamMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::LiveStream;
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
/// req.id = Some("dolore".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_streams().insert(&req)
///              .on_behalf_of_content_owner_channel("accusam")
///              .on_behalf_of_content_owner("elitr")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveStreamInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveStream,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveStreamInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveStreamInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveStream> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveStream = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &LiveStream) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveStreamInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Updates a channel's metadata.
///
/// A builder for the *update* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Channel;
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
/// req.id = Some("invidunt".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().update(&req)
///              .on_behalf_of_content_owner("dolor")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Channel,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Channel> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Channel = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Channel) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.    
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a collection of zero or more channel resources that match the request criteria.
///
/// A builder for the *list* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .page_token("sit")
///              .on_behalf_of_content_owner("justo")
///              .my_subscribers(false)
///              .mine(false)
///              .max_results(49)
///              .managed_by_me(true)
///              .id("diam")
///              .for_username("ipsum")
///              .category_id("voluptua.")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ChannelListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ChannelListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ChannelListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<ChannelListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(12 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._my_subscribers.is_some() {
            params.push(("mySubscribers", self._my_subscribers.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._managed_by_me.is_some() {
            params.push(("managedByMe", self._managed_by_me.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._for_username.is_some() {
            params.push(("forUsername", self._for_username.unwrap().to_string()));
        }
        if self._category_id.is_some() {
            params.push(("categoryId", self._category_id.unwrap().to_string()));
        }
        for &field in ["part", "pageToken", "onBehalfOfContentOwner", "mySubscribers", "mine", "maxResults", "managedByMe", "id", "forUsername", "categoryId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ChannelListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.    
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *my subscribers* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a list of channels that subscribed to the authenticated user's channel.    
    pub fn my_subscribers(mut self, new_value: bool) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._my_subscribers = Some(new_value);
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return channels owned by the authenticated user.    
    pub fn mine(mut self, new_value: bool) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *managed by me* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to instruct the API to only return channels managed by the content owner that the onBehalfOfContentOwner parameter specifies. The user must be authenticated as a CMS account linked to the specified content owner and onBehalfOfContentOwner must be provided.    
    pub fn managed_by_me(mut self, new_value: bool) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._managed_by_me = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of the YouTube channel ID(s) for the resource(s) that are being retrieved. In a channel resource, the id property specifies the channel's YouTube channel ID.    
    pub fn id(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *for username* query property to the given value.
    ///
    /// 
    /// The forUsername parameter specifies a YouTube username, thereby requesting the channel associated with that username.    
    pub fn for_username(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._for_username = Some(new_value.to_string());
        self
    }
    /// Sets the *category id* query property to the given value.
    ///
    /// 
    /// The categoryId parameter specifies a YouTube guide category, thereby requesting YouTube channels associated with that category.    
    pub fn category_id(mut self, new_value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ChannelListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a playlist item.
///
/// A builder for the *delete* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistItemDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        for &field in ["id"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube playlist item ID for the playlist item that is being deleted. In a playlistItem resource, the id property specifies the playlist item's ID.    
    pub fn id(mut self, new_value: &str) -> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistItemDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a collection of playlist items that match the API request parameters. You can retrieve all of the playlist items in a specified playlist or retrieve one or more playlist items by their unique IDs.
///
/// A builder for the *list* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .video_id("et")
///              .playlist_id("sadipscing")
///              .page_token("At")
///              .on_behalf_of_content_owner("et")
///              .max_results(25)
///              .id("sanctus")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistItemListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _video_id: Option<String>,
    _playlist_id: Option<String>,
    _page_token: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _max_results: Option<u32>,
    _id: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistItemListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<PlaylistItemListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._video_id.is_some() {
            params.push(("videoId", self._video_id.unwrap().to_string()));
        }
        if self._playlist_id.is_some() {
            params.push(("playlistId", self._playlist_id.unwrap().to_string()));
        }
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        for &field in ["part", "videoId", "playlistId", "pageToken", "onBehalfOfContentOwner", "maxResults", "id"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: PlaylistItemListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *video id* query property to the given value.
    ///
    /// 
    /// The videoId parameter specifies that the request should return only the playlist items that contain the specified video.    
    pub fn video_id(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// Sets the *playlist id* query property to the given value.
    ///
    /// 
    /// The playlistId parameter specifies the unique ID of the playlist for which you want to retrieve playlist items. Note that even though this is an optional parameter, every request to retrieve playlist items must specify a value for either the id parameter or the playlistId parameter.    
    pub fn playlist_id(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._playlist_id = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of one or more unique playlist item IDs.    
    pub fn id(mut self, new_value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistItemListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Adds a resource to a playlist.
///
/// A builder for the *insert* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::PlaylistItem;
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
///              .on_behalf_of_content_owner("duo")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistItemInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: PlaylistItem,
    _part: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistItemInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<PlaylistItem> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: PlaylistItem = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &PlaylistItem) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistItemInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Modifies a playlist item. For example, you could update the item's position in the playlist.
///
/// A builder for the *update* method supported by a *playlistItem* resource.
/// It is not used directly, but through a `PlaylistItemMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtubepartner*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::PlaylistItem;
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
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct PlaylistItemUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: PlaylistItem,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<PlaylistItem> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["part"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: PlaylistItem = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &PlaylistItem) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> PlaylistItemUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Uploads a watermark image to YouTube and sets it for a channel.
///
/// A builder for the *set* method supported by a *watermark* resource.
/// It is not used directly, but through a `WatermarkMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::InvideoBranding;
/// # use std::fs;
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
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.watermarks().set(&req, "channelId")
///              .on_behalf_of_content_owner("sanctus")
///              .scope(&Default::default())
///              .upload_resumable(fs::File::open("file.ext").unwrap(), 282, "application/octet-stream".parse().unwrap());
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct WatermarkSetMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: InvideoBranding,
    _channel_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for WatermarkSetMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkSetMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    fn doit<R, RS>(mut self, stream: Option<(R, u64, mime::Mime)>, resumeable_stream: Option<(RS, u64, mime::Mime)>) -> Result<()> where R: Read, RS: ReadSeek {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("channelId", self._channel_id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["channelId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = if stream.is_some() {
                "https://www.googleapis.com/upload/youtube/v3/watermarks/set".to_string()
            } else if resumeable_stream.is_some() {
                "https://www.googleapis.com/resumable/upload/youtube/v3/watermarks/set".to_string()
            } else { 
                unreachable!() 
        };

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }

    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *max size*: 10MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload<R>(mut self, stream: R, size: u64, mime_type: mime::Mime) -> Result<()>
                where R: Read {
        self.doit(Some((stream, size, mime_type)), None::<(fs::File, u64, mime::Mime)>, )
    }
    /// Upload media in a resumeable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a 
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// TODO: Write more about how delegation works in this particular case.
    ///
    /// * *max size*: 10MB
    /// * *multipart*: yes
    /// * *valid mime types*: 'application/octet-stream', 'image/jpeg' and 'image/png'
    pub fn upload_resumable<RS>(mut self, resumeable_stream: RS, size: u64, mime_type: mime::Mime) -> Result<()>
                where RS: ReadSeek {
        self.doit(None::<(fs::File, u64, mime::Mime)>, Some((resumeable_stream, size, mime_type)), )
    }

    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    pub fn request(mut self, new_value: &InvideoBranding) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        self._request = new_value.clone();
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The channelId parameter specifies a YouTube channel ID for which the watermark is being provided.    
    pub fn channel_id(mut self, new_value: &str) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        self._channel_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.    
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> WatermarkSetMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a watermark.
///
/// A builder for the *unset* method supported by a *watermark* resource.
/// It is not used directly, but through a `WatermarkMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner("justo")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct WatermarkUnsetMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _channel_id: String,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for WatermarkUnsetMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> WatermarkUnsetMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("channelId", self._channel_id.to_string()));
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["channelId", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *channel id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The channelId parameter specifies a YouTube channel ID for which the watermark is being unset.    
    pub fn channel_id(mut self, new_value: &str) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
        self._channel_id = new_value.to_string();
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner.    
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> WatermarkUnsetMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Controls the settings for a slate that can be displayed in the broadcast stream.
///
/// A builder for the *control* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .walltime("Stet")
///              .on_behalf_of_content_owner_channel("et")
///              .on_behalf_of_content_owner("amet.")
///              .offset_time_ms("ea")
///              .display_slate(false)
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastControlMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _part: String,
    _walltime: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _offset_time_ms: Option<String>,
    _display_slate: Option<bool>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastControlMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastControlMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcast> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if self._walltime.is_some() {
            params.push(("walltime", self._walltime.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._offset_time_ms.is_some() {
            params.push(("offsetTimeMs", self._offset_time_ms.unwrap().to_string()));
        }
        if self._display_slate.is_some() {
            params.push(("displaySlate", self._display_slate.unwrap().to_string()));
        }
        for &field in ["id", "part", "walltime", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "offsetTimeMs", "displaySlate"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcast = Default::default();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live broadcast ID that uniquely identifies the broadcast in which the slate is being updated.    
    pub fn id(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *walltime* query property to the given value.
    ///
    /// 
    /// The walltime parameter specifies the wall clock time at which the specified slate change will occur. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.    
    pub fn walltime(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
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
    pub fn offset_time_ms(mut self, new_value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._offset_time_ms = Some(new_value.to_string());
        self
    }
    /// Sets the *display slate* query property to the given value.
    ///
    /// 
    /// The displaySlate parameter specifies whether the slate is being enabled or disabled.    
    pub fn display_slate(mut self, new_value: bool) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastControlMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Updates a broadcast. For example, you could modify the broadcast settings defined in the liveBroadcast resource's contentDetails object.
///
/// A builder for the *update* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::LiveBroadcast;
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
/// req.id = Some("sit".to_string());
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.live_broadcasts().update(&req)
///              .on_behalf_of_content_owner_channel("ipsum")
///              .on_behalf_of_content_owner("est")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastUpdateMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveBroadcast,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcast> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcast = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &LiveBroadcast) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastUpdateMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Creates a broadcast.
///
/// A builder for the *insert* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::LiveBroadcast;
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
///              .on_behalf_of_content_owner_channel("diam")
///              .on_behalf_of_content_owner("dolores")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: LiveBroadcast,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcast> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcast = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &LiveBroadcast) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Binds a YouTube broadcast to a stream or removes an existing binding between a broadcast and a stream. A broadcast can only be bound to one video stream.
///
/// A builder for the *bind* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .stream_id("erat")
///              .on_behalf_of_content_owner_channel("erat")
///              .on_behalf_of_content_owner("invidunt")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastBindMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _part: String,
    _stream_id: Option<String>,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastBindMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastBindMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcast> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if self._stream_id.is_some() {
            params.push(("streamId", self._stream_id.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "part", "streamId", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcast = Default::default();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the unique ID of the broadcast that is being bound to a video stream.    
    pub fn id(mut self, new_value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *stream id* query property to the given value.
    ///
    /// 
    /// The streamId parameter specifies the unique ID of the video stream that is being bound to a broadcast. If this parameter is omitted, the API will remove any existing binding between the broadcast and a video stream.    
    pub fn stream_id(mut self, new_value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastBindMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of YouTube broadcasts that match the API request parameters.
///
/// A builder for the *list* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .page_token("justo")
///              .on_behalf_of_content_owner_channel("clita")
///              .on_behalf_of_content_owner("clita")
///              .mine(true)
///              .max_results(75)
///              .id("magna")
///              .broadcast_status("sanctus")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcastListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._broadcast_status.is_some() {
            params.push(("broadcastStatus", self._broadcast_status.unwrap().to_string()));
        }
        for &field in ["part", "pageToken", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner", "mine", "maxResults", "id", "broadcastStatus"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcastListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// The mine parameter can be used to instruct the API to only return broadcasts owned by the authenticated user. Set the parameter value to true to only retrieve your own broadcasts.    
    pub fn mine(mut self, new_value: bool) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of YouTube broadcast IDs that identify the broadcasts being retrieved. In a liveBroadcast resource, the id property specifies the broadcast's ID.    
    pub fn id(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *broadcast status* query property to the given value.
    ///
    /// 
    /// The broadcastStatus parameter filters the API response to only include broadcasts with the specified status.    
    pub fn broadcast_status(mut self, new_value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Deletes a broadcast.
///
/// A builder for the *delete* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner_channel("clita")
///              .on_behalf_of_content_owner("ipsum")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastDeleteMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _id: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<()> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("id", self._id.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["id", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response = ();




        Result::Success(response)
    }


    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the YouTube live broadcast ID for the resource that is being deleted.    
    pub fn id(mut self, new_value: &str) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastDeleteMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Changes the status of a YouTube live broadcast and initiates any processes associated with the new status. For example, when you transition a broadcast's status to testing, YouTube starts to transmit video to that broadcast's monitor stream. Before calling this method, you should confirm that the value of the status.streamStatus property for the stream bound to your broadcast is active.
///
/// A builder for the *transition* method supported by a *liveBroadcast* resource.
/// It is not used directly, but through a `LiveBroadcastMethodsBuilder`.
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
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .on_behalf_of_content_owner_channel("et")
///              .on_behalf_of_content_owner("dolor")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct LiveBroadcastTransitionMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _broadcast_status: String,
    _id: String,
    _part: String,
    _on_behalf_of_content_owner_channel: Option<String>,
    _on_behalf_of_content_owner: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<LiveBroadcast> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("broadcastStatus", self._broadcast_status.to_string()));
        params.push(("id", self._id.to_string()));
        params.push(("part", self._part.to_string()));
        if self._on_behalf_of_content_owner_channel.is_some() {
            params.push(("onBehalfOfContentOwnerChannel", self._on_behalf_of_content_owner_channel.unwrap().to_string()));
        }
        if self._on_behalf_of_content_owner.is_some() {
            params.push(("onBehalfOfContentOwner", self._on_behalf_of_content_owner.unwrap().to_string()));
        }
        for &field in ["broadcastStatus", "id", "part", "onBehalfOfContentOwnerChannel", "onBehalfOfContentOwner"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: LiveBroadcast = Default::default();




        Result::Success(response)
    }


    /// Sets the *broadcast status* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The broadcastStatus parameter identifies the state to which the broadcast is changing. Note that to transition a broadcast to either the testing or live state, the status.streamStatus must be active for the stream that the broadcast is bound to.    
    pub fn broadcast_status(mut self, new_value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        self._broadcast_status = new_value.to_string();
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    /// 
    /// The id parameter specifies the unique ID of the broadcast that is transitioning to another status.    
    pub fn id(mut self, new_value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
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
    pub fn on_behalf_of_content_owner_channel(mut self, new_value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        self._on_behalf_of_content_owner_channel = Some(new_value.to_string());
        self
    }
    /// Sets the *on behalf of content owner* query property to the given value.
    ///
    /// 
    /// Note: This parameter is intended exclusively for YouTube content partners.
    /// 
    /// The onBehalfOfContentOwner parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.
    pub fn on_behalf_of_content_owner(mut self, new_value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> LiveBroadcastTransitionMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of categories that can be associated with YouTube videos.
///
/// A builder for the *list* method supported by a *videoCategory* resource.
/// It is not used directly, but through a `VideoCategoryMethodsBuilder`.
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
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .region_code("invidunt")
///              .id("aliquyam")
///              .hl("clita")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct VideoCategoryListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _part: String,
    _region_code: Option<String>,
    _id: Option<String>,
    _hl: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for VideoCategoryListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> VideoCategoryListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<VideoCategoryListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._region_code.is_some() {
            params.push(("regionCode", self._region_code.unwrap().to_string()));
        }
        if self._id.is_some() {
            params.push(("id", self._id.unwrap().to_string()));
        }
        if self._hl.is_some() {
            params.push(("hl", self._hl.unwrap().to_string()));
        }
        for &field in ["part", "regionCode", "id", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: VideoCategoryListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return the list of video categories available in the specified country. The parameter value is an ISO 3166-1 alpha-2 country code.    
    pub fn region_code(mut self, new_value: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *id* query property to the given value.
    ///
    /// 
    /// The id parameter specifies a comma-separated list of video category IDs for the resources that you are retrieving.    
    pub fn id(mut self, new_value: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._id = Some(new_value.to_string());
        self
    }
    /// Sets the *hl* query property to the given value.
    ///
    /// 
    /// The hl parameter specifies the language that should be used for text values in the API response.    
    pub fn hl(mut self, new_value: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> VideoCategoryListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Returns a list of channel activity events that match the request criteria. For example, you can retrieve events associated with a particular channel, events associated with the user's subscriptions and Google+ friends, or the YouTube home page feed, which is customized for each user.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethodsBuilder`.
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
/// * *https://www.googleapis.com/auth/youtube.readonly*
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
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
///              .region_code("sanctus")
///              .published_before("no")
///              .published_after("sit")
///              .page_token("consetetur")
///              .mine(false)
///              .max_results(48)
///              .home(true)
///              .channel_id("amet")
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ActivityListMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

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
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ActivityListMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityListMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<ActivityListResponse> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("part", self._part.to_string()));
        if self._region_code.is_some() {
            params.push(("regionCode", self._region_code.unwrap().to_string()));
        }
        if self._published_before.is_some() {
            params.push(("publishedBefore", self._published_before.unwrap().to_string()));
        }
        if self._published_after.is_some() {
            params.push(("publishedAfter", self._published_after.unwrap().to_string()));
        }
        if self._page_token.is_some() {
            params.push(("pageToken", self._page_token.unwrap().to_string()));
        }
        if self._mine.is_some() {
            params.push(("mine", self._mine.unwrap().to_string()));
        }
        if self._max_results.is_some() {
            params.push(("maxResults", self._max_results.unwrap().to_string()));
        }
        if self._home.is_some() {
            params.push(("home", self._home.unwrap().to_string()));
        }
        if self._channel_id.is_some() {
            params.push(("channelId", self._channel_id.unwrap().to_string()));
        }
        for &field in ["part", "regionCode", "publishedBefore", "publishedAfter", "pageToken", "mine", "maxResults", "home", "channelId"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: ActivityListResponse = Default::default();




        Result::Success(response)
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
    pub fn part(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._part = new_value.to_string();
        self
    }
    /// Sets the *region code* query property to the given value.
    ///
    /// 
    /// The regionCode parameter instructs the API to return results for the specified country. The parameter value is an ISO 3166-1 alpha-2 country code. YouTube uses this value when the authorized user's previous activity on YouTube does not provide enough information to generate the activity feed.    
    pub fn region_code(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._region_code = Some(new_value.to_string());
        self
    }
    /// Sets the *published before* query property to the given value.
    ///
    /// 
    /// The publishedBefore parameter specifies the date and time before which an activity must have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be excluded from the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.    
    pub fn published_before(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._published_before = Some(new_value.to_string());
        self
    }
    /// Sets the *published after* query property to the given value.
    ///
    /// 
    /// The publishedAfter parameter specifies the earliest date and time that an activity could have occurred for that activity to be included in the API response. If the parameter value specifies a day, but not a time, then any activities that occurred that day will be included in the result set. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.    
    pub fn published_after(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._published_after = Some(new_value.to_string());
        self
    }
    /// Sets the *page token* query property to the given value.
    ///
    /// 
    /// The pageToken parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.    
    pub fn page_token(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Sets the *mine* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve a feed of the authenticated user's activities.    
    pub fn mine(mut self, new_value: bool) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._mine = Some(new_value);
        self
    }
    /// Sets the *max results* query property to the given value.
    ///
    /// 
    /// The maxResults parameter specifies the maximum number of items that should be returned in the result set.    
    pub fn max_results(mut self, new_value: u32) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._max_results = Some(new_value);
        self
    }
    /// Sets the *home* query property to the given value.
    ///
    /// 
    /// Set this parameter's value to true to retrieve the activity feed that displays on the YouTube home page for the currently authenticated user.    
    pub fn home(mut self, new_value: bool) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._home = Some(new_value);
        self
    }
    /// Sets the *channel id* query property to the given value.
    ///
    /// 
    /// The channelId parameter specifies a unique YouTube channel ID. The API will then return a list of that channel's activities.    
    pub fn channel_id(mut self, new_value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ActivityListMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


/// Posts a bulletin for a specific channel. (The user submitting the request must be authorized to act on the channel's behalf.)
/// 
/// Note: Even though an activity resource can contain information about actions like a user rating a video or marking a video as a favorite, you need to use other API methods to generate those activity resources. For example, you would use the API's videos.rate() method to rate a video and the playlistItems.insert() method to mark a video as a favorite.
///
/// A builder for the *insert* method supported by a *activity* resource.
/// It is not used directly, but through a `ActivityMethodsBuilder`.
///
/// **Settable Parts**
/// 
/// * *snippet*
/// * *contentDetails*
///
/// # Scopes
///
/// You will need authorization for the *https://www.googleapis.com/auth/youtube* scope to make a valid call.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate "yup-oauth2" as oauth2;
/// # extern crate "rustc-serialize" as rustc_serialize;
/// # extern crate youtube3;
/// # use youtube3::Activity;
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
///              .scope(&Default::default())
///              .doit();
/// // TODO: show how to handle the result !
/// # }
/// ```
pub struct ActivityInsertMethodBuilder<'a, C, NC, A>
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a YouTube<C, NC, A>,
    _request: Activity,
    _part: String,
    _delegate: Option<&'a mut Delegate>,
    _scope: Option<Scope>,
    _additional_params: HashMap<String, String>
}

impl<'a, C, NC, A> MethodBuilder for ActivityInsertMethodBuilder<'a, C, NC, A> {}

impl<'a, C, NC, A> ActivityInsertMethodBuilder<'a, C, NC, A> where NC: hyper::net::NetworkConnector, C: BorrowMut<hyper::Client<NC>> + 'a, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<Activity> {
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        if self._part.len() == 0 {
            self._part = self._request.to_parts();
        }
        params.push(("part", self._part.to_string()));
        for &field in ["part"].iter() {
            if self._additional_params.contains_key(field) {
                return Result::FieldClash(field);
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let mut url = "https://www.googleapis.com/youtube/v3/".to_string();

        url.push('?');
        url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));

        let response: Activity = Default::default();




        Result::Success(response)
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
    pub fn request(mut self, new_value: &Activity) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn part(mut self, new_value: &str) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
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
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
        self._delegate = Some(new_value);
        self
    }
    /// Sets the *scope* property to the given value.
    ///
    /// 
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the automated algorithm which 
    /// simply prefers read-only scopes over those who are not.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn scope(mut self, new_value: &Scope) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
        self._scope = Some(new_value.clone());
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
    pub fn param(mut self, name: &str, value: &str) -> ActivityInsertMethodBuilder<'a, C, NC, A> {
        self._additional_params.insert(name.to_string(), value.to_string());
        self
    }
}


