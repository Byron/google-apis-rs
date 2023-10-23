use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert abuse reports](AbuseReportInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbuseReport {
    /// no description provided
    #[serde(rename="abuseTypes")]
    
    pub abuse_types: Option<Vec<AbuseType>>,
    /// no description provided
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="relatedEntities")]
    
    pub related_entities: Option<Vec<RelatedEntity>>,
    /// no description provided
    
    pub subject: Option<Entity>,
}

impl client::RequestValue for AbuseReport {}
impl client::Resource for AbuseReport {}
impl client::ResponseResult for AbuseReport {}

impl client::ToParts for AbuseReport {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.abuse_types.is_some() { r = r + "abuseTypes,"; }
        if self.description.is_some() { r = r + "description,"; }
        if self.related_entities.is_some() { r = r + "relatedEntities,"; }
        if self.subject.is_some() { r = r + "subject,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AbuseType {
    /// no description provided
    
    pub id: Option<String>,
}

impl client::Part for AbuseType {}


/// Rights management policy for YouTube resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// The value of allowed indicates whether the access to the policy is allowed or denied by default.
    
    pub allowed: Option<bool>,
    /// A list of region codes that identify countries where the default policy do not apply.
    
    pub exception: Option<Vec<String>>,
}

impl client::Part for AccessPolicy {}


/// An *activity* resource contains information about an action that a particular channel, or user, has taken on YouTube.The actions reported in activity feeds include rating a video, sharing a video, marking a video as a favorite, commenting on a video, uploading a video, and so forth. Each activity resource identifies the type of action, the channel associated with the action, and the resource(s) associated with the action, such as the video that was rated or uploaded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    /// The contentDetails object contains information about the content associated with the activity. For example, if the snippet.type value is videoRated, then the contentDetails object's content identifies the rated video.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<ActivityContentDetails>,
    /// Etag of this resource
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the activity.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#activity".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the activity, including the activity's type and group ID.
    
    pub snippet: Option<ActivitySnippet>,
}

impl client::Part for Activity {}


/// Details about the content of an activity: the video that was shared, the channel that was subscribed to, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetails {
    /// The bulletin object contains details about a channel bulletin post. This object is only present if the snippet.type is bulletin.
    
    pub bulletin: Option<ActivityContentDetailsBulletin>,
    /// The channelItem object contains details about a resource which was added to a channel. This property is only present if the snippet.type is channelItem.
    #[serde(rename="channelItem")]
    
    pub channel_item: Option<ActivityContentDetailsChannelItem>,
    /// The comment object contains information about a resource that received a comment. This property is only present if the snippet.type is comment.
    
    pub comment: Option<ActivityContentDetailsComment>,
    /// The favorite object contains information about a video that was marked as a favorite video. This property is only present if the snippet.type is favorite.
    
    pub favorite: Option<ActivityContentDetailsFavorite>,
    /// The like object contains information about a resource that received a positive (like) rating. This property is only present if the snippet.type is like.
    
    pub like: Option<ActivityContentDetailsLike>,
    /// The playlistItem object contains information about a new playlist item. This property is only present if the snippet.type is playlistItem.
    #[serde(rename="playlistItem")]
    
    pub playlist_item: Option<ActivityContentDetailsPlaylistItem>,
    /// The promotedItem object contains details about a resource which is being promoted. This property is only present if the snippet.type is promotedItem.
    #[serde(rename="promotedItem")]
    
    pub promoted_item: Option<ActivityContentDetailsPromotedItem>,
    /// The recommendation object contains information about a recommended resource. This property is only present if the snippet.type is recommendation.
    
    pub recommendation: Option<ActivityContentDetailsRecommendation>,
    /// The social object contains details about a social network post. This property is only present if the snippet.type is social.
    
    pub social: Option<ActivityContentDetailsSocial>,
    /// The subscription object contains information about a channel that a user subscribed to. This property is only present if the snippet.type is subscription.
    
    pub subscription: Option<ActivityContentDetailsSubscription>,
    /// The upload object contains information about the uploaded video. This property is only present if the snippet.type is upload.
    
    pub upload: Option<ActivityContentDetailsUpload>,
}

impl client::Part for ActivityContentDetails {}


/// Details about a channel bulletin post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsBulletin {
    /// The resourceId object contains information that identifies the resource associated with a bulletin post. @mutable youtube.activities.insert
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsBulletin {}


/// Details about a resource which was added to a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsChannelItem {
    /// The resourceId object contains information that identifies the resource that was added to the channel.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsChannelItem {}


/// Information about a resource that received a comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsComment {
    /// The resourceId object contains information that identifies the resource associated with the comment.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsComment {}


/// Information about a video that was marked as a favorite video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsFavorite {
    /// The resourceId object contains information that identifies the resource that was marked as a favorite.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsFavorite {}


/// Information about a resource that received a positive (like) rating.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsLike {
    /// The resourceId object contains information that identifies the rated resource.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsLike {}


/// Information about a new playlist item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsPlaylistItem {
    /// The value that YouTube uses to uniquely identify the playlist.
    #[serde(rename="playlistId")]
    
    pub playlist_id: Option<String>,
    /// ID of the item within the playlist.
    #[serde(rename="playlistItemId")]
    
    pub playlist_item_id: Option<String>,
    /// The resourceId object contains information about the resource that was added to the playlist.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsPlaylistItem {}


/// Details about a resource which is being promoted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsPromotedItem {
    /// The URL the client should fetch to request a promoted item.
    #[serde(rename="adTag")]
    
    pub ad_tag: Option<String>,
    /// The URL the client should ping to indicate that the user clicked through on this promoted item.
    #[serde(rename="clickTrackingUrl")]
    
    pub click_tracking_url: Option<String>,
    /// The URL the client should ping to indicate that the user was shown this promoted item.
    #[serde(rename="creativeViewUrl")]
    
    pub creative_view_url: Option<String>,
    /// The type of call-to-action, a message to the user indicating action that can be taken.
    #[serde(rename="ctaType")]
    
    pub cta_type: Option<ActivityContentDetailsPromotedItemCtaTypeEnum>,
    /// The custom call-to-action button text. If specified, it will override the default button text for the cta_type.
    #[serde(rename="customCtaButtonText")]
    
    pub custom_cta_button_text: Option<String>,
    /// The text description to accompany the promoted item.
    #[serde(rename="descriptionText")]
    
    pub description_text: Option<String>,
    /// The URL the client should direct the user to, if the user chooses to visit the advertiser's website.
    #[serde(rename="destinationUrl")]
    
    pub destination_url: Option<String>,
    /// The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown.
    #[serde(rename="forecastingUrl")]
    
    pub forecasting_url: Option<Vec<String>>,
    /// The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item.
    #[serde(rename="impressionUrl")]
    
    pub impression_url: Option<Vec<String>>,
    /// The ID that YouTube uses to uniquely identify the promoted video.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for ActivityContentDetailsPromotedItem {}


/// Information that identifies the recommended resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsRecommendation {
    /// The reason that the resource is recommended to the user.
    
    pub reason: Option<ActivityContentDetailsRecommendationReasonEnum>,
    /// The resourceId object contains information that identifies the recommended resource.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
    /// The seedResourceId object contains information about the resource that caused the recommendation.
    #[serde(rename="seedResourceId")]
    
    pub seed_resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsRecommendation {}


/// Details about a social network post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsSocial {
    /// The author of the social network post.
    
    pub author: Option<String>,
    /// An image of the post's author.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// The URL of the social network post.
    #[serde(rename="referenceUrl")]
    
    pub reference_url: Option<String>,
    /// The resourceId object encapsulates information that identifies the resource associated with a social network post.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
    /// The name of the social network.
    #[serde(rename="type")]
    
    pub type_: Option<ActivityContentDetailsSocialTypeEnum>,
}

impl client::Part for ActivityContentDetailsSocial {}


/// Information about a channel that a user subscribed to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsSubscription {
    /// The resourceId object contains information that identifies the resource that the user subscribed to.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
}

impl client::Part for ActivityContentDetailsSubscription {}


/// Information about the uploaded video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityContentDetailsUpload {
    /// The ID that YouTube uses to uniquely identify the uploaded video.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for ActivityContentDetailsUpload {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](ActivityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<Activity>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#activityListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for ActivityListResponse {}

impl client::ToParts for ActivityListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about an activity, including title, description, thumbnails, activity type and group. Next ID: 12
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivitySnippet {
    /// The ID that YouTube uses to uniquely identify the channel associated with the activity.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Channel title for the channel responsible for this activity
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// The description of the resource primarily associated with the activity. @mutable youtube.activities.insert
    
    pub description: Option<String>,
    /// The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user's activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value.
    #[serde(rename="groupId")]
    
    pub group_id: Option<String>,
    /// The date and time that the video was uploaded.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map of thumbnail images associated with the resource that is primarily associated with the activity. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The title of the resource primarily associated with the activity.
    
    pub title: Option<String>,
    /// The type of activity that the resource describes.
    #[serde(rename="type")]
    
    pub type_: Option<ActivitySnippetTypeEnum>,
}

impl client::Part for ActivitySnippet {}


/// A *caption* resource represents a YouTube caption track. A caption track is associated with exactly one YouTube video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete captions](CaptionDeleteCall) (none)
/// * [download captions](CaptionDownloadCall) (none)
/// * [insert captions](CaptionInsertCall) (request|response)
/// * [list captions](CaptionListCall) (none)
/// * [update captions](CaptionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Caption {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the caption track.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#caption".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the caption.
    
    pub snippet: Option<CaptionSnippet>,
}

impl client::RequestValue for Caption {}
impl client::Resource for Caption {}
impl client::ResponseResult for Caption {}

impl client::ToParts for Caption {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list captions](CaptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaptionListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of captions that match the request criteria.
    
    pub items: Option<Vec<Caption>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#captionListResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for CaptionListResponse {}

impl client::ToParts for CaptionListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a caption track, such as its language and name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaptionSnippet {
    /// The type of audio track associated with the caption track.
    #[serde(rename="audioTrackType")]
    
    pub audio_track_type: Option<CaptionSnippetAudioTrackTypeEnum>,
    /// The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<CaptionSnippetFailureReasonEnum>,
    /// Indicates whether YouTube synchronized the caption track to the audio track in the video. The value will be true if a sync was explicitly requested when the caption track was uploaded. For example, when calling the captions.insert or captions.update methods, you can set the sync parameter to true to instruct YouTube to sync the uploaded track to the video. If the value is false, YouTube uses the time codes in the uploaded caption track to determine when to display captions.
    #[serde(rename="isAutoSynced")]
    
    pub is_auto_synced: Option<bool>,
    /// Indicates whether the track contains closed captions for the deaf and hard of hearing. The default value is false.
    #[serde(rename="isCC")]
    
    pub is_cc: Option<bool>,
    /// Indicates whether the caption track is a draft. If the value is true, then the track is not publicly visible. The default value is false. @mutable youtube.captions.insert youtube.captions.update
    #[serde(rename="isDraft")]
    
    pub is_draft: Option<bool>,
    /// Indicates whether caption track is formatted for "easy reader," meaning it is at a third-grade level for language learners. The default value is false.
    #[serde(rename="isEasyReader")]
    
    pub is_easy_reader: Option<bool>,
    /// Indicates whether the caption track uses large text for the vision-impaired. The default value is false.
    #[serde(rename="isLarge")]
    
    pub is_large: Option<bool>,
    /// The language of the caption track. The property value is a BCP-47 language tag.
    
    pub language: Option<String>,
    /// The date and time when the caption track was last updated.
    #[serde(rename="lastUpdated")]
    
    pub last_updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The name of the caption track. The name is intended to be visible to the user as an option during playback.
    
    pub name: Option<String>,
    /// The caption track's status.
    
    pub status: Option<CaptionSnippetStatusEnum>,
    /// The caption track's type.
    #[serde(rename="trackKind")]
    
    pub track_kind: Option<CaptionSnippetTrackKindEnum>,
    /// The ID that YouTube uses to uniquely identify the video associated with the caption track. @mutable youtube.captions.insert
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for CaptionSnippet {}


/// Brief description of the live stream cdn settings.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CdnSettings {
    /// The format of the video stream that you are sending to Youtube. 
    
    pub format: Option<String>,
    /// The frame rate of the inbound video data.
    #[serde(rename="frameRate")]
    
    pub frame_rate: Option<CdnSettingFrameRateEnum>,
    /// The ingestionInfo object contains information that YouTube provides that you need to transmit your RTMP or HTTP stream to YouTube.
    #[serde(rename="ingestionInfo")]
    
    pub ingestion_info: Option<IngestionInfo>,
    ///  The method or protocol used to transmit the video stream.
    #[serde(rename="ingestionType")]
    
    pub ingestion_type: Option<CdnSettingIngestionTypeEnum>,
    /// The resolution of the inbound video data.
    
    pub resolution: Option<CdnSettingResolutionEnum>,
}

impl client::Part for CdnSettings {}


/// A *channel* resource contains information about a YouTube channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channels](ChannelListCall) (none)
/// * [update channels](ChannelUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process.
    #[serde(rename="auditDetails")]
    
    pub audit_details: Option<ChannelAuditDetails>,
    /// The brandingSettings object encapsulates information about the branding of the channel.
    #[serde(rename="brandingSettings")]
    
    pub branding_settings: Option<ChannelBrandingSettings>,
    /// The contentDetails object encapsulates information about the channel's content.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<ChannelContentDetails>,
    /// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
    #[serde(rename="contentOwnerDetails")]
    
    pub content_owner_details: Option<ChannelContentOwnerDetails>,
    /// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
    #[serde(rename="conversionPings")]
    
    pub conversion_pings: Option<ChannelConversionPings>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the channel.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channel".
    
    pub kind: Option<String>,
    /// Localizations for different languages
    
    pub localizations: Option<HashMap<String, ChannelLocalization>>,
    /// The snippet object contains basic details about the channel, such as its title, description, and thumbnail images.
    
    pub snippet: Option<ChannelSnippet>,
    /// The statistics object encapsulates statistics for the channel.
    
    pub statistics: Option<ChannelStatistics>,
    /// The status object encapsulates information about the privacy status of the channel.
    
    pub status: Option<ChannelStatus>,
    /// The topicDetails object encapsulates information about Freebase topics associated with the channel.
    #[serde(rename="topicDetails")]
    
    pub topic_details: Option<ChannelTopicDetails>,
}

impl client::RequestValue for Channel {}
impl client::Resource for Channel {}
impl client::ResponseResult for Channel {}

impl client::ToParts for Channel {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.audit_details.is_some() { r = r + "auditDetails,"; }
        if self.branding_settings.is_some() { r = r + "brandingSettings,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.content_owner_details.is_some() { r = r + "contentOwnerDetails,"; }
        if self.conversion_pings.is_some() { r = r + "conversionPings,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.statistics.is_some() { r = r + "statistics,"; }
        if self.status.is_some() { r = r + "status,"; }
        if self.topic_details.is_some() { r = r + "topicDetails,"; }
        r.pop();
        r
    }
}

/// The auditDetails object encapsulates channel data that is relevant for YouTube Partners during the audit process.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelAuditDetails {
    /// Whether or not the channel respects the community guidelines.
    #[serde(rename="communityGuidelinesGoodStanding")]
    
    pub community_guidelines_good_standing: Option<bool>,
    /// Whether or not the channel has any unresolved claims.
    #[serde(rename="contentIdClaimsGoodStanding")]
    
    pub content_id_claims_good_standing: Option<bool>,
    /// Whether or not the channel has any copyright strikes.
    #[serde(rename="copyrightStrikesGoodStanding")]
    
    pub copyright_strikes_good_standing: Option<bool>,
}

impl client::Part for ChannelAuditDetails {}


/// A channel banner returned as the response to a channel_banner.insert call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert channel banners](ChannelBannerInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelBannerResource {
    /// no description provided
    
    pub etag: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelBannerResource".
    
    pub kind: Option<String>,
    /// The URL of this banner image.
    
    pub url: Option<String>,
}

impl client::RequestValue for ChannelBannerResource {}
impl client::ResponseResult for ChannelBannerResource {}


/// Branding properties of a YouTube channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelBrandingSettings {
    /// Branding properties for the channel view.
    
    pub channel: Option<ChannelSettings>,
    /// Additional experimental branding properties.
    
    pub hints: Option<Vec<PropertyValue>>,
    /// Branding properties for branding images.
    
    pub image: Option<ImageSettings>,
    /// Branding properties for the watch page.
    
    pub watch: Option<WatchSettings>,
}

impl client::Part for ChannelBrandingSettings {}


/// Details about the content of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentDetails {
    /// no description provided
    #[serde(rename="relatedPlaylists")]
    
    pub related_playlists: Option<ChannelContentDetailsRelatedPlaylists>,
}

impl client::Part for ChannelContentDetails {}


/// The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentOwnerDetails {
    /// The ID of the content owner linked to the channel.
    #[serde(rename="contentOwner")]
    
    pub content_owner: Option<String>,
    /// The date and time when the channel was linked to the content owner.
    #[serde(rename="timeLinked")]
    
    pub time_linked: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for ChannelContentOwnerDetails {}


/// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelConversionPing {
    /// Defines the context of the ping.
    
    pub context: Option<ChannelConversionPingContextEnum>,
    /// The url (without the schema) that the player shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D cview%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping.
    #[serde(rename="conversionUrl")]
    
    pub conversion_url: Option<String>,
}

impl client::Part for ChannelConversionPing {}


/// The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelConversionPings {
    /// Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.
    
    pub pings: Option<Vec<ChannelConversionPing>>,
}

impl client::Part for ChannelConversionPings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channels](ChannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<Channel>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for ChannelListResponse {}

impl client::ToParts for ChannelListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Channel localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelLocalization {
    /// The localized strings for channel's description.
    
    pub description: Option<String>,
    /// The localized strings for channel's title.
    
    pub title: Option<String>,
}

impl client::Part for ChannelLocalization {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelProfileDetails {
    /// The YouTube channel ID.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The channel's URL.
    #[serde(rename="channelUrl")]
    
    pub channel_url: Option<String>,
    /// The channel's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The channels's avatar URL.
    #[serde(rename="profileImageUrl")]
    
    pub profile_image_url: Option<String>,
}

impl client::Part for ChannelProfileDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete channel sections](ChannelSectionDeleteCall) (none)
/// * [insert channel sections](ChannelSectionInsertCall) (request|response)
/// * [list channel sections](ChannelSectionListCall) (none)
/// * [update channel sections](ChannelSectionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSection {
    /// The contentDetails object contains details about the channel section content, such as a list of playlists or channels featured in the section.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<ChannelSectionContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the channel section.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection".
    
    pub kind: Option<String>,
    /// Localizations for different languages
    
    pub localizations: Option<HashMap<String, ChannelSectionLocalization>>,
    /// The snippet object contains basic details about the channel section, such as its type, style and title.
    
    pub snippet: Option<ChannelSectionSnippet>,
    /// The targeting object contains basic targeting settings about the channel section.
    
    pub targeting: Option<ChannelSectionTargeting>,
}

impl client::RequestValue for ChannelSection {}
impl client::Resource for ChannelSection {}
impl client::ResponseResult for ChannelSection {}

impl client::ToParts for ChannelSection {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.targeting.is_some() { r = r + "targeting,"; }
        r.pop();
        r
    }
}

/// Details about a channelsection, including playlists and channels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionContentDetails {
    /// The channel ids for type multiple_channels.
    
    pub channels: Option<Vec<String>>,
    /// The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed.
    
    pub playlists: Option<Vec<String>>,
}

impl client::Part for ChannelSectionContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list channel sections](ChannelSectionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of ChannelSections that match the request criteria.
    
    pub items: Option<Vec<ChannelSection>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#channelSectionListResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for ChannelSectionListResponse {}

impl client::ToParts for ChannelSectionListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// ChannelSection localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionLocalization {
    /// The localized strings for channel section's title.
    
    pub title: Option<String>,
}

impl client::Part for ChannelSectionLocalization {}


/// Basic details about a channel section, including title, style and position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionSnippet {
    /// The ID that YouTube uses to uniquely identify the channel that published the channel section.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The language of the channel section's default title and description.
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// Localized title, read-only.
    
    pub localized: Option<ChannelSectionLocalization>,
    /// The position of the channel section in the channel.
    
    pub position: Option<u32>,
    /// The style of the channel section.
    
    pub style: Option<ChannelSectionSnippetStyleEnum>,
    /// The channel section's title for multiple_playlists and multiple_channels.
    
    pub title: Option<String>,
    /// The type of the channel section.
    #[serde(rename="type")]
    
    pub type_: Option<ChannelSectionSnippetTypeEnum>,
}

impl client::Part for ChannelSectionSnippet {}


/// ChannelSection targeting setting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSectionTargeting {
    /// The country the channel section is targeting.
    
    pub countries: Option<Vec<String>>,
    /// The language the channel section is targeting.
    
    pub languages: Option<Vec<String>>,
    /// The region the channel section is targeting.
    
    pub regions: Option<Vec<String>>,
}

impl client::Part for ChannelSectionTargeting {}


/// Branding properties for the channel view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSettings {
    /// The country of the channel.
    
    pub country: Option<String>,
    /// no description provided
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// Which content tab users should see when viewing the channel.
    #[serde(rename="defaultTab")]
    
    pub default_tab: Option<String>,
    /// Specifies the channel description.
    
    pub description: Option<String>,
    /// Title for the featured channels tab.
    #[serde(rename="featuredChannelsTitle")]
    
    pub featured_channels_title: Option<String>,
    /// The list of featured channels.
    #[serde(rename="featuredChannelsUrls")]
    
    pub featured_channels_urls: Option<Vec<String>>,
    /// Lists keywords associated with the channel, comma-separated.
    
    pub keywords: Option<String>,
    /// Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible.
    #[serde(rename="moderateComments")]
    
    pub moderate_comments: Option<bool>,
    /// A prominent color that can be rendered on this channel page.
    #[serde(rename="profileColor")]
    
    pub profile_color: Option<String>,
    /// Whether the tab to browse the videos should be displayed.
    #[serde(rename="showBrowseView")]
    
    pub show_browse_view: Option<bool>,
    /// Whether related channels should be proposed.
    #[serde(rename="showRelatedChannels")]
    
    pub show_related_channels: Option<bool>,
    /// Specifies the channel title.
    
    pub title: Option<String>,
    /// The ID for a Google Analytics account to track and measure traffic to the channels.
    #[serde(rename="trackingAnalyticsAccountId")]
    
    pub tracking_analytics_account_id: Option<String>,
    /// The trailer of the channel, for users that are not subscribers.
    #[serde(rename="unsubscribedTrailer")]
    
    pub unsubscribed_trailer: Option<String>,
}

impl client::Part for ChannelSettings {}


/// Basic details about a channel, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelSnippet {
    /// The country of the channel.
    
    pub country: Option<String>,
    /// The custom url of the channel.
    #[serde(rename="customUrl")]
    
    pub custom_url: Option<String>,
    /// The language of the channel's default title and description.
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// The description of the channel.
    
    pub description: Option<String>,
    /// Localized title and description, read-only.
    
    pub localized: Option<ChannelLocalization>,
    /// The date and time that the channel was created.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map of thumbnail images associated with the channel. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail. When displaying thumbnails in your application, make sure that your code uses the image URLs exactly as they are returned in API responses. For example, your application should not use the http domain instead of the https domain in a URL returned in an API response. Beginning in July 2018, channel thumbnail URLs will only be available in the https domain, which is how the URLs appear in API responses. After that time, you might see broken images in your application if it tries to load YouTube images from the http domain. Thumbnail images might be empty for newly created channels and might take up to one day to populate.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The channel's title.
    
    pub title: Option<String>,
}

impl client::Part for ChannelSnippet {}


/// Statistics about a channel: number of subscribers, number of videos in the channel, etc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelStatistics {
    /// The number of comments for the channel.
    #[serde(rename="commentCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub comment_count: Option<u64>,
    /// Whether or not the number of subscribers is shown for this user.
    #[serde(rename="hiddenSubscriberCount")]
    
    pub hidden_subscriber_count: Option<bool>,
    /// The number of subscribers that the channel has.
    #[serde(rename="subscriberCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub subscriber_count: Option<u64>,
    /// The number of videos uploaded to the channel.
    #[serde(rename="videoCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub video_count: Option<u64>,
    /// The number of times the channel has been viewed.
    #[serde(rename="viewCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub view_count: Option<u64>,
}

impl client::Part for ChannelStatistics {}


/// JSON template for the status part of a channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelStatus {
    /// If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity.
    #[serde(rename="isLinked")]
    
    pub is_linked: Option<bool>,
    /// The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information.
    #[serde(rename="longUploadsStatus")]
    
    pub long_uploads_status: Option<ChannelStatuLongUploadsStatusEnum>,
    /// no description provided
    #[serde(rename="madeForKids")]
    
    pub made_for_kids: Option<bool>,
    /// Privacy status of the channel.
    #[serde(rename="privacyStatus")]
    
    pub privacy_status: Option<ChannelStatuPrivacyStatusEnum>,
    /// no description provided
    #[serde(rename="selfDeclaredMadeForKids")]
    
    pub self_declared_made_for_kids: Option<bool>,
}

impl client::Part for ChannelStatus {}


/// Information specific to a store on a merchandising platform linked to a YouTube channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelToStoreLinkDetails {
    /// Google Merchant Center id of the store.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<u64>,
    /// Name of the store.
    #[serde(rename="storeName")]
    
    pub store_name: Option<String>,
    /// Landing page of the store.
    #[serde(rename="storeUrl")]
    
    pub store_url: Option<String>,
}

impl client::Part for ChannelToStoreLinkDetails {}


/// Freebase topic information related to the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelTopicDetails {
    /// A list of Wikipedia URLs that describe the channel's content.
    #[serde(rename="topicCategories")]
    
    pub topic_categories: Option<Vec<String>>,
    /// A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.
    #[serde(rename="topicIds")]
    
    pub topic_ids: Option<Vec<String>>,
}

impl client::Part for ChannelTopicDetails {}


/// A *comment* represents a single YouTube comment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete comments](CommentDeleteCall) (none)
/// * [insert comments](CommentInsertCall) (request|response)
/// * [list comments](CommentListCall) (none)
/// * [mark as spam comments](CommentMarkAsSpamCall) (none)
/// * [set moderation status comments](CommentSetModerationStatuCall) (none)
/// * [update comments](CommentUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the comment.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#comment".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the comment.
    
    pub snippet: Option<CommentSnippet>,
}

impl client::RequestValue for Comment {}
impl client::Resource for Comment {}
impl client::ResponseResult for Comment {}

impl client::ToParts for Comment {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list comments](CommentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of comments that match the request criteria.
    
    pub items: Option<Vec<Comment>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#commentListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for CommentListResponse {}

impl client::ToParts for CommentListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a comment, such as its author and text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentSnippet {
    /// no description provided
    #[serde(rename="authorChannelId")]
    
    pub author_channel_id: Option<CommentSnippetAuthorChannelId>,
    /// Link to the author's YouTube channel, if any.
    #[serde(rename="authorChannelUrl")]
    
    pub author_channel_url: Option<String>,
    /// The name of the user who posted the comment.
    #[serde(rename="authorDisplayName")]
    
    pub author_display_name: Option<String>,
    /// The URL for the avatar of the user who posted the comment.
    #[serde(rename="authorProfileImageUrl")]
    
    pub author_profile_image_url: Option<String>,
    /// Whether the current viewer can rate this comment.
    #[serde(rename="canRate")]
    
    pub can_rate: Option<bool>,
    /// The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video comment it's the video's channel.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The total number of likes this comment has received.
    #[serde(rename="likeCount")]
    
    pub like_count: Option<u32>,
    /// The comment's moderation status. Will not be set if the comments were requested through the id filter.
    #[serde(rename="moderationStatus")]
    
    pub moderation_status: Option<CommentSnippetModerationStatusEnum>,
    /// The unique id of the parent comment, only set for replies.
    #[serde(rename="parentId")]
    
    pub parent_id: Option<String>,
    /// The date and time when the comment was originally published.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The comment's text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc.
    #[serde(rename="textDisplay")]
    
    pub text_display: Option<String>,
    /// The comment's original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment's author.
    #[serde(rename="textOriginal")]
    
    pub text_original: Option<String>,
    /// The date and time when the comment was last updated.
    #[serde(rename="updatedAt")]
    
    pub updated_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID of the video the comment refers to, if any.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
    /// The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future.
    #[serde(rename="viewerRating")]
    
    pub viewer_rating: Option<CommentSnippetViewerRatingEnum>,
}

impl client::Part for CommentSnippet {}


/// The id of the author's YouTube channel, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentSnippetAuthorChannelId {
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for CommentSnippetAuthorChannelId {}


/// A *comment thread* represents information that applies to a top level comment and all its replies. It can also include the top level comment itself and some of the replies.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert comment threads](CommentThreadInsertCall) (request|response)
/// * [list comment threads](CommentThreadListCall) (none)
/// * [v3 update comment threads youtube](YoutubeV3UpdateCommentThreadCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentThread {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the comment thread.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#commentThread".
    
    pub kind: Option<String>,
    /// The replies object contains a limited number of replies (if any) to the top level comment found in the snippet.
    
    pub replies: Option<CommentThreadReplies>,
    /// The snippet object contains basic details about the comment thread and also the top level comment.
    
    pub snippet: Option<CommentThreadSnippet>,
}

impl client::RequestValue for CommentThread {}
impl client::Resource for CommentThread {}
impl client::ResponseResult for CommentThread {}

impl client::ToParts for CommentThread {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.replies.is_some() { r = r + "replies,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list comment threads](CommentThreadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentThreadListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of comment threads that match the request criteria.
    
    pub items: Option<Vec<CommentThread>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#commentThreadListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for CommentThreadListResponse {}

impl client::ToParts for CommentThreadListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Comments written in (direct or indirect) reply to the top level comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentThreadReplies {
    /// A limited number of replies. Unless the number of replies returned equals total_reply_count in the snippet the returned replies are only a subset of the total number of replies.
    
    pub comments: Option<Vec<Comment>>,
}

impl client::Part for CommentThreadReplies {}


/// Basic details about a comment thread.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentThreadSnippet {
    /// Whether the current viewer of the thread can reply to it. This is viewer specific - other viewers may see a different value for this field.
    #[serde(rename="canReply")]
    
    pub can_reply: Option<bool>,
    /// The YouTube channel the comments in the thread refer to or the channel with the video the comments refer to. If video_id isn't set the comments refer to the channel itself.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Whether the thread (and therefore all its comments) is visible to all YouTube users.
    #[serde(rename="isPublic")]
    
    pub is_public: Option<bool>,
    /// The top level comment of this thread.
    #[serde(rename="topLevelComment")]
    
    pub top_level_comment: Option<Comment>,
    /// The total number of replies (not including the top level comment).
    #[serde(rename="totalReplyCount")]
    
    pub total_reply_count: Option<u32>,
    /// The ID of the video the comments refer to, if any. No video_id implies a channel discussion comment.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for CommentThreadSnippet {}


/// Ratings schemes. The country-specific ratings are mostly for movies and shows. LINT.IfChange
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentRating {
    /// The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming.
    #[serde(rename="acbRating")]
    
    pub acb_rating: Option<ContentRatingAcbRatingEnum>,
    /// The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM).
    #[serde(rename="agcomRating")]
    
    pub agcom_rating: Option<ContentRatingAgcomRatingEnum>,
    /// The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television.
    #[serde(rename="anatelRating")]
    
    pub anatel_rating: Option<ContentRatingAnatelRatingEnum>,
    /// The video's British Board of Film Classification (BBFC) rating.
    #[serde(rename="bbfcRating")]
    
    pub bbfc_rating: Option<ContentRatingBbfcRatingEnum>,
    /// The video's rating from Thailand's Board of Film and Video Censors.
    #[serde(rename="bfvcRating")]
    
    pub bfvc_rating: Option<ContentRatingBfvcRatingEnum>,
    /// The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur).
    #[serde(rename="bmukkRating")]
    
    pub bmukk_rating: Option<ContentRatingBmukkRatingEnum>,
    /// Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
    #[serde(rename="catvRating")]
    
    pub catv_rating: Option<ContentRatingCatvRatingEnum>,
    /// The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
    #[serde(rename="catvfrRating")]
    
    pub catvfr_rating: Option<ContentRatingCatvfrRatingEnum>,
    /// The video's Central Board of Film Certification (CBFC - India) rating.
    #[serde(rename="cbfcRating")]
    
    pub cbfc_rating: Option<ContentRatingCbfcRatingEnum>,
    /// The video's Consejo de Calificación Cinematográfica (Chile) rating.
    #[serde(rename="cccRating")]
    
    pub ccc_rating: Option<ContentRatingCccRatingEnum>,
    /// The video's rating from Portugal's Comissão de Classificação de Espect´culos.
    #[serde(rename="cceRating")]
    
    pub cce_rating: Option<ContentRatingCceRatingEnum>,
    /// The video's rating in Switzerland.
    #[serde(rename="chfilmRating")]
    
    pub chfilm_rating: Option<ContentRatingChfilmRatingEnum>,
    /// The video's Canadian Home Video Rating System (CHVRS) rating.
    #[serde(rename="chvrsRating")]
    
    pub chvrs_rating: Option<ContentRatingChvrsRatingEnum>,
    /// The video's rating from the Commission de Contrôle des Films (Belgium).
    #[serde(rename="cicfRating")]
    
    pub cicf_rating: Option<ContentRatingCicfRatingEnum>,
    /// The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA).
    #[serde(rename="cnaRating")]
    
    pub cna_rating: Option<ContentRatingCnaRatingEnum>,
    /// Rating system in France - Commission de classification cinematographique
    #[serde(rename="cncRating")]
    
    pub cnc_rating: Option<ContentRatingCncRatingEnum>,
    /// The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content.
    #[serde(rename="csaRating")]
    
    pub csa_rating: Option<ContentRatingCsaRatingEnum>,
    /// The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF).
    #[serde(rename="cscfRating")]
    
    pub cscf_rating: Option<ContentRatingCscfRatingEnum>,
    /// The video's rating in the Czech Republic.
    #[serde(rename="czfilmRating")]
    
    pub czfilm_rating: Option<ContentRatingCzfilmRatingEnum>,
    /// The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating.
    #[serde(rename="djctqRating")]
    
    pub djctq_rating: Option<ContentRatingDjctqRatingEnum>,
    /// Reasons that explain why the video received its DJCQT (Brazil) rating.
    #[serde(rename="djctqRatingReasons")]
    
    pub djctq_rating_reasons: Option<Vec<ContentRatingDjctqRatingReasonsEnum>>,
    /// Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism
    #[serde(rename="ecbmctRating")]
    
    pub ecbmct_rating: Option<ContentRatingEcbmctRatingEnum>,
    /// The video's rating in Estonia.
    #[serde(rename="eefilmRating")]
    
    pub eefilm_rating: Option<ContentRatingEefilmRatingEnum>,
    /// The video's rating in Egypt.
    #[serde(rename="egfilmRating")]
    
    pub egfilm_rating: Option<ContentRatingEgfilmRatingEnum>,
    /// The video's Eirin (映倫) rating. Eirin is the Japanese rating system.
    #[serde(rename="eirinRating")]
    
    pub eirin_rating: Option<ContentRatingEirinRatingEnum>,
    /// The video's rating from Malaysia's Film Censorship Board.
    #[serde(rename="fcbmRating")]
    
    pub fcbm_rating: Option<ContentRatingFcbmRatingEnum>,
    /// The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration.
    #[serde(rename="fcoRating")]
    
    pub fco_rating: Option<ContentRatingFcoRatingEnum>,
    /// This property has been deprecated. Use the contentDetails.contentRating.cncRating instead.
    #[serde(rename="fmocRating")]
    
    pub fmoc_rating: Option<ContentRatingFmocRatingEnum>,
    /// The video's rating from South Africa's Film and Publication Board.
    #[serde(rename="fpbRating")]
    
    pub fpb_rating: Option<ContentRatingFpbRatingEnum>,
    /// Reasons that explain why the video received its FPB (South Africa) rating.
    #[serde(rename="fpbRatingReasons")]
    
    pub fpb_rating_reasons: Option<Vec<ContentRatingFpbRatingReasonsEnum>>,
    /// The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating.
    #[serde(rename="fskRating")]
    
    pub fsk_rating: Option<ContentRatingFskRatingEnum>,
    /// The video's rating in Greece.
    #[serde(rename="grfilmRating")]
    
    pub grfilm_rating: Option<ContentRatingGrfilmRatingEnum>,
    /// The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating.
    #[serde(rename="icaaRating")]
    
    pub icaa_rating: Option<ContentRatingIcaaRatingEnum>,
    /// The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information.
    #[serde(rename="ifcoRating")]
    
    pub ifco_rating: Option<ContentRatingIfcoRatingEnum>,
    /// The video's rating in Israel.
    #[serde(rename="ilfilmRating")]
    
    pub ilfilm_rating: Option<ContentRatingIlfilmRatingEnum>,
    /// The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating.
    #[serde(rename="incaaRating")]
    
    pub incaa_rating: Option<ContentRatingIncaaRatingEnum>,
    /// The video's rating from the Kenya Film Classification Board.
    #[serde(rename="kfcbRating")]
    
    pub kfcb_rating: Option<ContentRatingKfcbRatingEnum>,
    /// The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands).
    #[serde(rename="kijkwijzerRating")]
    
    pub kijkwijzer_rating: Option<ContentRatingKijkwijzerRatingEnum>,
    /// The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea.
    #[serde(rename="kmrbRating")]
    
    pub kmrb_rating: Option<ContentRatingKmrbRatingEnum>,
    /// The video's rating from Indonesia's Lembaga Sensor Film.
    #[serde(rename="lsfRating")]
    
    pub lsf_rating: Option<ContentRatingLsfRatingEnum>,
    /// The video's rating from Malta's Film Age-Classification Board.
    #[serde(rename="mccaaRating")]
    
    pub mccaa_rating: Option<ContentRatingMccaaRatingEnum>,
    /// The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People.
    #[serde(rename="mccypRating")]
    
    pub mccyp_rating: Option<ContentRatingMccypRatingEnum>,
    /// The video's rating system for Vietnam - MCST
    #[serde(rename="mcstRating")]
    
    pub mcst_rating: Option<ContentRatingMcstRatingEnum>,
    /// The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC).
    #[serde(rename="mdaRating")]
    
    pub mda_rating: Option<ContentRatingMdaRatingEnum>,
    /// The video's rating from Medietilsynet, the Norwegian Media Authority.
    #[serde(rename="medietilsynetRating")]
    
    pub medietilsynet_rating: Option<ContentRatingMedietilsynetRatingEnum>,
    /// The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute).
    #[serde(rename="mekuRating")]
    
    pub meku_rating: Option<ContentRatingMekuRatingEnum>,
    /// The rating system for MENA countries, a clone of MPAA. It is needed to prevent titles go live w/o additional QC check, since some of them can be inappropriate for the countries at all. See b/33408548 for more details.
    #[serde(rename="menaMpaaRating")]
    
    pub mena_mpaa_rating: Option<ContentRatingMenaMpaaRatingEnum>,
    /// The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy).
    #[serde(rename="mibacRating")]
    
    pub mibac_rating: Option<ContentRatingMibacRatingEnum>,
    /// The video's Ministerio de Cultura (Colombia) rating.
    #[serde(rename="mocRating")]
    
    pub moc_rating: Option<ContentRatingMocRatingEnum>,
    /// The video's rating from Taiwan's Ministry of Culture (文化部).
    #[serde(rename="moctwRating")]
    
    pub moctw_rating: Option<ContentRatingMoctwRatingEnum>,
    /// The video's Motion Picture Association of America (MPAA) rating.
    #[serde(rename="mpaaRating")]
    
    pub mpaa_rating: Option<ContentRatingMpaaRatingEnum>,
    /// The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html.
    #[serde(rename="mpaatRating")]
    
    pub mpaat_rating: Option<ContentRatingMpaatRatingEnum>,
    /// The video's rating from the Movie and Television Review and Classification Board (Philippines).
    #[serde(rename="mtrcbRating")]
    
    pub mtrcb_rating: Option<ContentRatingMtrcbRatingEnum>,
    /// The video's rating from the Maldives National Bureau of Classification.
    #[serde(rename="nbcRating")]
    
    pub nbc_rating: Option<ContentRatingNbcRatingEnum>,
    /// The video's rating in Poland.
    #[serde(rename="nbcplRating")]
    
    pub nbcpl_rating: Option<ContentRatingNbcplRatingEnum>,
    /// The video's rating from the Bulgarian National Film Center.
    #[serde(rename="nfrcRating")]
    
    pub nfrc_rating: Option<ContentRatingNfrcRatingEnum>,
    /// The video's rating from Nigeria's National Film and Video Censors Board.
    #[serde(rename="nfvcbRating")]
    
    pub nfvcb_rating: Option<ContentRatingNfvcbRatingEnum>,
    /// The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia).
    #[serde(rename="nkclvRating")]
    
    pub nkclv_rating: Option<ContentRatingNkclvRatingEnum>,
    /// The National Media Council ratings system for United Arab Emirates.
    #[serde(rename="nmcRating")]
    
    pub nmc_rating: Option<ContentRatingNmcRatingEnum>,
    /// The video's Office of Film and Literature Classification (OFLC - New Zealand) rating.
    #[serde(rename="oflcRating")]
    
    pub oflc_rating: Option<ContentRatingOflcRatingEnum>,
    /// The video's rating in Peru.
    #[serde(rename="pefilmRating")]
    
    pub pefilm_rating: Option<ContentRatingPefilmRatingEnum>,
    /// The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film.
    #[serde(rename="rcnofRating")]
    
    pub rcnof_rating: Option<ContentRatingRcnofRatingEnum>,
    /// The video's rating in Venezuela.
    #[serde(rename="resorteviolenciaRating")]
    
    pub resorteviolencia_rating: Option<ContentRatingResorteviolenciaRatingEnum>,
    /// The video's General Directorate of Radio, Television and Cinematography (Mexico) rating.
    #[serde(rename="rtcRating")]
    
    pub rtc_rating: Option<ContentRatingRtcRatingEnum>,
    /// The video's rating from Ireland's Raidió Teilifís Éireann.
    #[serde(rename="rteRating")]
    
    pub rte_rating: Option<ContentRatingRteRatingEnum>,
    /// The video's National Film Registry of the Russian Federation (MKRF - Russia) rating.
    #[serde(rename="russiaRating")]
    
    pub russia_rating: Option<ContentRatingRussiaRatingEnum>,
    /// The video's rating in Slovakia.
    #[serde(rename="skfilmRating")]
    
    pub skfilm_rating: Option<ContentRatingSkfilmRatingEnum>,
    /// The video's rating in Iceland.
    #[serde(rename="smaisRating")]
    
    pub smais_rating: Option<ContentRatingSmaisRatingEnum>,
    /// The video's rating from Statens medieråd (Sweden's National Media Council).
    #[serde(rename="smsaRating")]
    
    pub smsa_rating: Option<ContentRatingSmsaRatingEnum>,
    /// The video's TV Parental Guidelines (TVPG) rating.
    #[serde(rename="tvpgRating")]
    
    pub tvpg_rating: Option<ContentRatingTvpgRatingEnum>,
    /// A rating that YouTube uses to identify age-restricted content.
    #[serde(rename="ytRating")]
    
    pub yt_rating: Option<ContentRatingYtRatingEnum>,
}

impl client::Part for ContentRating {}


/// Note that there may be a 5-second end-point resolution issue. For instance, if a cuepoint comes in for 22:03:27, we may stuff the cuepoint into 22:03:25 or 22:03:30, depending. This is an artifact of HLS.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert cuepoint live broadcasts](LiveBroadcastInsertCuepointCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cuepoint {
    /// no description provided
    #[serde(rename="cueType")]
    
    pub cue_type: Option<CuepointCueTypeEnum>,
    /// The duration of this cuepoint.
    #[serde(rename="durationSecs")]
    
    pub duration_secs: Option<u32>,
    /// no description provided
    
    pub etag: Option<String>,
    /// The identifier for cuepoint resource.
    
    pub id: Option<String>,
    /// The time when the cuepoint should be inserted by offset to the broadcast actual start time.
    #[serde(rename="insertionOffsetTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub insertion_offset_time_ms: Option<i64>,
    /// The wall clock time at which the cuepoint should be inserted. Only one of insertion_offset_time_ms and walltime_ms may be set at a time.
    #[serde(rename="walltimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub walltime_ms: Option<u64>,
}

impl client::RequestValue for Cuepoint {}
impl client::ResponseResult for Cuepoint {}

impl client::ToParts for Cuepoint {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.cue_type.is_some() { r = r + "cueType,"; }
        if self.duration_secs.is_some() { r = r + "durationSecs,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.insertion_offset_time_ms.is_some() { r = r + "insertionOffsetTimeMs,"; }
        if self.walltime_ms.is_some() { r = r + "walltimeMs,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    #[serde(rename="typeId")]
    
    pub type_id: Option<String>,
    /// no description provided
    
    pub url: Option<String>,
}

impl client::Part for Entity {}


/// Geographical coordinates of a point, in WGS84.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GeoPoint {
    /// Altitude above the reference ellipsoid, in meters.
    
    pub altitude: Option<f64>,
    /// Latitude in degrees.
    
    pub latitude: Option<f64>,
    /// Longitude in degrees.
    
    pub longitude: Option<f64>,
}

impl client::Part for GeoPoint {}


/// An *i18nLanguage* resource identifies a UI language currently supported by YouTube.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n languages](I18nLanguageListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nLanguage {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the i18n language.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguage".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the i18n language, such as language code and human-readable name.
    
    pub snippet: Option<I18nLanguageSnippet>,
}

impl client::Resource for I18nLanguage {}

impl client::ToParts for I18nLanguage {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list i18n languages](I18nLanguageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nLanguageListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource.
    
    pub items: Option<Vec<I18nLanguage>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguageListResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for I18nLanguageListResponse {}

impl client::ToParts for I18nLanguageListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about an i18n language, such as language code and human-readable name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nLanguageSnippet {
    /// A short BCP-47 code that uniquely identifies a language.
    
    pub hl: Option<String>,
    /// The human-readable name of the language in the language itself.
    
    pub name: Option<String>,
}

impl client::Part for I18nLanguageSnippet {}


/// A *i18nRegion* resource identifies a region where YouTube is available.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list i18n regions](I18nRegionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nRegion {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the i18n region.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegion".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the i18n region, such as region code and human-readable name.
    
    pub snippet: Option<I18nRegionSnippet>,
}

impl client::Resource for I18nRegion {}

impl client::ToParts for I18nRegion {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list i18n regions](I18nRegionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nRegionListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource.
    
    pub items: Option<Vec<I18nRegion>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegionListResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for I18nRegionListResponse {}

impl client::ToParts for I18nRegionListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about an i18n region, such as region code and human-readable name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct I18nRegionSnippet {
    /// The region code as a 2-letter ISO country code.
    
    pub gl: Option<String>,
    /// The human-readable name of the region.
    
    pub name: Option<String>,
}

impl client::Part for I18nRegionSnippet {}


/// Branding properties for images associated with the channel.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageSettings {
    /// The URL for the background image shown on the video watch page. The image should be 1200px by 615px, with a maximum file size of 128k.
    #[serde(rename="backgroundImageUrl")]
    
    pub background_image_url: Option<LocalizedProperty>,
    /// This is generated when a ChannelBanner.Insert request has succeeded for the given channel.
    #[serde(rename="bannerExternalUrl")]
    
    pub banner_external_url: Option<String>,
    /// Banner image. Desktop size (1060x175).
    #[serde(rename="bannerImageUrl")]
    
    pub banner_image_url: Option<String>,
    /// Banner image. Mobile size high resolution (1440x395).
    #[serde(rename="bannerMobileExtraHdImageUrl")]
    
    pub banner_mobile_extra_hd_image_url: Option<String>,
    /// Banner image. Mobile size high resolution (1280x360).
    #[serde(rename="bannerMobileHdImageUrl")]
    
    pub banner_mobile_hd_image_url: Option<String>,
    /// Banner image. Mobile size (640x175).
    #[serde(rename="bannerMobileImageUrl")]
    
    pub banner_mobile_image_url: Option<String>,
    /// Banner image. Mobile size low resolution (320x88).
    #[serde(rename="bannerMobileLowImageUrl")]
    
    pub banner_mobile_low_image_url: Option<String>,
    /// Banner image. Mobile size medium/high resolution (960x263).
    #[serde(rename="bannerMobileMediumHdImageUrl")]
    
    pub banner_mobile_medium_hd_image_url: Option<String>,
    /// Banner image. Tablet size extra high resolution (2560x424).
    #[serde(rename="bannerTabletExtraHdImageUrl")]
    
    pub banner_tablet_extra_hd_image_url: Option<String>,
    /// Banner image. Tablet size high resolution (2276x377).
    #[serde(rename="bannerTabletHdImageUrl")]
    
    pub banner_tablet_hd_image_url: Option<String>,
    /// Banner image. Tablet size (1707x283).
    #[serde(rename="bannerTabletImageUrl")]
    
    pub banner_tablet_image_url: Option<String>,
    /// Banner image. Tablet size low resolution (1138x188).
    #[serde(rename="bannerTabletLowImageUrl")]
    
    pub banner_tablet_low_image_url: Option<String>,
    /// Banner image. TV size high resolution (1920x1080).
    #[serde(rename="bannerTvHighImageUrl")]
    
    pub banner_tv_high_image_url: Option<String>,
    /// Banner image. TV size extra high resolution (2120x1192).
    #[serde(rename="bannerTvImageUrl")]
    
    pub banner_tv_image_url: Option<String>,
    /// Banner image. TV size low resolution (854x480).
    #[serde(rename="bannerTvLowImageUrl")]
    
    pub banner_tv_low_image_url: Option<String>,
    /// Banner image. TV size medium resolution (1280x720).
    #[serde(rename="bannerTvMediumImageUrl")]
    
    pub banner_tv_medium_image_url: Option<String>,
    /// The image map script for the large banner image.
    #[serde(rename="largeBrandedBannerImageImapScript")]
    
    pub large_branded_banner_image_imap_script: Option<LocalizedProperty>,
    /// The URL for the 854px by 70px image that appears below the video player in the expanded video view of the video watch page.
    #[serde(rename="largeBrandedBannerImageUrl")]
    
    pub large_branded_banner_image_url: Option<LocalizedProperty>,
    /// The image map script for the small banner image.
    #[serde(rename="smallBrandedBannerImageImapScript")]
    
    pub small_branded_banner_image_imap_script: Option<LocalizedProperty>,
    /// The URL for the 640px by 70px banner image that appears below the video player in the default view of the video watch page. The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels.
    #[serde(rename="smallBrandedBannerImageUrl")]
    
    pub small_branded_banner_image_url: Option<LocalizedProperty>,
    /// The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.
    #[serde(rename="trackingImageUrl")]
    
    pub tracking_image_url: Option<String>,
    /// no description provided
    #[serde(rename="watchIconImageUrl")]
    
    pub watch_icon_image_url: Option<String>,
}

impl client::Part for ImageSettings {}


/// Describes information necessary for ingesting an RTMP, HTTP, or SRT stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IngestionInfo {
    /// The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL.
    #[serde(rename="backupIngestionAddress")]
    
    pub backup_ingestion_address: Option<String>,
    /// The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL. Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format: *STREAM_URL/STREAM_NAME* 
    #[serde(rename="ingestionAddress")]
    
    pub ingestion_address: Option<String>,
    /// This ingestion url may be used instead of backupIngestionAddress in order to stream via RTMPS. Not applicable to non-RTMP streams.
    #[serde(rename="rtmpsBackupIngestionAddress")]
    
    pub rtmps_backup_ingestion_address: Option<String>,
    /// This ingestion url may be used instead of ingestionAddress in order to stream via RTMPS. Not applicable to non-RTMP streams.
    #[serde(rename="rtmpsIngestionAddress")]
    
    pub rtmps_ingestion_address: Option<String>,
    /// The stream name that YouTube assigns to the video stream.
    #[serde(rename="streamName")]
    
    pub stream_name: Option<String>,
}

impl client::Part for IngestionInfo {}


/// LINT.IfChange Describes an invideo branding.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set watermarks](WatermarkSetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoBranding {
    /// The bytes the uploaded image. Only used in api to youtube communication.
    #[serde(rename="imageBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub image_bytes: Option<Vec<u8>>,
    /// The url of the uploaded image. Only used in apiary to api communication.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// The spatial position within the video where the branding watermark will be displayed.
    
    pub position: Option<InvideoPosition>,
    /// The channel to which this branding links. If not present it defaults to the current channel.
    #[serde(rename="targetChannelId")]
    
    pub target_channel_id: Option<String>,
    /// The temporal position within the video where watermark will be displayed.
    
    pub timing: Option<InvideoTiming>,
}

impl client::RequestValue for InvideoBranding {}


/// Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoPosition {
    /// Describes in which corner of the video the visual widget will appear.
    #[serde(rename="cornerPosition")]
    
    pub corner_position: Option<InvideoPositionCornerPositionEnum>,
    /// Defines the position type.
    #[serde(rename="type")]
    
    pub type_: Option<InvideoPositionTypeEnum>,
}

impl client::Part for InvideoPosition {}


/// Describes a temporal position of a visual widget inside a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InvideoTiming {
    /// Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.
    #[serde(rename="durationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_ms: Option<u64>,
    /// Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds.
    #[serde(rename="offsetMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub offset_ms: Option<u64>,
    /// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
    #[serde(rename="type")]
    
    pub type_: Option<InvideoTimingTypeEnum>,
}

impl client::Part for InvideoTiming {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageTag {
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for LanguageTag {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LevelDetails {
    /// The name that should be used when referring to this level.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
}

impl client::Part for LevelDetails {}


/// A *liveBroadcast* resource represents an event that will be streamed, via live video, on YouTube.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [bind live broadcasts](LiveBroadcastBindCall) (response)
/// * [delete live broadcasts](LiveBroadcastDeleteCall) (none)
/// * [insert live broadcasts](LiveBroadcastInsertCall) (request|response)
/// * [insert cuepoint live broadcasts](LiveBroadcastInsertCuepointCall) (none)
/// * [list live broadcasts](LiveBroadcastListCall) (none)
/// * [transition live broadcasts](LiveBroadcastTransitionCall) (response)
/// * [update live broadcasts](LiveBroadcastUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcast {
    /// The contentDetails object contains information about the event's video content, such as whether the content can be shown in an embedded video player or if it will be archived and therefore available for viewing after the event has concluded.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<LiveBroadcastContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the broadcast.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcast".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the event, including its title, description, start time, and end time.
    
    pub snippet: Option<LiveBroadcastSnippet>,
    /// The statistics object contains info about the event's current stats. These include concurrent viewers and total chat count. Statistics can change (in either direction) during the lifetime of an event. Statistics are only returned while the event is live.
    
    pub statistics: Option<LiveBroadcastStatistics>,
    /// The status object contains information about the event's status.
    
    pub status: Option<LiveBroadcastStatus>,
}

impl client::RequestValue for LiveBroadcast {}
impl client::Resource for LiveBroadcast {}
impl client::ResponseResult for LiveBroadcast {}

impl client::ToParts for LiveBroadcast {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.statistics.is_some() { r = r + "statistics,"; }
        if self.status.is_some() { r = r + "status,"; }
        r.pop();
        r
    }
}

/// Detailed settings of a broadcast.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastContentDetails {
    /// This value uniquely identifies the live stream bound to the broadcast.
    #[serde(rename="boundStreamId")]
    
    pub bound_stream_id: Option<String>,
    /// The date and time that the live stream referenced by boundStreamId was last updated.
    #[serde(rename="boundStreamLastUpdateTimeMs")]
    
    pub bound_stream_last_update_time_ms: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// no description provided
    #[serde(rename="closedCaptionsType")]
    
    pub closed_captions_type: Option<LiveBroadcastContentDetailClosedCaptionsTypeEnum>,
    /// This setting indicates whether auto start is enabled for this broadcast. The default value for this property is false. This setting can only be used by Events.
    #[serde(rename="enableAutoStart")]
    
    pub enable_auto_start: Option<bool>,
    /// This setting indicates whether auto stop is enabled for this broadcast. The default value for this property is false. This setting can only be used by Events.
    #[serde(rename="enableAutoStop")]
    
    pub enable_auto_stop: Option<bool>,
    /// This setting indicates whether HTTP POST closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API. This is mutually exclusive with using the closed_captions_type property, and is equivalent to setting closed_captions_type to CLOSED_CAPTIONS_HTTP_POST.
    #[serde(rename="enableClosedCaptions")]
    
    pub enable_closed_captions: Option<bool>,
    /// This setting indicates whether YouTube should enable content encryption for the broadcast.
    #[serde(rename="enableContentEncryption")]
    
    pub enable_content_encryption: Option<bool>,
    /// This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true. *Important:* You must set the value to true and also set the enableArchive property's value to true if you want to make playback available immediately after the broadcast ends.
    #[serde(rename="enableDvr")]
    
    pub enable_dvr: Option<bool>,
    /// This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video.
    #[serde(rename="enableEmbed")]
    
    pub enable_embed: Option<bool>,
    /// Indicates whether this broadcast has low latency enabled.
    #[serde(rename="enableLowLatency")]
    
    pub enable_low_latency: Option<bool>,
    /// If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted.
    #[serde(rename="latencyPreference")]
    
    pub latency_preference: Option<LiveBroadcastContentDetailLatencyPreferenceEnum>,
    /// The mesh for projecting the video if projection is mesh. The mesh value must be a UTF-8 string containing the base-64 encoding of 3D mesh data that follows the Spherical Video V2 RFC specification for an mshp box, excluding the box size and type but including the following four reserved zero bytes for the version and flags.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub mesh: Option<Vec<u8>>,
    /// The monitorStream object contains information about the monitor stream, which the broadcaster can use to review the event content before the broadcast stream is shown publicly.
    #[serde(rename="monitorStream")]
    
    pub monitor_stream: Option<MonitorStreamInfo>,
    /// The projection format of this broadcast. This defaults to rectangular.
    
    pub projection: Option<LiveBroadcastContentDetailProjectionEnum>,
    /// Automatically start recording after the event goes live. The default value for this property is true. *Important:* You must also set the enableDvr property's value to true if you want the playback to be available immediately after the broadcast ends. If you set this property's value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback.
    #[serde(rename="recordFromStart")]
    
    pub record_from_start: Option<bool>,
    /// This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast's status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint's eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers.
    #[serde(rename="startWithSlate")]
    
    pub start_with_slate: Option<bool>,
    /// The 3D stereo layout of this broadcast. This defaults to mono.
    #[serde(rename="stereoLayout")]
    
    pub stereo_layout: Option<LiveBroadcastContentDetailStereoLayoutEnum>,
}

impl client::Part for LiveBroadcastContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list live broadcasts](LiveBroadcastListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of broadcasts that match the request criteria.
    
    pub items: Option<Vec<LiveBroadcast>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcastListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for LiveBroadcastListResponse {}

impl client::ToParts for LiveBroadcastListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic broadcast information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastSnippet {
    /// The date and time that the broadcast actually ended. This information is only available once the broadcast's state is complete.
    #[serde(rename="actualEndTime")]
    
    pub actual_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The date and time that the broadcast actually started. This information is only available once the broadcast's state is live.
    #[serde(rename="actualStartTime")]
    
    pub actual_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The broadcast's description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource.
    
    pub description: Option<String>,
    /// Indicates whether this broadcast is the default broadcast. Internal only.
    #[serde(rename="isDefaultBroadcast")]
    
    pub is_default_broadcast: Option<bool>,
    /// The id of the live chat for this broadcast.
    #[serde(rename="liveChatId")]
    
    pub live_chat_id: Option<String>,
    /// The date and time that the broadcast was added to YouTube's live broadcast schedule.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The date and time that the broadcast is scheduled to end.
    #[serde(rename="scheduledEndTime")]
    
    pub scheduled_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The date and time that the broadcast is scheduled to start.
    #[serde(rename="scheduledStartTime")]
    
    pub scheduled_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map of thumbnail images associated with the broadcast. For each nested object in this object, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The broadcast's title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource.
    
    pub title: Option<String>,
}

impl client::Part for LiveBroadcastSnippet {}


/// Statistics about the live broadcast. These represent a snapshot of the values at the time of the request. Statistics are only returned for live broadcasts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastStatistics {
    /// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
    #[serde(rename="concurrentViewers")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub concurrent_viewers: Option<u64>,
    /// The total number of live chat messages currently on the broadcast. The property and its value will be present if the broadcast is public, has the live chat feature enabled, and has at least one message. Note that this field will not be filled after the broadcast ends. So this property would not identify the number of chat messages for an archived video of a completed live broadcast.
    #[serde(rename="totalChatCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_chat_count: Option<u64>,
}

impl client::Part for LiveBroadcastStatistics {}


/// Live broadcast state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveBroadcastStatus {
    /// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
    #[serde(rename="lifeCycleStatus")]
    
    pub life_cycle_status: Option<LiveBroadcastStatuLifeCycleStatusEnum>,
    /// Priority of the live broadcast event (internal state).
    #[serde(rename="liveBroadcastPriority")]
    
    pub live_broadcast_priority: Option<LiveBroadcastStatuLiveBroadcastPriorityEnum>,
    /// Whether the broadcast is made for kids or not, decided by YouTube instead of the creator. This field is read only.
    #[serde(rename="madeForKids")]
    
    pub made_for_kids: Option<bool>,
    /// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
    #[serde(rename="privacyStatus")]
    
    pub privacy_status: Option<LiveBroadcastStatuPrivacyStatusEnum>,
    /// The broadcast's recording status.
    #[serde(rename="recordingStatus")]
    
    pub recording_status: Option<LiveBroadcastStatuRecordingStatusEnum>,
    /// This field will be set to True if the creator declares the broadcast to be kids only: go/live-cw-work.
    #[serde(rename="selfDeclaredMadeForKids")]
    
    pub self_declared_made_for_kids: Option<bool>,
}

impl client::Part for LiveBroadcastStatus {}


/// A `__liveChatBan__` resource represents a ban for a YouTube live chat.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete live chat bans](LiveChatBanDeleteCall) (none)
/// * [insert live chat bans](LiveChatBanInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatBan {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the ban.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"youtube#liveChatBan"`.
    
    pub kind: Option<String>,
    /// The `snippet` object contains basic details about the ban.
    
    pub snippet: Option<LiveChatBanSnippet>,
}

impl client::RequestValue for LiveChatBan {}
impl client::Resource for LiveChatBan {}
impl client::ResponseResult for LiveChatBan {}

impl client::ToParts for LiveChatBan {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatBanSnippet {
    /// The duration of a ban, only filled if the ban has type TEMPORARY.
    #[serde(rename="banDurationSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ban_duration_seconds: Option<u64>,
    /// no description provided
    #[serde(rename="bannedUserDetails")]
    
    pub banned_user_details: Option<ChannelProfileDetails>,
    /// The chat this ban is pertinent to.
    #[serde(rename="liveChatId")]
    
    pub live_chat_id: Option<String>,
    /// The type of ban.
    #[serde(rename="type")]
    
    pub type_: Option<LiveChatBanSnippetTypeEnum>,
}

impl client::Part for LiveChatBanSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatFanFundingEventDetails {
    /// A rendered string that displays the fund amount and currency to the user.
    #[serde(rename="amountDisplayString")]
    
    pub amount_display_string: Option<String>,
    /// The amount of the fund.
    #[serde(rename="amountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amount_micros: Option<u64>,
    /// The currency in which the fund was made.
    
    pub currency: Option<String>,
    /// The comment added by the user to this fan funding event.
    #[serde(rename="userComment")]
    
    pub user_comment: Option<String>,
}

impl client::Part for LiveChatFanFundingEventDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatGiftMembershipReceivedDetails {
    /// The ID of the membership gifting message that is related to this gift membership. This ID will always refer to a message whose type is 'membershipGiftingEvent'.
    #[serde(rename="associatedMembershipGiftingMessageId")]
    
    pub associated_membership_gifting_message_id: Option<String>,
    /// The ID of the user that made the membership gifting purchase. This matches the `snippet.authorChannelId` of the associated membership gifting message.
    #[serde(rename="gifterChannelId")]
    
    pub gifter_channel_id: Option<String>,
    /// The name of the Level at which the viewer is a member. This matches the `snippet.membershipGiftingDetails.giftMembershipsLevelName` of the associated membership gifting message. The Level names are defined by the YouTube channel offering the Membership. In some situations this field isn't filled.
    #[serde(rename="memberLevelName")]
    
    pub member_level_name: Option<String>,
}

impl client::Part for LiveChatGiftMembershipReceivedDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMemberMilestoneChatDetails {
    /// The name of the Level at which the viever is a member. The Level names are defined by the YouTube channel offering the Membership. In some situations this field isn't filled.
    #[serde(rename="memberLevelName")]
    
    pub member_level_name: Option<String>,
    /// The total amount of months (rounded up) the viewer has been a member that granted them this Member Milestone Chat. This is the same number of months as is being displayed to YouTube users.
    #[serde(rename="memberMonth")]
    
    pub member_month: Option<u32>,
    /// The comment added by the member to this Member Milestone Chat. This field is empty for messages without a comment from the member.
    #[serde(rename="userComment")]
    
    pub user_comment: Option<String>,
}

impl client::Part for LiveChatMemberMilestoneChatDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMembershipGiftingDetails {
    /// The number of gift memberships purchased by the user.
    #[serde(rename="giftMembershipsCount")]
    
    pub gift_memberships_count: Option<i32>,
    /// The name of the level of the gift memberships purchased by the user. The Level names are defined by the YouTube channel offering the Membership. In some situations this field isn't filled.
    #[serde(rename="giftMembershipsLevelName")]
    
    pub gift_memberships_level_name: Option<String>,
}

impl client::Part for LiveChatMembershipGiftingDetails {}


/// A *liveChatMessage* resource represents a chat message in a YouTube Live Chat.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete live chat messages](LiveChatMessageDeleteCall) (none)
/// * [insert live chat messages](LiveChatMessageInsertCall) (request|response)
/// * [list live chat messages](LiveChatMessageListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessage {
    /// The authorDetails object contains basic details about the user that posted this message.
    #[serde(rename="authorDetails")]
    
    pub author_details: Option<LiveChatMessageAuthorDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the message.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatMessage".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the message.
    
    pub snippet: Option<LiveChatMessageSnippet>,
}

impl client::RequestValue for LiveChatMessage {}
impl client::Resource for LiveChatMessage {}
impl client::ResponseResult for LiveChatMessage {}

impl client::ToParts for LiveChatMessage {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.author_details.is_some() { r = r + "authorDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessageAuthorDetails {
    /// The YouTube channel ID.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The channel's URL.
    #[serde(rename="channelUrl")]
    
    pub channel_url: Option<String>,
    /// The channel's display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether the author is a moderator of the live chat.
    #[serde(rename="isChatModerator")]
    
    pub is_chat_moderator: Option<bool>,
    /// Whether the author is the owner of the live chat.
    #[serde(rename="isChatOwner")]
    
    pub is_chat_owner: Option<bool>,
    /// Whether the author is a sponsor of the live chat.
    #[serde(rename="isChatSponsor")]
    
    pub is_chat_sponsor: Option<bool>,
    /// Whether the author's identity has been verified by YouTube.
    #[serde(rename="isVerified")]
    
    pub is_verified: Option<bool>,
    /// The channels's avatar URL.
    #[serde(rename="profileImageUrl")]
    
    pub profile_image_url: Option<String>,
}

impl client::Part for LiveChatMessageAuthorDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessageDeletedDetails {
    /// no description provided
    #[serde(rename="deletedMessageId")]
    
    pub deleted_message_id: Option<String>,
}

impl client::Part for LiveChatMessageDeletedDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list live chat messages](LiveChatMessageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessageListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<LiveChatMessage>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatMessageListResponse".
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The date and time when the underlying stream went offline.
    #[serde(rename="offlineAt")]
    
    pub offline_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The amount of time the client should wait before polling again.
    #[serde(rename="pollingIntervalMillis")]
    
    pub polling_interval_millis: Option<u32>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for LiveChatMessageListResponse {}

impl client::ToParts for LiveChatMessageListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.offline_at.is_some() { r = r + "offlineAt,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.polling_interval_millis.is_some() { r = r + "pollingIntervalMillis,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessageRetractedDetails {
    /// no description provided
    #[serde(rename="retractedMessageId")]
    
    pub retracted_message_id: Option<String>,
}

impl client::Part for LiveChatMessageRetractedDetails {}


/// Next ID: 33
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatMessageSnippet {
    /// The ID of the user that authored this message, this field is not always filled. textMessageEvent - the user that wrote the message fanFundingEvent - the user that funded the broadcast newSponsorEvent - the user that just became a sponsor memberMilestoneChatEvent - the member that sent the message membershipGiftingEvent - the user that made the purchase giftMembershipReceivedEvent - the user that received the gift membership messageDeletedEvent - the moderator that took the action messageRetractedEvent - the author that retracted their message userBannedEvent - the moderator that took the action superChatEvent - the user that made the purchase superStickerEvent - the user that made the purchase
    #[serde(rename="authorChannelId")]
    
    pub author_channel_id: Option<String>,
    /// Contains a string that can be displayed to the user. If this field is not present the message is silent, at the moment only messages of type TOMBSTONE and CHAT_ENDED_EVENT are silent.
    #[serde(rename="displayMessage")]
    
    pub display_message: Option<String>,
    /// Details about the funding event, this is only set if the type is 'fanFundingEvent'.
    #[serde(rename="fanFundingEventDetails")]
    
    pub fan_funding_event_details: Option<LiveChatFanFundingEventDetails>,
    /// Details about the Gift Membership Received event, this is only set if the type is 'giftMembershipReceivedEvent'.
    #[serde(rename="giftMembershipReceivedDetails")]
    
    pub gift_membership_received_details: Option<LiveChatGiftMembershipReceivedDetails>,
    /// Whether the message has display content that should be displayed to users.
    #[serde(rename="hasDisplayContent")]
    
    pub has_display_content: Option<bool>,
    /// no description provided
    #[serde(rename="liveChatId")]
    
    pub live_chat_id: Option<String>,
    /// Details about the Member Milestone Chat event, this is only set if the type is 'memberMilestoneChatEvent'.
    #[serde(rename="memberMilestoneChatDetails")]
    
    pub member_milestone_chat_details: Option<LiveChatMemberMilestoneChatDetails>,
    /// Details about the Membership Gifting event, this is only set if the type is 'membershipGiftingEvent'.
    #[serde(rename="membershipGiftingDetails")]
    
    pub membership_gifting_details: Option<LiveChatMembershipGiftingDetails>,
    /// no description provided
    #[serde(rename="messageDeletedDetails")]
    
    pub message_deleted_details: Option<LiveChatMessageDeletedDetails>,
    /// no description provided
    #[serde(rename="messageRetractedDetails")]
    
    pub message_retracted_details: Option<LiveChatMessageRetractedDetails>,
    /// Details about the New Member Announcement event, this is only set if the type is 'newSponsorEvent'. Please note that "member" is the new term for "sponsor".
    #[serde(rename="newSponsorDetails")]
    
    pub new_sponsor_details: Option<LiveChatNewSponsorDetails>,
    /// The date and time when the message was orignally published.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Details about the Super Chat event, this is only set if the type is 'superChatEvent'.
    #[serde(rename="superChatDetails")]
    
    pub super_chat_details: Option<LiveChatSuperChatDetails>,
    /// Details about the Super Sticker event, this is only set if the type is 'superStickerEvent'.
    #[serde(rename="superStickerDetails")]
    
    pub super_sticker_details: Option<LiveChatSuperStickerDetails>,
    /// Details about the text message, this is only set if the type is 'textMessageEvent'.
    #[serde(rename="textMessageDetails")]
    
    pub text_message_details: Option<LiveChatTextMessageDetails>,
    /// The type of message, this will always be present, it determines the contents of the message as well as which fields will be present.
    #[serde(rename="type")]
    
    pub type_: Option<LiveChatMessageSnippetTypeEnum>,
    /// no description provided
    #[serde(rename="userBannedDetails")]
    
    pub user_banned_details: Option<LiveChatUserBannedMessageDetails>,
}

impl client::Part for LiveChatMessageSnippet {}


/// A *liveChatModerator* resource represents a moderator for a YouTube live chat. A chat moderator has the ability to ban/unban users from a chat, remove message, etc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete live chat moderators](LiveChatModeratorDeleteCall) (none)
/// * [insert live chat moderators](LiveChatModeratorInsertCall) (request|response)
/// * [list live chat moderators](LiveChatModeratorListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatModerator {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the moderator.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatModerator".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the moderator.
    
    pub snippet: Option<LiveChatModeratorSnippet>,
}

impl client::RequestValue for LiveChatModerator {}
impl client::Resource for LiveChatModerator {}
impl client::ResponseResult for LiveChatModerator {}

impl client::ToParts for LiveChatModerator {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list live chat moderators](LiveChatModeratorListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatModeratorListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of moderators that match the request criteria.
    
    pub items: Option<Vec<LiveChatModerator>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatModeratorListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for LiveChatModeratorListResponse {}

impl client::ToParts for LiveChatModeratorListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatModeratorSnippet {
    /// The ID of the live chat this moderator can act on.
    #[serde(rename="liveChatId")]
    
    pub live_chat_id: Option<String>,
    /// Details about the moderator.
    #[serde(rename="moderatorDetails")]
    
    pub moderator_details: Option<ChannelProfileDetails>,
}

impl client::Part for LiveChatModeratorSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatNewSponsorDetails {
    /// If the viewer just had upgraded from a lower level. For viewers that were not members at the time of purchase, this field is false.
    #[serde(rename="isUpgrade")]
    
    pub is_upgrade: Option<bool>,
    /// The name of the Level that the viewer just had joined. The Level names are defined by the YouTube channel offering the Membership. In some situations this field isn't filled.
    #[serde(rename="memberLevelName")]
    
    pub member_level_name: Option<String>,
}

impl client::Part for LiveChatNewSponsorDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatSuperChatDetails {
    /// A rendered string that displays the fund amount and currency to the user.
    #[serde(rename="amountDisplayString")]
    
    pub amount_display_string: Option<String>,
    /// The amount purchased by the user, in micros (1,750,000 micros = 1.75).
    #[serde(rename="amountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amount_micros: Option<u64>,
    /// The currency in which the purchase was made.
    
    pub currency: Option<String>,
    /// The tier in which the amount belongs. Lower amounts belong to lower tiers. The lowest tier is 1.
    
    pub tier: Option<u32>,
    /// The comment added by the user to this Super Chat event.
    #[serde(rename="userComment")]
    
    pub user_comment: Option<String>,
}

impl client::Part for LiveChatSuperChatDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatSuperStickerDetails {
    /// A rendered string that displays the fund amount and currency to the user.
    #[serde(rename="amountDisplayString")]
    
    pub amount_display_string: Option<String>,
    /// The amount purchased by the user, in micros (1,750,000 micros = 1.75).
    #[serde(rename="amountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amount_micros: Option<u64>,
    /// The currency in which the purchase was made.
    
    pub currency: Option<String>,
    /// Information about the Super Sticker.
    #[serde(rename="superStickerMetadata")]
    
    pub super_sticker_metadata: Option<SuperStickerMetadata>,
    /// The tier in which the amount belongs. Lower amounts belong to lower tiers. The lowest tier is 1.
    
    pub tier: Option<u32>,
}

impl client::Part for LiveChatSuperStickerDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatTextMessageDetails {
    /// The user's message.
    #[serde(rename="messageText")]
    
    pub message_text: Option<String>,
}

impl client::Part for LiveChatTextMessageDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveChatUserBannedMessageDetails {
    /// The duration of the ban. This property is only present if the banType is temporary.
    #[serde(rename="banDurationSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ban_duration_seconds: Option<u64>,
    /// The type of ban.
    #[serde(rename="banType")]
    
    pub ban_type: Option<LiveChatUserBannedMessageDetailBanTypeEnum>,
    /// The details of the user that was banned.
    #[serde(rename="bannedUserDetails")]
    
    pub banned_user_details: Option<ChannelProfileDetails>,
}

impl client::Part for LiveChatUserBannedMessageDetails {}


/// A live stream describes a live ingestion point.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete live streams](LiveStreamDeleteCall) (none)
/// * [insert live streams](LiveStreamInsertCall) (request|response)
/// * [list live streams](LiveStreamListCall) (none)
/// * [update live streams](LiveStreamUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStream {
    /// The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube.
    
    pub cdn: Option<CdnSettings>,
    /// The content_details object contains information about the stream, including the closed captions ingestion URL.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<LiveStreamContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the stream.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStream".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the stream, including its channel, title, and description.
    
    pub snippet: Option<LiveStreamSnippet>,
    /// The status object contains information about live stream's status.
    
    pub status: Option<LiveStreamStatus>,
}

impl client::RequestValue for LiveStream {}
impl client::Resource for LiveStream {}
impl client::ResponseResult for LiveStream {}

impl client::ToParts for LiveStream {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.cdn.is_some() { r = r + "cdn,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.status.is_some() { r = r + "status,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamConfigurationIssue {
    /// The long-form description of the issue and how to resolve it.
    
    pub description: Option<String>,
    /// The short-form reason for this issue.
    
    pub reason: Option<String>,
    /// How severe this issue is to the stream.
    
    pub severity: Option<LiveStreamConfigurationIssueSeverityEnum>,
    /// The kind of error happening.
    #[serde(rename="type")]
    
    pub type_: Option<LiveStreamConfigurationIssueTypeEnum>,
}

impl client::Part for LiveStreamConfigurationIssue {}


/// Detailed settings of a stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamContentDetails {
    /// The ingestion URL where the closed captions of this stream are sent.
    #[serde(rename="closedCaptionsIngestionUrl")]
    
    pub closed_captions_ingestion_url: Option<String>,
    /// Indicates whether the stream is reusable, which means that it can be bound to multiple broadcasts. It is common for broadcasters to reuse the same stream for many different broadcasts if those broadcasts occur at different times. If you set this value to false, then the stream will not be reusable, which means that it can only be bound to one broadcast. Non-reusable streams differ from reusable streams in the following ways: - A non-reusable stream can only be bound to one broadcast. - A non-reusable stream might be deleted by an automated process after the broadcast ends. - The liveStreams.list method does not list non-reusable streams if you call the method and set the mine parameter to true. The only way to use that method to retrieve the resource for a non-reusable stream is to use the id parameter to identify the stream. 
    #[serde(rename="isReusable")]
    
    pub is_reusable: Option<bool>,
}

impl client::Part for LiveStreamContentDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamHealthStatus {
    /// The configurations issues on this stream
    #[serde(rename="configurationIssues")]
    
    pub configuration_issues: Option<Vec<LiveStreamConfigurationIssue>>,
    /// The last time this status was updated (in seconds)
    #[serde(rename="lastUpdateTimeSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_update_time_seconds: Option<u64>,
    /// The status code of this stream
    
    pub status: Option<LiveStreamHealthStatuStatusEnum>,
}

impl client::Part for LiveStreamHealthStatus {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list live streams](LiveStreamListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of live streams that match the request criteria.
    
    pub items: Option<Vec<LiveStream>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#liveStreamListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for LiveStreamListResponse {}

impl client::ToParts for LiveStreamListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamSnippet {
    /// The ID that YouTube uses to uniquely identify the channel that is transmitting the stream.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The stream's description. The value cannot be longer than 10000 characters.
    
    pub description: Option<String>,
    /// no description provided
    #[serde(rename="isDefaultStream")]
    
    pub is_default_stream: Option<bool>,
    /// The date and time that the stream was created.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The stream's title. The value must be between 1 and 128 characters long.
    
    pub title: Option<String>,
}

impl client::Part for LiveStreamSnippet {}


/// Brief description of the live stream status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LiveStreamStatus {
    /// The health status of the stream.
    #[serde(rename="healthStatus")]
    
    pub health_status: Option<LiveStreamHealthStatus>,
    /// no description provided
    #[serde(rename="streamStatus")]
    
    pub stream_status: Option<LiveStreamStatuStreamStatusEnum>,
}

impl client::Part for LiveStreamStatus {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedProperty {
    /// no description provided
    
    pub default: Option<String>,
    /// The language of the default property.
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<LanguageTag>,
    /// no description provided
    
    pub localized: Option<Vec<LocalizedString>>,
}

impl client::Part for LocalizedProperty {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    /// no description provided
    
    pub language: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for LocalizedString {}


/// A *member* resource represents a member for a YouTube channel. A member provides recurring monetary support to a creator and receives special benefits.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list members](MemberListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#member".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the member.
    
    pub snippet: Option<MemberSnippet>,
}

impl client::Resource for Member {}

impl client::ToParts for Member {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list members](MemberListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemberListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of members that match the request criteria.
    
    pub items: Option<Vec<Member>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#memberListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for MemberListResponse {}

impl client::ToParts for MemberListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MemberSnippet {
    /// The id of the channel that's offering memberships.
    #[serde(rename="creatorChannelId")]
    
    pub creator_channel_id: Option<String>,
    /// Details about the member.
    #[serde(rename="memberDetails")]
    
    pub member_details: Option<ChannelProfileDetails>,
    /// Details about the user's membership.
    #[serde(rename="membershipsDetails")]
    
    pub memberships_details: Option<MembershipsDetails>,
}

impl client::Part for MemberSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsDetails {
    /// Ids of all levels that the user has access to. This includes the currently active level and all other levels that are included because of a higher purchase.
    #[serde(rename="accessibleLevels")]
    
    pub accessible_levels: Option<Vec<String>>,
    /// Id of the highest level that the user has access to at the moment.
    #[serde(rename="highestAccessibleLevel")]
    
    pub highest_accessible_level: Option<String>,
    /// Display name for the highest level that the user has access to at the moment.
    #[serde(rename="highestAccessibleLevelDisplayName")]
    
    pub highest_accessible_level_display_name: Option<String>,
    /// Data about memberships duration without taking into consideration pricing levels.
    #[serde(rename="membershipsDuration")]
    
    pub memberships_duration: Option<MembershipsDuration>,
    /// Data about memberships duration on particular pricing levels.
    #[serde(rename="membershipsDurationAtLevels")]
    
    pub memberships_duration_at_levels: Option<Vec<MembershipsDurationAtLevel>>,
}

impl client::Part for MembershipsDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsDuration {
    /// The date and time when the user became a continuous member across all levels.
    #[serde(rename="memberSince")]
    
    pub member_since: Option<String>,
    /// The cumulative time the user has been a member across all levels in complete months (the time is rounded down to the nearest integer).
    #[serde(rename="memberTotalDurationMonths")]
    
    pub member_total_duration_months: Option<i32>,
}

impl client::Part for MembershipsDuration {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsDurationAtLevel {
    /// Pricing level ID.
    
    pub level: Option<String>,
    /// The date and time when the user became a continuous member for the given level.
    #[serde(rename="memberSince")]
    
    pub member_since: Option<String>,
    /// The cumulative time the user has been a member for the given level in complete months (the time is rounded down to the nearest integer).
    #[serde(rename="memberTotalDurationMonths")]
    
    pub member_total_duration_months: Option<i32>,
}

impl client::Part for MembershipsDurationAtLevel {}


/// A *membershipsLevel* resource represents an offer made by YouTube creators for their fans. Users can become members of the channel by joining one of the available levels. They will provide recurring monetary support and receives special benefits.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list memberships levels](MembershipsLevelListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsLevel {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the memberships level.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#membershipsLevelListResponse".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the level.
    
    pub snippet: Option<MembershipsLevelSnippet>,
}

impl client::Resource for MembershipsLevel {}

impl client::ToParts for MembershipsLevel {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list memberships levels](MembershipsLevelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsLevelListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of pricing levels offered by a creator to the fans.
    
    pub items: Option<Vec<MembershipsLevel>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#membershipsLevelListResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for MembershipsLevelListResponse {}

impl client::ToParts for MembershipsLevelListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipsLevelSnippet {
    /// The id of the channel that's offering channel memberships.
    #[serde(rename="creatorChannelId")]
    
    pub creator_channel_id: Option<String>,
    /// Details about the pricing level.
    #[serde(rename="levelDetails")]
    
    pub level_details: Option<LevelDetails>,
}

impl client::Part for MembershipsLevelSnippet {}


/// Settings and Info of the monitor stream
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonitorStreamInfo {
    /// If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay.
    #[serde(rename="broadcastStreamDelayMs")]
    
    pub broadcast_stream_delay_ms: Option<u32>,
    /// HTML code that embeds a player that plays the monitor stream.
    #[serde(rename="embedHtml")]
    
    pub embed_html: Option<String>,
    /// This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster's consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints. You need to set this value to true if you intend to have a broadcast delay for your event. *Note:* This property cannot be updated once the broadcast is in the testing or live state.
    #[serde(rename="enableMonitorStream")]
    
    pub enable_monitor_stream: Option<bool>,
}

impl client::Part for MonitorStreamInfo {}


/// Paging details for lists of resources, including total number of items available and number of resources returned in a single page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageInfo {
    /// The number of results included in the API response.
    #[serde(rename="resultsPerPage")]
    
    pub results_per_page: Option<i32>,
    /// The total number of results in the result set.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<i32>,
}

impl client::Part for PageInfo {}


/// A *playlist* resource represents a YouTube playlist. A playlist is a collection of videos that can be viewed sequentially and shared with other users. A playlist can contain up to 200 videos, and YouTube does not limit the number of playlists that each user creates. By default, playlists are publicly visible to other users, but playlists can be public or private. YouTube also uses playlists to identify special collections of videos for a channel, such as: - uploaded videos - favorite videos - positively rated (liked) videos - watch history - watch later To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company’s videos, playlists, and other YouTube information. You can retrieve the playlist IDs for each of these lists from the channel resource for a given channel. You can then use the playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the playlistItems.insert and playlistItems.delete methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete playlists](PlaylistDeleteCall) (none)
/// * [insert playlists](PlaylistInsertCall) (request|response)
/// * [list playlists](PlaylistListCall) (none)
/// * [update playlists](PlaylistUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
    /// The contentDetails object contains information like video count.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<PlaylistContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the playlist.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".
    
    pub kind: Option<String>,
    /// Localizations for different languages
    
    pub localizations: Option<HashMap<String, PlaylistLocalization>>,
    /// The player object contains information that you would use to play the playlist in an embedded player.
    
    pub player: Option<PlaylistPlayer>,
    /// The snippet object contains basic details about the playlist, such as its title and description.
    
    pub snippet: Option<PlaylistSnippet>,
    /// The status object contains status information for the playlist.
    
    pub status: Option<PlaylistStatus>,
}

impl client::RequestValue for Playlist {}
impl client::Resource for Playlist {}
impl client::ResponseResult for Playlist {}

impl client::ToParts for Playlist {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        if self.player.is_some() { r = r + "player,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.status.is_some() { r = r + "status,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistContentDetails {
    /// The number of videos in the playlist.
    #[serde(rename="itemCount")]
    
    pub item_count: Option<u32>,
}

impl client::Part for PlaylistContentDetails {}


/// A *playlistItem* resource identifies another resource, such as a video, that is included in a playlist. In addition, the playlistItem resource contains details about the included resource that pertain specifically to how that resource is used in that playlist. YouTube uses playlists to identify special collections of videos for a channel, such as: - uploaded videos - favorite videos - positively rated (liked) videos - watch history - watch later To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company’s videos, playlists, and other YouTube information. You can retrieve the playlist IDs for each of these lists from the channel resource for a given channel. You can then use the playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the playlistItems.insert and playlistItems.delete methods. For example, if a user gives a positive rating to a video, you would insert that video into the liked videos playlist for that user’s channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete playlist items](PlaylistItemDeleteCall) (none)
/// * [insert playlist items](PlaylistItemInsertCall) (request|response)
/// * [list playlist items](PlaylistItemListCall) (none)
/// * [update playlist items](PlaylistItemUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItem {
    /// The contentDetails object is included in the resource if the included item is a YouTube video. The object contains additional information about the video.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<PlaylistItemContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the playlist item.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItem".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the playlist item, such as its title and position in the playlist.
    
    pub snippet: Option<PlaylistItemSnippet>,
    /// The status object contains information about the playlist item's privacy status.
    
    pub status: Option<PlaylistItemStatus>,
}

impl client::RequestValue for PlaylistItem {}
impl client::Resource for PlaylistItem {}
impl client::ResponseResult for PlaylistItem {}

impl client::ToParts for PlaylistItem {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.status.is_some() { r = r + "status,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemContentDetails {
    /// The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.
    #[serde(rename="endAt")]
    
    pub end_at: Option<String>,
    /// A user-generated note for this item.
    
    pub note: Option<String>,
    /// The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.
    #[serde(rename="startAt")]
    
    pub start_at: Option<String>,
    /// The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
    /// The date and time that the video was published to YouTube.
    #[serde(rename="videoPublishedAt")]
    
    pub video_published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for PlaylistItemContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list playlist items](PlaylistItemListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemListResponse {
    /// no description provided
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of playlist items that match the request criteria.
    
    pub items: Option<Vec<PlaylistItem>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItemListResponse". Etag of this resource.
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for PlaylistItemListResponse {}

impl client::ToParts for PlaylistItemListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a playlist, including title, description and thumbnails. Basic details of a YouTube Playlist item provided by the author. Next ID: 15
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemSnippet {
    /// The ID that YouTube uses to uniquely identify the user that added the item to the playlist.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Channel title for the channel that the playlist item belongs to.
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// The item's description.
    
    pub description: Option<String>,
    /// The ID that YouTube uses to uniquely identify thGe playlist that the playlist item is in.
    #[serde(rename="playlistId")]
    
    pub playlist_id: Option<String>,
    /// The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.
    
    pub position: Option<u32>,
    /// The date and time that the item was added to the playlist.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The id object contains information that can be used to uniquely identify the resource that is included in the playlist as the playlist item.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
    /// A map of thumbnail images associated with the playlist item. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The item's title.
    
    pub title: Option<String>,
    /// Channel id for the channel this video belongs to.
    #[serde(rename="videoOwnerChannelId")]
    
    pub video_owner_channel_id: Option<String>,
    /// Channel title for the channel this video belongs to.
    #[serde(rename="videoOwnerChannelTitle")]
    
    pub video_owner_channel_title: Option<String>,
}

impl client::Part for PlaylistItemSnippet {}


/// Information about the playlist item's privacy status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistItemStatus {
    /// This resource's privacy status.
    #[serde(rename="privacyStatus")]
    
    pub privacy_status: Option<PlaylistItemStatuPrivacyStatusEnum>,
}

impl client::Part for PlaylistItemStatus {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list playlists](PlaylistListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of playlists that match the request criteria
    
    pub items: Option<Vec<Playlist>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#playlistListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for PlaylistListResponse {}

impl client::ToParts for PlaylistListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Playlist localization setting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistLocalization {
    /// The localized strings for playlist's description.
    
    pub description: Option<String>,
    /// The localized strings for playlist's title.
    
    pub title: Option<String>,
}

impl client::Part for PlaylistLocalization {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistPlayer {
    /// An <iframe> tag that embeds a player that will play the playlist.
    #[serde(rename="embedHtml")]
    
    pub embed_html: Option<String>,
}

impl client::Part for PlaylistPlayer {}


/// Basic details about a playlist, including title, description and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistSnippet {
    /// The ID that YouTube uses to uniquely identify the channel that published the playlist.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The channel title of the channel that the video belongs to.
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// The language of the playlist's default title and description.
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// The playlist's description.
    
    pub description: Option<String>,
    /// Localized title and description, read-only.
    
    pub localized: Option<PlaylistLocalization>,
    /// The date and time that the playlist was created.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Keyword tags associated with the playlist.
    
    pub tags: Option<Vec<String>>,
    /// Note: if the playlist has a custom thumbnail, this field will not be populated. The video id selected by the user that will be used as the thumbnail of this playlist. This field defaults to the first publicly viewable video in the playlist, if: 1. The user has never selected a video to be the thumbnail of the playlist. 2. The user selects a video to be the thumbnail, and then removes that video from the playlist. 3. The user selects a non-owned video to be the thumbnail, but that video becomes private, or gets deleted.
    #[serde(rename="thumbnailVideoId")]
    
    pub thumbnail_video_id: Option<String>,
    /// A map of thumbnail images associated with the playlist. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The playlist's title.
    
    pub title: Option<String>,
}

impl client::Part for PlaylistSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistStatus {
    /// The playlist's privacy status.
    #[serde(rename="privacyStatus")]
    
    pub privacy_status: Option<PlaylistStatuPrivacyStatusEnum>,
}

impl client::Part for PlaylistStatus {}


/// A pair Property / Value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyValue {
    /// A property.
    
    pub property: Option<String>,
    /// The property's value.
    
    pub value: Option<String>,
}

impl client::Part for PropertyValue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedEntity {
    /// no description provided
    
    pub entity: Option<Entity>,
}

impl client::Part for RelatedEntity {}


/// A resource id is a generic reference that points to another YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceId {
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The type of the API resource.
    
    pub kind: Option<String>,
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.
    #[serde(rename="playlistId")]
    
    pub playlist_id: Option<String>,
    /// The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for ResourceId {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list search](SearchListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// Pagination information for token pagination.
    
    pub items: Option<Vec<SearchResult>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#searchListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for SearchListResponse {}

impl client::ToParts for SearchListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.region_code.is_some() { r = r + "regionCode,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// A search result contains information about a YouTube video, channel, or playlist that matches the search parameters specified in an API request. While a search result points to a uniquely identifiable resource, like a video, it does not have its own persistent data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The id object contains information that can be used to uniquely identify the resource that matches the search request.
    
    pub id: Option<ResourceId>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#searchResult".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about a search result, such as its title or description. For example, if the search result is a video, then the title will be the video's title and the description will be the video's description.
    
    pub snippet: Option<SearchResultSnippet>,
}

impl client::Part for SearchResult {}


/// Basic details about a search result, including title, description and thumbnails of the item referenced by the search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchResultSnippet {
    /// The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The title of the channel that published the resource that the search result identifies.
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// A description of the search result.
    
    pub description: Option<String>,
    /// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
    #[serde(rename="liveBroadcastContent")]
    
    pub live_broadcast_content: Option<SearchResultSnippetLiveBroadcastContentEnum>,
    /// The creation date and time of the resource that the search result identifies.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The title of the search result.
    
    pub title: Option<String>,
}

impl client::Part for SearchResultSnippet {}


/// A *subscription* resource contains information about a YouTube user subscription. A subscription notifies a user when new videos are added to a channel or when another user takes one of several actions on YouTube, such as uploading a video, rating a video, or commenting on a video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete subscriptions](SubscriptionDeleteCall) (none)
/// * [insert subscriptions](SubscriptionInsertCall) (request|response)
/// * [list subscriptions](SubscriptionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// The contentDetails object contains basic statistics about the subscription.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<SubscriptionContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the subscription.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#subscription".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the subscription, including its title and the channel that the user subscribed to.
    
    pub snippet: Option<SubscriptionSnippet>,
    /// The subscriberSnippet object contains basic details about the subscriber.
    #[serde(rename="subscriberSnippet")]
    
    pub subscriber_snippet: Option<SubscriptionSubscriberSnippet>,
}

impl client::RequestValue for Subscription {}
impl client::Resource for Subscription {}
impl client::ResponseResult for Subscription {}

impl client::ToParts for Subscription {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.subscriber_snippet.is_some() { r = r + "subscriberSnippet,"; }
        r.pop();
        r
    }
}

/// Details about the content to witch a subscription refers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionContentDetails {
    /// The type of activity this subscription is for (only uploads, everything).
    #[serde(rename="activityType")]
    
    pub activity_type: Option<SubscriptionContentDetailActivityTypeEnum>,
    /// The number of new items in the subscription since its content was last read.
    #[serde(rename="newItemCount")]
    
    pub new_item_count: Option<u32>,
    /// The approximate number of items that the subscription points to.
    #[serde(rename="totalItemCount")]
    
    pub total_item_count: Option<u32>,
}

impl client::Part for SubscriptionContentDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list subscriptions](SubscriptionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of subscriptions that match the request criteria.
    
    pub items: Option<Vec<Subscription>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#subscriptionListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for SubscriptionListResponse {}

impl client::ToParts for SubscriptionListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a subscription, including title, description and thumbnails of the subscribed item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionSnippet {
    /// The ID that YouTube uses to uniquely identify the subscriber's channel.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Channel title for the channel that the subscription belongs to.
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// The subscription's details.
    
    pub description: Option<String>,
    /// The date and time that the subscription was created.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The id object contains information about the channel that the user subscribed to.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<ResourceId>,
    /// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The subscription's title.
    
    pub title: Option<String>,
}

impl client::Part for SubscriptionSnippet {}


/// Basic details about a subscription's subscriber including title, description, channel ID and thumbnails.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionSubscriberSnippet {
    /// The channel ID of the subscriber.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The description of the subscriber.
    
    pub description: Option<String>,
    /// Thumbnails for this subscriber.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The title of the subscriber.
    
    pub title: Option<String>,
}

impl client::Part for SubscriptionSubscriberSnippet {}


/// A `__superChatEvent__` resource represents a Super Chat purchase on a YouTube channel.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list super chat events](SuperChatEventListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuperChatEvent {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube assigns to uniquely identify the Super Chat event.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"youtube#superChatEvent"`.
    
    pub kind: Option<String>,
    /// The `snippet` object contains basic details about the Super Chat event.
    
    pub snippet: Option<SuperChatEventSnippet>,
}

impl client::Resource for SuperChatEvent {}

impl client::ToParts for SuperChatEvent {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list super chat events](SuperChatEventListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuperChatEventListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of Super Chat purchases that match the request criteria.
    
    pub items: Option<Vec<SuperChatEvent>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#superChatEventListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// no description provided
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for SuperChatEventListResponse {}

impl client::ToParts for SuperChatEventListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuperChatEventSnippet {
    /// The purchase amount, in micros of the purchase currency. e.g., 1 is represented as 1000000.
    #[serde(rename="amountMicros")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub amount_micros: Option<u64>,
    /// Channel id where the event occurred.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The text contents of the comment left by the user.
    #[serde(rename="commentText")]
    
    pub comment_text: Option<String>,
    /// The date and time when the event occurred.
    #[serde(rename="createdAt")]
    
    pub created_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The currency in which the purchase was made. ISO 4217.
    
    pub currency: Option<String>,
    /// A rendered string that displays the purchase amount and currency (e.g., "$1.00"). The string is rendered for the given language.
    #[serde(rename="displayString")]
    
    pub display_string: Option<String>,
    /// True if this event is a Super Sticker event.
    #[serde(rename="isSuperStickerEvent")]
    
    pub is_super_sticker_event: Option<bool>,
    /// The tier for the paid message, which is based on the amount of money spent to purchase the message.
    #[serde(rename="messageType")]
    
    pub message_type: Option<u32>,
    /// If this event is a Super Sticker event, this field will contain metadata about the Super Sticker.
    #[serde(rename="superStickerMetadata")]
    
    pub super_sticker_metadata: Option<SuperStickerMetadata>,
    /// Details about the supporter.
    #[serde(rename="supporterDetails")]
    
    pub supporter_details: Option<ChannelProfileDetails>,
}

impl client::Part for SuperChatEventSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SuperStickerMetadata {
    /// Internationalized alt text that describes the sticker image and any animation associated with it.
    #[serde(rename="altText")]
    
    pub alt_text: Option<String>,
    /// Specifies the localization language in which the alt text is returned.
    #[serde(rename="altTextLanguage")]
    
    pub alt_text_language: Option<String>,
    /// Unique identifier of the Super Sticker. This is a shorter form of the alt_text that includes pack name and a recognizable characteristic of the sticker.
    #[serde(rename="stickerId")]
    
    pub sticker_id: Option<String>,
}

impl client::Part for SuperStickerMetadata {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert tests](TestInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestItem {
    /// no description provided
    #[serde(rename="featuredPart")]
    
    pub featured_part: Option<bool>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub gaia: Option<i64>,
    /// no description provided
    
    pub id: Option<String>,
    /// no description provided
    
    pub snippet: Option<TestItemTestItemSnippet>,
}

impl client::RequestValue for TestItem {}
impl client::ResponseResult for TestItem {}

impl client::ToParts for TestItem {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.featured_part.is_some() { r = r + "featuredPart,"; }
        if self.gaia.is_some() { r = r + "gaia,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        r.pop();
        r
    }
}

/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestItemTestItemSnippet { _never_set: Option<bool> }

impl client::Part for TestItemTestItemSnippet {}


/// A *third party account link* resource represents a link between a YouTube account or a channel and an account on a third-party service.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete third party links](ThirdPartyLinkDeleteCall) (none)
/// * [insert third party links](ThirdPartyLinkInsertCall) (request|response)
/// * [list third party links](ThirdPartyLinkListCall) (none)
/// * [update third party links](ThirdPartyLinkUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyLink {
    /// Etag of this resource
    
    pub etag: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#thirdPartyLink".
    
    pub kind: Option<String>,
    /// The linking_token identifies a YouTube account and channel with which the third party account is linked.
    #[serde(rename="linkingToken")]
    
    pub linking_token: Option<String>,
    /// The snippet object contains basic details about the third- party account link.
    
    pub snippet: Option<ThirdPartyLinkSnippet>,
    /// The status object contains information about the status of the link.
    
    pub status: Option<ThirdPartyLinkStatus>,
}

impl client::RequestValue for ThirdPartyLink {}
impl client::Resource for ThirdPartyLink {}
impl client::ResponseResult for ThirdPartyLink {}

impl client::ToParts for ThirdPartyLink {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.linking_token.is_some() { r = r + "linkingToken,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.status.is_some() { r = r + "status,"; }
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
/// * [list third party links](ThirdPartyLinkListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyLinkListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<ThirdPartyLink>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#thirdPartyLinkListResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ThirdPartyLinkListResponse {}

impl client::ToParts for ThirdPartyLinkListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        r.pop();
        r
    }
}

/// Basic information about a third party account link, including its type and type-specific information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyLinkSnippet {
    /// Information specific to a link between a channel and a store on a merchandising platform.
    #[serde(rename="channelToStoreLink")]
    
    pub channel_to_store_link: Option<ChannelToStoreLinkDetails>,
    /// Type of the link named after the entities that are being linked.
    #[serde(rename="type")]
    
    pub type_: Option<ThirdPartyLinkSnippetTypeEnum>,
}

impl client::Part for ThirdPartyLinkSnippet {}


/// The third-party link status object contains information about the status of the link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThirdPartyLinkStatus {
    /// no description provided
    #[serde(rename="linkStatus")]
    
    pub link_status: Option<ThirdPartyLinkStatuLinkStatusEnum>,
}

impl client::Part for ThirdPartyLinkStatus {}


/// A thumbnail is an image representing a YouTube resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set thumbnails](ThumbnailSetCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    /// (Optional) Height of the thumbnail image.
    
    pub height: Option<u32>,
    /// The thumbnail image's URL.
    
    pub url: Option<String>,
    /// (Optional) Width of the thumbnail image.
    
    pub width: Option<u32>,
}

impl client::Resource for Thumbnail {}


/// Internal representation of thumbnails for a YouTube resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThumbnailDetails {
    /// The default image for this resource.
    
    pub default: Option<Thumbnail>,
    /// The high quality image for this resource.
    
    pub high: Option<Thumbnail>,
    /// The maximum resolution quality image for this resource.
    
    pub maxres: Option<Thumbnail>,
    /// The medium quality image for this resource.
    
    pub medium: Option<Thumbnail>,
    /// The standard quality image for this resource.
    
    pub standard: Option<Thumbnail>,
}

impl client::Part for ThumbnailDetails {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set thumbnails](ThumbnailSetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThumbnailSetResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of thumbnails.
    
    pub items: Option<Vec<ThumbnailDetails>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#thumbnailSetResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for ThumbnailSetResponse {}


/// Stub token pagination template to suppress results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPagination { _never_set: Option<bool> }

impl client::Part for TokenPagination {}


/// A *video* resource represents a YouTube video.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete videos](VideoDeleteCall) (none)
/// * [get rating videos](VideoGetRatingCall) (none)
/// * [insert videos](VideoInsertCall) (request|response)
/// * [list videos](VideoListCall) (none)
/// * [rate videos](VideoRateCall) (none)
/// * [report abuse videos](VideoReportAbuseCall) (none)
/// * [update videos](VideoUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    /// Age restriction details related to a video. This data can only be retrieved by the video owner.
    #[serde(rename="ageGating")]
    
    pub age_gating: Option<VideoAgeGating>,
    /// The contentDetails object contains information about the video content, including the length of the video and its aspect ratio.
    #[serde(rename="contentDetails")]
    
    pub content_details: Option<VideoContentDetails>,
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner.
    #[serde(rename="fileDetails")]
    
    pub file_details: Option<VideoFileDetails>,
    /// The ID that YouTube uses to uniquely identify the video.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#video".
    
    pub kind: Option<String>,
    /// The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast.
    #[serde(rename="liveStreamingDetails")]
    
    pub live_streaming_details: Option<VideoLiveStreamingDetails>,
    /// The localizations object contains localized versions of the basic details about the video, such as its title and description.
    
    pub localizations: Option<HashMap<String, VideoLocalization>>,
    /// The monetizationDetails object encapsulates information about the monetization status of the video.
    #[serde(rename="monetizationDetails")]
    
    pub monetization_details: Option<VideoMonetizationDetails>,
    /// The player object contains information that you would use to play the video in an embedded player.
    
    pub player: Option<VideoPlayer>,
    /// The processingDetails object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video. The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner.
    #[serde(rename="processingDetails")]
    
    pub processing_details: Option<VideoProcessingDetails>,
    /// The projectDetails object contains information about the project specific video metadata. b/157517979: This part was never populated after it was added. However, it sees non-zero traffic because there is generated client code in the wild that refers to it [1]. We keep this field and do NOT remove it because otherwise V3 would return an error when this part gets requested [2]. [1] https://developers.google.com/resources/api-libraries/documentation/youtube/v3/csharp/latest/classGoogle_1_1Apis_1_1YouTube_1_1v3_1_1Data_1_1VideoProjectDetails.html [2] http://google3/video/youtube/src/python/servers/data_api/common.py?l=1565-1569&rcl=344141677
    #[serde(rename="projectDetails")]
    
    pub project_details: Option<VideoProjectDetails>,
    /// The recordingDetails object encapsulates information about the location, date and address where the video was recorded.
    #[serde(rename="recordingDetails")]
    
    pub recording_details: Option<VideoRecordingDetails>,
    /// The snippet object contains basic details about the video, such as its title, description, and category.
    
    pub snippet: Option<VideoSnippet>,
    /// The statistics object contains statistics about the video.
    
    pub statistics: Option<VideoStatistics>,
    /// The status object contains information about the video's uploading, processing, and privacy statuses.
    
    pub status: Option<VideoStatus>,
    /// The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner.
    
    pub suggestions: Option<VideoSuggestions>,
    /// The topicDetails object encapsulates information about Freebase topics associated with the video.
    #[serde(rename="topicDetails")]
    
    pub topic_details: Option<VideoTopicDetails>,
}

impl client::RequestValue for Video {}
impl client::Resource for Video {}
impl client::ResponseResult for Video {}

impl client::ToParts for Video {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.age_gating.is_some() { r = r + "ageGating,"; }
        if self.content_details.is_some() { r = r + "contentDetails,"; }
        if self.etag.is_some() { r = r + "etag,"; }
        if self.file_details.is_some() { r = r + "fileDetails,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.live_streaming_details.is_some() { r = r + "liveStreamingDetails,"; }
        if self.localizations.is_some() { r = r + "localizations,"; }
        if self.monetization_details.is_some() { r = r + "monetizationDetails,"; }
        if self.player.is_some() { r = r + "player,"; }
        if self.processing_details.is_some() { r = r + "processingDetails,"; }
        if self.project_details.is_some() { r = r + "projectDetails,"; }
        if self.recording_details.is_some() { r = r + "recordingDetails,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
        if self.statistics.is_some() { r = r + "statistics,"; }
        if self.status.is_some() { r = r + "status,"; }
        if self.suggestions.is_some() { r = r + "suggestions,"; }
        if self.topic_details.is_some() { r = r + "topicDetails,"; }
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
/// * [report abuse videos](VideoReportAbuseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAbuseReport {
    /// Additional comments regarding the abuse report.
    
    pub comments: Option<String>,
    /// The language that the content was viewed in.
    
    pub language: Option<String>,
    /// The high-level, or primary, reason that the content is abusive. The value is an abuse report reason ID.
    #[serde(rename="reasonId")]
    
    pub reason_id: Option<String>,
    /// The specific, or secondary, reason that this content is abusive (if available). The value is an abuse report reason ID that is a valid secondary reason for the primary reason.
    #[serde(rename="secondaryReasonId")]
    
    pub secondary_reason_id: Option<String>,
    /// The ID that YouTube uses to uniquely identify the video.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::RequestValue for VideoAbuseReport {}


/// A `__videoAbuseReportReason__` resource identifies a reason that a video could be reported as abusive. Video abuse report reasons are used with `video.ReportAbuse`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list video abuse report reasons](VideoAbuseReportReasonListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAbuseReportReason {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID of this abuse report reason.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"youtube#videoAbuseReportReason"`.
    
    pub kind: Option<String>,
    /// The `snippet` object contains basic details about the abuse report reason.
    
    pub snippet: Option<VideoAbuseReportReasonSnippet>,
}

impl client::Resource for VideoAbuseReportReason {}

impl client::ToParts for VideoAbuseReportReason {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.id.is_some() { r = r + "id,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.snippet.is_some() { r = r + "snippet,"; }
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
/// * [list video abuse report reasons](VideoAbuseReportReasonListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAbuseReportReasonListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of valid abuse reasons that are used with `video.ReportAbuse`.
    
    pub items: Option<Vec<VideoAbuseReportReason>>,
    /// Identifies what kind of resource this is. Value: the fixed string `"youtube#videoAbuseReportReasonListResponse"`.
    
    pub kind: Option<String>,
    /// The `visitorId` identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for VideoAbuseReportReasonListResponse {}

impl client::ToParts for VideoAbuseReportReasonListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAbuseReportReasonSnippet {
    /// The localized label belonging to this abuse report reason.
    
    pub label: Option<String>,
    /// The secondary reasons associated with this reason, if any are available. (There might be 0 or more.)
    #[serde(rename="secondaryReasons")]
    
    pub secondary_reasons: Option<Vec<VideoAbuseReportSecondaryReason>>,
}

impl client::Part for VideoAbuseReportReasonSnippet {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAbuseReportSecondaryReason {
    /// The ID of this abuse report secondary reason.
    
    pub id: Option<String>,
    /// The localized label for this abuse report secondary reason.
    
    pub label: Option<String>,
}

impl client::Part for VideoAbuseReportSecondaryReason {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoAgeGating {
    /// Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.
    #[serde(rename="alcoholContent")]
    
    pub alcohol_content: Option<bool>,
    /// Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present.
    
    pub restricted: Option<bool>,
    /// Video game rating, if any.
    #[serde(rename="videoGameRating")]
    
    pub video_game_rating: Option<VideoAgeGatingVideoGameRatingEnum>,
}

impl client::Part for VideoAgeGating {}


/// A *videoCategory* resource identifies a category that has been or could be associated with uploaded videos.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoCategory {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the video category.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategory".
    
    pub kind: Option<String>,
    /// The snippet object contains basic details about the video category, including its title.
    
    pub snippet: Option<VideoCategorySnippet>,
}

impl client::Part for VideoCategory {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list video categories](VideoCategoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoCategoryListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource.
    
    pub items: Option<Vec<VideoCategory>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for VideoCategoryListResponse {}

impl client::ToParts for VideoCategoryListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Basic details about a video category, such as its localized title.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoCategorySnippet {
    /// no description provided
    
    pub assignable: Option<bool>,
    /// The YouTube channel that created the video category.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// The video category's title.
    
    pub title: Option<String>,
}

impl client::Part for VideoCategorySnippet {}


/// Details about the content of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContentDetails {
    /// The value of captions indicates whether the video has captions or not.
    
    pub caption: Option<VideoContentDetailCaptionEnum>,
    /// Specifies the ratings that the video received under various rating schemes.
    #[serde(rename="contentRating")]
    
    pub content_rating: Option<ContentRating>,
    /// The countryRestriction object contains information about the countries where a video is (or is not) viewable.
    #[serde(rename="countryRestriction")]
    
    pub country_restriction: Option<AccessPolicy>,
    /// The value of definition indicates whether the video is available in high definition or only in standard definition.
    
    pub definition: Option<VideoContentDetailDefinitionEnum>,
    /// The value of dimension indicates whether the video is available in 3D or in 2D.
    
    pub dimension: Option<String>,
    /// The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.
    
    pub duration: Option<String>,
    /// Indicates whether the video uploader has provided a custom thumbnail image for the video. This property is only visible to the video uploader.
    #[serde(rename="hasCustomThumbnail")]
    
    pub has_custom_thumbnail: Option<bool>,
    /// The value of is_license_content indicates whether the video is licensed content.
    #[serde(rename="licensedContent")]
    
    pub licensed_content: Option<bool>,
    /// Specifies the projection format of the video.
    
    pub projection: Option<VideoContentDetailProjectionEnum>,
    /// The regionRestriction object contains information about the countries where a video is (or is not) viewable. The object will contain either the contentDetails.regionRestriction.allowed property or the contentDetails.regionRestriction.blocked property.
    #[serde(rename="regionRestriction")]
    
    pub region_restriction: Option<VideoContentDetailsRegionRestriction>,
}

impl client::Part for VideoContentDetails {}


/// DEPRECATED Region restriction of the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoContentDetailsRegionRestriction {
    /// A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.
    
    pub allowed: Option<Vec<String>>,
    /// A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.
    
    pub blocked: Option<Vec<String>>,
}

impl client::Part for VideoContentDetailsRegionRestriction {}


/// Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetails {
    /// A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.
    #[serde(rename="audioStreams")]
    
    pub audio_streams: Option<Vec<VideoFileDetailsAudioStream>>,
    /// The uploaded video file's combined (video and audio) bitrate in bits per second.
    #[serde(rename="bitrateBps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bitrate_bps: Option<u64>,
    /// The uploaded video file's container format.
    
    pub container: Option<String>,
    /// The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported: - Date only: YYYY-MM-DD - Naive time: YYYY-MM-DDTHH:MM:SS - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM 
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<String>,
    /// The length of the uploaded video in milliseconds.
    #[serde(rename="durationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_ms: Option<u64>,
    /// The uploaded file's name. This field is present whether a video file or another type of file was uploaded.
    #[serde(rename="fileName")]
    
    pub file_name: Option<String>,
    /// The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.
    #[serde(rename="fileSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_size: Option<u64>,
    /// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
    #[serde(rename="fileType")]
    
    pub file_type: Option<VideoFileDetailFileTypeEnum>,
    /// A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.
    #[serde(rename="videoStreams")]
    
    pub video_streams: Option<Vec<VideoFileDetailsVideoStream>>,
}

impl client::Part for VideoFileDetails {}


/// Information about an audio stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetailsAudioStream {
    /// The audio stream's bitrate, in bits per second.
    #[serde(rename="bitrateBps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bitrate_bps: Option<u64>,
    /// The number of audio channels that the stream contains.
    #[serde(rename="channelCount")]
    
    pub channel_count: Option<u32>,
    /// The audio codec that the stream uses.
    
    pub codec: Option<String>,
    /// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
    
    pub vendor: Option<String>,
}

impl client::Part for VideoFileDetailsAudioStream {}


/// Information about a video stream.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoFileDetailsVideoStream {
    /// The video content's display aspect ratio, which specifies the aspect ratio in which the video should be displayed.
    #[serde(rename="aspectRatio")]
    
    pub aspect_ratio: Option<f64>,
    /// The video stream's bitrate, in bits per second.
    #[serde(rename="bitrateBps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bitrate_bps: Option<u64>,
    /// The video codec that the stream uses.
    
    pub codec: Option<String>,
    /// The video stream's frame rate, in frames per second.
    #[serde(rename="frameRateFps")]
    
    pub frame_rate_fps: Option<f64>,
    /// The encoded video content's height in pixels.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<u32>,
    /// The amount that YouTube needs to rotate the original source content to properly display the video.
    
    pub rotation: Option<VideoFileDetailsVideoStreamRotationEnum>,
    /// A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code.
    
    pub vendor: Option<String>,
    /// The encoded video content's width in pixels. You can calculate the video's encoding aspect ratio as width_pixels / height_pixels.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<u32>,
}

impl client::Part for VideoFileDetailsVideoStream {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get rating videos](VideoGetRatingCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoGetRatingResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// A list of ratings that match the request criteria.
    
    pub items: Option<Vec<VideoRating>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoGetRatingResponse".
    
    pub kind: Option<String>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for VideoGetRatingResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list videos](VideoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoListResponse {
    /// Etag of this resource.
    
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// no description provided
    
    pub items: Option<Vec<Video>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoListResponse".
    
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename="pageInfo")]
    
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename="tokenPagination")]
    
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename="visitorId")]
    
    pub visitor_id: Option<String>,
}

impl client::ResponseResult for VideoListResponse {}

impl client::ToParts for VideoListResponse {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        if self.etag.is_some() { r = r + "etag,"; }
        if self.event_id.is_some() { r = r + "eventId,"; }
        if self.items.is_some() { r = r + "items,"; }
        if self.kind.is_some() { r = r + "kind,"; }
        if self.next_page_token.is_some() { r = r + "nextPageToken,"; }
        if self.page_info.is_some() { r = r + "pageInfo,"; }
        if self.prev_page_token.is_some() { r = r + "prevPageToken,"; }
        if self.token_pagination.is_some() { r = r + "tokenPagination,"; }
        if self.visitor_id.is_some() { r = r + "visitorId,"; }
        r.pop();
        r
    }
}

/// Details about the live streaming metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoLiveStreamingDetails {
    /// The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video but rather to the new video being displayed at the persistent page.
    #[serde(rename="activeLiveChatId")]
    
    pub active_live_chat_id: Option<String>,
    /// The time that the broadcast actually ended. This value will not be available until the broadcast is over.
    #[serde(rename="actualEndTime")]
    
    pub actual_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time that the broadcast actually started. This value will not be available until the broadcast begins.
    #[serde(rename="actualStartTime")]
    
    pub actual_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended.
    #[serde(rename="concurrentViewers")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub concurrent_viewers: Option<u64>,
    /// The time that the broadcast is scheduled to end. If the value is empty or the property is not present, then the broadcast is scheduled to contiue indefinitely.
    #[serde(rename="scheduledEndTime")]
    
    pub scheduled_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time that the broadcast is scheduled to begin.
    #[serde(rename="scheduledStartTime")]
    
    pub scheduled_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for VideoLiveStreamingDetails {}


/// Localized versions of certain video properties (e.g. title).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoLocalization {
    /// Localized version of the video's description.
    
    pub description: Option<String>,
    /// Localized version of the video's title.
    
    pub title: Option<String>,
}

impl client::Part for VideoLocalization {}


/// Details about monetization of a YouTube Video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoMonetizationDetails {
    /// The value of access indicates whether the video can be monetized or not.
    
    pub access: Option<AccessPolicy>,
}

impl client::Part for VideoMonetizationDetails {}


/// Player to be used for a video playback.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoPlayer {
    /// no description provided
    #[serde(rename="embedHeight")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub embed_height: Option<i64>,
    /// An <iframe> tag that embeds a player that will play the video.
    #[serde(rename="embedHtml")]
    
    pub embed_html: Option<String>,
    /// The embed width
    #[serde(rename="embedWidth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub embed_width: Option<i64>,
}

impl client::Part for VideoPlayer {}


/// Describes processing status and progress and availability of some other Video resource parts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProcessingDetails {
    /// This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(rename="editorSuggestionsAvailability")]
    
    pub editor_suggestions_availability: Option<String>,
    /// This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.
    #[serde(rename="fileDetailsAvailability")]
    
    pub file_details_availability: Option<String>,
    /// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
    #[serde(rename="processingFailureReason")]
    
    pub processing_failure_reason: Option<VideoProcessingDetailProcessingFailureReasonEnum>,
    /// This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(rename="processingIssuesAvailability")]
    
    pub processing_issues_availability: Option<String>,
    /// The processingProgress object contains information about the progress YouTube has made in processing the video. The values are really only relevant if the video's processing status is processing.
    #[serde(rename="processingProgress")]
    
    pub processing_progress: Option<VideoProcessingDetailsProcessingProgress>,
    /// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
    #[serde(rename="processingStatus")]
    
    pub processing_status: Option<VideoProcessingDetailProcessingStatusEnum>,
    /// This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.
    #[serde(rename="tagSuggestionsAvailability")]
    
    pub tag_suggestions_availability: Option<String>,
    /// This value indicates whether thumbnail images have been generated for the video.
    #[serde(rename="thumbnailsAvailability")]
    
    pub thumbnails_availability: Option<String>,
}

impl client::Part for VideoProcessingDetails {}


/// Video processing progress and completion time estimate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProcessingDetailsProcessingProgress {
    /// The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating: 100 * parts_processed / parts_total Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.
    #[serde(rename="partsProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parts_processed: Option<u64>,
    /// An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.
    #[serde(rename="partsTotal")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parts_total: Option<u64>,
    /// An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.
    #[serde(rename="timeLeftMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub time_left_ms: Option<u64>,
}

impl client::Part for VideoProcessingDetailsProcessingProgress {}


/// DEPRECATED. b/157517979: This part was never populated after it was added. However, it sees non-zero traffic because there is generated client code in the wild that refers to it [1]. We keep this field and do NOT remove it because otherwise V3 would return an error when this part gets requested [2]. [1] https://developers.google.com/resources/api-libraries/documentation/youtube/v3/csharp/latest/classGoogle_1_1Apis_1_1YouTube_1_1v3_1_1Data_1_1VideoProjectDetails.html [2] http://google3/video/youtube/src/python/servers/data_api/common.py?l=1565-1569&rcl=344141677
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoProjectDetails { _never_set: Option<bool> }

impl client::Part for VideoProjectDetails {}


/// Basic details about rating of a video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoRating {
    /// Rating of a video.
    
    pub rating: Option<VideoRatingRatingEnum>,
    /// The ID that YouTube uses to uniquely identify the video.
    #[serde(rename="videoId")]
    
    pub video_id: Option<String>,
}

impl client::Part for VideoRating {}


/// Recording information associated with the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoRecordingDetails {
    /// The geolocation information associated with the video.
    
    pub location: Option<GeoPoint>,
    /// The text description of the location where the video was recorded.
    #[serde(rename="locationDescription")]
    
    pub location_description: Option<String>,
    /// The date and time when the video was recorded.
    #[serde(rename="recordingDate")]
    
    pub recording_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for VideoRecordingDetails {}


/// Basic details about a video, including title, description, uploader, thumbnails and category.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSnippet {
    /// The YouTube video category associated with the video.
    #[serde(rename="categoryId")]
    
    pub category_id: Option<String>,
    /// The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
    /// Channel title for the channel that the video belongs to.
    #[serde(rename="channelTitle")]
    
    pub channel_title: Option<String>,
    /// The default_audio_language property specifies the language spoken in the video's default audio track.
    #[serde(rename="defaultAudioLanguage")]
    
    pub default_audio_language: Option<String>,
    /// The language of the videos's default snippet.
    #[serde(rename="defaultLanguage")]
    
    pub default_language: Option<String>,
    /// The video's description. @mutable youtube.videos.insert youtube.videos.update
    
    pub description: Option<String>,
    /// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
    #[serde(rename="liveBroadcastContent")]
    
    pub live_broadcast_content: Option<VideoSnippetLiveBroadcastContentEnum>,
    /// Localized snippet selected with the hl parameter. If no such localization exists, this field is populated with the default snippet. (Read-only)
    
    pub localized: Option<VideoLocalization>,
    /// The date and time when the video was uploaded.
    #[serde(rename="publishedAt")]
    
    pub published_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A list of keyword tags associated with the video. Tags may contain spaces.
    
    pub tags: Option<Vec<String>>,
    /// A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail.
    
    pub thumbnails: Option<ThumbnailDetails>,
    /// The video's title. @mutable youtube.videos.insert youtube.videos.update
    
    pub title: Option<String>,
}

impl client::Part for VideoSnippet {}


/// Statistics about the video, such as the number of times the video was viewed or liked.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoStatistics {
    /// The number of comments for the video.
    #[serde(rename="commentCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub comment_count: Option<u64>,
    /// The number of users who have indicated that they disliked the video by giving it a negative rating.
    #[serde(rename="dislikeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub dislike_count: Option<u64>,
    /// The number of users who currently have the video marked as a favorite video.
    #[serde(rename="favoriteCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub favorite_count: Option<u64>,
    /// The number of users who have indicated that they liked the video by giving it a positive rating.
    #[serde(rename="likeCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub like_count: Option<u64>,
    /// The number of times the video has been viewed.
    #[serde(rename="viewCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub view_count: Option<u64>,
}

impl client::Part for VideoStatistics {}


/// Basic details about a video category, such as its localized title. Next Id: 18
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoStatus {
    /// This value indicates if the video can be embedded on another website. @mutable youtube.videos.insert youtube.videos.update
    
    pub embeddable: Option<bool>,
    /// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<VideoStatuFailureReasonEnum>,
    /// The video's license. @mutable youtube.videos.insert youtube.videos.update
    
    pub license: Option<VideoStatuLicenseEnum>,
    /// no description provided
    #[serde(rename="madeForKids")]
    
    pub made_for_kids: Option<bool>,
    /// The video's privacy status.
    #[serde(rename="privacyStatus")]
    
    pub privacy_status: Option<VideoStatuPrivacyStatusEnum>,
    /// This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled. @mutable youtube.videos.insert youtube.videos.update
    #[serde(rename="publicStatsViewable")]
    
    pub public_stats_viewable: Option<bool>,
    /// The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private..
    #[serde(rename="publishAt")]
    
    pub publish_at: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
    #[serde(rename="rejectionReason")]
    
    pub rejection_reason: Option<VideoStatuRejectionReasonEnum>,
    /// no description provided
    #[serde(rename="selfDeclaredMadeForKids")]
    
    pub self_declared_made_for_kids: Option<bool>,
    /// The status of the uploaded video.
    #[serde(rename="uploadStatus")]
    
    pub upload_status: Option<VideoStatuUploadStatusEnum>,
}

impl client::Part for VideoStatus {}


/// Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSuggestions {
    /// A list of video editing operations that might improve the video quality or playback experience of the uploaded video.
    #[serde(rename="editorSuggestions")]
    
    pub editor_suggestions: Option<Vec<VideoSuggestionEditorSuggestionsEnum>>,
    /// A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.
    #[serde(rename="processingErrors")]
    
    pub processing_errors: Option<Vec<VideoSuggestionProcessingErrorsEnum>>,
    /// A list of suggestions that may improve YouTube's ability to process the video.
    #[serde(rename="processingHints")]
    
    pub processing_hints: Option<Vec<VideoSuggestionProcessingHintsEnum>>,
    /// A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.
    #[serde(rename="processingWarnings")]
    
    pub processing_warnings: Option<Vec<VideoSuggestionProcessingWarningsEnum>>,
    /// A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.
    #[serde(rename="tagSuggestions")]
    
    pub tag_suggestions: Option<Vec<VideoSuggestionsTagSuggestion>>,
}

impl client::Part for VideoSuggestions {}


/// A single tag suggestion with it's relevance information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoSuggestionsTagSuggestion {
    /// A set of video categories for which the tag is relevant. You can use this information to display appropriate tag suggestions based on the video category that the video uploader associates with the video. By default, tag suggestions are relevant for all categories if there are no restricts defined for the keyword.
    #[serde(rename="categoryRestricts")]
    
    pub category_restricts: Option<Vec<String>>,
    /// The keyword tag suggested for the video.
    
    pub tag: Option<String>,
}

impl client::Part for VideoSuggestionsTagSuggestion {}


/// Freebase topic information related to the video.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VideoTopicDetails {
    /// Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.
    #[serde(rename="relevantTopicIds")]
    
    pub relevant_topic_ids: Option<Vec<String>>,
    /// A list of Wikipedia URLs that provide a high-level description of the video's content.
    #[serde(rename="topicCategories")]
    
    pub topic_categories: Option<Vec<String>>,
    /// A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the < a href="http://wiki.freebase.com/wiki/Topic_API">Freebase Topic API.
    #[serde(rename="topicIds")]
    
    pub topic_ids: Option<Vec<String>>,
}

impl client::Part for VideoTopicDetails {}


/// Branding properties for the watch. All deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchSettings {
    /// The text color for the video watch page's branded area.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<String>,
    /// An ID that uniquely identifies a playlist that displays next to the video player.
    #[serde(rename="featuredPlaylistId")]
    
    pub featured_playlist_id: Option<String>,
    /// The background color for the video watch page's branded area.
    #[serde(rename="textColor")]
    
    pub text_color: Option<String>,
}

impl client::Part for WatchSettings {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChannelContentDetailsRelatedPlaylists {
    /// The ID of the playlist that contains the channel"s favorite videos. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list.
    
    pub favorites: Option<String>,
    /// The ID of the playlist that contains the channel"s liked videos. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list.
    
    pub likes: Option<String>,
    /// The ID of the playlist that contains the channel"s uploaded videos. Use the videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos.
    
    pub uploads: Option<String>,
    /// The ID of the playlist that contains the channel"s watch history. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list.
    #[serde(rename="watchHistory")]
    
    pub watch_history: Option<String>,
    /// The ID of the playlist that contains the channel"s watch later playlist. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list.
    #[serde(rename="watchLater")]
    
    pub watch_later: Option<String>,
}

impl client::NestedType for ChannelContentDetailsRelatedPlaylists {}
impl client::Part for ChannelContentDetailsRelatedPlaylists {}


