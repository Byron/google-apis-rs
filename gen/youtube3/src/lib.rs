// DO NOT EDIT !
// This file was generated automatically from 'src/mako/lib.rs.mako'
// DO NOT EDIT !

//! TODO: Library level fully fledged documentation, incuding **summary** and **usage**.
//! And another line, for testing
//! 
//! # Activities
//! 
//! 
#![feature(core)]
#![allow(non_snake_case)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

mod cmn;

use std::collections::HashMap;

pub use cmn::{Resource, Part, ResponseResult, RequestResult, NestedType};

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
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#subscriptionListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of subscriptions that match the request criteria.
	pub items: Vec<Subscription>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for SubscriptionListResponse {}

/// Information about a resource that received a positive (like) rating.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsLike {
	/// The resourceId object contains information that identifies the rated resource.
	pub resourceId: Option<ResourceId>,
}

impl Part for ActivityContentDetailsLike {}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastSnippet {
	/// The date and time that the broadcast actually ended. This information is only available once the broadcast's state is complete. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub actualEndTime: Option<String>,
	/// The broadcast's description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource.
	pub description: Option<String>,
	/// The broadcast's title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource.
	pub title: Option<String>,
	/// The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast.
	pub channelId: Option<String>,
	/// The date and time that the broadcast was added to YouTube's live broadcast schedule. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// The date and time that the broadcast is scheduled to start. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduledStartTime: Option<String>,
	/// The date and time that the broadcast actually started. This information is only available once the broadcast's state is live. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub actualStartTime: Option<String>,
	/// The date and time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduledEndTime: Option<String>,
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
	pub bitrateBps: Option<String>,
	/// The uploaded video file's container format.
	pub container: Option<String>,
	/// Geographic coordinates that identify the place where the uploaded video was recorded. Coordinates are defined using WGS 84.
	pub recordingLocation: Option<GeoPoint>,
	/// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
	pub fileType: Option<String>,
	/// The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported:  
/// - Date only: YYYY-MM-DD 
/// - Naive time: YYYY-MM-DDTHH:MM:SS 
/// - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM
	pub creationTime: Option<String>,
	/// The length of the uploaded video in milliseconds.
	pub durationMs: Option<String>,
	/// The uploaded file's name. This field is present whether a video file or another type of file was uploaded.
	pub fileName: Option<String>,
	/// The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.
	pub fileSize: Option<String>,
	/// A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.
	pub videoStreams: Vec<VideoFileDetailsVideoStream>,
	/// A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.
	pub audioStreams: Vec<VideoFileDetailsAudioStream>,
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
/// * insert (request|response)
/// * delete (none)
/// * list (none)
/// * update (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Playlist {
	/// The status object contains status information for the playlist.
	pub status: Option<PlaylistStatus>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".
	pub kind: Option<String>,
	/// The contentDetails object contains information like video count.
	pub contentDetails: Option<PlaylistContentDetails>,
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

impl RequestResult for Playlist {}
impl Resource for Playlist {}
impl ResponseResult for Playlist {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItemListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of playlist items that match the request criteria.
	pub items: Vec<PlaylistItem>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
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
	pub offsetMs: Option<String>,
	/// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
	pub type_: Option<String>,
	/// Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.
	pub durationMs: Option<String>,
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
	pub channelId: Option<String>,
	/// The date and time that the playlist was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// The channel title of the channel that the video belongs to.
	pub channelTitle: Option<String>,
	/// The playlist's title.
	pub title: Option<String>,
	/// The language of the playlist's default title and description.
	pub defaultLanguage: Option<String>,
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
	pub copyrightStrikesGoodStanding: Option<bool>,
	/// Whether or not the channel respects the community guidelines.
	pub communityGuidelinesGoodStanding: Option<bool>,
	/// Whether or not the channel has any unresolved claims.
	pub contentIdClaimsGoodStanding: Option<bool>,
	/// Describes the general state of the channel. This field will always show if there are any issues whatsoever with the channel. Currently this field represents the result of the logical and operation over the community guidelines good standing, the copyright strikes good standing and the content ID claims good standing, but this may change in the future.
	pub overallGoodStanding: Option<bool>,
}

impl Part for ChannelAuditDetails {}

/// A live stream describes a live ingestion point.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * update (request|response)
/// * insert (request|response)
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
	pub contentDetails: Option<LiveStreamContentDetails>,
	/// The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube.
	pub cdn: Option<CdnSettings>,
	/// The ID that YouTube assigns to uniquely identify the stream.
	pub id: Option<String>,
}

impl RequestResult for LiveStream {}
impl Resource for LiveStream {}
impl ResponseResult for LiveStream {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * set (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ThumbnailSetResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// A list of thumbnails.
	pub items: Vec<ThumbnailDetails>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#thumbnailSetResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
}

impl ResponseResult for ThumbnailSetResponse {}

/// Information about the uploaded video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsUpload {
	/// The ID that YouTube uses to uniquely identify the uploaded video.
	pub videoId: Option<String>,
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
	pub moderateComments: Option<bool>,
	/// Whether the tab to browse the videos should be displayed.
	pub showBrowseView: Option<bool>,
	/// Title for the featured channels tab.
	pub featuredChannelsTitle: Option<String>,
	/// no description provided
	pub defaultLanguage: Option<String>,
	/// The trailer of the channel, for users that are not subscribers.
	pub unsubscribedTrailer: Option<String>,
	/// The list of featured channels.
	pub featuredChannelsUrls: Vec<String>,
	/// A prominent color that can be rendered on this channel page.
	pub profileColor: Option<String>,
	/// Which content tab users should see when viewing the channel.
	pub defaultTab: Option<String>,
	/// Lists keywords associated with the channel, comma-separated.
	pub keywords: Option<String>,
	/// Whether related channels should be proposed.
	pub showRelatedChannels: Option<bool>,
	/// The ID for a Google Analytics account to track and measure traffic to the channels.
	pub trackingAnalyticsAccountId: Option<String>,
}

impl Part for ChannelSettings {}

/// Statistics about the video, such as the number of times the video was viewed or liked.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoStatistics {
	/// The number of comments for the video.
	pub commentCount: Option<i64>,
	/// The number of times the video has been viewed.
	pub viewCount: Option<i64>,
	/// The number of users who currently have the video marked as a favorite video.
	pub favoriteCount: Option<i64>,
	/// The number of users who have indicated that they disliked the video by giving it a negative rating.
	pub dislikeCount: Option<i64>,
	/// The number of users who have indicated that they liked the video by giving it a positive rating.
	pub likeCount: Option<i64>,
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
	pub ingestionInfo: Option<IngestionInfo>,
	/// The method or protocol used to transmit the video stream.
	pub ingestionType: Option<String>,
}

impl Part for CdnSettings {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * getRating (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoGetRatingResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// A list of ratings that match the request criteria.
	pub items: Vec<VideoRating>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoGetRatingResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
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
	pub channelId: Option<String>,
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
	pub resourceId: Option<ResourceId>,
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
	pub resourceId: Option<ResourceId>,
	/// The subscription's details.
	pub description: Option<String>,
	/// The ID that YouTube uses to uniquely identify the subscriber's channel.
	pub channelId: Option<String>,
	/// The date and time that the subscription was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// Channel title for the channel that the subscription belongs to.
	pub channelTitle: Option<String>,
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
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nRegionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource.
	pub items: Vec<I18nRegion>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegionListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
}

impl ResponseResult for I18nRegionListResponse {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStreamListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of live streams that match the request criteria.
	pub items: Vec<LiveStream>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
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
	pub promotedByContentOwner: Option<bool>,
	/// A custom message to display for this promotion. This field is currently ignored unless the promoted item is a website.
	pub customMessage: Option<String>,
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
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of playlists that match the request criteria.
	pub items: Vec<Playlist>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for PlaylistListResponse {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * set (request)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoBranding {
	/// no description provided
	pub targetChannelId: Option<String>,
	/// no description provided
	pub position: Option<InvideoPosition>,
	/// no description provided
	pub imageUrl: Option<String>,
	/// no description provided
	pub timing: Option<InvideoTiming>,
	/// no description provided
	pub imageBytes: Option<String>,
}

impl RequestResult for InvideoBranding {}

/// Information about the playlist item's privacy status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistItemStatus {
	/// This resource's privacy status.
	pub privacyStatus: Option<String>,
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
	pub conversionUrl: Option<String>,
}

impl Part for ChannelConversionPing {}

/// Describes an invideo promotion campaign consisting of multiple promoted items. A campaign belongs to a single channel_id.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoPromotion {
	/// The default temporal position within the video where the promoted item will be displayed. Can be overriden by more specific timing in the item.
	pub defaultTiming: Option<InvideoTiming>,
	/// List of promoted items in decreasing priority.
	pub items: Vec<PromotedItem>,
	/// Indicates whether the channel's promotional campaign uses "smart timing." This feature attempts to show promotions at a point in the video when they are more likely to be clicked and less likely to disrupt the viewing experience. This feature also picks up a single promotion to show on each video.
	pub useSmartTiming: Option<bool>,
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
/// * insert (request|response)
/// * update (request|response)
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
	pub contentDetails: Option<PlaylistItemContentDetails>,
	/// The ID that YouTube uses to uniquely identify the playlist item.
	pub id: Option<String>,
}

impl RequestResult for PlaylistItem {}
impl Resource for PlaylistItem {}
impl ResponseResult for PlaylistItem {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GuideCategoryListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#guideCategoryListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of categories that can be associated with YouTube channels. In this map, the category ID is the map key, and its value is the corresponding guideCategory resource.
	pub items: Vec<GuideCategory>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
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
	pub channelId: Option<String>,
	/// The type of the channel section.
	pub type_: Option<String>,
	/// The language of the channel section's default title and description.
	pub defaultLanguage: Option<String>,
}

impl Part for ChannelSectionSnippet {}

/// Details about the content of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelContentDetails {
	/// no description provided
	pub relatedPlaylists: HashMap<String, ChannelContentDetailsRelatedplaylists>,
	/// The googlePlusUserId object identifies the Google+ profile ID associated with this channel.
	pub googlePlusUserId: Option<String>,
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
	pub startAt: Option<String>,
	/// The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.
	pub endAt: Option<String>,
	/// The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request.
	pub videoId: Option<String>,
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
	pub resourceId: Option<ResourceId>,
	/// The reason that the resource is recommended to the user.
	pub reason: Option<String>,
	/// The seedResourceId object contains information about the resource that caused the recommendation.
	pub seedResourceId: Option<ResourceId>,
}

impl Part for ActivityContentDetailsRecommendation {}

/// Recording information associated with the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoRecordingDetails {
	/// The date and time when the video was recorded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sssZ) format.
	pub recordingDate: Option<String>,
	/// The text description of the location where the video was recorded.
	pub locationDescription: Option<String>,
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
	pub resourceId: Option<ResourceId>,
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
	pub playlistItem: Option<ActivityContentDetailsPlaylistItem>,
	/// The like object contains information about a resource that received a positive (like) rating. This property is only present if the snippet.type is like.
	pub like: Option<ActivityContentDetailsLike>,
	/// The promotedItem object contains details about a resource which is being promoted. This property is only present if the snippet.type is promotedItem.
	pub promotedItem: Option<ActivityContentDetailsPromotedItem>,
	/// The recommendation object contains information about a recommended resource. This property is only present if the snippet.type is recommendation.
	pub recommendation: Option<ActivityContentDetailsRecommendation>,
	/// The favorite object contains information about a video that was marked as a favorite video. This property is only present if the snippet.type is favorite.
	pub favorite: Option<ActivityContentDetailsFavorite>,
	/// The upload object contains information about the uploaded video. This property is only present if the snippet.type is upload.
	pub upload: Option<ActivityContentDetailsUpload>,
	/// The social object contains details about a social network post. This property is only present if the snippet.type is social.
	pub social: Option<ActivityContentDetailsSocial>,
	/// The channelItem object contains details about a resource which was added to a channel. This property is only present if the snippet.type is channelItem.
	pub channelItem: Option<ActivityContentDetailsChannelItem>,
	/// The bulletin object contains details about a channel bulletin post. This object is only present if the snippet.type is bulletin.
	pub bulletin: Option<ActivityContentDetailsBulletin>,
	/// The subscription object contains information about a channel that a user subscribed to. This property is only present if the snippet.type is subscription.
	pub subscription: Option<ActivityContentDetailsSubscription>,
}

impl Part for ActivityContentDetails {}

/// A i18nRegion resource identifies a region where YouTube is available.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
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

impl Part for I18nRegion {}

/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelContentOwnerDetails {
	/// The ID of the content owner linked to the channel.
	pub contentOwner: Option<String>,
	/// The date and time of when the channel was linked to the content owner. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub timeLinked: Option<String>,
}

impl Part for ChannelContentOwnerDetails {}

/// Describes processing status and progress and availability of some other Video resource parts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoProcessingDetails {
	/// This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.
	pub fileDetailsAvailability: Option<String>,
	/// This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub editorSuggestionsAvailability: Option<String>,
	/// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
	pub processingStatus: Option<String>,
	/// This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub processingIssuesAvailability: Option<String>,
	/// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
	pub processingFailureReason: Option<String>,
	/// This value indicates whether thumbnail images have been generated for the video.
	pub thumbnailsAvailability: Option<String>,
	/// The processingProgress object contains information about the progress YouTube has made in processing the video. The values are really only relevant if the video's processing status is processing.
	pub processingProgress: Option<VideoProcessingDetailsProcessingProgress>,
	/// This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
	pub tagSuggestionsAvailability: Option<String>,
}

impl Part for VideoProcessingDetails {}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastStatus {
	/// The broadcast's recording status.
	pub recordingStatus: Option<String>,
	/// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
	pub privacyStatus: Option<String>,
	/// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
	pub lifeCycleStatus: Option<String>,
	/// Priority of the live broadcast event (internal state).
	pub liveBroadcastPriority: Option<String>,
}

impl Part for LiveBroadcastStatus {}

/// Details about the content to witch a subscription refers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionContentDetails {
	/// The number of new items in the subscription since its content was last read.
	pub newItemCount: Option<u32>,
	/// The type of activity this subscription is for (only uploads, everything).
	pub activityType: Option<String>,
	/// The approximate number of items that the subscription points to.
	pub totalItemCount: Option<u32>,
}

impl Part for SubscriptionContentDetails {}

/// A video resource represents a YouTube video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * rate (none)
/// * getRating (none)
/// * list (none)
/// * insert (request|response)
/// * update (request|response)
/// * delete (none)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Video {
	/// The status object contains information about the video's uploading, processing, and privacy statuses.
	pub status: Option<VideoStatus>,
	/// The topicDetails object encapsulates information about Freebase topics associated with the video.
	pub topicDetails: Option<VideoTopicDetails>,
	/// The monetizationDetails object encapsulates information about the monetization status of the video.
	pub monetizationDetails: Option<VideoMonetizationDetails>,
	/// The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner.
	pub suggestions: Option<VideoSuggestions>,
	/// Age restriction details related to a video.
	pub ageGating: Option<VideoAgeGating>,
	/// The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner.
	pub fileDetails: Option<VideoFileDetails>,
	/// The player object contains information that you would use to play the video in an embedded player.
	pub player: Option<VideoPlayer>,
	/// The ID that YouTube uses to uniquely identify the video.
	pub id: Option<String>,
	/// List with all localizations.
	pub localizations: HashMap<String, VideoLocalization>,
	/// The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast.
	pub liveStreamingDetails: Option<VideoLiveStreamingDetails>,
	/// The processingProgress object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video.
/// 
/// The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner.
	pub processingDetails: Option<VideoProcessingDetails>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#video".
	pub kind: Option<String>,
	/// The statistics object contains statistics about the video.
	pub statistics: Option<VideoStatistics>,
	/// The contentDetails object contains information about the video content, including the length of the video and its aspect ratio.
	pub contentDetails: Option<VideoContentDetails>,
	/// The conversionPings object encapsulates information about url pings that need to be respected by the App in different video contexts.
	pub conversionPings: Option<VideoConversionPings>,
	/// The snippet object contains basic details about the video, such as its title, description, and category.
	pub snippet: Option<VideoSnippet>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The projectDetails object contains information about the project specific video metadata.
	pub projectDetails: Option<VideoProjectDetails>,
	/// The recordingDetails object encapsulates information about the location, date and address where the video was recorded.
	pub recordingDetails: Option<VideoRecordingDetails>,
}

impl RequestResult for Video {}
impl Resource for Video {}
impl ResponseResult for Video {}

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
	pub alcoholContent: Option<bool>,
	/// Video game rating, if any.
	pub videoGameRating: Option<String>,
}

impl Part for VideoAgeGating {}

/// Player to be used for a video playback.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoPlayer {
	/// An <iframe> tag that embeds a player that will play the video.
	pub embedHtml: Option<String>,
}

impl Part for VideoPlayer {}

/// Basic details about a channel, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSnippet {
	/// The date and time that the channel was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// A map of thumbnail images associated with the channel. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The channel's title.
	pub title: Option<String>,
	/// Localized title and description, read-only.
	pub localized: Option<ChannelLocalization>,
	/// The language of the channel's default title and description.
	pub defaultLanguage: Option<String>,
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
	pub textColor: Option<String>,
	/// An ID that uniquely identifies a playlist that displays next to the video player.
	pub featuredPlaylistId: Option<String>,
	/// The text color for the video watch page's branded area.
	pub backgroundColor: Option<String>,
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
	pub videoId: Option<String>,
}

impl Part for VideoRating {}

/// Describes a single promoted item id. It is a union of various possible types.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PromotedItemId {
	/// If the promoted item represents a website, this field represents the url pointing to the website. This field will be present only if type has the value website.
	pub websiteUrl: Option<String>,
	/// If type is recentUpload, this field identifies the channel from which to take the recent upload. If missing, the channel is assumed to be the same channel for which the invideoPromotion is set.
	pub recentlyUploadedBy: Option<String>,
	/// Describes the type of the promoted item.
	pub type_: Option<String>,
	/// If the promoted item represents a video, this field represents the unique YouTube ID identifying it. This field will be present only if type has the value video.
	pub videoId: Option<String>,
}

impl Part for PromotedItemId {}

/// A subscription resource contains information about a YouTube user subscription. A subscription notifies a user when new videos are added to a channel or when another user takes one of several actions on YouTube, such as uploading a video, rating a video, or commenting on a video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * insert (request|response)
/// * list (none)
/// * delete (none)
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
	pub contentDetails: Option<SubscriptionContentDetails>,
	/// The subscriberSnippet object contains basic details about the sbuscriber.
	pub subscriberSnippet: Option<SubscriptionSubscriberSnippet>,
	/// The ID that YouTube uses to uniquely identify the subscription.
	pub id: Option<String>,
}

impl RequestResult for Subscription {}
impl Resource for Subscription {}
impl ResponseResult for Subscription {}

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
	pub resourceId: Option<ResourceId>,
	/// The value that YouTube uses to uniquely identify the playlist.
	pub playlistId: Option<String>,
	/// ID of the item within the playlist.
	pub playlistItemId: Option<String>,
}

impl Part for ActivityContentDetailsPlaylistItem {}

/// Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct InvideoPosition {
	/// Describes in which corner of the video the visual widget will appear.
	pub cornerPosition: Option<String>,
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
	pub resourceId: Option<ResourceId>,
}

impl Part for ActivityContentDetailsComment {}

/// Basic details about a guide category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct GuideCategorySnippet {
	/// no description provided
	pub channelId: Option<String>,
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
	pub channelId: Option<String>,
	/// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
	pub liveBroadcastContent: Option<String>,
	/// The language of the videos's default snippet.
	pub defaultLanguage: Option<String>,
	/// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The video's title.
	pub title: Option<String>,
	/// The YouTube video category associated with the video.
	pub categoryId: Option<String>,
	/// Localized snippet selected with the hl parameter. If no such localization exists, this field is populated with the default snippet. (Read-only)
	pub localized: Option<VideoLocalization>,
	/// Channel title for the channel that the video belongs to.
	pub channelTitle: Option<String>,
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
	pub isReusable: Option<bool>,
	/// The ingestion URL where the closed captions of this stream are sent.
	pub closedCaptionsIngestionUrl: Option<String>,
}

impl Part for LiveStreamContentDetails {}

/// Detailed settings of a broadcast.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastContentDetails {
	/// This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast's status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint's eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers.
	pub startWithSlate: Option<bool>,
	/// This value uniquely identifies the live stream bound to the broadcast.
	pub boundStreamId: Option<String>,
	/// This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video.
	pub enableEmbed: Option<bool>,
	/// This setting indicates whether closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API.
	pub enableClosedCaptions: Option<bool>,
	/// This setting indicates whether YouTube should enable content encryption for the broadcast.
	pub enableContentEncryption: Option<bool>,
	/// Automatically start recording after the event goes live. The default value for this property is true.
/// 
/// 
/// 
/// Important: You must also set the enableDvr property's value to true if you want the playback to be available immediately after the broadcast ends. If you set this property's value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback.
	pub recordFromStart: Option<bool>,
	/// This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true.
/// 
/// 
/// 
/// Important: You must set the value to true and also set the enableArchive property's value to true if you want to make playback available immediately after the broadcast ends.
	pub enableDvr: Option<bool>,
	/// The monitorStream object contains information about the monitor stream, which the broadcaster can use to review the event content before the broadcast stream is shown publicly.
	pub monitorStream: Option<MonitorStreamInfo>,
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
	pub privacyStatus: Option<String>,
	/// The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishAt: Option<String>,
	/// This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled.
	pub publicStatsViewable: Option<bool>,
	/// The status of the uploaded video.
	pub uploadStatus: Option<String>,
	/// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
	pub rejectionReason: Option<String>,
	/// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
	pub failureReason: Option<String>,
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
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelSectionListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// A list of ChannelSections that match the request criteria.
	pub items: Vec<ChannelSection>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSectionListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
}

impl ResponseResult for ChannelSectionListResponse {}

/// Settings and Info of the monitor stream
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct MonitorStreamInfo {
	/// If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay.
	pub broadcastStreamDelayMs: Option<u32>,
	/// HTML code that embeds a player that plays the monitor stream.
	pub embedHtml: Option<String>,
	/// This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster's consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints.
/// 
/// You need to set this value to true if you intend to have a broadcast delay for your event.
/// 
/// Note: This property cannot be updated once the broadcast is in the testing or live state.
	pub enableMonitorStream: Option<bool>,
}

impl Part for MonitorStreamInfo {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct I18nLanguageListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource.
	pub items: Vec<I18nLanguage>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguageListResponse".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
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
	pub defaultLanguage: Option<LanguageTag>,
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
/// * control (response)
/// * insert (request|response)
/// * update (request|response)
/// * transition (response)
/// * bind (response)
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
	pub contentDetails: Option<LiveBroadcastContentDetails>,
	/// The ID that YouTube assigns to uniquely identify the broadcast.
	pub id: Option<String>,
}

impl RequestResult for LiveBroadcast {}
impl Resource for LiveBroadcast {}
impl ResponseResult for LiveBroadcast {}

/// Information about a video stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoFileDetailsVideoStream {
	/// The video stream's bitrate, in bits per second.
	pub bitrateBps: Option<String>,
	/// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
	pub vendor: Option<String>,
	/// The video codec that the stream uses.
	pub codec: Option<String>,
	/// The encoded video content's width in pixels. You can calculate the video's encoding aspect ratio as width_pixels / height_pixels.
	pub widthPixels: Option<u32>,
	/// The encoded video content's height in pixels.
	pub heightPixels: Option<u32>,
	/// The video content's display aspect ratio, which specifies the aspect ratio in which the video should be displayed.
	pub aspectRatio: Option<f64>,
	/// The amount that YouTube needs to rotate the original source content to properly display the video.
	pub rotation: Option<String>,
	/// The video stream's frame rate, in frames per second.
	pub frameRateFps: Option<f64>,
}

impl Part for VideoFileDetailsVideoStream {}

/// A thumbnail is an image representing a YouTube resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * set (none)
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
/// * list (none)
/// * update (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Channel {
	/// The status object encapsulates information about the privacy status of the channel.
	pub status: Option<ChannelStatus>,
	/// The invideoPromotion object encapsulates information about promotion campaign associated with the channel.
	pub invideoPromotion: Option<InvideoPromotion>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channel".
	pub kind: Option<String>,
	/// The statistics object encapsulates statistics for the channel.
	pub statistics: Option<ChannelStatistics>,
	/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
	pub contentOwnerDetails: Option<ChannelContentOwnerDetails>,
	/// The topicDetails object encapsulates information about Freebase topics associated with the channel.
	pub topicDetails: Option<ChannelTopicDetails>,
	/// The contentDetails object encapsulates information about the channel's content.
	pub contentDetails: Option<ChannelContentDetails>,
	/// The brandingSettings object encapsulates information about the branding of the channel.
	pub brandingSettings: Option<ChannelBrandingSettings>,
	/// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
	pub conversionPings: Option<ChannelConversionPings>,
	/// The snippet object contains basic details about the channel, such as its title, description, and thumbnail images.
	pub snippet: Option<ChannelSnippet>,
	/// The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process.
	pub auditDetails: Option<ChannelAuditDetails>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The ID that YouTube uses to uniquely identify the channel.
	pub id: Option<String>,
	/// Localizations for different languages
	pub localizations: HashMap<String, ChannelLocalization>,
}

impl RequestResult for Channel {}
impl Resource for Channel {}
impl ResponseResult for Channel {}

/// Statistics about a channel: number of subscribers, number of videos in the channel, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelStatistics {
	/// The number of comments for the channel.
	pub commentCount: Option<i64>,
	/// The number of subscribers that the channel has.
	pub subscriberCount: Option<i64>,
	/// The number of videos uploaded to the channel.
	pub videoCount: Option<i64>,
	/// Whether or not the number of subscribers is shown for this user.
	pub hiddenSubscriberCount: Option<bool>,
	/// The number of times the channel has been viewed.
	pub viewCount: Option<i64>,
}

impl Part for ChannelStatistics {}

/// Details about a social network post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsSocial {
	/// The resourceId object encapsulates information that identifies the resource associated with a social network post.
	pub resourceId: Option<ResourceId>,
	/// An image of the post's author.
	pub imageUrl: Option<String>,
	/// The name of the social network.
	pub type_: Option<String>,
	/// The URL of the social network post.
	pub referenceUrl: Option<String>,
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
	pub channelId: Option<String>,
	/// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
	pub playlistId: Option<String>,
	/// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.
	pub videoId: Option<String>,
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
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoCategoryListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource.
	pub items: Vec<VideoCategory>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
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
	pub channelId: Option<String>,
	/// The date and time that the video was uploaded. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// Channel title for the channel responsible for this activity
	pub channelTitle: Option<String>,
	/// The type of activity that the resource describes.
	pub type_: Option<String>,
	/// The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user's activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value.
	pub groupId: Option<String>,
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
	pub timeLeftMs: Option<String>,
	/// The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating:
/// 100 * parts_processed / parts_total
/// 
/// Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.
	pub partsProcessed: Option<String>,
	/// An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.
	pub partsTotal: Option<String>,
}

impl Part for VideoProcessingDetailsProcessingProgress {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SearchListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#searchListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of results that match the search criteria.
	pub items: Vec<SearchResult>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for SearchListResponse {}

/// Freebase topic information related to the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelTopicDetails {
	/// A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.
	pub topicIds: Vec<String>,
}

impl Part for ChannelTopicDetails {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#videoListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of videos that match the request criteria.
	pub items: Vec<Video>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
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
	pub privacyStatus: Option<String>,
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
	pub countryRestriction: Option<AccessPolicy>,
	/// Specifies the ratings that the video received under various rating schemes.
	pub contentRating: Option<ContentRating>,
	/// The value of captions indicates whether the video has captions or not.
	pub caption: Option<String>,
	/// The regionRestriction object contains information about the countries where a video is (or is not) viewable. The object will contain either the contentDetails.regionRestriction.allowed property or the contentDetails.regionRestriction.blocked property.
	pub regionRestriction: Option<VideoContentDetailsRegionRestriction>,
	/// The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.
	pub duration: Option<String>,
	/// The value of is_license_content indicates whether the video is licensed content.
	pub licensedContent: Option<bool>,
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
	pub channelId: Option<String>,
	/// The stream's description. The value cannot be longer than 10000 characters.
	pub description: Option<String>,
	/// The date and time that the stream was created. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
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
	pub privacyStatus: Option<String>,
	/// If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity.
	pub isLinked: Option<bool>,
	/// The long uploads status of this channel. See
	pub longUploadsStatus: Option<String>,
}

impl Part for ChannelStatus {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ChannelListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#channelListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of channels that match the request criteria.
	pub items: Vec<Channel>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for ChannelListResponse {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * update (request|response)
/// * insert (request|response)
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
	pub contentDetails: Option<ChannelSectionContentDetails>,
	/// The ID that YouTube uses to uniquely identify the channel section.
	pub id: Option<String>,
	/// Localizations for different languages
	pub localizations: HashMap<String, ChannelSectionLocalization>,
}

impl RequestResult for ChannelSection {}
impl Resource for ChannelSection {}
impl ResponseResult for ChannelSection {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveBroadcastListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcastListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of broadcasts that match the request criteria.
	pub items: Vec<LiveBroadcast>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for LiveBroadcastListResponse {}

/// Brief description of the live stream status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct LiveStreamStatus {
	/// no description provided
	pub streamStatus: Option<String>,
}

impl Part for LiveStreamStatus {}

/// Details about the live streaming metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoLiveStreamingDetails {
	/// The time that the broadcast is scheduled to begin. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub scheduledStartTime: Option<String>,
	/// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
	pub concurrentViewers: Option<String>,
	/// The time that the broadcast actually started. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast begins.
	pub actualStartTime: Option<String>,
	/// The time that the broadcast is scheduled to end. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.
	pub scheduledEndTime: Option<String>,
	/// The time that the broadcast actually ended. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format. This value will not be available until the broadcast is over.
	pub actualEndTime: Option<String>,
}

impl Part for VideoLiveStreamingDetails {}

/// Ratings schemes. The country-specific ratings are mostly for movies and shows. NEXT_ID: 65
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ContentRating {
	/// Internal YouTube rating.
	pub ytRating: Option<String>,
	/// Rating system for French Canadian TV - Regie du cinema
	pub catvfrRating: Option<String>,
	/// Rating system in India - Central Board of Film Certification
	pub cbfcRating: Option<String>,
	/// Rating system for Thailand - Board of Filmand Video Censors
	pub bfvcRating: Option<String>,
	/// Rating system for Austria - Bundesministeriums f�r Unterricht, Kunst und Kultur!
	pub bmukkRating: Option<String>,
	/// Rating system for Switzerland - Switzerland Rating System
	pub chfilmRating: Option<String>,
	/// Rating system for Taiwan - Ministry of Culture - Tawan
	pub moctwRating: Option<String>,
	/// Rating system for Canadian TV - Canadian TV Classification System
	pub catvRating: Option<String>,
	/// Rating system for Peru - Peru Rating System
	pub pefilmRating: Option<String>,
	/// no description provided
	pub djctqRatingReasons: Vec<String>,
	/// Rating system for Argentina - Instituto Nacional de Cine y Artes Audiovisuales
	pub incaaRating: Option<String>,
	/// Rating system for Israel - Israel Rating System
	pub ilfilmRating: Option<String>,
	/// Rating system for Luxembourg - Commission de surveillance de la classification des films
	pub cscfRating: Option<String>,
	/// Rating system in Germany - Voluntary Self Regulation of the Movie Industry
	pub fskRating: Option<String>,
	/// Rating system in South Korea - Korea Media Rating Board
	pub kmrbRating: Option<String>,
	/// Rating system in Brazil - Department of Justice, Rating, Titles and Qualification
	pub djctqRating: Option<String>,
	/// Rating system for Indonesia - Lembaga Sensor Film
	pub lsfRating: Option<String>,
	/// Rating system for Hong kong - Office for Film, Newspaper and Article Administration
	pub fcoRating: Option<String>,
	/// Rating system for Norway - Medietilsynet
	pub medietilsynetRating: Option<String>,
	/// Rating system for Greece - Greece Rating System
	pub grfilmRating: Option<String>,
	/// Rating system for Chile - Consejo de Calificaci�n Cinematogr�fica
	pub cccRating: Option<String>,
	/// Rating system for Ireland - Raidi� Teilif�s �ireann
	pub rteRating: Option<String>,
	/// Rating system in France - French Minister of Culture
	pub fmocRating: Option<String>,
	/// Rating system for Sweden - Statens medier�d (National Media Council)
	pub smsaRating: Option<String>,
	/// Rating system for Portugal - Comiss�o de Classifica��o de Espect�culos
	pub cceRating: Option<String>,
	/// Rating system for Latvia - National Film Center of Latvia
	pub nkclvRating: Option<String>,
	/// Rating system for South africa - Film & Publication Board
	pub fpbRating: Option<String>,
	/// Rating system for Iceland - SMAIS
	pub smaisRating: Option<String>,
	/// Canadian Home Video Rating System
	pub chvrsRating: Option<String>,
	/// Rating system for Italy - Autorit� per le Garanzie nelle Comunicazioni
	pub agcomRating: Option<String>,
	/// Rating system for Colombia - MoC
	pub mocRating: Option<String>,
	/// Rating system for Hungary - Rating Committee of the National Office of Film
	pub rcnofRating: Option<String>,
	/// Rating system for Malaysia - Film Censorship Board of Malaysia
	pub fcbmRating: Option<String>,
	/// Rating system for Netherlands - Nederlands Instituut voor de Classificatie van Audiovisuele Media
	pub kijkwijzerRating: Option<String>,
	/// Rating system for Singapore - Media Development Authority
	pub mdaRating: Option<String>,
	/// Rating system for Nigeria - National Film and Video Censors Board
	pub nfvcbRating: Option<String>,
	/// Rating system for Venezuela - SiBCI
	pub resorteviolenciaRating: Option<String>,
	/// Rating system for France - Conseil sup�rieur de l?audiovisuel
	pub csaRating: Option<String>,
	/// Rating system in New Zealand - Office of Film and Literature Classification
	pub oflcRating: Option<String>,
	/// TV Parental Guidelines rating of the content.
	pub tvpgRating: Option<String>,
	/// Rating system for Bulgaria - National Film Centre
	pub nfrcRating: Option<String>,
	/// Rating system for Malta - Film Age-Classification Board
	pub mccaaRating: Option<String>,
	/// Rating system in Mexico - General Directorate of Radio, Television and Cinematography
	pub rtcRating: Option<String>,
	/// Rating system in Italy - Ministero dei Beni e delle Attivita Culturali e del Turismo
	pub mibacRating: Option<String>,
	/// British Board of Film Classification
	pub bbfcRating: Option<String>,
	/// Rating system for Egypt - Egypt Rating System
	pub egfilmRating: Option<String>,
	/// Rating system for Belgium - Belgium Rating System
	pub cicfRating: Option<String>,
	/// Rating system for Poland - National Broadcasting Council
	pub nbcplRating: Option<String>,
	/// Rating system for Maldives - National Bureau of Classification
	pub nbcRating: Option<String>,
	/// Motion Picture Association of America rating for the content.
	pub mpaaRating: Option<String>,
	/// Rating system in Ireland - Irish Film Classification Office
	pub ifcoRating: Option<String>,
	/// Rating system in Australia - Australian Classification Board
	pub acbRating: Option<String>,
	/// Rating system for Estonia - Estonia Rating System
	pub eefilmRating: Option<String>,
	/// Rating system for Czech republic - Czech republic Rating System
	pub czfilmRating: Option<String>,
	/// Rating system for Kenya - Kenya Film Classification Board
	pub kfcbRating: Option<String>,
	/// Rating system in Russia
	pub russiaRating: Option<String>,
	/// Rating system for Philippines - MOVIE AND TELEVISION REVIEW AND CLASSIFICATION BOARD
	pub mtrcbRating: Option<String>,
	/// Rating system for Chile - Asociaci�n Nacional de Televisi�n
	pub anatelRating: Option<String>,
	/// Rating system in Japan - Eiga Rinri Kanri Iinkai
	pub eirinRating: Option<String>,
	/// Rating system for Romania - CONSILIUL NATIONAL AL AUDIOVIZUALULUI - CNA
	pub cnaRating: Option<String>,
	/// Rating system in Spain - Instituto de Cinematografia y de las Artes Audiovisuales
	pub icaaRating: Option<String>,
	/// Rating system for Denmark - The Media Council for Children and Young People
	pub mccypRating: Option<String>,
	/// Rating system for Slovakia - Slovakia Rating System
	pub skfilmRating: Option<String>,
	/// Rating system for Finland - Finnish Centre for Media Education and Audiovisual Media
	pub mekuRating: Option<String>,
}

impl Part for ContentRating {}

/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * list (response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityListResponse {
	/// Serialized EventId of the request which produced this response.
	pub eventId: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
	pub nextPageToken: Option<String>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#activityListResponse".
	pub kind: Option<String>,
	/// The visitorId identifies the visitor.
	pub visitorId: Option<String>,
	/// A list of activities, or events, that match the request criteria.
	pub items: Vec<Activity>,
	/// no description provided
	pub tokenPagination: Option<TokenPagination>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
	pub prevPageToken: Option<String>,
	/// no description provided
	pub pageInfo: Option<PageInfo>,
}

impl ResponseResult for ActivityListResponse {}

/// An activity resource contains information about an action that a particular channel, or user, has taken on YouTube.The actions reported in activity feeds include rating a video, sharing a video, marking a video as a favorite, commenting on a video, uploading a video, and so forth. Each activity resource identifies the type of action, the channel associated with the action, and the resource(s) associated with the action, such as the video that was rated or uploaded.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * insert (request|response)
/// 
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct Activity {
	/// The snippet object contains basic details about the activity, including the activity's type and group ID.
	pub snippet: Option<ActivitySnippet>,
	/// The contentDetails object contains information about the content associated with the activity. For example, if the snippet.type value is videoRated, then the contentDetails object's content identifies the rated video.
	pub contentDetails: Option<ActivityContentDetails>,
	/// Identifies what kind of resource this is. Value: the fixed string "youtube#activity".
	pub kind: Option<String>,
	/// Etag of this resource.
	pub etag: Option<String>,
	/// The ID that YouTube uses to uniquely identify the activity.
	pub id: Option<String>,
}

impl RequestResult for Activity {}
impl ResponseResult for Activity {}

/// Basic details about a subscription's subscriber including title, description, channel ID and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SubscriptionSubscriberSnippet {
	/// The channel ID of the subscriber.
	pub channelId: Option<String>,
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
	pub bannerTvMediumImageUrl: Option<String>,
	/// Banner image. Tablet size low resolution (1138x188).
	pub bannerTabletLowImageUrl: Option<String>,
	/// The image map script for the large banner image.
	pub largeBrandedBannerImageImapScript: Option<LocalizedProperty>,
	/// Banner image. Mobile size (640x175).
	pub bannerMobileImageUrl: Option<String>,
	/// The URL for the 640px by 70px banner image that appears below the video player in the default view of the video watch page.
	pub smallBrandedBannerImageUrl: Option<LocalizedProperty>,
	/// Banner image. Tablet size high resolution (2276x377).
	pub bannerTabletHdImageUrl: Option<String>,
	/// Banner image. TV size high resolution (1920x1080).
	pub bannerTvHighImageUrl: Option<String>,
	/// Banner image. Mobile size medium/high resolution (960x263).
	pub bannerMobileMediumHdImageUrl: Option<String>,
	/// The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.
	pub trackingImageUrl: Option<String>,
	/// Banner image. Mobile size high resolution (1440x395).
	pub bannerMobileExtraHdImageUrl: Option<String>,
	/// Banner image. Tablet size (1707x283).
	pub bannerTabletImageUrl: Option<String>,
	/// Banner image. Mobile size low resolution (320x88).
	pub bannerMobileLowImageUrl: Option<String>,
	/// Banner image. TV size extra high resolution (2120x1192).
	pub bannerTvImageUrl: Option<String>,
	/// Banner image. TV size low resolution (854x480).
	pub bannerTvLowImageUrl: Option<String>,
	/// Banner image. Tablet size extra high resolution (2560x424).
	pub bannerTabletExtraHdImageUrl: Option<String>,
	/// The URL for the 854px by 70px image that appears below the video player in the expanded video view of the video watch page.
	pub largeBrandedBannerImageUrl: Option<LocalizedProperty>,
	/// Banner image. Desktop size (1060x175).
	pub bannerImageUrl: Option<String>,
	/// The URL for the background image shown on the video watch page. The image should be 1200px by 615px, with a maximum file size of 128k.
	pub backgroundImageUrl: Option<LocalizedProperty>,
	/// The image map script for the small banner image.
	pub smallBrandedBannerImageImapScript: Option<LocalizedProperty>,
	/// Banner image. Mobile size high resolution (1280x360).
	pub bannerMobileHdImageUrl: Option<String>,
	/// This is used only in update requests; if it's set, we use this URL to generate all of the above banner URLs.
	pub bannerExternalUrl: Option<String>,
	/// The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels.
	pub watchIconImageUrl: Option<String>,
}

impl Part for ImageSettings {}

/// Details about a resource which is being promoted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct ActivityContentDetailsPromotedItem {
	/// The type of call-to-action, a message to the user indicating action that can be taken.
	pub ctaType: Option<String>,
	/// The URL the client should fetch to request a promoted item.
	pub adTag: Option<String>,
	/// The URL the client should direct the user to, if the user chooses to visit the advertiser's website.
	pub destinationUrl: Option<String>,
	/// The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown.
	pub forecastingUrl: Vec<String>,
	/// The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item.
	pub impressionUrl: Vec<String>,
	/// The URL the client should ping to indicate that the user was shown this promoted item.
	pub creativeViewUrl: Option<String>,
	/// The ID that YouTube uses to uniquely identify the promoted video.
	pub videoId: Option<String>,
	/// The text description to accompany the promoted item.
	pub descriptionText: Option<String>,
	/// The custom call-to-action button text. If specified, it will override the default button text for the cta_type.
	pub customCtaButtonText: Option<String>,
	/// The URL the client should ping to indicate that the user clicked through on this promoted item.
	pub clickTrackingUrl: Option<String>,
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
	pub resourceId: Option<ResourceId>,
}

impl Part for ActivityContentDetailsBulletin {}

/// An i18nLanguage resource identifies a UI language currently supported by YouTube.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
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

impl Part for I18nLanguage {}

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
	pub bitrateBps: Option<String>,
	/// The audio codec that the stream uses.
	pub codec: Option<String>,
	/// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
	pub vendor: Option<String>,
	/// The number of audio channels that the stream contains.
	pub channelCount: Option<u32>,
}

impl Part for VideoFileDetailsAudioStream {}

/// Freebase topic information related to the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoTopicDetails {
	/// A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.
	pub topicIds: Vec<String>,
	/// Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.
	pub relevantTopicIds: Vec<String>,
}

impl Part for VideoTopicDetails {}

/// Describes information necessary for ingesting an RTMP or an HTTP stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct IngestionInfo {
	/// The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL.
	pub backupIngestionAddress: Option<String>,
	/// The HTTP or RTMP stream name that YouTube assigns to the video stream.
	pub streamName: Option<String>,
	/// The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL.
/// 
/// Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format:
/// 
/// STREAM_URL/STREAM_NAME
	pub ingestionAddress: Option<String>,
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
	pub conversionUrl: Option<String>,
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
	pub channelId: Option<String>,
	/// The item's description.
	pub description: Option<String>,
	/// The item's title.
	pub title: Option<String>,
	/// The id object contains information that can be used to uniquely identify the resource that is included in the playlist as the playlist item.
	pub resourceId: Option<ResourceId>,
	/// The ID that YouTube uses to uniquely identify the playlist that the playlist item is in.
	pub playlistId: Option<String>,
	/// The date and time that the item was added to the playlist. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// Channel title for the channel that the playlist item belongs to.
	pub channelTitle: Option<String>,
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
	pub resourceId: Option<ResourceId>,
}

impl Part for ActivityContentDetailsFavorite {}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistPlayer {
	/// An <iframe> tag that embeds a player that will play the playlist.
	pub embedHtml: Option<String>,
}

impl Part for PlaylistPlayer {}

/// A single tag suggestion with it's relevance information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct VideoSuggestionsTagSuggestion {
	/// A set of video categories for which the tag is relevant. You can use this information to display appropriate tag suggestions based on the video category that the video uploader associates with the video. By default, tag suggestions are relevant for all categories if there are no restricts defined for the keyword.
	pub categoryRestricts: Vec<String>,
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
	pub processingErrors: Vec<String>,
	/// A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.
	pub tagSuggestions: Vec<VideoSuggestionsTagSuggestion>,
	/// A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
	pub editorSuggestions: Vec<String>,
	/// A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
	pub processingWarnings: Vec<String>,
	/// A list of suggestions that may improve YouTube's ability to process the video.
	pub processingHints: Vec<String>,
}

impl Part for VideoSuggestions {}

/// Basic details about a search result, including title, description and thumbnails of the item referenced by the search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct SearchResultSnippet {
	/// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
	pub liveBroadcastContent: Option<String>,
	/// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
	pub thumbnails: Option<ThumbnailDetails>,
	/// The title of the search result.
	pub title: Option<String>,
	/// A description of the search result.
	pub description: Option<String>,
	/// The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies.
	pub channelId: Option<String>,
	/// The creation date and time of the resource that the search result identifies. The value is specified in ISO 8601 (YYYY-MM-DDThh:mm:ss.sZ) format.
	pub publishedAt: Option<String>,
	/// The title of the channel that published the resource that the search result identifies.
	pub channelTitle: Option<String>,
}

impl Part for SearchResultSnippet {}

/// A channel banner returned as the response to a channel_banner.insert call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * insert (request|response)
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

impl RequestResult for ChannelBannerResource {}
impl ResponseResult for ChannelBannerResource {}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PlaylistContentDetails {
	/// The number of videos in the playlist.
	pub itemCount: Option<u32>,
}

impl Part for PlaylistContentDetails {}

/// Paging details for lists of resources, including total number of items available and number of resources returned in a single page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(RustcEncodable, RustcDecodable, Default, Clone)]
pub struct PageInfo {
	/// The number of results included in the API response.
	pub resultsPerPage: Option<i32>,
	/// The total number of results in the result set.
	pub totalResults: Option<i32>,
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
pub struct ChannelContentDetailsRelatedplaylists {
	/// The ID of the playlist that contains the channel"s uploaded videos. Use the  videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos.
	pub uploads: Option<String>,
	/// The ID of the playlist that contains the channel"s watch history. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub watchHistory: Option<String>,
	/// The ID of the playlist that contains the channel"s liked videos. Use the   playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub likes: Option<String>,
	/// The ID of the playlist that contains the channel"s favorite videos. Use the  playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub favorites: Option<String>,
	/// The ID of the playlist that contains the channel"s watch later playlist. Use the playlistItems.insert and  playlistItems.delete to add or remove items from that list.
	pub watchLater: Option<String>,
}

impl NestedType for ChannelContentDetailsRelatedplaylists {}
impl Part for ChannelContentDetailsRelatedplaylists {}

