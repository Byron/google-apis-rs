use super::*;



#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of call-to-action, a message to the user indicating action that can be taken.
pub enum ActivityContentDetailsPromotedItemCtaTypeEnum {
    
    /// "ctaTypeUnspecified"
    #[serde(rename="ctaTypeUnspecified")]
    CtaTypeUnspecified,
    
    /// "visitAdvertiserSite"
    #[serde(rename="visitAdvertiserSite")]
    VisitAdvertiserSite,
}

impl AsRef<str> for ActivityContentDetailsPromotedItemCtaTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsPromotedItemCtaTypeEnum::CtaTypeUnspecified => "visitAdvertiserSite",
            ActivityContentDetailsPromotedItemCtaTypeEnum::VisitAdvertiserSite => "visitAdvertiserSite",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsPromotedItemCtaTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that the resource is recommended to the user.
pub enum ActivityContentDetailsRecommendationReasonEnum {
    
    /// "reasonUnspecified"
    #[serde(rename="reasonUnspecified")]
    ReasonUnspecified,
    
    /// "videoFavorited"
    #[serde(rename="videoFavorited")]
    VideoFavorited,
    
    /// "videoLiked"
    #[serde(rename="videoLiked")]
    VideoLiked,
    
    /// "videoWatched"
    #[serde(rename="videoWatched")]
    VideoWatched,
}

impl AsRef<str> for ActivityContentDetailsRecommendationReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsRecommendationReasonEnum::ReasonUnspecified => "videoWatched",
            ActivityContentDetailsRecommendationReasonEnum::VideoFavorited => "videoWatched",
            ActivityContentDetailsRecommendationReasonEnum::VideoLiked => "videoWatched",
            ActivityContentDetailsRecommendationReasonEnum::VideoWatched => "videoWatched",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsRecommendationReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The name of the social network.
pub enum ActivityContentDetailsSocialTypeEnum {
    
    /// "unspecified"
    #[serde(rename="unspecified")]
    Unspecified,
    
    /// "googlePlus"
    #[serde(rename="googlePlus")]
    GooglePlus,
    
    /// "facebook"
    #[serde(rename="facebook")]
    Facebook,
    
    /// "twitter"
    #[serde(rename="twitter")]
    Twitter,
}

impl AsRef<str> for ActivityContentDetailsSocialTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivityContentDetailsSocialTypeEnum::Unspecified => "twitter",
            ActivityContentDetailsSocialTypeEnum::GooglePlus => "twitter",
            ActivityContentDetailsSocialTypeEnum::Facebook => "twitter",
            ActivityContentDetailsSocialTypeEnum::Twitter => "twitter",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivityContentDetailsSocialTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of activity that the resource describes.
pub enum ActivitySnippetTypeEnum {
    
    /// "typeUnspecified"
    #[serde(rename="typeUnspecified")]
    TypeUnspecified,
    
    /// "upload"
    #[serde(rename="upload")]
    Upload,
    
    /// "like"
    #[serde(rename="like")]
    Like,
    
    /// "favorite"
    #[serde(rename="favorite")]
    Favorite,
    
    /// "comment"
    #[serde(rename="comment")]
    Comment,
    
    /// "subscription"
    #[serde(rename="subscription")]
    Subscription,
    
    /// "playlistItem"
    #[serde(rename="playlistItem")]
    PlaylistItem,
    
    /// "recommendation"
    #[serde(rename="recommendation")]
    Recommendation,
    
    /// "bulletin"
    #[serde(rename="bulletin")]
    Bulletin,
    
    /// "social"
    #[serde(rename="social")]
    Social,
    
    /// "channelItem"
    #[serde(rename="channelItem")]
    ChannelItem,
    
    /// "promotedItem"
    #[serde(rename="promotedItem")]
    PromotedItem,
}

impl AsRef<str> for ActivitySnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ActivitySnippetTypeEnum::TypeUnspecified => "promotedItem",
            ActivitySnippetTypeEnum::Upload => "promotedItem",
            ActivitySnippetTypeEnum::Like => "promotedItem",
            ActivitySnippetTypeEnum::Favorite => "promotedItem",
            ActivitySnippetTypeEnum::Comment => "promotedItem",
            ActivitySnippetTypeEnum::Subscription => "promotedItem",
            ActivitySnippetTypeEnum::PlaylistItem => "promotedItem",
            ActivitySnippetTypeEnum::Recommendation => "promotedItem",
            ActivitySnippetTypeEnum::Bulletin => "promotedItem",
            ActivitySnippetTypeEnum::Social => "promotedItem",
            ActivitySnippetTypeEnum::ChannelItem => "promotedItem",
            ActivitySnippetTypeEnum::PromotedItem => "promotedItem",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ActivitySnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of audio track associated with the caption track.
pub enum CaptionSnippetAudioTrackTypeEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "primary"
    #[serde(rename="primary")]
    Primary,
    
    /// "commentary"
    #[serde(rename="commentary")]
    Commentary,
    
    /// "descriptive"
    #[serde(rename="descriptive")]
    Descriptive,
}

impl AsRef<str> for CaptionSnippetAudioTrackTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetAudioTrackTypeEnum::Unknown => "descriptive",
            CaptionSnippetAudioTrackTypeEnum::Primary => "descriptive",
            CaptionSnippetAudioTrackTypeEnum::Commentary => "descriptive",
            CaptionSnippetAudioTrackTypeEnum::Descriptive => "descriptive",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetAudioTrackTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed.
pub enum CaptionSnippetFailureReasonEnum {
    
    /// "unknownFormat"
    #[serde(rename="unknownFormat")]
    UnknownFormat,
    
    /// "unsupportedFormat"
    #[serde(rename="unsupportedFormat")]
    UnsupportedFormat,
    
    /// "processingFailed"
    #[serde(rename="processingFailed")]
    ProcessingFailed,
}

impl AsRef<str> for CaptionSnippetFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetFailureReasonEnum::UnknownFormat => "processingFailed",
            CaptionSnippetFailureReasonEnum::UnsupportedFormat => "processingFailed",
            CaptionSnippetFailureReasonEnum::ProcessingFailed => "processingFailed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The caption track's status.
pub enum CaptionSnippetStatusEnum {
    
    /// "serving"
    #[serde(rename="serving")]
    Serving,
    
    /// "syncing"
    #[serde(rename="syncing")]
    Syncing,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
}

impl AsRef<str> for CaptionSnippetStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetStatusEnum::Serving => "failed",
            CaptionSnippetStatusEnum::Syncing => "failed",
            CaptionSnippetStatusEnum::Failed => "failed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The caption track's type.
pub enum CaptionSnippetTrackKindEnum {
    
    /// "standard"
    #[serde(rename="standard")]
    Standard,
    
    /// "ASR"
    #[serde(rename="ASR")]
    ASR,
    
    /// "forced"
    #[serde(rename="forced")]
    Forced,
}

impl AsRef<str> for CaptionSnippetTrackKindEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CaptionSnippetTrackKindEnum::Standard => "forced",
            CaptionSnippetTrackKindEnum::ASR => "forced",
            CaptionSnippetTrackKindEnum::Forced => "forced",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CaptionSnippetTrackKindEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The frame rate of the inbound video data.
pub enum CdnSettingFrameRateEnum {
    
    /// "30fps"
    #[serde(rename="30fps")]
    _30fps,
    
    /// "60fps"
    #[serde(rename="60fps")]
    _60fps,
    
    /// "variable"
    #[serde(rename="variable")]
    Variable,
}

impl AsRef<str> for CdnSettingFrameRateEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingFrameRateEnum::_30fps => "variable",
            CdnSettingFrameRateEnum::_60fps => "variable",
            CdnSettingFrameRateEnum::Variable => "variable",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingFrameRateEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
///  The method or protocol used to transmit the video stream.
pub enum CdnSettingIngestionTypeEnum {
    
    /// "rtmp"
    #[serde(rename="rtmp")]
    Rtmp,
    
    /// "dash"
    #[serde(rename="dash")]
    Dash,
    
    /// "webrtc"
    #[serde(rename="webrtc")]
    Webrtc,
    
    /// "hls"
    #[serde(rename="hls")]
    Hls,
}

impl AsRef<str> for CdnSettingIngestionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingIngestionTypeEnum::Rtmp => "hls",
            CdnSettingIngestionTypeEnum::Dash => "hls",
            CdnSettingIngestionTypeEnum::Webrtc => "hls",
            CdnSettingIngestionTypeEnum::Hls => "hls",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingIngestionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The resolution of the inbound video data.
pub enum CdnSettingResolutionEnum {
    
    /// "240p"
    #[serde(rename="240p")]
    _240p,
    
    /// "360p"
    #[serde(rename="360p")]
    _360p,
    
    /// "480p"
    #[serde(rename="480p")]
    _480p,
    
    /// "720p"
    #[serde(rename="720p")]
    _720p,
    
    /// "1080p"
    #[serde(rename="1080p")]
    _1080p,
    
    /// "1440p"
    #[serde(rename="1440p")]
    _1440p,
    
    /// "2160p"
    #[serde(rename="2160p")]
    _2160p,
    
    /// "variable"
    #[serde(rename="variable")]
    Variable,
}

impl AsRef<str> for CdnSettingResolutionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CdnSettingResolutionEnum::_240p => "variable",
            CdnSettingResolutionEnum::_360p => "variable",
            CdnSettingResolutionEnum::_480p => "variable",
            CdnSettingResolutionEnum::_720p => "variable",
            CdnSettingResolutionEnum::_1080p => "variable",
            CdnSettingResolutionEnum::_1440p => "variable",
            CdnSettingResolutionEnum::_2160p => "variable",
            CdnSettingResolutionEnum::Variable => "variable",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CdnSettingResolutionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the context of the ping.
pub enum ChannelConversionPingContextEnum {
    
    /// "subscribe"
    #[serde(rename="subscribe")]
    Subscribe,
    
    /// "unsubscribe"
    #[serde(rename="unsubscribe")]
    Unsubscribe,
    
    /// "cview"
    #[serde(rename="cview")]
    Cview,
}

impl AsRef<str> for ChannelConversionPingContextEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelConversionPingContextEnum::Subscribe => "cview",
            ChannelConversionPingContextEnum::Unsubscribe => "cview",
            ChannelConversionPingContextEnum::Cview => "cview",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelConversionPingContextEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The style of the channel section.
pub enum ChannelSectionSnippetStyleEnum {
    
    /// "channelsectionStyleUnspecified"
    #[serde(rename="channelsectionStyleUnspecified")]
    ChannelsectionStyleUnspecified,
    
    /// "horizontalRow"
    #[serde(rename="horizontalRow")]
    HorizontalRow,
    
    /// "verticalList"
    #[serde(rename="verticalList")]
    VerticalList,
}

impl AsRef<str> for ChannelSectionSnippetStyleEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelSectionSnippetStyleEnum::ChannelsectionStyleUnspecified => "verticalList",
            ChannelSectionSnippetStyleEnum::HorizontalRow => "verticalList",
            ChannelSectionSnippetStyleEnum::VerticalList => "verticalList",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelSectionSnippetStyleEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of the channel section.
pub enum ChannelSectionSnippetTypeEnum {
    
    /// "channelsectionTypeUndefined"
    #[serde(rename="channelsectionTypeUndefined")]
    ChannelsectionTypeUndefined,
    
    /// "singlePlaylist"
    #[serde(rename="singlePlaylist")]
    SinglePlaylist,
    
    /// "multiplePlaylists"
    #[serde(rename="multiplePlaylists")]
    MultiplePlaylists,
    
    /// "popularUploads"
    #[serde(rename="popularUploads")]
    PopularUploads,
    
    /// "recentUploads"
    #[serde(rename="recentUploads")]
    RecentUploads,
    
    /// "likes"
    #[serde(rename="likes")]
    Likes,
    
    /// "allPlaylists"
    #[serde(rename="allPlaylists")]
    AllPlaylists,
    
    /// "likedPlaylists"
    #[serde(rename="likedPlaylists")]
    LikedPlaylists,
    
    /// "recentPosts"
    #[serde(rename="recentPosts")]
    RecentPosts,
    
    /// "recentActivity"
    #[serde(rename="recentActivity")]
    RecentActivity,
    
    /// "liveEvents"
    #[serde(rename="liveEvents")]
    LiveEvents,
    
    /// "upcomingEvents"
    #[serde(rename="upcomingEvents")]
    UpcomingEvents,
    
    /// "completedEvents"
    #[serde(rename="completedEvents")]
    CompletedEvents,
    
    /// "multipleChannels"
    #[serde(rename="multipleChannels")]
    MultipleChannels,
    
    /// "postedVideos"
    #[serde(rename="postedVideos")]
    PostedVideos,
    
    /// "postedPlaylists"
    #[serde(rename="postedPlaylists")]
    PostedPlaylists,
    
    /// "subscriptions"
    #[serde(rename="subscriptions")]
    Subscriptions,
}

impl AsRef<str> for ChannelSectionSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelSectionSnippetTypeEnum::ChannelsectionTypeUndefined => "subscriptions",
            ChannelSectionSnippetTypeEnum::SinglePlaylist => "subscriptions",
            ChannelSectionSnippetTypeEnum::MultiplePlaylists => "subscriptions",
            ChannelSectionSnippetTypeEnum::PopularUploads => "subscriptions",
            ChannelSectionSnippetTypeEnum::RecentUploads => "subscriptions",
            ChannelSectionSnippetTypeEnum::Likes => "subscriptions",
            ChannelSectionSnippetTypeEnum::AllPlaylists => "subscriptions",
            ChannelSectionSnippetTypeEnum::LikedPlaylists => "subscriptions",
            ChannelSectionSnippetTypeEnum::RecentPosts => "subscriptions",
            ChannelSectionSnippetTypeEnum::RecentActivity => "subscriptions",
            ChannelSectionSnippetTypeEnum::LiveEvents => "subscriptions",
            ChannelSectionSnippetTypeEnum::UpcomingEvents => "subscriptions",
            ChannelSectionSnippetTypeEnum::CompletedEvents => "subscriptions",
            ChannelSectionSnippetTypeEnum::MultipleChannels => "subscriptions",
            ChannelSectionSnippetTypeEnum::PostedVideos => "subscriptions",
            ChannelSectionSnippetTypeEnum::PostedPlaylists => "subscriptions",
            ChannelSectionSnippetTypeEnum::Subscriptions => "subscriptions",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelSectionSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information.
pub enum ChannelStatuLongUploadsStatusEnum {
    
    /// "longUploadsUnspecified"
    #[serde(rename="longUploadsUnspecified")]
    LongUploadsUnspecified,
    
    /// "allowed"
    #[serde(rename="allowed")]
    Allowed,
    
    /// "eligible"
    #[serde(rename="eligible")]
    Eligible,
    
    /// "disallowed"
    #[serde(rename="disallowed")]
    Disallowed,
}

impl AsRef<str> for ChannelStatuLongUploadsStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelStatuLongUploadsStatusEnum::LongUploadsUnspecified => "disallowed",
            ChannelStatuLongUploadsStatusEnum::Allowed => "disallowed",
            ChannelStatuLongUploadsStatusEnum::Eligible => "disallowed",
            ChannelStatuLongUploadsStatusEnum::Disallowed => "disallowed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelStatuLongUploadsStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Privacy status of the channel.
pub enum ChannelStatuPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for ChannelStatuPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ChannelStatuPrivacyStatusEnum::Public => "private",
            ChannelStatuPrivacyStatusEnum::Unlisted => "private",
            ChannelStatuPrivacyStatusEnum::Private => "private",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ChannelStatuPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The comment's moderation status. Will not be set if the comments were requested through the id filter.
pub enum CommentSnippetModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentSnippetModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentSnippetModerationStatusEnum::Published => "rejected",
            CommentSnippetModerationStatusEnum::HeldForReview => "rejected",
            CommentSnippetModerationStatusEnum::LikelySpam => "rejected",
            CommentSnippetModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentSnippetModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future.
pub enum CommentSnippetViewerRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for CommentSnippetViewerRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentSnippetViewerRatingEnum::None => "dislike",
            CommentSnippetViewerRatingEnum::Like => "dislike",
            CommentSnippetViewerRatingEnum::Dislike => "dislike",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentSnippetViewerRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming.
pub enum ContentRatingAcbRatingEnum {
    
    /// "acbUnspecified"
    #[serde(rename="acbUnspecified")]
    AcbUnspecified,
    

    /// E
    ///
    /// "acbE"
    #[serde(rename="acbE")]
    AcbE,
    

    /// Programs that have been given a P classification by the Australian Communications and Media Authority. These programs are intended for preschool children.
    ///
    /// "acbP"
    #[serde(rename="acbP")]
    AcbP,
    

    /// Programs that have been given a C classification by the Australian Communications and Media Authority. These programs are intended for children (other than preschool children) who are younger than 14 years of age.
    ///
    /// "acbC"
    #[serde(rename="acbC")]
    AcbC,
    

    /// G
    ///
    /// "acbG"
    #[serde(rename="acbG")]
    AcbG,
    

    /// PG
    ///
    /// "acbPg"
    #[serde(rename="acbPg")]
    AcbPg,
    

    /// M
    ///
    /// "acbM"
    #[serde(rename="acbM")]
    AcbM,
    

    /// MA15+
    ///
    /// "acbMa15plus"
    #[serde(rename="acbMa15plus")]
    AcbMa15plus,
    

    /// R18+
    ///
    /// "acbR18plus"
    #[serde(rename="acbR18plus")]
    AcbR18plus,
    
    /// "acbUnrated"
    #[serde(rename="acbUnrated")]
    AcbUnrated,
}

impl AsRef<str> for ContentRatingAcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAcbRatingEnum::AcbUnspecified => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbE => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbP => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbC => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbG => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbPg => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbM => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbMa15plus => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbR18plus => "acbUnrated",
            ContentRatingAcbRatingEnum::AcbUnrated => "acbUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM).
pub enum ContentRatingAgcomRatingEnum {
    
    /// "agcomUnspecified"
    #[serde(rename="agcomUnspecified")]
    AgcomUnspecified,
    

    /// T
    ///
    /// "agcomT"
    #[serde(rename="agcomT")]
    AgcomT,
    

    /// VM14
    ///
    /// "agcomVm14"
    #[serde(rename="agcomVm14")]
    AgcomVm14,
    

    /// VM18
    ///
    /// "agcomVm18"
    #[serde(rename="agcomVm18")]
    AgcomVm18,
    
    /// "agcomUnrated"
    #[serde(rename="agcomUnrated")]
    AgcomUnrated,
}

impl AsRef<str> for ContentRatingAgcomRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAgcomRatingEnum::AgcomUnspecified => "agcomUnrated",
            ContentRatingAgcomRatingEnum::AgcomT => "agcomUnrated",
            ContentRatingAgcomRatingEnum::AgcomVm14 => "agcomUnrated",
            ContentRatingAgcomRatingEnum::AgcomVm18 => "agcomUnrated",
            ContentRatingAgcomRatingEnum::AgcomUnrated => "agcomUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAgcomRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television.
pub enum ContentRatingAnatelRatingEnum {
    
    /// "anatelUnspecified"
    #[serde(rename="anatelUnspecified")]
    AnatelUnspecified,
    

    /// F
    ///
    /// "anatelF"
    #[serde(rename="anatelF")]
    AnatelF,
    

    /// I
    ///
    /// "anatelI"
    #[serde(rename="anatelI")]
    AnatelI,
    

    /// I-7
    ///
    /// "anatelI7"
    #[serde(rename="anatelI7")]
    AnatelI7,
    

    /// I-10
    ///
    /// "anatelI10"
    #[serde(rename="anatelI10")]
    AnatelI10,
    

    /// I-12
    ///
    /// "anatelI12"
    #[serde(rename="anatelI12")]
    AnatelI12,
    

    /// R
    ///
    /// "anatelR"
    #[serde(rename="anatelR")]
    AnatelR,
    

    /// A
    ///
    /// "anatelA"
    #[serde(rename="anatelA")]
    AnatelA,
    
    /// "anatelUnrated"
    #[serde(rename="anatelUnrated")]
    AnatelUnrated,
}

impl AsRef<str> for ContentRatingAnatelRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingAnatelRatingEnum::AnatelUnspecified => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelF => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelI => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelI7 => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelI10 => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelI12 => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelR => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelA => "anatelUnrated",
            ContentRatingAnatelRatingEnum::AnatelUnrated => "anatelUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingAnatelRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's British Board of Film Classification (BBFC) rating.
pub enum ContentRatingBbfcRatingEnum {
    
    /// "bbfcUnspecified"
    #[serde(rename="bbfcUnspecified")]
    BbfcUnspecified,
    

    /// U
    ///
    /// "bbfcU"
    #[serde(rename="bbfcU")]
    BbfcU,
    

    /// PG
    ///
    /// "bbfcPg"
    #[serde(rename="bbfcPg")]
    BbfcPg,
    

    /// 12A
    ///
    /// "bbfc12a"
    #[serde(rename="bbfc12a")]
    Bbfc12a,
    

    /// 12
    ///
    /// "bbfc12"
    #[serde(rename="bbfc12")]
    Bbfc12,
    

    /// 15
    ///
    /// "bbfc15"
    #[serde(rename="bbfc15")]
    Bbfc15,
    

    /// 18
    ///
    /// "bbfc18"
    #[serde(rename="bbfc18")]
    Bbfc18,
    

    /// R18
    ///
    /// "bbfcR18"
    #[serde(rename="bbfcR18")]
    BbfcR18,
    
    /// "bbfcUnrated"
    #[serde(rename="bbfcUnrated")]
    BbfcUnrated,
}

impl AsRef<str> for ContentRatingBbfcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBbfcRatingEnum::BbfcUnspecified => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::BbfcU => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::BbfcPg => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::Bbfc12a => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::Bbfc12 => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::Bbfc15 => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::Bbfc18 => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::BbfcR18 => "bbfcUnrated",
            ContentRatingBbfcRatingEnum::BbfcUnrated => "bbfcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBbfcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Thailand's Board of Film and Video Censors.
pub enum ContentRatingBfvcRatingEnum {
    
    /// "bfvcUnspecified"
    #[serde(rename="bfvcUnspecified")]
    BfvcUnspecified,
    

    /// G
    ///
    /// "bfvcG"
    #[serde(rename="bfvcG")]
    BfvcG,
    

    /// E
    ///
    /// "bfvcE"
    #[serde(rename="bfvcE")]
    BfvcE,
    

    /// 13
    ///
    /// "bfvc13"
    #[serde(rename="bfvc13")]
    Bfvc13,
    

    /// 15
    ///
    /// "bfvc15"
    #[serde(rename="bfvc15")]
    Bfvc15,
    

    /// 18
    ///
    /// "bfvc18"
    #[serde(rename="bfvc18")]
    Bfvc18,
    

    /// 20
    ///
    /// "bfvc20"
    #[serde(rename="bfvc20")]
    Bfvc20,
    

    /// B
    ///
    /// "bfvcB"
    #[serde(rename="bfvcB")]
    BfvcB,
    
    /// "bfvcUnrated"
    #[serde(rename="bfvcUnrated")]
    BfvcUnrated,
}

impl AsRef<str> for ContentRatingBfvcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBfvcRatingEnum::BfvcUnspecified => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::BfvcG => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::BfvcE => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::Bfvc13 => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::Bfvc15 => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::Bfvc18 => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::Bfvc20 => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::BfvcB => "bfvcUnrated",
            ContentRatingBfvcRatingEnum::BfvcUnrated => "bfvcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBfvcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur).
pub enum ContentRatingBmukkRatingEnum {
    
    /// "bmukkUnspecified"
    #[serde(rename="bmukkUnspecified")]
    BmukkUnspecified,
    

    /// Unrestricted
    ///
    /// "bmukkAa"
    #[serde(rename="bmukkAa")]
    BmukkAa,
    

    /// 6+
    ///
    /// "bmukk6"
    #[serde(rename="bmukk6")]
    Bmukk6,
    

    /// 8+
    ///
    /// "bmukk8"
    #[serde(rename="bmukk8")]
    Bmukk8,
    

    /// 10+
    ///
    /// "bmukk10"
    #[serde(rename="bmukk10")]
    Bmukk10,
    

    /// 12+
    ///
    /// "bmukk12"
    #[serde(rename="bmukk12")]
    Bmukk12,
    

    /// 14+
    ///
    /// "bmukk14"
    #[serde(rename="bmukk14")]
    Bmukk14,
    

    /// 16+
    ///
    /// "bmukk16"
    #[serde(rename="bmukk16")]
    Bmukk16,
    
    /// "bmukkUnrated"
    #[serde(rename="bmukkUnrated")]
    BmukkUnrated,
}

impl AsRef<str> for ContentRatingBmukkRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingBmukkRatingEnum::BmukkUnspecified => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::BmukkAa => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk6 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk8 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk10 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk12 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk14 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::Bmukk16 => "bmukkUnrated",
            ContentRatingBmukkRatingEnum::BmukkUnrated => "bmukkUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingBmukkRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
pub enum ContentRatingCatvRatingEnum {
    
    /// "catvUnspecified"
    #[serde(rename="catvUnspecified")]
    CatvUnspecified,
    

    /// C
    ///
    /// "catvC"
    #[serde(rename="catvC")]
    CatvC,
    

    /// C8
    ///
    /// "catvC8"
    #[serde(rename="catvC8")]
    CatvC8,
    

    /// G
    ///
    /// "catvG"
    #[serde(rename="catvG")]
    CatvG,
    

    /// PG
    ///
    /// "catvPg"
    #[serde(rename="catvPg")]
    CatvPg,
    

    /// 14+
    ///
    /// "catv14plus"
    #[serde(rename="catv14plus")]
    Catv14plus,
    

    /// 18+
    ///
    /// "catv18plus"
    #[serde(rename="catv18plus")]
    Catv18plus,
    
    /// "catvUnrated"
    #[serde(rename="catvUnrated")]
    CatvUnrated,
    
    /// "catvE"
    #[serde(rename="catvE")]
    CatvE,
}

impl AsRef<str> for ContentRatingCatvRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCatvRatingEnum::CatvUnspecified => "catvE",
            ContentRatingCatvRatingEnum::CatvC => "catvE",
            ContentRatingCatvRatingEnum::CatvC8 => "catvE",
            ContentRatingCatvRatingEnum::CatvG => "catvE",
            ContentRatingCatvRatingEnum::CatvPg => "catvE",
            ContentRatingCatvRatingEnum::Catv14plus => "catvE",
            ContentRatingCatvRatingEnum::Catv18plus => "catvE",
            ContentRatingCatvRatingEnum::CatvUnrated => "catvE",
            ContentRatingCatvRatingEnum::CatvE => "catvE",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCatvRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website.
pub enum ContentRatingCatvfrRatingEnum {
    
    /// "catvfrUnspecified"
    #[serde(rename="catvfrUnspecified")]
    CatvfrUnspecified,
    

    /// G
    ///
    /// "catvfrG"
    #[serde(rename="catvfrG")]
    CatvfrG,
    

    /// 8+
    ///
    /// "catvfr8plus"
    #[serde(rename="catvfr8plus")]
    Catvfr8plus,
    

    /// 13+
    ///
    /// "catvfr13plus"
    #[serde(rename="catvfr13plus")]
    Catvfr13plus,
    

    /// 16+
    ///
    /// "catvfr16plus"
    #[serde(rename="catvfr16plus")]
    Catvfr16plus,
    

    /// 18+
    ///
    /// "catvfr18plus"
    #[serde(rename="catvfr18plus")]
    Catvfr18plus,
    
    /// "catvfrUnrated"
    #[serde(rename="catvfrUnrated")]
    CatvfrUnrated,
    
    /// "catvfrE"
    #[serde(rename="catvfrE")]
    CatvfrE,
}

impl AsRef<str> for ContentRatingCatvfrRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCatvfrRatingEnum::CatvfrUnspecified => "catvfrE",
            ContentRatingCatvfrRatingEnum::CatvfrG => "catvfrE",
            ContentRatingCatvfrRatingEnum::Catvfr8plus => "catvfrE",
            ContentRatingCatvfrRatingEnum::Catvfr13plus => "catvfrE",
            ContentRatingCatvfrRatingEnum::Catvfr16plus => "catvfrE",
            ContentRatingCatvfrRatingEnum::Catvfr18plus => "catvfrE",
            ContentRatingCatvfrRatingEnum::CatvfrUnrated => "catvfrE",
            ContentRatingCatvfrRatingEnum::CatvfrE => "catvfrE",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCatvfrRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Central Board of Film Certification (CBFC - India) rating.
pub enum ContentRatingCbfcRatingEnum {
    
    /// "cbfcUnspecified"
    #[serde(rename="cbfcUnspecified")]
    CbfcUnspecified,
    

    /// U
    ///
    /// "cbfcU"
    #[serde(rename="cbfcU")]
    CbfcU,
    

    /// U/A
    ///
    /// "cbfcUA"
    #[serde(rename="cbfcUA")]
    CbfcUA,
    

    /// U/A 7+
    ///
    /// "cbfcUA7plus"
    #[serde(rename="cbfcUA7plus")]
    CbfcUA7plus,
    

    /// U/A 13+
    ///
    /// "cbfcUA13plus"
    #[serde(rename="cbfcUA13plus")]
    CbfcUA13plus,
    

    /// U/A 16+
    ///
    /// "cbfcUA16plus"
    #[serde(rename="cbfcUA16plus")]
    CbfcUA16plus,
    

    /// A
    ///
    /// "cbfcA"
    #[serde(rename="cbfcA")]
    CbfcA,
    

    /// S
    ///
    /// "cbfcS"
    #[serde(rename="cbfcS")]
    CbfcS,
    
    /// "cbfcUnrated"
    #[serde(rename="cbfcUnrated")]
    CbfcUnrated,
}

impl AsRef<str> for ContentRatingCbfcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCbfcRatingEnum::CbfcUnspecified => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcU => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcUA => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcUA7plus => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcUA13plus => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcUA16plus => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcA => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcS => "cbfcUnrated",
            ContentRatingCbfcRatingEnum::CbfcUnrated => "cbfcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCbfcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Consejo de Calificación Cinematográfica (Chile) rating.
pub enum ContentRatingCccRatingEnum {
    
    /// "cccUnspecified"
    #[serde(rename="cccUnspecified")]
    CccUnspecified,
    

    /// Todo espectador
    ///
    /// "cccTe"
    #[serde(rename="cccTe")]
    CccTe,
    

    /// 6+ - Inconveniente para menores de 7 años
    ///
    /// "ccc6"
    #[serde(rename="ccc6")]
    Ccc6,
    

    /// 14+
    ///
    /// "ccc14"
    #[serde(rename="ccc14")]
    Ccc14,
    

    /// 18+
    ///
    /// "ccc18"
    #[serde(rename="ccc18")]
    Ccc18,
    

    /// 18+ - contenido excesivamente violento
    ///
    /// "ccc18v"
    #[serde(rename="ccc18v")]
    Ccc18v,
    

    /// 18+ - contenido pornográfico
    ///
    /// "ccc18s"
    #[serde(rename="ccc18s")]
    Ccc18s,
    
    /// "cccUnrated"
    #[serde(rename="cccUnrated")]
    CccUnrated,
}

impl AsRef<str> for ContentRatingCccRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCccRatingEnum::CccUnspecified => "cccUnrated",
            ContentRatingCccRatingEnum::CccTe => "cccUnrated",
            ContentRatingCccRatingEnum::Ccc6 => "cccUnrated",
            ContentRatingCccRatingEnum::Ccc14 => "cccUnrated",
            ContentRatingCccRatingEnum::Ccc18 => "cccUnrated",
            ContentRatingCccRatingEnum::Ccc18v => "cccUnrated",
            ContentRatingCccRatingEnum::Ccc18s => "cccUnrated",
            ContentRatingCccRatingEnum::CccUnrated => "cccUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCccRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Portugal's Comissão de Classificação de Espect´culos.
pub enum ContentRatingCceRatingEnum {
    
    /// "cceUnspecified"
    #[serde(rename="cceUnspecified")]
    CceUnspecified,
    

    /// 4
    ///
    /// "cceM4"
    #[serde(rename="cceM4")]
    CceM4,
    

    /// 6
    ///
    /// "cceM6"
    #[serde(rename="cceM6")]
    CceM6,
    

    /// 12
    ///
    /// "cceM12"
    #[serde(rename="cceM12")]
    CceM12,
    

    /// 16
    ///
    /// "cceM16"
    #[serde(rename="cceM16")]
    CceM16,
    

    /// 18
    ///
    /// "cceM18"
    #[serde(rename="cceM18")]
    CceM18,
    
    /// "cceUnrated"
    #[serde(rename="cceUnrated")]
    CceUnrated,
    

    /// 14
    ///
    /// "cceM14"
    #[serde(rename="cceM14")]
    CceM14,
}

impl AsRef<str> for ContentRatingCceRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCceRatingEnum::CceUnspecified => "cceM14",
            ContentRatingCceRatingEnum::CceM4 => "cceM14",
            ContentRatingCceRatingEnum::CceM6 => "cceM14",
            ContentRatingCceRatingEnum::CceM12 => "cceM14",
            ContentRatingCceRatingEnum::CceM16 => "cceM14",
            ContentRatingCceRatingEnum::CceM18 => "cceM14",
            ContentRatingCceRatingEnum::CceUnrated => "cceM14",
            ContentRatingCceRatingEnum::CceM14 => "cceM14",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCceRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Switzerland.
pub enum ContentRatingChfilmRatingEnum {
    
    /// "chfilmUnspecified"
    #[serde(rename="chfilmUnspecified")]
    ChfilmUnspecified,
    

    /// 0
    ///
    /// "chfilm0"
    #[serde(rename="chfilm0")]
    Chfilm0,
    

    /// 6
    ///
    /// "chfilm6"
    #[serde(rename="chfilm6")]
    Chfilm6,
    

    /// 12
    ///
    /// "chfilm12"
    #[serde(rename="chfilm12")]
    Chfilm12,
    

    /// 16
    ///
    /// "chfilm16"
    #[serde(rename="chfilm16")]
    Chfilm16,
    

    /// 18
    ///
    /// "chfilm18"
    #[serde(rename="chfilm18")]
    Chfilm18,
    
    /// "chfilmUnrated"
    #[serde(rename="chfilmUnrated")]
    ChfilmUnrated,
}

impl AsRef<str> for ContentRatingChfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingChfilmRatingEnum::ChfilmUnspecified => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::Chfilm0 => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::Chfilm6 => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::Chfilm12 => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::Chfilm16 => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::Chfilm18 => "chfilmUnrated",
            ContentRatingChfilmRatingEnum::ChfilmUnrated => "chfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingChfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Canadian Home Video Rating System (CHVRS) rating.
pub enum ContentRatingChvrsRatingEnum {
    
    /// "chvrsUnspecified"
    #[serde(rename="chvrsUnspecified")]
    ChvrsUnspecified,
    

    /// G
    ///
    /// "chvrsG"
    #[serde(rename="chvrsG")]
    ChvrsG,
    

    /// PG
    ///
    /// "chvrsPg"
    #[serde(rename="chvrsPg")]
    ChvrsPg,
    

    /// 14A
    ///
    /// "chvrs14a"
    #[serde(rename="chvrs14a")]
    Chvrs14a,
    

    /// 18A
    ///
    /// "chvrs18a"
    #[serde(rename="chvrs18a")]
    Chvrs18a,
    

    /// R
    ///
    /// "chvrsR"
    #[serde(rename="chvrsR")]
    ChvrsR,
    

    /// E
    ///
    /// "chvrsE"
    #[serde(rename="chvrsE")]
    ChvrsE,
    
    /// "chvrsUnrated"
    #[serde(rename="chvrsUnrated")]
    ChvrsUnrated,
}

impl AsRef<str> for ContentRatingChvrsRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingChvrsRatingEnum::ChvrsUnspecified => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::ChvrsG => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::ChvrsPg => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::Chvrs14a => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::Chvrs18a => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::ChvrsR => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::ChvrsE => "chvrsUnrated",
            ContentRatingChvrsRatingEnum::ChvrsUnrated => "chvrsUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingChvrsRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Commission de Contrôle des Films (Belgium).
pub enum ContentRatingCicfRatingEnum {
    
    /// "cicfUnspecified"
    #[serde(rename="cicfUnspecified")]
    CicfUnspecified,
    

    /// E
    ///
    /// "cicfE"
    #[serde(rename="cicfE")]
    CicfE,
    

    /// KT/EA
    ///
    /// "cicfKtEa"
    #[serde(rename="cicfKtEa")]
    CicfKtEa,
    

    /// KNT/ENA
    ///
    /// "cicfKntEna"
    #[serde(rename="cicfKntEna")]
    CicfKntEna,
    
    /// "cicfUnrated"
    #[serde(rename="cicfUnrated")]
    CicfUnrated,
}

impl AsRef<str> for ContentRatingCicfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCicfRatingEnum::CicfUnspecified => "cicfUnrated",
            ContentRatingCicfRatingEnum::CicfE => "cicfUnrated",
            ContentRatingCicfRatingEnum::CicfKtEa => "cicfUnrated",
            ContentRatingCicfRatingEnum::CicfKntEna => "cicfUnrated",
            ContentRatingCicfRatingEnum::CicfUnrated => "cicfUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCicfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA).
pub enum ContentRatingCnaRatingEnum {
    
    /// "cnaUnspecified"
    #[serde(rename="cnaUnspecified")]
    CnaUnspecified,
    

    /// AP
    ///
    /// "cnaAp"
    #[serde(rename="cnaAp")]
    CnaAp,
    

    /// 12
    ///
    /// "cna12"
    #[serde(rename="cna12")]
    Cna12,
    

    /// 15
    ///
    /// "cna15"
    #[serde(rename="cna15")]
    Cna15,
    

    /// 18
    ///
    /// "cna18"
    #[serde(rename="cna18")]
    Cna18,
    

    /// 18+
    ///
    /// "cna18plus"
    #[serde(rename="cna18plus")]
    Cna18plus,
    
    /// "cnaUnrated"
    #[serde(rename="cnaUnrated")]
    CnaUnrated,
}

impl AsRef<str> for ContentRatingCnaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCnaRatingEnum::CnaUnspecified => "cnaUnrated",
            ContentRatingCnaRatingEnum::CnaAp => "cnaUnrated",
            ContentRatingCnaRatingEnum::Cna12 => "cnaUnrated",
            ContentRatingCnaRatingEnum::Cna15 => "cnaUnrated",
            ContentRatingCnaRatingEnum::Cna18 => "cnaUnrated",
            ContentRatingCnaRatingEnum::Cna18plus => "cnaUnrated",
            ContentRatingCnaRatingEnum::CnaUnrated => "cnaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCnaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system in France - Commission de classification cinematographique
pub enum ContentRatingCncRatingEnum {
    
    /// "cncUnspecified"
    #[serde(rename="cncUnspecified")]
    CncUnspecified,
    

    /// T
    ///
    /// "cncT"
    #[serde(rename="cncT")]
    CncT,
    

    /// 10
    ///
    /// "cnc10"
    #[serde(rename="cnc10")]
    Cnc10,
    

    /// 12
    ///
    /// "cnc12"
    #[serde(rename="cnc12")]
    Cnc12,
    

    /// 16
    ///
    /// "cnc16"
    #[serde(rename="cnc16")]
    Cnc16,
    

    /// 18
    ///
    /// "cnc18"
    #[serde(rename="cnc18")]
    Cnc18,
    

    /// E
    ///
    /// "cncE"
    #[serde(rename="cncE")]
    CncE,
    

    /// interdiction
    ///
    /// "cncInterdiction"
    #[serde(rename="cncInterdiction")]
    CncInterdiction,
    
    /// "cncUnrated"
    #[serde(rename="cncUnrated")]
    CncUnrated,
}

impl AsRef<str> for ContentRatingCncRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCncRatingEnum::CncUnspecified => "cncUnrated",
            ContentRatingCncRatingEnum::CncT => "cncUnrated",
            ContentRatingCncRatingEnum::Cnc10 => "cncUnrated",
            ContentRatingCncRatingEnum::Cnc12 => "cncUnrated",
            ContentRatingCncRatingEnum::Cnc16 => "cncUnrated",
            ContentRatingCncRatingEnum::Cnc18 => "cncUnrated",
            ContentRatingCncRatingEnum::CncE => "cncUnrated",
            ContentRatingCncRatingEnum::CncInterdiction => "cncUnrated",
            ContentRatingCncRatingEnum::CncUnrated => "cncUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCncRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content.
pub enum ContentRatingCsaRatingEnum {
    
    /// "csaUnspecified"
    #[serde(rename="csaUnspecified")]
    CsaUnspecified,
    

    /// T
    ///
    /// "csaT"
    #[serde(rename="csaT")]
    CsaT,
    

    /// 10
    ///
    /// "csa10"
    #[serde(rename="csa10")]
    Csa10,
    

    /// 12
    ///
    /// "csa12"
    #[serde(rename="csa12")]
    Csa12,
    

    /// 16
    ///
    /// "csa16"
    #[serde(rename="csa16")]
    Csa16,
    

    /// 18
    ///
    /// "csa18"
    #[serde(rename="csa18")]
    Csa18,
    

    /// Interdiction
    ///
    /// "csaInterdiction"
    #[serde(rename="csaInterdiction")]
    CsaInterdiction,
    
    /// "csaUnrated"
    #[serde(rename="csaUnrated")]
    CsaUnrated,
}

impl AsRef<str> for ContentRatingCsaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCsaRatingEnum::CsaUnspecified => "csaUnrated",
            ContentRatingCsaRatingEnum::CsaT => "csaUnrated",
            ContentRatingCsaRatingEnum::Csa10 => "csaUnrated",
            ContentRatingCsaRatingEnum::Csa12 => "csaUnrated",
            ContentRatingCsaRatingEnum::Csa16 => "csaUnrated",
            ContentRatingCsaRatingEnum::Csa18 => "csaUnrated",
            ContentRatingCsaRatingEnum::CsaInterdiction => "csaUnrated",
            ContentRatingCsaRatingEnum::CsaUnrated => "csaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCsaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF).
pub enum ContentRatingCscfRatingEnum {
    
    /// "cscfUnspecified"
    #[serde(rename="cscfUnspecified")]
    CscfUnspecified,
    

    /// AL
    ///
    /// "cscfAl"
    #[serde(rename="cscfAl")]
    CscfAl,
    

    /// A
    ///
    /// "cscfA"
    #[serde(rename="cscfA")]
    CscfA,
    

    /// 6
    ///
    /// "cscf6"
    #[serde(rename="cscf6")]
    Cscf6,
    

    /// 9
    ///
    /// "cscf9"
    #[serde(rename="cscf9")]
    Cscf9,
    

    /// 12
    ///
    /// "cscf12"
    #[serde(rename="cscf12")]
    Cscf12,
    

    /// 16
    ///
    /// "cscf16"
    #[serde(rename="cscf16")]
    Cscf16,
    

    /// 18
    ///
    /// "cscf18"
    #[serde(rename="cscf18")]
    Cscf18,
    
    /// "cscfUnrated"
    #[serde(rename="cscfUnrated")]
    CscfUnrated,
}

impl AsRef<str> for ContentRatingCscfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCscfRatingEnum::CscfUnspecified => "cscfUnrated",
            ContentRatingCscfRatingEnum::CscfAl => "cscfUnrated",
            ContentRatingCscfRatingEnum::CscfA => "cscfUnrated",
            ContentRatingCscfRatingEnum::Cscf6 => "cscfUnrated",
            ContentRatingCscfRatingEnum::Cscf9 => "cscfUnrated",
            ContentRatingCscfRatingEnum::Cscf12 => "cscfUnrated",
            ContentRatingCscfRatingEnum::Cscf16 => "cscfUnrated",
            ContentRatingCscfRatingEnum::Cscf18 => "cscfUnrated",
            ContentRatingCscfRatingEnum::CscfUnrated => "cscfUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCscfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in the Czech Republic.
pub enum ContentRatingCzfilmRatingEnum {
    
    /// "czfilmUnspecified"
    #[serde(rename="czfilmUnspecified")]
    CzfilmUnspecified,
    

    /// U
    ///
    /// "czfilmU"
    #[serde(rename="czfilmU")]
    CzfilmU,
    

    /// 12
    ///
    /// "czfilm12"
    #[serde(rename="czfilm12")]
    Czfilm12,
    

    /// 14
    ///
    /// "czfilm14"
    #[serde(rename="czfilm14")]
    Czfilm14,
    

    /// 18
    ///
    /// "czfilm18"
    #[serde(rename="czfilm18")]
    Czfilm18,
    
    /// "czfilmUnrated"
    #[serde(rename="czfilmUnrated")]
    CzfilmUnrated,
}

impl AsRef<str> for ContentRatingCzfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingCzfilmRatingEnum::CzfilmUnspecified => "czfilmUnrated",
            ContentRatingCzfilmRatingEnum::CzfilmU => "czfilmUnrated",
            ContentRatingCzfilmRatingEnum::Czfilm12 => "czfilmUnrated",
            ContentRatingCzfilmRatingEnum::Czfilm14 => "czfilmUnrated",
            ContentRatingCzfilmRatingEnum::Czfilm18 => "czfilmUnrated",
            ContentRatingCzfilmRatingEnum::CzfilmUnrated => "czfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingCzfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating.
pub enum ContentRatingDjctqRatingEnum {
    
    /// "djctqUnspecified"
    #[serde(rename="djctqUnspecified")]
    DjctqUnspecified,
    

    /// L
    ///
    /// "djctqL"
    #[serde(rename="djctqL")]
    DjctqL,
    

    /// 10
    ///
    /// "djctq10"
    #[serde(rename="djctq10")]
    Djctq10,
    

    /// 12
    ///
    /// "djctq12"
    #[serde(rename="djctq12")]
    Djctq12,
    

    /// 14
    ///
    /// "djctq14"
    #[serde(rename="djctq14")]
    Djctq14,
    

    /// 16
    ///
    /// "djctq16"
    #[serde(rename="djctq16")]
    Djctq16,
    

    /// 18
    ///
    /// "djctq18"
    #[serde(rename="djctq18")]
    Djctq18,
    
    /// "djctqEr"
    #[serde(rename="djctqEr")]
    DjctqEr,
    
    /// "djctqL10"
    #[serde(rename="djctqL10")]
    DjctqL10,
    
    /// "djctqL12"
    #[serde(rename="djctqL12")]
    DjctqL12,
    
    /// "djctqL14"
    #[serde(rename="djctqL14")]
    DjctqL14,
    
    /// "djctqL16"
    #[serde(rename="djctqL16")]
    DjctqL16,
    
    /// "djctqL18"
    #[serde(rename="djctqL18")]
    DjctqL18,
    
    /// "djctq1012"
    #[serde(rename="djctq1012")]
    Djctq1012,
    
    /// "djctq1014"
    #[serde(rename="djctq1014")]
    Djctq1014,
    
    /// "djctq1016"
    #[serde(rename="djctq1016")]
    Djctq1016,
    
    /// "djctq1018"
    #[serde(rename="djctq1018")]
    Djctq1018,
    
    /// "djctq1214"
    #[serde(rename="djctq1214")]
    Djctq1214,
    
    /// "djctq1216"
    #[serde(rename="djctq1216")]
    Djctq1216,
    
    /// "djctq1218"
    #[serde(rename="djctq1218")]
    Djctq1218,
    
    /// "djctq1416"
    #[serde(rename="djctq1416")]
    Djctq1416,
    
    /// "djctq1418"
    #[serde(rename="djctq1418")]
    Djctq1418,
    
    /// "djctq1618"
    #[serde(rename="djctq1618")]
    Djctq1618,
    
    /// "djctqUnrated"
    #[serde(rename="djctqUnrated")]
    DjctqUnrated,
}

impl AsRef<str> for ContentRatingDjctqRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingDjctqRatingEnum::DjctqUnspecified => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq10 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq12 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq14 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq16 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq18 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqEr => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL10 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL12 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL14 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL16 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqL18 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1012 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1014 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1016 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1018 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1214 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1216 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1218 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1416 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1418 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::Djctq1618 => "djctqUnrated",
            ContentRatingDjctqRatingEnum::DjctqUnrated => "djctqUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingDjctqRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentRatingDjctqRatingReasonsEnum {
    
    /// "djctqRatingReasonUnspecified"
    #[serde(rename="djctqRatingReasonUnspecified")]
    DjctqRatingReasonUnspecified,
    

    /// Brazil rating content descriptors. See http://go/brazilratings section F. Violência (Violence)
    ///
    /// "djctqViolence"
    #[serde(rename="djctqViolence")]
    DjctqViolence,
    

    /// Violência extrema (Extreme violence)
    ///
    /// "djctqExtremeViolence"
    #[serde(rename="djctqExtremeViolence")]
    DjctqExtremeViolence,
    

    /// Conteúdo sexual (Sexual content)
    ///
    /// "djctqSexualContent"
    #[serde(rename="djctqSexualContent")]
    DjctqSexualContent,
    

    /// Nudez (Nudity)
    ///
    /// "djctqNudity"
    #[serde(rename="djctqNudity")]
    DjctqNudity,
    

    /// Sexo (Sex)
    ///
    /// "djctqSex"
    #[serde(rename="djctqSex")]
    DjctqSex,
    

    /// Sexo Explícito (Explicit sex)
    ///
    /// "djctqExplicitSex"
    #[serde(rename="djctqExplicitSex")]
    DjctqExplicitSex,
    

    /// Drogas (Drugs)
    ///
    /// "djctqDrugs"
    #[serde(rename="djctqDrugs")]
    DjctqDrugs,
    

    /// Drogas Lícitas (Legal drugs)
    ///
    /// "djctqLegalDrugs"
    #[serde(rename="djctqLegalDrugs")]
    DjctqLegalDrugs,
    

    /// Drogas Ilícitas (Illegal drugs)
    ///
    /// "djctqIllegalDrugs"
    #[serde(rename="djctqIllegalDrugs")]
    DjctqIllegalDrugs,
    

    /// Linguagem Imprópria (Inappropriate language)
    ///
    /// "djctqInappropriateLanguage"
    #[serde(rename="djctqInappropriateLanguage")]
    DjctqInappropriateLanguage,
    

    /// Atos Criminosos (Criminal Acts)
    ///
    /// "djctqCriminalActs"
    #[serde(rename="djctqCriminalActs")]
    DjctqCriminalActs,
    

    /// Conteúdo Impactante (Impacting content)
    ///
    /// "djctqImpactingContent"
    #[serde(rename="djctqImpactingContent")]
    DjctqImpactingContent,
}

impl AsRef<str> for ContentRatingDjctqRatingReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingDjctqRatingReasonsEnum::DjctqRatingReasonUnspecified => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqViolence => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqExtremeViolence => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqSexualContent => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqNudity => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqSex => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqExplicitSex => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqDrugs => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqLegalDrugs => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqIllegalDrugs => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqInappropriateLanguage => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqCriminalActs => "djctqImpactingContent",
            ContentRatingDjctqRatingReasonsEnum::DjctqImpactingContent => "djctqImpactingContent",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingDjctqRatingReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism
pub enum ContentRatingEcbmctRatingEnum {
    
    /// "ecbmctUnspecified"
    #[serde(rename="ecbmctUnspecified")]
    EcbmctUnspecified,
    

    /// G
    ///
    /// "ecbmctG"
    #[serde(rename="ecbmctG")]
    EcbmctG,
    

    /// 7A
    ///
    /// "ecbmct7a"
    #[serde(rename="ecbmct7a")]
    Ecbmct7a,
    

    /// 7+
    ///
    /// "ecbmct7plus"
    #[serde(rename="ecbmct7plus")]
    Ecbmct7plus,
    

    /// 13A
    ///
    /// "ecbmct13a"
    #[serde(rename="ecbmct13a")]
    Ecbmct13a,
    

    /// 13+
    ///
    /// "ecbmct13plus"
    #[serde(rename="ecbmct13plus")]
    Ecbmct13plus,
    

    /// 15A
    ///
    /// "ecbmct15a"
    #[serde(rename="ecbmct15a")]
    Ecbmct15a,
    

    /// 15+
    ///
    /// "ecbmct15plus"
    #[serde(rename="ecbmct15plus")]
    Ecbmct15plus,
    

    /// 18+
    ///
    /// "ecbmct18plus"
    #[serde(rename="ecbmct18plus")]
    Ecbmct18plus,
    
    /// "ecbmctUnrated"
    #[serde(rename="ecbmctUnrated")]
    EcbmctUnrated,
}

impl AsRef<str> for ContentRatingEcbmctRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEcbmctRatingEnum::EcbmctUnspecified => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::EcbmctG => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct7a => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct7plus => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct13a => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct13plus => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct15a => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct15plus => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::Ecbmct18plus => "ecbmctUnrated",
            ContentRatingEcbmctRatingEnum::EcbmctUnrated => "ecbmctUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEcbmctRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Estonia.
pub enum ContentRatingEefilmRatingEnum {
    
    /// "eefilmUnspecified"
    #[serde(rename="eefilmUnspecified")]
    EefilmUnspecified,
    

    /// Pere
    ///
    /// "eefilmPere"
    #[serde(rename="eefilmPere")]
    EefilmPere,
    

    /// L
    ///
    /// "eefilmL"
    #[serde(rename="eefilmL")]
    EefilmL,
    

    /// MS-6
    ///
    /// "eefilmMs6"
    #[serde(rename="eefilmMs6")]
    EefilmMs6,
    

    /// K-6
    ///
    /// "eefilmK6"
    #[serde(rename="eefilmK6")]
    EefilmK6,
    

    /// MS-12
    ///
    /// "eefilmMs12"
    #[serde(rename="eefilmMs12")]
    EefilmMs12,
    

    /// K-12
    ///
    /// "eefilmK12"
    #[serde(rename="eefilmK12")]
    EefilmK12,
    

    /// K-14
    ///
    /// "eefilmK14"
    #[serde(rename="eefilmK14")]
    EefilmK14,
    

    /// K-16
    ///
    /// "eefilmK16"
    #[serde(rename="eefilmK16")]
    EefilmK16,
    
    /// "eefilmUnrated"
    #[serde(rename="eefilmUnrated")]
    EefilmUnrated,
}

impl AsRef<str> for ContentRatingEefilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEefilmRatingEnum::EefilmUnspecified => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmPere => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmL => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmMs6 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmK6 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmMs12 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmK12 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmK14 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmK16 => "eefilmUnrated",
            ContentRatingEefilmRatingEnum::EefilmUnrated => "eefilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEefilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Egypt.
pub enum ContentRatingEgfilmRatingEnum {
    
    /// "egfilmUnspecified"
    #[serde(rename="egfilmUnspecified")]
    EgfilmUnspecified,
    

    /// GN
    ///
    /// "egfilmGn"
    #[serde(rename="egfilmGn")]
    EgfilmGn,
    

    /// 18
    ///
    /// "egfilm18"
    #[serde(rename="egfilm18")]
    Egfilm18,
    

    /// BN
    ///
    /// "egfilmBn"
    #[serde(rename="egfilmBn")]
    EgfilmBn,
    
    /// "egfilmUnrated"
    #[serde(rename="egfilmUnrated")]
    EgfilmUnrated,
}

impl AsRef<str> for ContentRatingEgfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEgfilmRatingEnum::EgfilmUnspecified => "egfilmUnrated",
            ContentRatingEgfilmRatingEnum::EgfilmGn => "egfilmUnrated",
            ContentRatingEgfilmRatingEnum::Egfilm18 => "egfilmUnrated",
            ContentRatingEgfilmRatingEnum::EgfilmBn => "egfilmUnrated",
            ContentRatingEgfilmRatingEnum::EgfilmUnrated => "egfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEgfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Eirin (映倫) rating. Eirin is the Japanese rating system.
pub enum ContentRatingEirinRatingEnum {
    
    /// "eirinUnspecified"
    #[serde(rename="eirinUnspecified")]
    EirinUnspecified,
    

    /// G
    ///
    /// "eirinG"
    #[serde(rename="eirinG")]
    EirinG,
    

    /// PG-12
    ///
    /// "eirinPg12"
    #[serde(rename="eirinPg12")]
    EirinPg12,
    

    /// R15+
    ///
    /// "eirinR15plus"
    #[serde(rename="eirinR15plus")]
    EirinR15plus,
    

    /// R18+
    ///
    /// "eirinR18plus"
    #[serde(rename="eirinR18plus")]
    EirinR18plus,
    
    /// "eirinUnrated"
    #[serde(rename="eirinUnrated")]
    EirinUnrated,
}

impl AsRef<str> for ContentRatingEirinRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingEirinRatingEnum::EirinUnspecified => "eirinUnrated",
            ContentRatingEirinRatingEnum::EirinG => "eirinUnrated",
            ContentRatingEirinRatingEnum::EirinPg12 => "eirinUnrated",
            ContentRatingEirinRatingEnum::EirinR15plus => "eirinUnrated",
            ContentRatingEirinRatingEnum::EirinR18plus => "eirinUnrated",
            ContentRatingEirinRatingEnum::EirinUnrated => "eirinUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingEirinRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Malaysia's Film Censorship Board.
pub enum ContentRatingFcbmRatingEnum {
    
    /// "fcbmUnspecified"
    #[serde(rename="fcbmUnspecified")]
    FcbmUnspecified,
    

    /// U
    ///
    /// "fcbmU"
    #[serde(rename="fcbmU")]
    FcbmU,
    

    /// PG13
    ///
    /// "fcbmPg13"
    #[serde(rename="fcbmPg13")]
    FcbmPg13,
    

    /// P13
    ///
    /// "fcbmP13"
    #[serde(rename="fcbmP13")]
    FcbmP13,
    

    /// 18
    ///
    /// "fcbm18"
    #[serde(rename="fcbm18")]
    Fcbm18,
    

    /// 18SX
    ///
    /// "fcbm18sx"
    #[serde(rename="fcbm18sx")]
    Fcbm18sx,
    

    /// 18PA
    ///
    /// "fcbm18pa"
    #[serde(rename="fcbm18pa")]
    Fcbm18pa,
    

    /// 18SG
    ///
    /// "fcbm18sg"
    #[serde(rename="fcbm18sg")]
    Fcbm18sg,
    

    /// 18PL
    ///
    /// "fcbm18pl"
    #[serde(rename="fcbm18pl")]
    Fcbm18pl,
    
    /// "fcbmUnrated"
    #[serde(rename="fcbmUnrated")]
    FcbmUnrated,
}

impl AsRef<str> for ContentRatingFcbmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFcbmRatingEnum::FcbmUnspecified => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::FcbmU => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::FcbmPg13 => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::FcbmP13 => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::Fcbm18 => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::Fcbm18sx => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::Fcbm18pa => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::Fcbm18sg => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::Fcbm18pl => "fcbmUnrated",
            ContentRatingFcbmRatingEnum::FcbmUnrated => "fcbmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFcbmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration.
pub enum ContentRatingFcoRatingEnum {
    
    /// "fcoUnspecified"
    #[serde(rename="fcoUnspecified")]
    FcoUnspecified,
    

    /// I
    ///
    /// "fcoI"
    #[serde(rename="fcoI")]
    FcoI,
    

    /// IIA
    ///
    /// "fcoIia"
    #[serde(rename="fcoIia")]
    FcoIia,
    

    /// IIB
    ///
    /// "fcoIib"
    #[serde(rename="fcoIib")]
    FcoIib,
    

    /// II
    ///
    /// "fcoIi"
    #[serde(rename="fcoIi")]
    FcoIi,
    

    /// III
    ///
    /// "fcoIii"
    #[serde(rename="fcoIii")]
    FcoIii,
    
    /// "fcoUnrated"
    #[serde(rename="fcoUnrated")]
    FcoUnrated,
}

impl AsRef<str> for ContentRatingFcoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFcoRatingEnum::FcoUnspecified => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoI => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoIia => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoIib => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoIi => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoIii => "fcoUnrated",
            ContentRatingFcoRatingEnum::FcoUnrated => "fcoUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFcoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This property has been deprecated. Use the contentDetails.contentRating.cncRating instead.
pub enum ContentRatingFmocRatingEnum {
    
    /// "fmocUnspecified"
    #[serde(rename="fmocUnspecified")]
    FmocUnspecified,
    

    /// U
    ///
    /// "fmocU"
    #[serde(rename="fmocU")]
    FmocU,
    

    /// 10
    ///
    /// "fmoc10"
    #[serde(rename="fmoc10")]
    Fmoc10,
    

    /// 12
    ///
    /// "fmoc12"
    #[serde(rename="fmoc12")]
    Fmoc12,
    

    /// 16
    ///
    /// "fmoc16"
    #[serde(rename="fmoc16")]
    Fmoc16,
    

    /// 18
    ///
    /// "fmoc18"
    #[serde(rename="fmoc18")]
    Fmoc18,
    

    /// E
    ///
    /// "fmocE"
    #[serde(rename="fmocE")]
    FmocE,
    
    /// "fmocUnrated"
    #[serde(rename="fmocUnrated")]
    FmocUnrated,
}

impl AsRef<str> for ContentRatingFmocRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFmocRatingEnum::FmocUnspecified => "fmocUnrated",
            ContentRatingFmocRatingEnum::FmocU => "fmocUnrated",
            ContentRatingFmocRatingEnum::Fmoc10 => "fmocUnrated",
            ContentRatingFmocRatingEnum::Fmoc12 => "fmocUnrated",
            ContentRatingFmocRatingEnum::Fmoc16 => "fmocUnrated",
            ContentRatingFmocRatingEnum::Fmoc18 => "fmocUnrated",
            ContentRatingFmocRatingEnum::FmocE => "fmocUnrated",
            ContentRatingFmocRatingEnum::FmocUnrated => "fmocUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFmocRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from South Africa's Film and Publication Board.
pub enum ContentRatingFpbRatingEnum {
    
    /// "fpbUnspecified"
    #[serde(rename="fpbUnspecified")]
    FpbUnspecified,
    

    /// A
    ///
    /// "fpbA"
    #[serde(rename="fpbA")]
    FpbA,
    

    /// PG
    ///
    /// "fpbPg"
    #[serde(rename="fpbPg")]
    FpbPg,
    

    /// 7-9PG
    ///
    /// "fpb79Pg"
    #[serde(rename="fpb79Pg")]
    Fpb79Pg,
    

    /// 10-12PG
    ///
    /// "fpb1012Pg"
    #[serde(rename="fpb1012Pg")]
    Fpb1012Pg,
    

    /// 13
    ///
    /// "fpb13"
    #[serde(rename="fpb13")]
    Fpb13,
    

    /// 16
    ///
    /// "fpb16"
    #[serde(rename="fpb16")]
    Fpb16,
    

    /// 18
    ///
    /// "fpb18"
    #[serde(rename="fpb18")]
    Fpb18,
    

    /// X18
    ///
    /// "fpbX18"
    #[serde(rename="fpbX18")]
    FpbX18,
    

    /// XX
    ///
    /// "fpbXx"
    #[serde(rename="fpbXx")]
    FpbXx,
    
    /// "fpbUnrated"
    #[serde(rename="fpbUnrated")]
    FpbUnrated,
    

    /// 10
    ///
    /// "fpb10"
    #[serde(rename="fpb10")]
    Fpb10,
}

impl AsRef<str> for ContentRatingFpbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFpbRatingEnum::FpbUnspecified => "fpb10",
            ContentRatingFpbRatingEnum::FpbA => "fpb10",
            ContentRatingFpbRatingEnum::FpbPg => "fpb10",
            ContentRatingFpbRatingEnum::Fpb79Pg => "fpb10",
            ContentRatingFpbRatingEnum::Fpb1012Pg => "fpb10",
            ContentRatingFpbRatingEnum::Fpb13 => "fpb10",
            ContentRatingFpbRatingEnum::Fpb16 => "fpb10",
            ContentRatingFpbRatingEnum::Fpb18 => "fpb10",
            ContentRatingFpbRatingEnum::FpbX18 => "fpb10",
            ContentRatingFpbRatingEnum::FpbXx => "fpb10",
            ContentRatingFpbRatingEnum::FpbUnrated => "fpb10",
            ContentRatingFpbRatingEnum::Fpb10 => "fpb10",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFpbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentRatingFpbRatingReasonsEnum {
    
    /// "fpbRatingReasonUnspecified"
    #[serde(rename="fpbRatingReasonUnspecified")]
    FpbRatingReasonUnspecified,
    

    /// South Africa rating content descriptors.
    ///
    /// "fpbBlasphemy"
    #[serde(rename="fpbBlasphemy")]
    FpbBlasphemy,
    
    /// "fpbLanguage"
    #[serde(rename="fpbLanguage")]
    FpbLanguage,
    
    /// "fpbNudity"
    #[serde(rename="fpbNudity")]
    FpbNudity,
    
    /// "fpbPrejudice"
    #[serde(rename="fpbPrejudice")]
    FpbPrejudice,
    
    /// "fpbSex"
    #[serde(rename="fpbSex")]
    FpbSex,
    
    /// "fpbViolence"
    #[serde(rename="fpbViolence")]
    FpbViolence,
    
    /// "fpbDrugs"
    #[serde(rename="fpbDrugs")]
    FpbDrugs,
    
    /// "fpbSexualViolence"
    #[serde(rename="fpbSexualViolence")]
    FpbSexualViolence,
    
    /// "fpbHorror"
    #[serde(rename="fpbHorror")]
    FpbHorror,
    
    /// "fpbCriminalTechniques"
    #[serde(rename="fpbCriminalTechniques")]
    FpbCriminalTechniques,
    
    /// "fpbImitativeActsTechniques"
    #[serde(rename="fpbImitativeActsTechniques")]
    FpbImitativeActsTechniques,
}

impl AsRef<str> for ContentRatingFpbRatingReasonsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFpbRatingReasonsEnum::FpbRatingReasonUnspecified => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbBlasphemy => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbLanguage => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbNudity => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbPrejudice => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbSex => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbViolence => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbDrugs => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbSexualViolence => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbHorror => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbCriminalTechniques => "fpbImitativeActsTechniques",
            ContentRatingFpbRatingReasonsEnum::FpbImitativeActsTechniques => "fpbImitativeActsTechniques",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFpbRatingReasonsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating.
pub enum ContentRatingFskRatingEnum {
    
    /// "fskUnspecified"
    #[serde(rename="fskUnspecified")]
    FskUnspecified,
    

    /// FSK 0
    ///
    /// "fsk0"
    #[serde(rename="fsk0")]
    Fsk0,
    

    /// FSK 6
    ///
    /// "fsk6"
    #[serde(rename="fsk6")]
    Fsk6,
    

    /// FSK 12
    ///
    /// "fsk12"
    #[serde(rename="fsk12")]
    Fsk12,
    

    /// FSK 16
    ///
    /// "fsk16"
    #[serde(rename="fsk16")]
    Fsk16,
    

    /// FSK 18
    ///
    /// "fsk18"
    #[serde(rename="fsk18")]
    Fsk18,
    
    /// "fskUnrated"
    #[serde(rename="fskUnrated")]
    FskUnrated,
}

impl AsRef<str> for ContentRatingFskRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingFskRatingEnum::FskUnspecified => "fskUnrated",
            ContentRatingFskRatingEnum::Fsk0 => "fskUnrated",
            ContentRatingFskRatingEnum::Fsk6 => "fskUnrated",
            ContentRatingFskRatingEnum::Fsk12 => "fskUnrated",
            ContentRatingFskRatingEnum::Fsk16 => "fskUnrated",
            ContentRatingFskRatingEnum::Fsk18 => "fskUnrated",
            ContentRatingFskRatingEnum::FskUnrated => "fskUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingFskRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Greece.
pub enum ContentRatingGrfilmRatingEnum {
    
    /// "grfilmUnspecified"
    #[serde(rename="grfilmUnspecified")]
    GrfilmUnspecified,
    

    /// K
    ///
    /// "grfilmK"
    #[serde(rename="grfilmK")]
    GrfilmK,
    

    /// E
    ///
    /// "grfilmE"
    #[serde(rename="grfilmE")]
    GrfilmE,
    

    /// K-12
    ///
    /// "grfilmK12"
    #[serde(rename="grfilmK12")]
    GrfilmK12,
    

    /// K-13
    ///
    /// "grfilmK13"
    #[serde(rename="grfilmK13")]
    GrfilmK13,
    

    /// K-15
    ///
    /// "grfilmK15"
    #[serde(rename="grfilmK15")]
    GrfilmK15,
    

    /// K-17
    ///
    /// "grfilmK17"
    #[serde(rename="grfilmK17")]
    GrfilmK17,
    

    /// K-18
    ///
    /// "grfilmK18"
    #[serde(rename="grfilmK18")]
    GrfilmK18,
    
    /// "grfilmUnrated"
    #[serde(rename="grfilmUnrated")]
    GrfilmUnrated,
}

impl AsRef<str> for ContentRatingGrfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingGrfilmRatingEnum::GrfilmUnspecified => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmE => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK12 => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK13 => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK15 => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK17 => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmK18 => "grfilmUnrated",
            ContentRatingGrfilmRatingEnum::GrfilmUnrated => "grfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingGrfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating.
pub enum ContentRatingIcaaRatingEnum {
    
    /// "icaaUnspecified"
    #[serde(rename="icaaUnspecified")]
    IcaaUnspecified,
    

    /// APTA
    ///
    /// "icaaApta"
    #[serde(rename="icaaApta")]
    IcaaApta,
    

    /// 7
    ///
    /// "icaa7"
    #[serde(rename="icaa7")]
    Icaa7,
    

    /// 12
    ///
    /// "icaa12"
    #[serde(rename="icaa12")]
    Icaa12,
    

    /// 13
    ///
    /// "icaa13"
    #[serde(rename="icaa13")]
    Icaa13,
    

    /// 16
    ///
    /// "icaa16"
    #[serde(rename="icaa16")]
    Icaa16,
    

    /// 18
    ///
    /// "icaa18"
    #[serde(rename="icaa18")]
    Icaa18,
    

    /// X
    ///
    /// "icaaX"
    #[serde(rename="icaaX")]
    IcaaX,
    
    /// "icaaUnrated"
    #[serde(rename="icaaUnrated")]
    IcaaUnrated,
}

impl AsRef<str> for ContentRatingIcaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIcaaRatingEnum::IcaaUnspecified => "icaaUnrated",
            ContentRatingIcaaRatingEnum::IcaaApta => "icaaUnrated",
            ContentRatingIcaaRatingEnum::Icaa7 => "icaaUnrated",
            ContentRatingIcaaRatingEnum::Icaa12 => "icaaUnrated",
            ContentRatingIcaaRatingEnum::Icaa13 => "icaaUnrated",
            ContentRatingIcaaRatingEnum::Icaa16 => "icaaUnrated",
            ContentRatingIcaaRatingEnum::Icaa18 => "icaaUnrated",
            ContentRatingIcaaRatingEnum::IcaaX => "icaaUnrated",
            ContentRatingIcaaRatingEnum::IcaaUnrated => "icaaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIcaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information.
pub enum ContentRatingIfcoRatingEnum {
    
    /// "ifcoUnspecified"
    #[serde(rename="ifcoUnspecified")]
    IfcoUnspecified,
    

    /// G
    ///
    /// "ifcoG"
    #[serde(rename="ifcoG")]
    IfcoG,
    

    /// PG
    ///
    /// "ifcoPg"
    #[serde(rename="ifcoPg")]
    IfcoPg,
    

    /// 12
    ///
    /// "ifco12"
    #[serde(rename="ifco12")]
    Ifco12,
    

    /// 12A
    ///
    /// "ifco12a"
    #[serde(rename="ifco12a")]
    Ifco12a,
    

    /// 15
    ///
    /// "ifco15"
    #[serde(rename="ifco15")]
    Ifco15,
    

    /// 15A
    ///
    /// "ifco15a"
    #[serde(rename="ifco15a")]
    Ifco15a,
    

    /// 16
    ///
    /// "ifco16"
    #[serde(rename="ifco16")]
    Ifco16,
    

    /// 18
    ///
    /// "ifco18"
    #[serde(rename="ifco18")]
    Ifco18,
    
    /// "ifcoUnrated"
    #[serde(rename="ifcoUnrated")]
    IfcoUnrated,
}

impl AsRef<str> for ContentRatingIfcoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIfcoRatingEnum::IfcoUnspecified => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::IfcoG => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::IfcoPg => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco12 => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco12a => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco15 => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco15a => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco16 => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::Ifco18 => "ifcoUnrated",
            ContentRatingIfcoRatingEnum::IfcoUnrated => "ifcoUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIfcoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Israel.
pub enum ContentRatingIlfilmRatingEnum {
    
    /// "ilfilmUnspecified"
    #[serde(rename="ilfilmUnspecified")]
    IlfilmUnspecified,
    

    /// AA
    ///
    /// "ilfilmAa"
    #[serde(rename="ilfilmAa")]
    IlfilmAa,
    

    /// 12
    ///
    /// "ilfilm12"
    #[serde(rename="ilfilm12")]
    Ilfilm12,
    

    /// 14
    ///
    /// "ilfilm14"
    #[serde(rename="ilfilm14")]
    Ilfilm14,
    

    /// 16
    ///
    /// "ilfilm16"
    #[serde(rename="ilfilm16")]
    Ilfilm16,
    

    /// 18
    ///
    /// "ilfilm18"
    #[serde(rename="ilfilm18")]
    Ilfilm18,
    
    /// "ilfilmUnrated"
    #[serde(rename="ilfilmUnrated")]
    IlfilmUnrated,
}

impl AsRef<str> for ContentRatingIlfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIlfilmRatingEnum::IlfilmUnspecified => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::IlfilmAa => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::Ilfilm12 => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::Ilfilm14 => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::Ilfilm16 => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::Ilfilm18 => "ilfilmUnrated",
            ContentRatingIlfilmRatingEnum::IlfilmUnrated => "ilfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIlfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating.
pub enum ContentRatingIncaaRatingEnum {
    
    /// "incaaUnspecified"
    #[serde(rename="incaaUnspecified")]
    IncaaUnspecified,
    

    /// ATP (Apta para todo publico)
    ///
    /// "incaaAtp"
    #[serde(rename="incaaAtp")]
    IncaaAtp,
    

    /// 13 (Solo apta para mayores de 13 años)
    ///
    /// "incaaSam13"
    #[serde(rename="incaaSam13")]
    IncaaSam13,
    

    /// 16 (Solo apta para mayores de 16 años)
    ///
    /// "incaaSam16"
    #[serde(rename="incaaSam16")]
    IncaaSam16,
    

    /// 18 (Solo apta para mayores de 18 años)
    ///
    /// "incaaSam18"
    #[serde(rename="incaaSam18")]
    IncaaSam18,
    

    /// X (Solo apta para mayores de 18 años, de exhibición condicionada)
    ///
    /// "incaaC"
    #[serde(rename="incaaC")]
    IncaaC,
    
    /// "incaaUnrated"
    #[serde(rename="incaaUnrated")]
    IncaaUnrated,
}

impl AsRef<str> for ContentRatingIncaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingIncaaRatingEnum::IncaaUnspecified => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaAtp => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaSam13 => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaSam16 => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaSam18 => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaC => "incaaUnrated",
            ContentRatingIncaaRatingEnum::IncaaUnrated => "incaaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingIncaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Kenya Film Classification Board.
pub enum ContentRatingKfcbRatingEnum {
    
    /// "kfcbUnspecified"
    #[serde(rename="kfcbUnspecified")]
    KfcbUnspecified,
    

    /// GE
    ///
    /// "kfcbG"
    #[serde(rename="kfcbG")]
    KfcbG,
    

    /// PG
    ///
    /// "kfcbPg"
    #[serde(rename="kfcbPg")]
    KfcbPg,
    

    /// 16
    ///
    /// "kfcb16plus"
    #[serde(rename="kfcb16plus")]
    Kfcb16plus,
    

    /// 18
    ///
    /// "kfcbR"
    #[serde(rename="kfcbR")]
    KfcbR,
    
    /// "kfcbUnrated"
    #[serde(rename="kfcbUnrated")]
    KfcbUnrated,
}

impl AsRef<str> for ContentRatingKfcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKfcbRatingEnum::KfcbUnspecified => "kfcbUnrated",
            ContentRatingKfcbRatingEnum::KfcbG => "kfcbUnrated",
            ContentRatingKfcbRatingEnum::KfcbPg => "kfcbUnrated",
            ContentRatingKfcbRatingEnum::Kfcb16plus => "kfcbUnrated",
            ContentRatingKfcbRatingEnum::KfcbR => "kfcbUnrated",
            ContentRatingKfcbRatingEnum::KfcbUnrated => "kfcbUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKfcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands).
pub enum ContentRatingKijkwijzerRatingEnum {
    
    /// "kijkwijzerUnspecified"
    #[serde(rename="kijkwijzerUnspecified")]
    KijkwijzerUnspecified,
    

    /// AL
    ///
    /// "kijkwijzerAl"
    #[serde(rename="kijkwijzerAl")]
    KijkwijzerAl,
    

    /// 6
    ///
    /// "kijkwijzer6"
    #[serde(rename="kijkwijzer6")]
    Kijkwijzer6,
    

    /// 9
    ///
    /// "kijkwijzer9"
    #[serde(rename="kijkwijzer9")]
    Kijkwijzer9,
    

    /// 12
    ///
    /// "kijkwijzer12"
    #[serde(rename="kijkwijzer12")]
    Kijkwijzer12,
    

    /// 16
    ///
    /// "kijkwijzer16"
    #[serde(rename="kijkwijzer16")]
    Kijkwijzer16,
    
    /// "kijkwijzer18"
    #[serde(rename="kijkwijzer18")]
    Kijkwijzer18,
    
    /// "kijkwijzerUnrated"
    #[serde(rename="kijkwijzerUnrated")]
    KijkwijzerUnrated,
}

impl AsRef<str> for ContentRatingKijkwijzerRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKijkwijzerRatingEnum::KijkwijzerUnspecified => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::KijkwijzerAl => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer6 => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer9 => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer12 => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer16 => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::Kijkwijzer18 => "kijkwijzerUnrated",
            ContentRatingKijkwijzerRatingEnum::KijkwijzerUnrated => "kijkwijzerUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKijkwijzerRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea.
pub enum ContentRatingKmrbRatingEnum {
    
    /// "kmrbUnspecified"
    #[serde(rename="kmrbUnspecified")]
    KmrbUnspecified,
    

    /// 전체관람가
    ///
    /// "kmrbAll"
    #[serde(rename="kmrbAll")]
    KmrbAll,
    

    /// 12세 이상 관람가
    ///
    /// "kmrb12plus"
    #[serde(rename="kmrb12plus")]
    Kmrb12plus,
    

    /// 15세 이상 관람가
    ///
    /// "kmrb15plus"
    #[serde(rename="kmrb15plus")]
    Kmrb15plus,
    
    /// "kmrbTeenr"
    #[serde(rename="kmrbTeenr")]
    KmrbTeenr,
    

    /// 청소년 관람불가
    ///
    /// "kmrbR"
    #[serde(rename="kmrbR")]
    KmrbR,
    
    /// "kmrbUnrated"
    #[serde(rename="kmrbUnrated")]
    KmrbUnrated,
}

impl AsRef<str> for ContentRatingKmrbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingKmrbRatingEnum::KmrbUnspecified => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::KmrbAll => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::Kmrb12plus => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::Kmrb15plus => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::KmrbTeenr => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::KmrbR => "kmrbUnrated",
            ContentRatingKmrbRatingEnum::KmrbUnrated => "kmrbUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingKmrbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Indonesia's Lembaga Sensor Film.
pub enum ContentRatingLsfRatingEnum {
    
    /// "lsfUnspecified"
    #[serde(rename="lsfUnspecified")]
    LsfUnspecified,
    

    /// SU
    ///
    /// "lsfSu"
    #[serde(rename="lsfSu")]
    LsfSu,
    

    /// A
    ///
    /// "lsfA"
    #[serde(rename="lsfA")]
    LsfA,
    

    /// BO
    ///
    /// "lsfBo"
    #[serde(rename="lsfBo")]
    LsfBo,
    

    /// 13
    ///
    /// "lsf13"
    #[serde(rename="lsf13")]
    Lsf13,
    

    /// R
    ///
    /// "lsfR"
    #[serde(rename="lsfR")]
    LsfR,
    

    /// 17
    ///
    /// "lsf17"
    #[serde(rename="lsf17")]
    Lsf17,
    

    /// D
    ///
    /// "lsfD"
    #[serde(rename="lsfD")]
    LsfD,
    

    /// 21
    ///
    /// "lsf21"
    #[serde(rename="lsf21")]
    Lsf21,
    
    /// "lsfUnrated"
    #[serde(rename="lsfUnrated")]
    LsfUnrated,
}

impl AsRef<str> for ContentRatingLsfRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingLsfRatingEnum::LsfUnspecified => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfSu => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfA => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfBo => "lsfUnrated",
            ContentRatingLsfRatingEnum::Lsf13 => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfR => "lsfUnrated",
            ContentRatingLsfRatingEnum::Lsf17 => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfD => "lsfUnrated",
            ContentRatingLsfRatingEnum::Lsf21 => "lsfUnrated",
            ContentRatingLsfRatingEnum::LsfUnrated => "lsfUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingLsfRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Malta's Film Age-Classification Board.
pub enum ContentRatingMccaaRatingEnum {
    
    /// "mccaaUnspecified"
    #[serde(rename="mccaaUnspecified")]
    MccaaUnspecified,
    

    /// U
    ///
    /// "mccaaU"
    #[serde(rename="mccaaU")]
    MccaaU,
    

    /// PG
    ///
    /// "mccaaPg"
    #[serde(rename="mccaaPg")]
    MccaaPg,
    

    /// 12A
    ///
    /// "mccaa12a"
    #[serde(rename="mccaa12a")]
    Mccaa12a,
    

    /// 12
    ///
    /// "mccaa12"
    #[serde(rename="mccaa12")]
    Mccaa12,
    

    /// 14 - this rating was removed from the new classification structure introduced in 2013.
    ///
    /// "mccaa14"
    #[serde(rename="mccaa14")]
    Mccaa14,
    

    /// 15
    ///
    /// "mccaa15"
    #[serde(rename="mccaa15")]
    Mccaa15,
    

    /// 16 - this rating was removed from the new classification structure introduced in 2013.
    ///
    /// "mccaa16"
    #[serde(rename="mccaa16")]
    Mccaa16,
    

    /// 18
    ///
    /// "mccaa18"
    #[serde(rename="mccaa18")]
    Mccaa18,
    
    /// "mccaaUnrated"
    #[serde(rename="mccaaUnrated")]
    MccaaUnrated,
}

impl AsRef<str> for ContentRatingMccaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMccaaRatingEnum::MccaaUnspecified => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::MccaaU => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::MccaaPg => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa12a => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa12 => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa14 => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa15 => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa16 => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::Mccaa18 => "mccaaUnrated",
            ContentRatingMccaaRatingEnum::MccaaUnrated => "mccaaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMccaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People.
pub enum ContentRatingMccypRatingEnum {
    
    /// "mccypUnspecified"
    #[serde(rename="mccypUnspecified")]
    MccypUnspecified,
    

    /// A
    ///
    /// "mccypA"
    #[serde(rename="mccypA")]
    MccypA,
    

    /// 7
    ///
    /// "mccyp7"
    #[serde(rename="mccyp7")]
    Mccyp7,
    

    /// 11
    ///
    /// "mccyp11"
    #[serde(rename="mccyp11")]
    Mccyp11,
    

    /// 15
    ///
    /// "mccyp15"
    #[serde(rename="mccyp15")]
    Mccyp15,
    
    /// "mccypUnrated"
    #[serde(rename="mccypUnrated")]
    MccypUnrated,
}

impl AsRef<str> for ContentRatingMccypRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMccypRatingEnum::MccypUnspecified => "mccypUnrated",
            ContentRatingMccypRatingEnum::MccypA => "mccypUnrated",
            ContentRatingMccypRatingEnum::Mccyp7 => "mccypUnrated",
            ContentRatingMccypRatingEnum::Mccyp11 => "mccypUnrated",
            ContentRatingMccypRatingEnum::Mccyp15 => "mccypUnrated",
            ContentRatingMccypRatingEnum::MccypUnrated => "mccypUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMccypRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating system for Vietnam - MCST
pub enum ContentRatingMcstRatingEnum {
    
    /// "mcstUnspecified"
    #[serde(rename="mcstUnspecified")]
    McstUnspecified,
    

    /// P
    ///
    /// "mcstP"
    #[serde(rename="mcstP")]
    McstP,
    

    /// 0
    ///
    /// "mcst0"
    #[serde(rename="mcst0")]
    Mcst0,
    

    /// C13
    ///
    /// "mcstC13"
    #[serde(rename="mcstC13")]
    McstC13,
    

    /// C16
    ///
    /// "mcstC16"
    #[serde(rename="mcstC16")]
    McstC16,
    

    /// 16+
    ///
    /// "mcst16plus"
    #[serde(rename="mcst16plus")]
    Mcst16plus,
    

    /// C18
    ///
    /// "mcstC18"
    #[serde(rename="mcstC18")]
    McstC18,
    

    /// MCST_G_PG
    ///
    /// "mcstGPg"
    #[serde(rename="mcstGPg")]
    McstGPg,
    
    /// "mcstUnrated"
    #[serde(rename="mcstUnrated")]
    McstUnrated,
}

impl AsRef<str> for ContentRatingMcstRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMcstRatingEnum::McstUnspecified => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstP => "mcstUnrated",
            ContentRatingMcstRatingEnum::Mcst0 => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstC13 => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstC16 => "mcstUnrated",
            ContentRatingMcstRatingEnum::Mcst16plus => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstC18 => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstGPg => "mcstUnrated",
            ContentRatingMcstRatingEnum::McstUnrated => "mcstUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMcstRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC).
pub enum ContentRatingMdaRatingEnum {
    
    /// "mdaUnspecified"
    #[serde(rename="mdaUnspecified")]
    MdaUnspecified,
    

    /// G
    ///
    /// "mdaG"
    #[serde(rename="mdaG")]
    MdaG,
    

    /// PG
    ///
    /// "mdaPg"
    #[serde(rename="mdaPg")]
    MdaPg,
    

    /// PG13
    ///
    /// "mdaPg13"
    #[serde(rename="mdaPg13")]
    MdaPg13,
    

    /// NC16
    ///
    /// "mdaNc16"
    #[serde(rename="mdaNc16")]
    MdaNc16,
    

    /// M18
    ///
    /// "mdaM18"
    #[serde(rename="mdaM18")]
    MdaM18,
    

    /// R21
    ///
    /// "mdaR21"
    #[serde(rename="mdaR21")]
    MdaR21,
    
    /// "mdaUnrated"
    #[serde(rename="mdaUnrated")]
    MdaUnrated,
}

impl AsRef<str> for ContentRatingMdaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMdaRatingEnum::MdaUnspecified => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaG => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaPg => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaPg13 => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaNc16 => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaM18 => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaR21 => "mdaUnrated",
            ContentRatingMdaRatingEnum::MdaUnrated => "mdaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMdaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Medietilsynet, the Norwegian Media Authority.
pub enum ContentRatingMedietilsynetRatingEnum {
    
    /// "medietilsynetUnspecified"
    #[serde(rename="medietilsynetUnspecified")]
    MedietilsynetUnspecified,
    

    /// A
    ///
    /// "medietilsynetA"
    #[serde(rename="medietilsynetA")]
    MedietilsynetA,
    

    /// 6
    ///
    /// "medietilsynet6"
    #[serde(rename="medietilsynet6")]
    Medietilsynet6,
    

    /// 7
    ///
    /// "medietilsynet7"
    #[serde(rename="medietilsynet7")]
    Medietilsynet7,
    

    /// 9
    ///
    /// "medietilsynet9"
    #[serde(rename="medietilsynet9")]
    Medietilsynet9,
    

    /// 11
    ///
    /// "medietilsynet11"
    #[serde(rename="medietilsynet11")]
    Medietilsynet11,
    

    /// 12
    ///
    /// "medietilsynet12"
    #[serde(rename="medietilsynet12")]
    Medietilsynet12,
    

    /// 15
    ///
    /// "medietilsynet15"
    #[serde(rename="medietilsynet15")]
    Medietilsynet15,
    

    /// 18
    ///
    /// "medietilsynet18"
    #[serde(rename="medietilsynet18")]
    Medietilsynet18,
    
    /// "medietilsynetUnrated"
    #[serde(rename="medietilsynetUnrated")]
    MedietilsynetUnrated,
}

impl AsRef<str> for ContentRatingMedietilsynetRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMedietilsynetRatingEnum::MedietilsynetUnspecified => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::MedietilsynetA => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet6 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet7 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet9 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet11 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet12 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet15 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::Medietilsynet18 => "medietilsynetUnrated",
            ContentRatingMedietilsynetRatingEnum::MedietilsynetUnrated => "medietilsynetUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMedietilsynetRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute).
pub enum ContentRatingMekuRatingEnum {
    
    /// "mekuUnspecified"
    #[serde(rename="mekuUnspecified")]
    MekuUnspecified,
    

    /// S
    ///
    /// "mekuS"
    #[serde(rename="mekuS")]
    MekuS,
    

    /// 7
    ///
    /// "meku7"
    #[serde(rename="meku7")]
    Meku7,
    

    /// 12
    ///
    /// "meku12"
    #[serde(rename="meku12")]
    Meku12,
    

    /// 16
    ///
    /// "meku16"
    #[serde(rename="meku16")]
    Meku16,
    

    /// 18
    ///
    /// "meku18"
    #[serde(rename="meku18")]
    Meku18,
    
    /// "mekuUnrated"
    #[serde(rename="mekuUnrated")]
    MekuUnrated,
}

impl AsRef<str> for ContentRatingMekuRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMekuRatingEnum::MekuUnspecified => "mekuUnrated",
            ContentRatingMekuRatingEnum::MekuS => "mekuUnrated",
            ContentRatingMekuRatingEnum::Meku7 => "mekuUnrated",
            ContentRatingMekuRatingEnum::Meku12 => "mekuUnrated",
            ContentRatingMekuRatingEnum::Meku16 => "mekuUnrated",
            ContentRatingMekuRatingEnum::Meku18 => "mekuUnrated",
            ContentRatingMekuRatingEnum::MekuUnrated => "mekuUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMekuRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating system for MENA countries, a clone of MPAA. It is needed to prevent titles go live w/o additional QC check, since some of them can be inappropriate for the countries at all. See b/33408548 for more details.
pub enum ContentRatingMenaMpaaRatingEnum {
    
    /// "menaMpaaUnspecified"
    #[serde(rename="menaMpaaUnspecified")]
    MenaMpaaUnspecified,
    

    /// G
    ///
    /// "menaMpaaG"
    #[serde(rename="menaMpaaG")]
    MenaMpaaG,
    

    /// PG
    ///
    /// "menaMpaaPg"
    #[serde(rename="menaMpaaPg")]
    MenaMpaaPg,
    

    /// PG-13
    ///
    /// "menaMpaaPg13"
    #[serde(rename="menaMpaaPg13")]
    MenaMpaaPg13,
    

    /// R
    ///
    /// "menaMpaaR"
    #[serde(rename="menaMpaaR")]
    MenaMpaaR,
    

    /// To keep the same enum values as MPAA's items have, skip NC_17.
    ///
    /// "menaMpaaUnrated"
    #[serde(rename="menaMpaaUnrated")]
    MenaMpaaUnrated,
}

impl AsRef<str> for ContentRatingMenaMpaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMenaMpaaRatingEnum::MenaMpaaUnspecified => "menaMpaaUnrated",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaG => "menaMpaaUnrated",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaPg => "menaMpaaUnrated",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaPg13 => "menaMpaaUnrated",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaR => "menaMpaaUnrated",
            ContentRatingMenaMpaaRatingEnum::MenaMpaaUnrated => "menaMpaaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMenaMpaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy).
pub enum ContentRatingMibacRatingEnum {
    
    /// "mibacUnspecified"
    #[serde(rename="mibacUnspecified")]
    MibacUnspecified,
    
    /// "mibacT"
    #[serde(rename="mibacT")]
    MibacT,
    
    /// "mibacVap"
    #[serde(rename="mibacVap")]
    MibacVap,
    
    /// "mibacVm6"
    #[serde(rename="mibacVm6")]
    MibacVm6,
    
    /// "mibacVm12"
    #[serde(rename="mibacVm12")]
    MibacVm12,
    
    /// "mibacVm14"
    #[serde(rename="mibacVm14")]
    MibacVm14,
    
    /// "mibacVm16"
    #[serde(rename="mibacVm16")]
    MibacVm16,
    
    /// "mibacVm18"
    #[serde(rename="mibacVm18")]
    MibacVm18,
    
    /// "mibacUnrated"
    #[serde(rename="mibacUnrated")]
    MibacUnrated,
}

impl AsRef<str> for ContentRatingMibacRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMibacRatingEnum::MibacUnspecified => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacT => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVap => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVm6 => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVm12 => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVm14 => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVm16 => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacVm18 => "mibacUnrated",
            ContentRatingMibacRatingEnum::MibacUnrated => "mibacUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMibacRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Ministerio de Cultura (Colombia) rating.
pub enum ContentRatingMocRatingEnum {
    
    /// "mocUnspecified"
    #[serde(rename="mocUnspecified")]
    MocUnspecified,
    

    /// E
    ///
    /// "mocE"
    #[serde(rename="mocE")]
    MocE,
    

    /// T
    ///
    /// "mocT"
    #[serde(rename="mocT")]
    MocT,
    

    /// 7
    ///
    /// "moc7"
    #[serde(rename="moc7")]
    Moc7,
    

    /// 12
    ///
    /// "moc12"
    #[serde(rename="moc12")]
    Moc12,
    

    /// 15
    ///
    /// "moc15"
    #[serde(rename="moc15")]
    Moc15,
    

    /// 18
    ///
    /// "moc18"
    #[serde(rename="moc18")]
    Moc18,
    

    /// X
    ///
    /// "mocX"
    #[serde(rename="mocX")]
    MocX,
    

    /// Banned
    ///
    /// "mocBanned"
    #[serde(rename="mocBanned")]
    MocBanned,
    
    /// "mocUnrated"
    #[serde(rename="mocUnrated")]
    MocUnrated,
}

impl AsRef<str> for ContentRatingMocRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMocRatingEnum::MocUnspecified => "mocUnrated",
            ContentRatingMocRatingEnum::MocE => "mocUnrated",
            ContentRatingMocRatingEnum::MocT => "mocUnrated",
            ContentRatingMocRatingEnum::Moc7 => "mocUnrated",
            ContentRatingMocRatingEnum::Moc12 => "mocUnrated",
            ContentRatingMocRatingEnum::Moc15 => "mocUnrated",
            ContentRatingMocRatingEnum::Moc18 => "mocUnrated",
            ContentRatingMocRatingEnum::MocX => "mocUnrated",
            ContentRatingMocRatingEnum::MocBanned => "mocUnrated",
            ContentRatingMocRatingEnum::MocUnrated => "mocUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMocRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Taiwan's Ministry of Culture (文化部).
pub enum ContentRatingMoctwRatingEnum {
    
    /// "moctwUnspecified"
    #[serde(rename="moctwUnspecified")]
    MoctwUnspecified,
    

    /// G
    ///
    /// "moctwG"
    #[serde(rename="moctwG")]
    MoctwG,
    

    /// P
    ///
    /// "moctwP"
    #[serde(rename="moctwP")]
    MoctwP,
    

    /// PG
    ///
    /// "moctwPg"
    #[serde(rename="moctwPg")]
    MoctwPg,
    

    /// R
    ///
    /// "moctwR"
    #[serde(rename="moctwR")]
    MoctwR,
    
    /// "moctwUnrated"
    #[serde(rename="moctwUnrated")]
    MoctwUnrated,
    

    /// R-12
    ///
    /// "moctwR12"
    #[serde(rename="moctwR12")]
    MoctwR12,
    

    /// R-15
    ///
    /// "moctwR15"
    #[serde(rename="moctwR15")]
    MoctwR15,
}

impl AsRef<str> for ContentRatingMoctwRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMoctwRatingEnum::MoctwUnspecified => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwG => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwP => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwPg => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwR => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwUnrated => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwR12 => "moctwR15",
            ContentRatingMoctwRatingEnum::MoctwR15 => "moctwR15",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMoctwRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Motion Picture Association of America (MPAA) rating.
pub enum ContentRatingMpaaRatingEnum {
    
    /// "mpaaUnspecified"
    #[serde(rename="mpaaUnspecified")]
    MpaaUnspecified,
    

    /// G
    ///
    /// "mpaaG"
    #[serde(rename="mpaaG")]
    MpaaG,
    

    /// PG
    ///
    /// "mpaaPg"
    #[serde(rename="mpaaPg")]
    MpaaPg,
    

    /// PG-13
    ///
    /// "mpaaPg13"
    #[serde(rename="mpaaPg13")]
    MpaaPg13,
    

    /// R
    ///
    /// "mpaaR"
    #[serde(rename="mpaaR")]
    MpaaR,
    

    /// NC-17
    ///
    /// "mpaaNc17"
    #[serde(rename="mpaaNc17")]
    MpaaNc17,
    

    /// ! X
    ///
    /// "mpaaX"
    #[serde(rename="mpaaX")]
    MpaaX,
    
    /// "mpaaUnrated"
    #[serde(rename="mpaaUnrated")]
    MpaaUnrated,
}

impl AsRef<str> for ContentRatingMpaaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMpaaRatingEnum::MpaaUnspecified => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaG => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaPg => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaPg13 => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaR => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaNc17 => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaX => "mpaaUnrated",
            ContentRatingMpaaRatingEnum::MpaaUnrated => "mpaaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMpaaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html.
pub enum ContentRatingMpaatRatingEnum {
    
    /// "mpaatUnspecified"
    #[serde(rename="mpaatUnspecified")]
    MpaatUnspecified,
    

    /// GB
    ///
    /// "mpaatGb"
    #[serde(rename="mpaatGb")]
    MpaatGb,
    

    /// RB
    ///
    /// "mpaatRb"
    #[serde(rename="mpaatRb")]
    MpaatRb,
}

impl AsRef<str> for ContentRatingMpaatRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMpaatRatingEnum::MpaatUnspecified => "mpaatRb",
            ContentRatingMpaatRatingEnum::MpaatGb => "mpaatRb",
            ContentRatingMpaatRatingEnum::MpaatRb => "mpaatRb",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMpaatRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Movie and Television Review and Classification Board (Philippines).
pub enum ContentRatingMtrcbRatingEnum {
    
    /// "mtrcbUnspecified"
    #[serde(rename="mtrcbUnspecified")]
    MtrcbUnspecified,
    

    /// G
    ///
    /// "mtrcbG"
    #[serde(rename="mtrcbG")]
    MtrcbG,
    

    /// PG
    ///
    /// "mtrcbPg"
    #[serde(rename="mtrcbPg")]
    MtrcbPg,
    

    /// R-13
    ///
    /// "mtrcbR13"
    #[serde(rename="mtrcbR13")]
    MtrcbR13,
    

    /// R-16
    ///
    /// "mtrcbR16"
    #[serde(rename="mtrcbR16")]
    MtrcbR16,
    

    /// R-18
    ///
    /// "mtrcbR18"
    #[serde(rename="mtrcbR18")]
    MtrcbR18,
    

    /// X
    ///
    /// "mtrcbX"
    #[serde(rename="mtrcbX")]
    MtrcbX,
    
    /// "mtrcbUnrated"
    #[serde(rename="mtrcbUnrated")]
    MtrcbUnrated,
}

impl AsRef<str> for ContentRatingMtrcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingMtrcbRatingEnum::MtrcbUnspecified => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbG => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbPg => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbR13 => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbR16 => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbR18 => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbX => "mtrcbUnrated",
            ContentRatingMtrcbRatingEnum::MtrcbUnrated => "mtrcbUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingMtrcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Maldives National Bureau of Classification.
pub enum ContentRatingNbcRatingEnum {
    
    /// "nbcUnspecified"
    #[serde(rename="nbcUnspecified")]
    NbcUnspecified,
    

    /// G
    ///
    /// "nbcG"
    #[serde(rename="nbcG")]
    NbcG,
    

    /// PG
    ///
    /// "nbcPg"
    #[serde(rename="nbcPg")]
    NbcPg,
    

    /// 12+
    ///
    /// "nbc12plus"
    #[serde(rename="nbc12plus")]
    Nbc12plus,
    

    /// 15+
    ///
    /// "nbc15plus"
    #[serde(rename="nbc15plus")]
    Nbc15plus,
    

    /// 18+
    ///
    /// "nbc18plus"
    #[serde(rename="nbc18plus")]
    Nbc18plus,
    

    /// 18+R
    ///
    /// "nbc18plusr"
    #[serde(rename="nbc18plusr")]
    Nbc18plusr,
    

    /// PU
    ///
    /// "nbcPu"
    #[serde(rename="nbcPu")]
    NbcPu,
    
    /// "nbcUnrated"
    #[serde(rename="nbcUnrated")]
    NbcUnrated,
}

impl AsRef<str> for ContentRatingNbcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNbcRatingEnum::NbcUnspecified => "nbcUnrated",
            ContentRatingNbcRatingEnum::NbcG => "nbcUnrated",
            ContentRatingNbcRatingEnum::NbcPg => "nbcUnrated",
            ContentRatingNbcRatingEnum::Nbc12plus => "nbcUnrated",
            ContentRatingNbcRatingEnum::Nbc15plus => "nbcUnrated",
            ContentRatingNbcRatingEnum::Nbc18plus => "nbcUnrated",
            ContentRatingNbcRatingEnum::Nbc18plusr => "nbcUnrated",
            ContentRatingNbcRatingEnum::NbcPu => "nbcUnrated",
            ContentRatingNbcRatingEnum::NbcUnrated => "nbcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNbcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Poland.
pub enum ContentRatingNbcplRatingEnum {
    
    /// "nbcplUnspecified"
    #[serde(rename="nbcplUnspecified")]
    NbcplUnspecified,
    
    /// "nbcplI"
    #[serde(rename="nbcplI")]
    NbcplI,
    
    /// "nbcplIi"
    #[serde(rename="nbcplIi")]
    NbcplIi,
    
    /// "nbcplIii"
    #[serde(rename="nbcplIii")]
    NbcplIii,
    
    /// "nbcplIv"
    #[serde(rename="nbcplIv")]
    NbcplIv,
    
    /// "nbcpl18plus"
    #[serde(rename="nbcpl18plus")]
    Nbcpl18plus,
    
    /// "nbcplUnrated"
    #[serde(rename="nbcplUnrated")]
    NbcplUnrated,
}

impl AsRef<str> for ContentRatingNbcplRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNbcplRatingEnum::NbcplUnspecified => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::NbcplI => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::NbcplIi => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::NbcplIii => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::NbcplIv => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::Nbcpl18plus => "nbcplUnrated",
            ContentRatingNbcplRatingEnum::NbcplUnrated => "nbcplUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNbcplRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Bulgarian National Film Center.
pub enum ContentRatingNfrcRatingEnum {
    
    /// "nfrcUnspecified"
    #[serde(rename="nfrcUnspecified")]
    NfrcUnspecified,
    

    /// A
    ///
    /// "nfrcA"
    #[serde(rename="nfrcA")]
    NfrcA,
    

    /// B
    ///
    /// "nfrcB"
    #[serde(rename="nfrcB")]
    NfrcB,
    

    /// C
    ///
    /// "nfrcC"
    #[serde(rename="nfrcC")]
    NfrcC,
    

    /// D
    ///
    /// "nfrcD"
    #[serde(rename="nfrcD")]
    NfrcD,
    

    /// X
    ///
    /// "nfrcX"
    #[serde(rename="nfrcX")]
    NfrcX,
    
    /// "nfrcUnrated"
    #[serde(rename="nfrcUnrated")]
    NfrcUnrated,
}

impl AsRef<str> for ContentRatingNfrcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNfrcRatingEnum::NfrcUnspecified => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcA => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcB => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcC => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcD => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcX => "nfrcUnrated",
            ContentRatingNfrcRatingEnum::NfrcUnrated => "nfrcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNfrcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Nigeria's National Film and Video Censors Board.
pub enum ContentRatingNfvcbRatingEnum {
    
    /// "nfvcbUnspecified"
    #[serde(rename="nfvcbUnspecified")]
    NfvcbUnspecified,
    

    /// G
    ///
    /// "nfvcbG"
    #[serde(rename="nfvcbG")]
    NfvcbG,
    

    /// PG
    ///
    /// "nfvcbPg"
    #[serde(rename="nfvcbPg")]
    NfvcbPg,
    

    /// 12
    ///
    /// "nfvcb12"
    #[serde(rename="nfvcb12")]
    Nfvcb12,
    

    /// 12A
    ///
    /// "nfvcb12a"
    #[serde(rename="nfvcb12a")]
    Nfvcb12a,
    

    /// 15
    ///
    /// "nfvcb15"
    #[serde(rename="nfvcb15")]
    Nfvcb15,
    

    /// 18
    ///
    /// "nfvcb18"
    #[serde(rename="nfvcb18")]
    Nfvcb18,
    

    /// RE
    ///
    /// "nfvcbRe"
    #[serde(rename="nfvcbRe")]
    NfvcbRe,
    
    /// "nfvcbUnrated"
    #[serde(rename="nfvcbUnrated")]
    NfvcbUnrated,
}

impl AsRef<str> for ContentRatingNfvcbRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNfvcbRatingEnum::NfvcbUnspecified => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::NfvcbG => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::NfvcbPg => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::Nfvcb12 => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::Nfvcb12a => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::Nfvcb15 => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::Nfvcb18 => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::NfvcbRe => "nfvcbUnrated",
            ContentRatingNfvcbRatingEnum::NfvcbUnrated => "nfvcbUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNfvcbRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia).
pub enum ContentRatingNkclvRatingEnum {
    
    /// "nkclvUnspecified"
    #[serde(rename="nkclvUnspecified")]
    NkclvUnspecified,
    

    /// U
    ///
    /// "nkclvU"
    #[serde(rename="nkclvU")]
    NkclvU,
    

    /// 7+
    ///
    /// "nkclv7plus"
    #[serde(rename="nkclv7plus")]
    Nkclv7plus,
    

    /// 12+
    ///
    /// "nkclv12plus"
    #[serde(rename="nkclv12plus")]
    Nkclv12plus,
    

    /// ! 16+
    ///
    /// "nkclv16plus"
    #[serde(rename="nkclv16plus")]
    Nkclv16plus,
    

    /// 18+
    ///
    /// "nkclv18plus"
    #[serde(rename="nkclv18plus")]
    Nkclv18plus,
    
    /// "nkclvUnrated"
    #[serde(rename="nkclvUnrated")]
    NkclvUnrated,
}

impl AsRef<str> for ContentRatingNkclvRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNkclvRatingEnum::NkclvUnspecified => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::NkclvU => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::Nkclv7plus => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::Nkclv12plus => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::Nkclv16plus => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::Nkclv18plus => "nkclvUnrated",
            ContentRatingNkclvRatingEnum::NkclvUnrated => "nkclvUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNkclvRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The National Media Council ratings system for United Arab Emirates.
pub enum ContentRatingNmcRatingEnum {
    
    /// "nmcUnspecified"
    #[serde(rename="nmcUnspecified")]
    NmcUnspecified,
    

    /// G
    ///
    /// "nmcG"
    #[serde(rename="nmcG")]
    NmcG,
    

    /// PG
    ///
    /// "nmcPg"
    #[serde(rename="nmcPg")]
    NmcPg,
    

    /// PG-13
    ///
    /// "nmcPg13"
    #[serde(rename="nmcPg13")]
    NmcPg13,
    

    /// PG-15
    ///
    /// "nmcPg15"
    #[serde(rename="nmcPg15")]
    NmcPg15,
    

    /// 15+
    ///
    /// "nmc15plus"
    #[serde(rename="nmc15plus")]
    Nmc15plus,
    

    /// 18+
    ///
    /// "nmc18plus"
    #[serde(rename="nmc18plus")]
    Nmc18plus,
    

    /// 18TC
    ///
    /// "nmc18tc"
    #[serde(rename="nmc18tc")]
    Nmc18tc,
    
    /// "nmcUnrated"
    #[serde(rename="nmcUnrated")]
    NmcUnrated,
}

impl AsRef<str> for ContentRatingNmcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingNmcRatingEnum::NmcUnspecified => "nmcUnrated",
            ContentRatingNmcRatingEnum::NmcG => "nmcUnrated",
            ContentRatingNmcRatingEnum::NmcPg => "nmcUnrated",
            ContentRatingNmcRatingEnum::NmcPg13 => "nmcUnrated",
            ContentRatingNmcRatingEnum::NmcPg15 => "nmcUnrated",
            ContentRatingNmcRatingEnum::Nmc15plus => "nmcUnrated",
            ContentRatingNmcRatingEnum::Nmc18plus => "nmcUnrated",
            ContentRatingNmcRatingEnum::Nmc18tc => "nmcUnrated",
            ContentRatingNmcRatingEnum::NmcUnrated => "nmcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingNmcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's Office of Film and Literature Classification (OFLC - New Zealand) rating.
pub enum ContentRatingOflcRatingEnum {
    
    /// "oflcUnspecified"
    #[serde(rename="oflcUnspecified")]
    OflcUnspecified,
    

    /// G
    ///
    /// "oflcG"
    #[serde(rename="oflcG")]
    OflcG,
    

    /// PG
    ///
    /// "oflcPg"
    #[serde(rename="oflcPg")]
    OflcPg,
    

    /// M
    ///
    /// "oflcM"
    #[serde(rename="oflcM")]
    OflcM,
    

    /// R13
    ///
    /// "oflcR13"
    #[serde(rename="oflcR13")]
    OflcR13,
    

    /// R15
    ///
    /// "oflcR15"
    #[serde(rename="oflcR15")]
    OflcR15,
    

    /// R16
    ///
    /// "oflcR16"
    #[serde(rename="oflcR16")]
    OflcR16,
    

    /// R18
    ///
    /// "oflcR18"
    #[serde(rename="oflcR18")]
    OflcR18,
    
    /// "oflcUnrated"
    #[serde(rename="oflcUnrated")]
    OflcUnrated,
    

    /// RP13
    ///
    /// "oflcRp13"
    #[serde(rename="oflcRp13")]
    OflcRp13,
    

    /// RP16
    ///
    /// "oflcRp16"
    #[serde(rename="oflcRp16")]
    OflcRp16,
    

    /// RP18
    ///
    /// "oflcRp18"
    #[serde(rename="oflcRp18")]
    OflcRp18,
}

impl AsRef<str> for ContentRatingOflcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingOflcRatingEnum::OflcUnspecified => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcG => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcPg => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcM => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcR13 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcR15 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcR16 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcR18 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcUnrated => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcRp13 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcRp16 => "oflcRp18",
            ContentRatingOflcRatingEnum::OflcRp18 => "oflcRp18",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingOflcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Peru.
pub enum ContentRatingPefilmRatingEnum {
    
    /// "pefilmUnspecified"
    #[serde(rename="pefilmUnspecified")]
    PefilmUnspecified,
    

    /// PT
    ///
    /// "pefilmPt"
    #[serde(rename="pefilmPt")]
    PefilmPt,
    

    /// PG
    ///
    /// "pefilmPg"
    #[serde(rename="pefilmPg")]
    PefilmPg,
    

    /// 14
    ///
    /// "pefilm14"
    #[serde(rename="pefilm14")]
    Pefilm14,
    

    /// 18
    ///
    /// "pefilm18"
    #[serde(rename="pefilm18")]
    Pefilm18,
    
    /// "pefilmUnrated"
    #[serde(rename="pefilmUnrated")]
    PefilmUnrated,
}

impl AsRef<str> for ContentRatingPefilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingPefilmRatingEnum::PefilmUnspecified => "pefilmUnrated",
            ContentRatingPefilmRatingEnum::PefilmPt => "pefilmUnrated",
            ContentRatingPefilmRatingEnum::PefilmPg => "pefilmUnrated",
            ContentRatingPefilmRatingEnum::Pefilm14 => "pefilmUnrated",
            ContentRatingPefilmRatingEnum::Pefilm18 => "pefilmUnrated",
            ContentRatingPefilmRatingEnum::PefilmUnrated => "pefilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingPefilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film.
pub enum ContentRatingRcnofRatingEnum {
    
    /// "rcnofUnspecified"
    #[serde(rename="rcnofUnspecified")]
    RcnofUnspecified,
    
    /// "rcnofI"
    #[serde(rename="rcnofI")]
    RcnofI,
    
    /// "rcnofIi"
    #[serde(rename="rcnofIi")]
    RcnofIi,
    
    /// "rcnofIii"
    #[serde(rename="rcnofIii")]
    RcnofIii,
    
    /// "rcnofIv"
    #[serde(rename="rcnofIv")]
    RcnofIv,
    
    /// "rcnofV"
    #[serde(rename="rcnofV")]
    RcnofV,
    
    /// "rcnofVi"
    #[serde(rename="rcnofVi")]
    RcnofVi,
    
    /// "rcnofUnrated"
    #[serde(rename="rcnofUnrated")]
    RcnofUnrated,
}

impl AsRef<str> for ContentRatingRcnofRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRcnofRatingEnum::RcnofUnspecified => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofI => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofIi => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofIii => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofIv => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofV => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofVi => "rcnofUnrated",
            ContentRatingRcnofRatingEnum::RcnofUnrated => "rcnofUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRcnofRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Venezuela.
pub enum ContentRatingResorteviolenciaRatingEnum {
    
    /// "resorteviolenciaUnspecified"
    #[serde(rename="resorteviolenciaUnspecified")]
    ResorteviolenciaUnspecified,
    

    /// A
    ///
    /// "resorteviolenciaA"
    #[serde(rename="resorteviolenciaA")]
    ResorteviolenciaA,
    

    /// B
    ///
    /// "resorteviolenciaB"
    #[serde(rename="resorteviolenciaB")]
    ResorteviolenciaB,
    

    /// C
    ///
    /// "resorteviolenciaC"
    #[serde(rename="resorteviolenciaC")]
    ResorteviolenciaC,
    

    /// D
    ///
    /// "resorteviolenciaD"
    #[serde(rename="resorteviolenciaD")]
    ResorteviolenciaD,
    

    /// E
    ///
    /// "resorteviolenciaE"
    #[serde(rename="resorteviolenciaE")]
    ResorteviolenciaE,
    
    /// "resorteviolenciaUnrated"
    #[serde(rename="resorteviolenciaUnrated")]
    ResorteviolenciaUnrated,
}

impl AsRef<str> for ContentRatingResorteviolenciaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnspecified => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaA => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaB => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaC => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaD => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaE => "resorteviolenciaUnrated",
            ContentRatingResorteviolenciaRatingEnum::ResorteviolenciaUnrated => "resorteviolenciaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingResorteviolenciaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's General Directorate of Radio, Television and Cinematography (Mexico) rating.
pub enum ContentRatingRtcRatingEnum {
    
    /// "rtcUnspecified"
    #[serde(rename="rtcUnspecified")]
    RtcUnspecified,
    

    /// AA
    ///
    /// "rtcAa"
    #[serde(rename="rtcAa")]
    RtcAa,
    

    /// A
    ///
    /// "rtcA"
    #[serde(rename="rtcA")]
    RtcA,
    

    /// B
    ///
    /// "rtcB"
    #[serde(rename="rtcB")]
    RtcB,
    

    /// B15
    ///
    /// "rtcB15"
    #[serde(rename="rtcB15")]
    RtcB15,
    

    /// C
    ///
    /// "rtcC"
    #[serde(rename="rtcC")]
    RtcC,
    

    /// D
    ///
    /// "rtcD"
    #[serde(rename="rtcD")]
    RtcD,
    
    /// "rtcUnrated"
    #[serde(rename="rtcUnrated")]
    RtcUnrated,
}

impl AsRef<str> for ContentRatingRtcRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRtcRatingEnum::RtcUnspecified => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcAa => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcA => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcB => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcB15 => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcC => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcD => "rtcUnrated",
            ContentRatingRtcRatingEnum::RtcUnrated => "rtcUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRtcRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Ireland's Raidió Teilifís Éireann.
pub enum ContentRatingRteRatingEnum {
    
    /// "rteUnspecified"
    #[serde(rename="rteUnspecified")]
    RteUnspecified,
    

    /// GA
    ///
    /// "rteGa"
    #[serde(rename="rteGa")]
    RteGa,
    

    /// CH
    ///
    /// "rteCh"
    #[serde(rename="rteCh")]
    RteCh,
    

    /// PS
    ///
    /// "rtePs"
    #[serde(rename="rtePs")]
    RtePs,
    

    /// MA
    ///
    /// "rteMa"
    #[serde(rename="rteMa")]
    RteMa,
    
    /// "rteUnrated"
    #[serde(rename="rteUnrated")]
    RteUnrated,
}

impl AsRef<str> for ContentRatingRteRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRteRatingEnum::RteUnspecified => "rteUnrated",
            ContentRatingRteRatingEnum::RteGa => "rteUnrated",
            ContentRatingRteRatingEnum::RteCh => "rteUnrated",
            ContentRatingRteRatingEnum::RtePs => "rteUnrated",
            ContentRatingRteRatingEnum::RteMa => "rteUnrated",
            ContentRatingRteRatingEnum::RteUnrated => "rteUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRteRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's National Film Registry of the Russian Federation (MKRF - Russia) rating.
pub enum ContentRatingRussiaRatingEnum {
    
    /// "russiaUnspecified"
    #[serde(rename="russiaUnspecified")]
    RussiaUnspecified,
    

    /// 0+
    ///
    /// "russia0"
    #[serde(rename="russia0")]
    Russia0,
    

    /// 6+
    ///
    /// "russia6"
    #[serde(rename="russia6")]
    Russia6,
    

    /// 12+
    ///
    /// "russia12"
    #[serde(rename="russia12")]
    Russia12,
    

    /// 16+
    ///
    /// "russia16"
    #[serde(rename="russia16")]
    Russia16,
    

    /// 18+
    ///
    /// "russia18"
    #[serde(rename="russia18")]
    Russia18,
    
    /// "russiaUnrated"
    #[serde(rename="russiaUnrated")]
    RussiaUnrated,
}

impl AsRef<str> for ContentRatingRussiaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingRussiaRatingEnum::RussiaUnspecified => "russiaUnrated",
            ContentRatingRussiaRatingEnum::Russia0 => "russiaUnrated",
            ContentRatingRussiaRatingEnum::Russia6 => "russiaUnrated",
            ContentRatingRussiaRatingEnum::Russia12 => "russiaUnrated",
            ContentRatingRussiaRatingEnum::Russia16 => "russiaUnrated",
            ContentRatingRussiaRatingEnum::Russia18 => "russiaUnrated",
            ContentRatingRussiaRatingEnum::RussiaUnrated => "russiaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingRussiaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Slovakia.
pub enum ContentRatingSkfilmRatingEnum {
    
    /// "skfilmUnspecified"
    #[serde(rename="skfilmUnspecified")]
    SkfilmUnspecified,
    

    /// G
    ///
    /// "skfilmG"
    #[serde(rename="skfilmG")]
    SkfilmG,
    

    /// P2
    ///
    /// "skfilmP2"
    #[serde(rename="skfilmP2")]
    SkfilmP2,
    

    /// P5
    ///
    /// "skfilmP5"
    #[serde(rename="skfilmP5")]
    SkfilmP5,
    

    /// P8
    ///
    /// "skfilmP8"
    #[serde(rename="skfilmP8")]
    SkfilmP8,
    
    /// "skfilmUnrated"
    #[serde(rename="skfilmUnrated")]
    SkfilmUnrated,
}

impl AsRef<str> for ContentRatingSkfilmRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSkfilmRatingEnum::SkfilmUnspecified => "skfilmUnrated",
            ContentRatingSkfilmRatingEnum::SkfilmG => "skfilmUnrated",
            ContentRatingSkfilmRatingEnum::SkfilmP2 => "skfilmUnrated",
            ContentRatingSkfilmRatingEnum::SkfilmP5 => "skfilmUnrated",
            ContentRatingSkfilmRatingEnum::SkfilmP8 => "skfilmUnrated",
            ContentRatingSkfilmRatingEnum::SkfilmUnrated => "skfilmUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSkfilmRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating in Iceland.
pub enum ContentRatingSmaisRatingEnum {
    
    /// "smaisUnspecified"
    #[serde(rename="smaisUnspecified")]
    SmaisUnspecified,
    

    /// L
    ///
    /// "smaisL"
    #[serde(rename="smaisL")]
    SmaisL,
    

    /// 7
    ///
    /// "smais7"
    #[serde(rename="smais7")]
    Smais7,
    

    /// 12
    ///
    /// "smais12"
    #[serde(rename="smais12")]
    Smais12,
    

    /// 14
    ///
    /// "smais14"
    #[serde(rename="smais14")]
    Smais14,
    

    /// 16
    ///
    /// "smais16"
    #[serde(rename="smais16")]
    Smais16,
    

    /// 18
    ///
    /// "smais18"
    #[serde(rename="smais18")]
    Smais18,
    
    /// "smaisUnrated"
    #[serde(rename="smaisUnrated")]
    SmaisUnrated,
}

impl AsRef<str> for ContentRatingSmaisRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSmaisRatingEnum::SmaisUnspecified => "smaisUnrated",
            ContentRatingSmaisRatingEnum::SmaisL => "smaisUnrated",
            ContentRatingSmaisRatingEnum::Smais7 => "smaisUnrated",
            ContentRatingSmaisRatingEnum::Smais12 => "smaisUnrated",
            ContentRatingSmaisRatingEnum::Smais14 => "smaisUnrated",
            ContentRatingSmaisRatingEnum::Smais16 => "smaisUnrated",
            ContentRatingSmaisRatingEnum::Smais18 => "smaisUnrated",
            ContentRatingSmaisRatingEnum::SmaisUnrated => "smaisUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSmaisRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's rating from Statens medieråd (Sweden's National Media Council).
pub enum ContentRatingSmsaRatingEnum {
    
    /// "smsaUnspecified"
    #[serde(rename="smsaUnspecified")]
    SmsaUnspecified,
    

    /// All ages
    ///
    /// "smsaA"
    #[serde(rename="smsaA")]
    SmsaA,
    

    /// 7
    ///
    /// "smsa7"
    #[serde(rename="smsa7")]
    Smsa7,
    

    /// 11
    ///
    /// "smsa11"
    #[serde(rename="smsa11")]
    Smsa11,
    

    /// 15
    ///
    /// "smsa15"
    #[serde(rename="smsa15")]
    Smsa15,
    
    /// "smsaUnrated"
    #[serde(rename="smsaUnrated")]
    SmsaUnrated,
}

impl AsRef<str> for ContentRatingSmsaRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingSmsaRatingEnum::SmsaUnspecified => "smsaUnrated",
            ContentRatingSmsaRatingEnum::SmsaA => "smsaUnrated",
            ContentRatingSmsaRatingEnum::Smsa7 => "smsaUnrated",
            ContentRatingSmsaRatingEnum::Smsa11 => "smsaUnrated",
            ContentRatingSmsaRatingEnum::Smsa15 => "smsaUnrated",
            ContentRatingSmsaRatingEnum::SmsaUnrated => "smsaUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingSmsaRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's TV Parental Guidelines (TVPG) rating.
pub enum ContentRatingTvpgRatingEnum {
    
    /// "tvpgUnspecified"
    #[serde(rename="tvpgUnspecified")]
    TvpgUnspecified,
    

    /// TV-Y
    ///
    /// "tvpgY"
    #[serde(rename="tvpgY")]
    TvpgY,
    

    /// TV-Y7
    ///
    /// "tvpgY7"
    #[serde(rename="tvpgY7")]
    TvpgY7,
    

    /// TV-Y7-FV
    ///
    /// "tvpgY7Fv"
    #[serde(rename="tvpgY7Fv")]
    TvpgY7Fv,
    

    /// TV-G
    ///
    /// "tvpgG"
    #[serde(rename="tvpgG")]
    TvpgG,
    

    /// TV-PG
    ///
    /// "tvpgPg"
    #[serde(rename="tvpgPg")]
    TvpgPg,
    

    /// TV-14
    ///
    /// "pg14"
    #[serde(rename="pg14")]
    Pg14,
    

    /// TV-MA
    ///
    /// "tvpgMa"
    #[serde(rename="tvpgMa")]
    TvpgMa,
    
    /// "tvpgUnrated"
    #[serde(rename="tvpgUnrated")]
    TvpgUnrated,
}

impl AsRef<str> for ContentRatingTvpgRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingTvpgRatingEnum::TvpgUnspecified => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgY => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgY7 => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgY7Fv => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgG => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgPg => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::Pg14 => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgMa => "tvpgUnrated",
            ContentRatingTvpgRatingEnum::TvpgUnrated => "tvpgUnrated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingTvpgRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// A rating that YouTube uses to identify age-restricted content.
pub enum ContentRatingYtRatingEnum {
    
    /// "ytUnspecified"
    #[serde(rename="ytUnspecified")]
    YtUnspecified,
    
    /// "ytAgeRestricted"
    #[serde(rename="ytAgeRestricted")]
    YtAgeRestricted,
}

impl AsRef<str> for ContentRatingYtRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ContentRatingYtRatingEnum::YtUnspecified => "ytAgeRestricted",
            ContentRatingYtRatingEnum::YtAgeRestricted => "ytAgeRestricted",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ContentRatingYtRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CuepointCueTypeEnum {
    
    /// "cueTypeUnspecified"
    #[serde(rename="cueTypeUnspecified")]
    CueTypeUnspecified,
    
    /// "cueTypeAd"
    #[serde(rename="cueTypeAd")]
    CueTypeAd,
}

impl AsRef<str> for CuepointCueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CuepointCueTypeEnum::CueTypeUnspecified => "cueTypeAd",
            CuepointCueTypeEnum::CueTypeAd => "cueTypeAd",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CuepointCueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes in which corner of the video the visual widget will appear.
pub enum InvideoPositionCornerPositionEnum {
    
    /// "topLeft"
    #[serde(rename="topLeft")]
    TopLeft,
    
    /// "topRight"
    #[serde(rename="topRight")]
    TopRight,
    
    /// "bottomLeft"
    #[serde(rename="bottomLeft")]
    BottomLeft,
    
    /// "bottomRight"
    #[serde(rename="bottomRight")]
    BottomRight,
}

impl AsRef<str> for InvideoPositionCornerPositionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoPositionCornerPositionEnum::TopLeft => "bottomRight",
            InvideoPositionCornerPositionEnum::TopRight => "bottomRight",
            InvideoPositionCornerPositionEnum::BottomLeft => "bottomRight",
            InvideoPositionCornerPositionEnum::BottomRight => "bottomRight",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoPositionCornerPositionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Defines the position type.
pub enum InvideoPositionTypeEnum {
    
    /// "corner"
    #[serde(rename="corner")]
    Corner,
}

impl AsRef<str> for InvideoPositionTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoPositionTypeEnum::Corner => "corner",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoPositionTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video.
pub enum InvideoTimingTypeEnum {
    
    /// "offsetFromStart"
    #[serde(rename="offsetFromStart")]
    OffsetFromStart,
    
    /// "offsetFromEnd"
    #[serde(rename="offsetFromEnd")]
    OffsetFromEnd,
}

impl AsRef<str> for InvideoTimingTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            InvideoTimingTypeEnum::OffsetFromStart => "offsetFromEnd",
            InvideoTimingTypeEnum::OffsetFromEnd => "offsetFromEnd",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a InvideoTimingTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    
    /// "closedCaptionsTypeUnspecified"
    #[serde(rename="closedCaptionsTypeUnspecified")]
    ClosedCaptionsTypeUnspecified,
    
    /// "closedCaptionsDisabled"
    #[serde(rename="closedCaptionsDisabled")]
    ClosedCaptionsDisabled,
    
    /// "closedCaptionsHttpPost"
    #[serde(rename="closedCaptionsHttpPost")]
    ClosedCaptionsHttpPost,
    
    /// "closedCaptionsEmbedded"
    #[serde(rename="closedCaptionsEmbedded")]
    ClosedCaptionsEmbedded,
}

impl AsRef<str> for LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsTypeUnspecified => "closedCaptionsEmbedded",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsDisabled => "closedCaptionsEmbedded",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsHttpPost => "closedCaptionsEmbedded",
            LiveBroadcastContentDetailClosedCaptionsTypeEnum::ClosedCaptionsEmbedded => "closedCaptionsEmbedded",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailClosedCaptionsTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted.
pub enum LiveBroadcastContentDetailLatencyPreferenceEnum {
    
    /// "latencyPreferenceUnspecified"
    #[serde(rename="latencyPreferenceUnspecified")]
    LatencyPreferenceUnspecified,
    

    /// Best for: highest quality viewer playbacks and higher resolutions.
    ///
    /// "normal"
    #[serde(rename="normal")]
    Normal,
    

    /// Best for: near real-time interaction, with minimal playback buffering.
    ///
    /// "low"
    #[serde(rename="low")]
    Low,
    

    /// Best for: real-time interaction Does not support: Closed captions, 1440p, and 4k resolutions
    ///
    /// "ultraLow"
    #[serde(rename="ultraLow")]
    UltraLow,
}

impl AsRef<str> for LiveBroadcastContentDetailLatencyPreferenceEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailLatencyPreferenceEnum::LatencyPreferenceUnspecified => "ultraLow",
            LiveBroadcastContentDetailLatencyPreferenceEnum::Normal => "ultraLow",
            LiveBroadcastContentDetailLatencyPreferenceEnum::Low => "ultraLow",
            LiveBroadcastContentDetailLatencyPreferenceEnum::UltraLow => "ultraLow",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailLatencyPreferenceEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The projection format of this broadcast. This defaults to rectangular.
pub enum LiveBroadcastContentDetailProjectionEnum {
    
    /// "projectionUnspecified"
    #[serde(rename="projectionUnspecified")]
    ProjectionUnspecified,
    
    /// "rectangular"
    #[serde(rename="rectangular")]
    Rectangular,
    
    /// "360"
    #[serde(rename="360")]
    _360,
    
    /// "mesh"
    #[serde(rename="mesh")]
    Mesh,
}

impl AsRef<str> for LiveBroadcastContentDetailProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailProjectionEnum::ProjectionUnspecified => "mesh",
            LiveBroadcastContentDetailProjectionEnum::Rectangular => "mesh",
            LiveBroadcastContentDetailProjectionEnum::_360 => "mesh",
            LiveBroadcastContentDetailProjectionEnum::Mesh => "mesh",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The 3D stereo layout of this broadcast. This defaults to mono.
pub enum LiveBroadcastContentDetailStereoLayoutEnum {
    
    /// "stereoLayoutUnspecified"
    #[serde(rename="stereoLayoutUnspecified")]
    StereoLayoutUnspecified,
    
    /// "mono"
    #[serde(rename="mono")]
    Mono,
    
    /// "leftRight"
    #[serde(rename="leftRight")]
    LeftRight,
    
    /// "topBottom"
    #[serde(rename="topBottom")]
    TopBottom,
}

impl AsRef<str> for LiveBroadcastContentDetailStereoLayoutEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastContentDetailStereoLayoutEnum::StereoLayoutUnspecified => "topBottom",
            LiveBroadcastContentDetailStereoLayoutEnum::Mono => "topBottom",
            LiveBroadcastContentDetailStereoLayoutEnum::LeftRight => "topBottom",
            LiveBroadcastContentDetailStereoLayoutEnum::TopBottom => "topBottom",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastContentDetailStereoLayoutEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method.
pub enum LiveBroadcastStatuLifeCycleStatusEnum {
    

    /// No value or the value is unknown.
    ///
    /// "lifeCycleStatusUnspecified"
    #[serde(rename="lifeCycleStatusUnspecified")]
    LifeCycleStatusUnspecified,
    

    /// Incomplete settings, but otherwise valid
    ///
    /// "created"
    #[serde(rename="created")]
    Created,
    

    /// Complete settings
    ///
    /// "ready"
    #[serde(rename="ready")]
    Ready,
    

    /// Visible only to partner, may need special UI treatment
    ///
    /// "testing"
    #[serde(rename="testing")]
    Testing,
    

    /// Viper is recording; this means the "clock" is running
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The broadcast is finished.
    ///
    /// "complete"
    #[serde(rename="complete")]
    Complete,
    

    /// This broadcast was removed by admin action
    ///
    /// "revoked"
    #[serde(rename="revoked")]
    Revoked,
    

    /// Transition into TESTING has been requested
    ///
    /// "testStarting"
    #[serde(rename="testStarting")]
    TestStarting,
    

    /// Transition into LIVE has been requested
    ///
    /// "liveStarting"
    #[serde(rename="liveStarting")]
    LiveStarting,
}

impl AsRef<str> for LiveBroadcastStatuLifeCycleStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatuLifeCycleStatusEnum::LifeCycleStatusUnspecified => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Created => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Ready => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Testing => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Live => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Complete => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::Revoked => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::TestStarting => "liveStarting",
            LiveBroadcastStatuLifeCycleStatusEnum::LiveStarting => "liveStarting",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatuLifeCycleStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Priority of the live broadcast event (internal state).
pub enum LiveBroadcastStatuLiveBroadcastPriorityEnum {
    
    /// "liveBroadcastPriorityUnspecified"
    #[serde(rename="liveBroadcastPriorityUnspecified")]
    LiveBroadcastPriorityUnspecified,
    

    /// Low priority broadcast: for low view count HoAs or other low priority broadcasts.
    ///
    /// "low"
    #[serde(rename="low")]
    Low,
    

    /// Normal priority broadcast: for regular HoAs and broadcasts.
    ///
    /// "normal"
    #[serde(rename="normal")]
    Normal,
    

    /// High priority broadcast: for high profile HoAs, like PixelCorp ones.
    ///
    /// "high"
    #[serde(rename="high")]
    High,
}

impl AsRef<str> for LiveBroadcastStatuLiveBroadcastPriorityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatuLiveBroadcastPriorityEnum::LiveBroadcastPriorityUnspecified => "high",
            LiveBroadcastStatuLiveBroadcastPriorityEnum::Low => "high",
            LiveBroadcastStatuLiveBroadcastPriorityEnum::Normal => "high",
            LiveBroadcastStatuLiveBroadcastPriorityEnum::High => "high",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatuLiveBroadcastPriorityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource.
pub enum LiveBroadcastStatuPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for LiveBroadcastStatuPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatuPrivacyStatusEnum::Public => "private",
            LiveBroadcastStatuPrivacyStatusEnum::Unlisted => "private",
            LiveBroadcastStatuPrivacyStatusEnum::Private => "private",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatuPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The broadcast's recording status.
pub enum LiveBroadcastStatuRecordingStatusEnum {
    

    /// No value or the value is unknown.
    ///
    /// "liveBroadcastRecordingStatusUnspecified"
    #[serde(rename="liveBroadcastRecordingStatusUnspecified")]
    LiveBroadcastRecordingStatusUnspecified,
    

    /// The recording has not yet been started.
    ///
    /// "notRecording"
    #[serde(rename="notRecording")]
    NotRecording,
    

    /// The recording is currently on.
    ///
    /// "recording"
    #[serde(rename="recording")]
    Recording,
    

    /// The recording is completed, and cannot be started again.
    ///
    /// "recorded"
    #[serde(rename="recorded")]
    Recorded,
}

impl AsRef<str> for LiveBroadcastStatuRecordingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastStatuRecordingStatusEnum::LiveBroadcastRecordingStatusUnspecified => "recorded",
            LiveBroadcastStatuRecordingStatusEnum::NotRecording => "recorded",
            LiveBroadcastStatuRecordingStatusEnum::Recording => "recorded",
            LiveBroadcastStatuRecordingStatusEnum::Recorded => "recorded",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastStatuRecordingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of ban.
pub enum LiveChatBanSnippetTypeEnum {
    

    /// An invalid ban type.
    ///
    /// "liveChatBanTypeUnspecified"
    #[serde(rename="liveChatBanTypeUnspecified")]
    LiveChatBanTypeUnspecified,
    

    /// A permanent ban.
    ///
    /// "permanent"
    #[serde(rename="permanent")]
    Permanent,
    

    /// A temporary ban.
    ///
    /// "temporary"
    #[serde(rename="temporary")]
    Temporary,
}

impl AsRef<str> for LiveChatBanSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatBanSnippetTypeEnum::LiveChatBanTypeUnspecified => "temporary",
            LiveChatBanSnippetTypeEnum::Permanent => "temporary",
            LiveChatBanSnippetTypeEnum::Temporary => "temporary",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatBanSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of message, this will always be present, it determines the contents of the message as well as which fields will be present.
pub enum LiveChatMessageSnippetTypeEnum {
    
    /// "invalidType"
    #[serde(rename="invalidType")]
    InvalidType,
    
    /// "textMessageEvent"
    #[serde(rename="textMessageEvent")]
    TextMessageEvent,
    
    /// "tombstone"
    #[serde(rename="tombstone")]
    Tombstone,
    
    /// "fanFundingEvent"
    #[serde(rename="fanFundingEvent")]
    FanFundingEvent,
    
    /// "chatEndedEvent"
    #[serde(rename="chatEndedEvent")]
    ChatEndedEvent,
    
    /// "sponsorOnlyModeStartedEvent"
    #[serde(rename="sponsorOnlyModeStartedEvent")]
    SponsorOnlyModeStartedEvent,
    
    /// "sponsorOnlyModeEndedEvent"
    #[serde(rename="sponsorOnlyModeEndedEvent")]
    SponsorOnlyModeEndedEvent,
    
    /// "newSponsorEvent"
    #[serde(rename="newSponsorEvent")]
    NewSponsorEvent,
    
    /// "memberMilestoneChatEvent"
    #[serde(rename="memberMilestoneChatEvent")]
    MemberMilestoneChatEvent,
    
    /// "membershipGiftingEvent"
    #[serde(rename="membershipGiftingEvent")]
    MembershipGiftingEvent,
    
    /// "giftMembershipReceivedEvent"
    #[serde(rename="giftMembershipReceivedEvent")]
    GiftMembershipReceivedEvent,
    
    /// "messageDeletedEvent"
    #[serde(rename="messageDeletedEvent")]
    MessageDeletedEvent,
    
    /// "messageRetractedEvent"
    #[serde(rename="messageRetractedEvent")]
    MessageRetractedEvent,
    
    /// "userBannedEvent"
    #[serde(rename="userBannedEvent")]
    UserBannedEvent,
    
    /// "superChatEvent"
    #[serde(rename="superChatEvent")]
    SuperChatEvent,
    
    /// "superStickerEvent"
    #[serde(rename="superStickerEvent")]
    SuperStickerEvent,
}

impl AsRef<str> for LiveChatMessageSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatMessageSnippetTypeEnum::InvalidType => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::TextMessageEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::Tombstone => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::FanFundingEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::ChatEndedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::SponsorOnlyModeStartedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::SponsorOnlyModeEndedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::NewSponsorEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::MemberMilestoneChatEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::MembershipGiftingEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::GiftMembershipReceivedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::MessageDeletedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::MessageRetractedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::UserBannedEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::SuperChatEvent => "superStickerEvent",
            LiveChatMessageSnippetTypeEnum::SuperStickerEvent => "superStickerEvent",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatMessageSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of ban.
pub enum LiveChatUserBannedMessageDetailBanTypeEnum {
    
    /// "permanent"
    #[serde(rename="permanent")]
    Permanent,
    
    /// "temporary"
    #[serde(rename="temporary")]
    Temporary,
}

impl AsRef<str> for LiveChatUserBannedMessageDetailBanTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveChatUserBannedMessageDetailBanTypeEnum::Permanent => "temporary",
            LiveChatUserBannedMessageDetailBanTypeEnum::Temporary => "temporary",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveChatUserBannedMessageDetailBanTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// How severe this issue is to the stream.
pub enum LiveStreamConfigurationIssueSeverityEnum {
    
    /// "info"
    #[serde(rename="info")]
    Info,
    
    /// "warning"
    #[serde(rename="warning")]
    Warning,
    
    /// "error"
    #[serde(rename="error")]
    Error,
}

impl AsRef<str> for LiveStreamConfigurationIssueSeverityEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamConfigurationIssueSeverityEnum::Info => "error",
            LiveStreamConfigurationIssueSeverityEnum::Warning => "error",
            LiveStreamConfigurationIssueSeverityEnum::Error => "error",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamConfigurationIssueSeverityEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The kind of error happening.
pub enum LiveStreamConfigurationIssueTypeEnum {
    
    /// "gopSizeOver"
    #[serde(rename="gopSizeOver")]
    GopSizeOver,
    
    /// "gopSizeLong"
    #[serde(rename="gopSizeLong")]
    GopSizeLong,
    
    /// "gopSizeShort"
    #[serde(rename="gopSizeShort")]
    GopSizeShort,
    
    /// "openGop"
    #[serde(rename="openGop")]
    OpenGop,
    
    /// "badContainer"
    #[serde(rename="badContainer")]
    BadContainer,
    
    /// "audioBitrateHigh"
    #[serde(rename="audioBitrateHigh")]
    AudioBitrateHigh,
    
    /// "audioBitrateLow"
    #[serde(rename="audioBitrateLow")]
    AudioBitrateLow,
    
    /// "audioSampleRate"
    #[serde(rename="audioSampleRate")]
    AudioSampleRate,
    
    /// "bitrateHigh"
    #[serde(rename="bitrateHigh")]
    BitrateHigh,
    
    /// "bitrateLow"
    #[serde(rename="bitrateLow")]
    BitrateLow,
    
    /// "audioCodec"
    #[serde(rename="audioCodec")]
    AudioCodec,
    
    /// "videoCodec"
    #[serde(rename="videoCodec")]
    VideoCodec,
    
    /// "noAudioStream"
    #[serde(rename="noAudioStream")]
    NoAudioStream,
    
    /// "noVideoStream"
    #[serde(rename="noVideoStream")]
    NoVideoStream,
    
    /// "multipleVideoStreams"
    #[serde(rename="multipleVideoStreams")]
    MultipleVideoStreams,
    
    /// "multipleAudioStreams"
    #[serde(rename="multipleAudioStreams")]
    MultipleAudioStreams,
    
    /// "audioTooManyChannels"
    #[serde(rename="audioTooManyChannels")]
    AudioTooManyChannels,
    
    /// "interlacedVideo"
    #[serde(rename="interlacedVideo")]
    InterlacedVideo,
    
    /// "frameRateHigh"
    #[serde(rename="frameRateHigh")]
    FrameRateHigh,
    
    /// "resolutionMismatch"
    #[serde(rename="resolutionMismatch")]
    ResolutionMismatch,
    
    /// "videoCodecMismatch"
    #[serde(rename="videoCodecMismatch")]
    VideoCodecMismatch,
    
    /// "videoInterlaceMismatch"
    #[serde(rename="videoInterlaceMismatch")]
    VideoInterlaceMismatch,
    
    /// "videoProfileMismatch"
    #[serde(rename="videoProfileMismatch")]
    VideoProfileMismatch,
    
    /// "videoBitrateMismatch"
    #[serde(rename="videoBitrateMismatch")]
    VideoBitrateMismatch,
    
    /// "framerateMismatch"
    #[serde(rename="framerateMismatch")]
    FramerateMismatch,
    
    /// "gopMismatch"
    #[serde(rename="gopMismatch")]
    GopMismatch,
    
    /// "audioSampleRateMismatch"
    #[serde(rename="audioSampleRateMismatch")]
    AudioSampleRateMismatch,
    
    /// "audioStereoMismatch"
    #[serde(rename="audioStereoMismatch")]
    AudioStereoMismatch,
    
    /// "audioCodecMismatch"
    #[serde(rename="audioCodecMismatch")]
    AudioCodecMismatch,
    
    /// "audioBitrateMismatch"
    #[serde(rename="audioBitrateMismatch")]
    AudioBitrateMismatch,
    
    /// "videoResolutionSuboptimal"
    #[serde(rename="videoResolutionSuboptimal")]
    VideoResolutionSuboptimal,
    
    /// "videoResolutionUnsupported"
    #[serde(rename="videoResolutionUnsupported")]
    VideoResolutionUnsupported,
    
    /// "videoIngestionStarved"
    #[serde(rename="videoIngestionStarved")]
    VideoIngestionStarved,
    
    /// "videoIngestionFasterThanRealtime"
    #[serde(rename="videoIngestionFasterThanRealtime")]
    VideoIngestionFasterThanRealtime,
}

impl AsRef<str> for LiveStreamConfigurationIssueTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamConfigurationIssueTypeEnum::GopSizeOver => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::GopSizeLong => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::GopSizeShort => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::OpenGop => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::BadContainer => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateHigh => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateLow => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioSampleRate => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::BitrateHigh => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::BitrateLow => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioCodec => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoCodec => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::NoAudioStream => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::NoVideoStream => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::MultipleVideoStreams => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::MultipleAudioStreams => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioTooManyChannels => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::InterlacedVideo => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::FrameRateHigh => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::ResolutionMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoCodecMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoInterlaceMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoProfileMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoBitrateMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::FramerateMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::GopMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioSampleRateMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioStereoMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioCodecMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::AudioBitrateMismatch => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoResolutionSuboptimal => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoResolutionUnsupported => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoIngestionStarved => "videoIngestionFasterThanRealtime",
            LiveStreamConfigurationIssueTypeEnum::VideoIngestionFasterThanRealtime => "videoIngestionFasterThanRealtime",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamConfigurationIssueTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status code of this stream
pub enum LiveStreamHealthStatuStatusEnum {
    
    /// "good"
    #[serde(rename="good")]
    Good,
    
    /// "ok"
    #[serde(rename="ok")]
    Ok,
    
    /// "bad"
    #[serde(rename="bad")]
    Bad,
    
    /// "noData"
    #[serde(rename="noData")]
    NoData,
    
    /// "revoked"
    #[serde(rename="revoked")]
    Revoked,
}

impl AsRef<str> for LiveStreamHealthStatuStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamHealthStatuStatusEnum::Good => "revoked",
            LiveStreamHealthStatuStatusEnum::Ok => "revoked",
            LiveStreamHealthStatuStatusEnum::Bad => "revoked",
            LiveStreamHealthStatuStatusEnum::NoData => "revoked",
            LiveStreamHealthStatuStatusEnum::Revoked => "revoked",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamHealthStatuStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiveStreamStatuStreamStatusEnum {
    
    /// "created"
    #[serde(rename="created")]
    Created,
    
    /// "ready"
    #[serde(rename="ready")]
    Ready,
    
    /// "active"
    #[serde(rename="active")]
    Active,
    
    /// "inactive"
    #[serde(rename="inactive")]
    Inactive,
    
    /// "error"
    #[serde(rename="error")]
    Error,
}

impl AsRef<str> for LiveStreamStatuStreamStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveStreamStatuStreamStatusEnum::Created => "error",
            LiveStreamStatuStreamStatusEnum::Ready => "error",
            LiveStreamStatuStreamStatusEnum::Active => "error",
            LiveStreamStatuStreamStatusEnum::Inactive => "error",
            LiveStreamStatuStreamStatusEnum::Error => "error",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveStreamStatuStreamStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This resource's privacy status.
pub enum PlaylistItemStatuPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for PlaylistItemStatuPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaylistItemStatuPrivacyStatusEnum::Public => "private",
            PlaylistItemStatuPrivacyStatusEnum::Unlisted => "private",
            PlaylistItemStatuPrivacyStatusEnum::Private => "private",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaylistItemStatuPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The playlist's privacy status.
pub enum PlaylistStatuPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for PlaylistStatuPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            PlaylistStatuPrivacyStatusEnum::Public => "private",
            PlaylistStatuPrivacyStatusEnum::Unlisted => "private",
            PlaylistStatuPrivacyStatusEnum::Private => "private",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a PlaylistStatuPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's "none" if there is not any upcoming/active live broadcasts.
pub enum SearchResultSnippetLiveBroadcastContentEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for SearchResultSnippetLiveBroadcastContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchResultSnippetLiveBroadcastContentEnum::None => "completed",
            SearchResultSnippetLiveBroadcastContentEnum::Upcoming => "completed",
            SearchResultSnippetLiveBroadcastContentEnum::Live => "completed",
            SearchResultSnippetLiveBroadcastContentEnum::Completed => "completed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchResultSnippetLiveBroadcastContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The type of activity this subscription is for (only uploads, everything).
pub enum SubscriptionContentDetailActivityTypeEnum {
    
    /// "subscriptionActivityTypeUnspecified"
    #[serde(rename="subscriptionActivityTypeUnspecified")]
    SubscriptionActivityTypeUnspecified,
    
    /// "all"
    #[serde(rename="all")]
    All,
    
    /// "uploads"
    #[serde(rename="uploads")]
    Uploads,
}

impl AsRef<str> for SubscriptionContentDetailActivityTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionContentDetailActivityTypeEnum::SubscriptionActivityTypeUnspecified => "uploads",
            SubscriptionContentDetailActivityTypeEnum::All => "uploads",
            SubscriptionContentDetailActivityTypeEnum::Uploads => "uploads",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionContentDetailActivityTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Type of the link named after the entities that are being linked.
pub enum ThirdPartyLinkSnippetTypeEnum {
    
    /// "linkUnspecified"
    #[serde(rename="linkUnspecified")]
    LinkUnspecified,
    

    /// A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube.
    ///
    /// "channelToStoreLink"
    #[serde(rename="channelToStoreLink")]
    ChannelToStoreLink,
}

impl AsRef<str> for ThirdPartyLinkSnippetTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkSnippetTypeEnum::LinkUnspecified => "channelToStoreLink",
            ThirdPartyLinkSnippetTypeEnum::ChannelToStoreLink => "channelToStoreLink",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkSnippetTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum ThirdPartyLinkStatuLinkStatusEnum {
    
    /// "unknown"
    #[serde(rename="unknown")]
    Unknown,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    
    /// "pending"
    #[serde(rename="pending")]
    Pending,
    
    /// "linked"
    #[serde(rename="linked")]
    Linked,
}

impl AsRef<str> for ThirdPartyLinkStatuLinkStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkStatuLinkStatusEnum::Unknown => "linked",
            ThirdPartyLinkStatuLinkStatusEnum::Failed => "linked",
            ThirdPartyLinkStatuLinkStatusEnum::Pending => "linked",
            ThirdPartyLinkStatuLinkStatusEnum::Linked => "linked",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkStatuLinkStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Video game rating, if any.
pub enum VideoAgeGatingVideoGameRatingEnum {
    
    /// "anyone"
    #[serde(rename="anyone")]
    Anyone,
    
    /// "m15Plus"
    #[serde(rename="m15Plus")]
    M15Plus,
    
    /// "m16Plus"
    #[serde(rename="m16Plus")]
    M16Plus,
    
    /// "m17Plus"
    #[serde(rename="m17Plus")]
    M17Plus,
}

impl AsRef<str> for VideoAgeGatingVideoGameRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoAgeGatingVideoGameRatingEnum::Anyone => "m17Plus",
            VideoAgeGatingVideoGameRatingEnum::M15Plus => "m17Plus",
            VideoAgeGatingVideoGameRatingEnum::M16Plus => "m17Plus",
            VideoAgeGatingVideoGameRatingEnum::M17Plus => "m17Plus",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoAgeGatingVideoGameRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value of captions indicates whether the video has captions or not.
pub enum VideoContentDetailCaptionEnum {
    
    /// "true"
    #[serde(rename="true")]
    True,
    
    /// "false"
    #[serde(rename="false")]
    False,
}

impl AsRef<str> for VideoContentDetailCaptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailCaptionEnum::True => "false",
            VideoContentDetailCaptionEnum::False => "false",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailCaptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The value of definition indicates whether the video is available in high definition or only in standard definition.
pub enum VideoContentDetailDefinitionEnum {
    

    /// sd
    ///
    /// "sd"
    #[serde(rename="sd")]
    Sd,
    

    /// hd
    ///
    /// "hd"
    #[serde(rename="hd")]
    Hd,
}

impl AsRef<str> for VideoContentDetailDefinitionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailDefinitionEnum::Sd => "hd",
            VideoContentDetailDefinitionEnum::Hd => "hd",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailDefinitionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the projection format of the video.
pub enum VideoContentDetailProjectionEnum {
    
    /// "rectangular"
    #[serde(rename="rectangular")]
    Rectangular,
    
    /// "360"
    #[serde(rename="360")]
    _360,
}

impl AsRef<str> for VideoContentDetailProjectionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoContentDetailProjectionEnum::Rectangular => "360",
            VideoContentDetailProjectionEnum::_360 => "360",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoContentDetailProjectionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded.
pub enum VideoFileDetailFileTypeEnum {
    

    /// Known video file (e.g., an MP4 file).
    ///
    /// "video"
    #[serde(rename="video")]
    Video,
    

    /// Audio only file (e.g., an MP3 file).
    ///
    /// "audio"
    #[serde(rename="audio")]
    Audio,
    

    /// Image file (e.g., a JPEG image).
    ///
    /// "image"
    #[serde(rename="image")]
    Image,
    

    /// Archive file (e.g., a ZIP archive).
    ///
    /// "archive"
    #[serde(rename="archive")]
    Archive,
    

    /// Document or text file (e.g., MS Word document).
    ///
    /// "document"
    #[serde(rename="document")]
    Document,
    

    /// Movie project file (e.g., Microsoft Windows Movie Maker project).
    ///
    /// "project"
    #[serde(rename="project")]
    Project,
    

    /// Other non-video file type.
    ///
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoFileDetailFileTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoFileDetailFileTypeEnum::Video => "other",
            VideoFileDetailFileTypeEnum::Audio => "other",
            VideoFileDetailFileTypeEnum::Image => "other",
            VideoFileDetailFileTypeEnum::Archive => "other",
            VideoFileDetailFileTypeEnum::Document => "other",
            VideoFileDetailFileTypeEnum::Project => "other",
            VideoFileDetailFileTypeEnum::Other => "other",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoFileDetailFileTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The amount that YouTube needs to rotate the original source content to properly display the video.
pub enum VideoFileDetailsVideoStreamRotationEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    
    /// "clockwise"
    #[serde(rename="clockwise")]
    Clockwise,
    
    /// "upsideDown"
    #[serde(rename="upsideDown")]
    UpsideDown,
    
    /// "counterClockwise"
    #[serde(rename="counterClockwise")]
    CounterClockwise,
    
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoFileDetailsVideoStreamRotationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoFileDetailsVideoStreamRotationEnum::None => "other",
            VideoFileDetailsVideoStreamRotationEnum::Clockwise => "other",
            VideoFileDetailsVideoStreamRotationEnum::UpsideDown => "other",
            VideoFileDetailsVideoStreamRotationEnum::CounterClockwise => "other",
            VideoFileDetailsVideoStreamRotationEnum::Other => "other",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoFileDetailsVideoStreamRotationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed.
pub enum VideoProcessingDetailProcessingFailureReasonEnum {
    
    /// "uploadFailed"
    #[serde(rename="uploadFailed")]
    UploadFailed,
    
    /// "transcodeFailed"
    #[serde(rename="transcodeFailed")]
    TranscodeFailed,
    
    /// "streamingFailed"
    #[serde(rename="streamingFailed")]
    StreamingFailed,
    
    /// "other"
    #[serde(rename="other")]
    Other,
}

impl AsRef<str> for VideoProcessingDetailProcessingFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoProcessingDetailProcessingFailureReasonEnum::UploadFailed => "other",
            VideoProcessingDetailProcessingFailureReasonEnum::TranscodeFailed => "other",
            VideoProcessingDetailProcessingFailureReasonEnum::StreamingFailed => "other",
            VideoProcessingDetailProcessingFailureReasonEnum::Other => "other",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoProcessingDetailProcessingFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed.
pub enum VideoProcessingDetailProcessingStatusEnum {
    
    /// "processing"
    #[serde(rename="processing")]
    Processing,
    
    /// "succeeded"
    #[serde(rename="succeeded")]
    Succeeded,
    
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    
    /// "terminated"
    #[serde(rename="terminated")]
    Terminated,
}

impl AsRef<str> for VideoProcessingDetailProcessingStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoProcessingDetailProcessingStatusEnum::Processing => "terminated",
            VideoProcessingDetailProcessingStatusEnum::Succeeded => "terminated",
            VideoProcessingDetailProcessingStatusEnum::Failed => "terminated",
            VideoProcessingDetailProcessingStatusEnum::Terminated => "terminated",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoProcessingDetailProcessingStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Rating of a video.
pub enum VideoRatingRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoRatingRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoRatingRatingEnum::None => "dislike",
            VideoRatingRatingEnum::Like => "dislike",
            VideoRatingRatingEnum::Dislike => "dislike",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoRatingRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast.
pub enum VideoSnippetLiveBroadcastContentEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for VideoSnippetLiveBroadcastContentEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSnippetLiveBroadcastContentEnum::None => "completed",
            VideoSnippetLiveBroadcastContentEnum::Upcoming => "completed",
            VideoSnippetLiveBroadcastContentEnum::Live => "completed",
            VideoSnippetLiveBroadcastContentEnum::Completed => "completed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSnippetLiveBroadcastContentEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed.
pub enum VideoStatuFailureReasonEnum {
    

    /// Unable to convert video content.
    ///
    /// "conversion"
    #[serde(rename="conversion")]
    Conversion,
    

    /// Invalid file format.
    ///
    /// "invalidFile"
    #[serde(rename="invalidFile")]
    InvalidFile,
    

    /// Empty file.
    ///
    /// "emptyFile"
    #[serde(rename="emptyFile")]
    EmptyFile,
    

    /// File was too small.
    ///
    /// "tooSmall"
    #[serde(rename="tooSmall")]
    TooSmall,
    

    /// Unsupported codec.
    ///
    /// "codec"
    #[serde(rename="codec")]
    Codec,
    

    /// Upload wasn't finished.
    ///
    /// "uploadAborted"
    #[serde(rename="uploadAborted")]
    UploadAborted,
}

impl AsRef<str> for VideoStatuFailureReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatuFailureReasonEnum::Conversion => "uploadAborted",
            VideoStatuFailureReasonEnum::InvalidFile => "uploadAborted",
            VideoStatuFailureReasonEnum::EmptyFile => "uploadAborted",
            VideoStatuFailureReasonEnum::TooSmall => "uploadAborted",
            VideoStatuFailureReasonEnum::Codec => "uploadAborted",
            VideoStatuFailureReasonEnum::UploadAborted => "uploadAborted",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatuFailureReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's license. @mutable youtube.videos.insert youtube.videos.update
pub enum VideoStatuLicenseEnum {
    
    /// "youtube"
    #[serde(rename="youtube")]
    Youtube,
    
    /// "creativeCommon"
    #[serde(rename="creativeCommon")]
    CreativeCommon,
}

impl AsRef<str> for VideoStatuLicenseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatuLicenseEnum::Youtube => "creativeCommon",
            VideoStatuLicenseEnum::CreativeCommon => "creativeCommon",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatuLicenseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The video's privacy status.
pub enum VideoStatuPrivacyStatusEnum {
    
    /// "public"
    #[serde(rename="public")]
    Public,
    
    /// "unlisted"
    #[serde(rename="unlisted")]
    Unlisted,
    
    /// "private"
    #[serde(rename="private")]
    Private,
}

impl AsRef<str> for VideoStatuPrivacyStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatuPrivacyStatusEnum::Public => "private",
            VideoStatuPrivacyStatusEnum::Unlisted => "private",
            VideoStatuPrivacyStatusEnum::Private => "private",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatuPrivacyStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected.
pub enum VideoStatuRejectionReasonEnum {
    

    /// Copyright infringement.
    ///
    /// "copyright"
    #[serde(rename="copyright")]
    Copyright,
    

    /// Inappropriate video content.
    ///
    /// "inappropriate"
    #[serde(rename="inappropriate")]
    Inappropriate,
    

    /// Duplicate upload in the same channel.
    ///
    /// "duplicate"
    #[serde(rename="duplicate")]
    Duplicate,
    

    /// Terms of use violation.
    ///
    /// "termsOfUse"
    #[serde(rename="termsOfUse")]
    TermsOfUse,
    

    /// Uploader account was suspended.
    ///
    /// "uploaderAccountSuspended"
    #[serde(rename="uploaderAccountSuspended")]
    UploaderAccountSuspended,
    

    /// Video duration was too long.
    ///
    /// "length"
    #[serde(rename="length")]
    Length,
    

    /// Blocked by content owner.
    ///
    /// "claim"
    #[serde(rename="claim")]
    Claim,
    

    /// Uploader closed his/her account.
    ///
    /// "uploaderAccountClosed"
    #[serde(rename="uploaderAccountClosed")]
    UploaderAccountClosed,
    

    /// Trademark infringement.
    ///
    /// "trademark"
    #[serde(rename="trademark")]
    Trademark,
    

    /// An unspecified legal reason.
    ///
    /// "legal"
    #[serde(rename="legal")]
    Legal,
}

impl AsRef<str> for VideoStatuRejectionReasonEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatuRejectionReasonEnum::Copyright => "legal",
            VideoStatuRejectionReasonEnum::Inappropriate => "legal",
            VideoStatuRejectionReasonEnum::Duplicate => "legal",
            VideoStatuRejectionReasonEnum::TermsOfUse => "legal",
            VideoStatuRejectionReasonEnum::UploaderAccountSuspended => "legal",
            VideoStatuRejectionReasonEnum::Length => "legal",
            VideoStatuRejectionReasonEnum::Claim => "legal",
            VideoStatuRejectionReasonEnum::UploaderAccountClosed => "legal",
            VideoStatuRejectionReasonEnum::Trademark => "legal",
            VideoStatuRejectionReasonEnum::Legal => "legal",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatuRejectionReasonEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status of the uploaded video.
pub enum VideoStatuUploadStatusEnum {
    

    /// Video has been uploaded but not processed yet.
    ///
    /// "uploaded"
    #[serde(rename="uploaded")]
    Uploaded,
    

    /// Video has been successfully processed.
    ///
    /// "processed"
    #[serde(rename="processed")]
    Processed,
    

    /// Processing has failed. See FailureReason.
    ///
    /// "failed"
    #[serde(rename="failed")]
    Failed,
    

    /// Video has been rejected. See RejectionReason.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
    

    /// Video has been deleted.
    ///
    /// "deleted"
    #[serde(rename="deleted")]
    Deleted,
}

impl AsRef<str> for VideoStatuUploadStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoStatuUploadStatusEnum::Uploaded => "deleted",
            VideoStatuUploadStatusEnum::Processed => "deleted",
            VideoStatuUploadStatusEnum::Failed => "deleted",
            VideoStatuUploadStatusEnum::Rejected => "deleted",
            VideoStatuUploadStatusEnum::Deleted => "deleted",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoStatuUploadStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSuggestionEditorSuggestionsEnum {
    

    /// Picture brightness levels seem off and could be corrected.
    ///
    /// "videoAutoLevels"
    #[serde(rename="videoAutoLevels")]
    VideoAutoLevels,
    

    /// The video appears shaky and could be stabilized.
    ///
    /// "videoStabilize"
    #[serde(rename="videoStabilize")]
    VideoStabilize,
    

    /// Margins (mattes) detected around the picture could be cropped.
    ///
    /// "videoCrop"
    #[serde(rename="videoCrop")]
    VideoCrop,
    

    /// The audio track appears silent and could be swapped with a better quality one.
    ///
    /// "audioQuietAudioSwap"
    #[serde(rename="audioQuietAudioSwap")]
    AudioQuietAudioSwap,
}

impl AsRef<str> for VideoSuggestionEditorSuggestionsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionEditorSuggestionsEnum::VideoAutoLevels => "audioQuietAudioSwap",
            VideoSuggestionEditorSuggestionsEnum::VideoStabilize => "audioQuietAudioSwap",
            VideoSuggestionEditorSuggestionsEnum::VideoCrop => "audioQuietAudioSwap",
            VideoSuggestionEditorSuggestionsEnum::AudioQuietAudioSwap => "audioQuietAudioSwap",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionEditorSuggestionsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSuggestionProcessingErrorsEnum {
    

    /// File contains audio only (e.g., an MP3 file).
    ///
    /// "audioFile"
    #[serde(rename="audioFile")]
    AudioFile,
    

    /// Image file (e.g., a JPEG image).
    ///
    /// "imageFile"
    #[serde(rename="imageFile")]
    ImageFile,
    

    /// Movie project file (e.g., Microsoft Windows Movie Maker project).
    ///
    /// "projectFile"
    #[serde(rename="projectFile")]
    ProjectFile,
    

    /// Other non-video file.
    ///
    /// "notAVideoFile"
    #[serde(rename="notAVideoFile")]
    NotAVideoFile,
    

    /// Document or text file (e.g., MS Word document).
    ///
    /// "docFile"
    #[serde(rename="docFile")]
    DocFile,
    

    /// An archive file (e.g., a ZIP archive).
    ///
    /// "archiveFile"
    #[serde(rename="archiveFile")]
    ArchiveFile,
    

    /// Unsupported spatial audio layout type.
    ///
    /// "unsupportedSpatialAudioLayout"
    #[serde(rename="unsupportedSpatialAudioLayout")]
    UnsupportedSpatialAudioLayout,
}

impl AsRef<str> for VideoSuggestionProcessingErrorsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingErrorsEnum::AudioFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::ImageFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::ProjectFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::NotAVideoFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::DocFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::ArchiveFile => "unsupportedSpatialAudioLayout",
            VideoSuggestionProcessingErrorsEnum::UnsupportedSpatialAudioLayout => "unsupportedSpatialAudioLayout",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingErrorsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSuggestionProcessingHintsEnum {
    

    /// The MP4 file is not streamable, this will slow down the processing. MOOV atom was not found at the beginning of the file.
    ///
    /// "nonStreamableMov"
    #[serde(rename="nonStreamableMov")]
    NonStreamableMov,
    

    /// Probably a better quality version of the video exists. The video has wide screen aspect ratio, but is not an HD video.
    ///
    /// "sendBestQualityVideo"
    #[serde(rename="sendBestQualityVideo")]
    SendBestQualityVideo,
    

    /// Uploaded video is spherical video.
    ///
    /// "sphericalVideo"
    #[serde(rename="sphericalVideo")]
    SphericalVideo,
    

    /// Uploaded video has spatial audio.
    ///
    /// "spatialAudio"
    #[serde(rename="spatialAudio")]
    SpatialAudio,
    

    /// Uploaded video is VR video.
    ///
    /// "vrVideo"
    #[serde(rename="vrVideo")]
    VrVideo,
    

    /// Uploaded video is HDR video.
    ///
    /// "hdrVideo"
    #[serde(rename="hdrVideo")]
    HdrVideo,
}

impl AsRef<str> for VideoSuggestionProcessingHintsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingHintsEnum::NonStreamableMov => "hdrVideo",
            VideoSuggestionProcessingHintsEnum::SendBestQualityVideo => "hdrVideo",
            VideoSuggestionProcessingHintsEnum::SphericalVideo => "hdrVideo",
            VideoSuggestionProcessingHintsEnum::SpatialAudio => "hdrVideo",
            VideoSuggestionProcessingHintsEnum::VrVideo => "hdrVideo",
            VideoSuggestionProcessingHintsEnum::HdrVideo => "hdrVideo",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingHintsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoSuggestionProcessingWarningsEnum {
    

    /// Unrecognized file format, transcoding is likely to fail.
    ///
    /// "unknownContainer"
    #[serde(rename="unknownContainer")]
    UnknownContainer,
    

    /// Unrecognized video codec, transcoding is likely to fail.
    ///
    /// "unknownVideoCodec"
    #[serde(rename="unknownVideoCodec")]
    UnknownVideoCodec,
    

    /// Unrecognized audio codec, transcoding is likely to fail.
    ///
    /// "unknownAudioCodec"
    #[serde(rename="unknownAudioCodec")]
    UnknownAudioCodec,
    

    /// Conflicting container and stream resolutions.
    ///
    /// "inconsistentResolution"
    #[serde(rename="inconsistentResolution")]
    InconsistentResolution,
    

    /// Edit lists are not currently supported.
    ///
    /// "hasEditlist"
    #[serde(rename="hasEditlist")]
    HasEditlist,
    

    /// Video codec that is known to cause problems was used.
    ///
    /// "problematicVideoCodec"
    #[serde(rename="problematicVideoCodec")]
    ProblematicVideoCodec,
    

    /// Audio codec that is known to cause problems was used.
    ///
    /// "problematicAudioCodec"
    #[serde(rename="problematicAudioCodec")]
    ProblematicAudioCodec,
    

    /// Unsupported VR video stereo mode.
    ///
    /// "unsupportedVrStereoMode"
    #[serde(rename="unsupportedVrStereoMode")]
    UnsupportedVrStereoMode,
    

    /// Unsupported spherical video projection type.
    ///
    /// "unsupportedSphericalProjectionType"
    #[serde(rename="unsupportedSphericalProjectionType")]
    UnsupportedSphericalProjectionType,
    

    /// Unsupported HDR pixel format.
    ///
    /// "unsupportedHdrPixelFormat"
    #[serde(rename="unsupportedHdrPixelFormat")]
    UnsupportedHdrPixelFormat,
    

    /// Unspecified HDR color metadata.
    ///
    /// "unsupportedHdrColorMetadata"
    #[serde(rename="unsupportedHdrColorMetadata")]
    UnsupportedHdrColorMetadata,
    

    /// Problematic HDR lookup table attached.
    ///
    /// "problematicHdrLookupTable"
    #[serde(rename="problematicHdrLookupTable")]
    ProblematicHdrLookupTable,
}

impl AsRef<str> for VideoSuggestionProcessingWarningsEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoSuggestionProcessingWarningsEnum::UnknownContainer => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnknownVideoCodec => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnknownAudioCodec => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::InconsistentResolution => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::HasEditlist => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::ProblematicVideoCodec => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::ProblematicAudioCodec => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnsupportedVrStereoMode => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnsupportedSphericalProjectionType => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnsupportedHdrPixelFormat => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::UnsupportedHdrColorMetadata => "problematicHdrLookupTable",
            VideoSuggestionProcessingWarningsEnum::ProblematicHdrLookupTable => "problematicHdrLookupTable",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoSuggestionProcessingWarningsEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Limits the returned comment threads to those with the specified moderation status. Not compatible with the 'id' filter. Valid values: published, heldForReview, likelySpam.
pub enum CommentThreadModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentThreadModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadModerationStatusEnum::Published => "rejected",
            CommentThreadModerationStatusEnum::HeldForReview => "rejected",
            CommentThreadModerationStatusEnum::LikelySpam => "rejected",
            CommentThreadModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadModerationStatusEnum {
    fn default() -> CommentThreadModerationStatusEnum {
        CommentThreadModerationStatusEnum::Published
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum CommentThreadOrderEnum {
    
    /// "orderUnspecified"
    #[serde(rename="orderUnspecified")]
    OrderUnspecified,
    

    /// Order by time.
    ///
    /// "time"
    #[serde(rename="time")]
    Time,
    

    /// Order by relevance.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
}

impl AsRef<str> for CommentThreadOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadOrderEnum::OrderUnspecified => "relevance",
            CommentThreadOrderEnum::Time => "relevance",
            CommentThreadOrderEnum::Relevance => "relevance",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadOrderEnum {
    fn default() -> CommentThreadOrderEnum {
        CommentThreadOrderEnum::Time
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested text format for the returned comments.
pub enum CommentThreadTextFormatEnum {
    
    /// "textFormatUnspecified"
    #[serde(rename="textFormatUnspecified")]
    TextFormatUnspecified,
    

    /// Returns the comments in HTML format. This is the default value.
    ///
    /// "html"
    #[serde(rename="html")]
    Html,
    

    /// Returns the comments in plain text format.
    ///
    /// "plainText"
    #[serde(rename="plainText")]
    PlainText,
}

impl AsRef<str> for CommentThreadTextFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentThreadTextFormatEnum::TextFormatUnspecified => "plainText",
            CommentThreadTextFormatEnum::Html => "plainText",
            CommentThreadTextFormatEnum::PlainText => "plainText",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentThreadTextFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentThreadTextFormatEnum {
    fn default() -> CommentThreadTextFormatEnum {
        CommentThreadTextFormatEnum::Html
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The requested text format for the returned comments.
pub enum CommentTextFormatEnum {
    
    /// "textFormatUnspecified"
    #[serde(rename="textFormatUnspecified")]
    TextFormatUnspecified,
    

    /// Returns the comments in HTML format. This is the default value.
    ///
    /// "html"
    #[serde(rename="html")]
    Html,
    

    /// Returns the comments in plain text format.
    ///
    /// "plainText"
    #[serde(rename="plainText")]
    PlainText,
}

impl AsRef<str> for CommentTextFormatEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentTextFormatEnum::TextFormatUnspecified => "plainText",
            CommentTextFormatEnum::Html => "plainText",
            CommentTextFormatEnum::PlainText => "plainText",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentTextFormatEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for CommentTextFormatEnum {
    fn default() -> CommentTextFormatEnum {
        CommentTextFormatEnum::Html
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: MODERATION_STATUS_PUBLISHED, MODERATION_STATUS_HELD_FOR_REVIEW, MODERATION_STATUS_REJECTED.
pub enum CommentModerationStatusEnum {
    

    /// The comment is available for public display.
    ///
    /// "published"
    #[serde(rename="published")]
    Published,
    

    /// The comment is awaiting review by a moderator.
    ///
    /// "heldForReview"
    #[serde(rename="heldForReview")]
    HeldForReview,
    
    /// "likelySpam"
    #[serde(rename="likelySpam")]
    LikelySpam,
    

    /// The comment is unfit for display.
    ///
    /// "rejected"
    #[serde(rename="rejected")]
    Rejected,
}

impl AsRef<str> for CommentModerationStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            CommentModerationStatusEnum::Published => "rejected",
            CommentModerationStatusEnum::HeldForReview => "rejected",
            CommentModerationStatusEnum::LikelySpam => "rejected",
            CommentModerationStatusEnum::Rejected => "rejected",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a CommentModerationStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The status to which the broadcast is going to transition.
pub enum LiveBroadcastBroadcastStatusEnum {
    
    /// "statusUnspecified"
    #[serde(rename="statusUnspecified")]
    StatusUnspecified,
    

    /// Start testing the broadcast. YouTube transmits video to the broadcast's monitor stream. Note that you can only transition a broadcast to the testing state if its contentDetails.monitorStream.enableMonitorStream property is set to true.",
    ///
    /// "testing"
    #[serde(rename="testing")]
    Testing,
    

    /// Return only persistent broadcasts.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The broadcast is over. YouTube stops transmitting video.
    ///
    /// "complete"
    #[serde(rename="complete")]
    Complete,
}

impl AsRef<str> for LiveBroadcastBroadcastStatusEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastBroadcastStatusEnum::StatusUnspecified => "complete",
            LiveBroadcastBroadcastStatusEnum::Testing => "complete",
            LiveBroadcastBroadcastStatusEnum::Live => "complete",
            LiveBroadcastBroadcastStatusEnum::Complete => "complete",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastBroadcastStatusEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return only broadcasts with the selected type.
pub enum LiveBroadcastBroadcastTypeEnum {
    
    /// "broadcastTypeFilterUnspecified"
    #[serde(rename="broadcastTypeFilterUnspecified")]
    BroadcastTypeFilterUnspecified,
    

    /// Return all broadcasts.
    ///
    /// "all"
    #[serde(rename="all")]
    All,
    

    /// Return only scheduled event broadcasts.
    ///
    /// "event"
    #[serde(rename="event")]
    Event,
    

    /// Return only persistent broadcasts.
    ///
    /// "persistent"
    #[serde(rename="persistent")]
    Persistent,
}

impl AsRef<str> for LiveBroadcastBroadcastTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            LiveBroadcastBroadcastTypeEnum::BroadcastTypeFilterUnspecified => "persistent",
            LiveBroadcastBroadcastTypeEnum::All => "persistent",
            LiveBroadcastBroadcastTypeEnum::Event => "persistent",
            LiveBroadcastBroadcastTypeEnum::Persistent => "persistent",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a LiveBroadcastBroadcastTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for LiveBroadcastBroadcastTypeEnum {
    fn default() -> LiveBroadcastBroadcastTypeEnum {
        LiveBroadcastBroadcastTypeEnum::Event
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Parameter that specifies which channel members to return.
pub enum MemberModeEnum {
    
    /// "listMembersModeUnknown"
    #[serde(rename="listMembersModeUnknown")]
    ListMembersModeUnknown,
    

    /// Return only members that joined after the first call with this mode was made.
    ///
    /// "updates"
    #[serde(rename="updates")]
    Updates,
    

    /// Return all current members, from newest to oldest.
    ///
    /// "all_current"
    #[serde(rename="all_current")]
    AllCurrent,
}

impl AsRef<str> for MemberModeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            MemberModeEnum::ListMembersModeUnknown => "all_current",
            MemberModeEnum::Updates => "all_current",
            MemberModeEnum::AllCurrent => "all_current",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a MemberModeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for MemberModeEnum {
    fn default() -> MemberModeEnum {
        MemberModeEnum::AllCurrent
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Add a filter on the channel search.
pub enum SearchChannelTypeEnum {
    
    /// "channelTypeUnspecified"
    #[serde(rename="channelTypeUnspecified")]
    ChannelTypeUnspecified,
    

    /// Return all channels.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve shows.
    ///
    /// "show"
    #[serde(rename="show")]
    Show,
}

impl AsRef<str> for SearchChannelTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchChannelTypeEnum::ChannelTypeUnspecified => "show",
            SearchChannelTypeEnum::Any => "show",
            SearchChannelTypeEnum::Show => "show",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchChannelTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the livestream status of the videos.
pub enum SearchEventTypeEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The live broadcast is upcoming.
    ///
    /// "upcoming"
    #[serde(rename="upcoming")]
    Upcoming,
    

    /// The live broadcast is active.
    ///
    /// "live"
    #[serde(rename="live")]
    Live,
    

    /// The live broadcast has been completed.
    ///
    /// "completed"
    #[serde(rename="completed")]
    Completed,
}

impl AsRef<str> for SearchEventTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchEventTypeEnum::None => "completed",
            SearchEventTypeEnum::Upcoming => "completed",
            SearchEventTypeEnum::Live => "completed",
            SearchEventTypeEnum::Completed => "completed",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchEventTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Sort order of the results.
pub enum SearchOrderEnum {
    
    /// "searchSortUnspecified"
    #[serde(rename="searchSortUnspecified")]
    SearchSortUnspecified,
    

    /// Resources are sorted in reverse chronological order based on the date they were created.
    ///
    /// "date"
    #[serde(rename="date")]
    Date,
    

    /// Resources are sorted from highest to lowest rating.
    ///
    /// "rating"
    #[serde(rename="rating")]
    Rating,
    

    /// Resources are sorted from highest to lowest number of views.
    ///
    /// "viewCount"
    #[serde(rename="viewCount")]
    ViewCount,
    

    /// Resources are sorted based on their relevance to the search query. This is the default value for this parameter.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
    

    /// Resources are sorted alphabetically by title.
    ///
    /// "title"
    #[serde(rename="title")]
    Title,
    

    /// Channels are sorted in descending order of their number of uploaded videos.
    ///
    /// "videoCount"
    #[serde(rename="videoCount")]
    VideoCount,
}

impl AsRef<str> for SearchOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchOrderEnum::SearchSortUnspecified => "videoCount",
            SearchOrderEnum::Date => "videoCount",
            SearchOrderEnum::Rating => "videoCount",
            SearchOrderEnum::ViewCount => "videoCount",
            SearchOrderEnum::Relevance => "videoCount",
            SearchOrderEnum::Title => "videoCount",
            SearchOrderEnum::VideoCount => "videoCount",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SearchOrderEnum {
    fn default() -> SearchOrderEnum {
        SearchOrderEnum::Relevance
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Indicates whether the search results should include restricted content as well as standard content.
pub enum SearchSafeSearchEnum {
    
    /// "safeSearchSettingUnspecified"
    #[serde(rename="safeSearchSettingUnspecified")]
    SafeSearchSettingUnspecified,
    

    /// YouTube will not filter the search result set.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// YouTube will filter some content from search results and, at the least, will filter content that is restricted in your locale. Based on their content, search results could be removed from search results or demoted in search results. This is the default parameter value.
    ///
    /// "moderate"
    #[serde(rename="moderate")]
    Moderate,
    

    /// YouTube will try to exclude all restricted content from the search result set. Based on their content, search results could be removed from search results or demoted in search results.
    ///
    /// "strict"
    #[serde(rename="strict")]
    Strict,
}

impl AsRef<str> for SearchSafeSearchEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchSafeSearchEnum::SafeSearchSettingUnspecified => "strict",
            SearchSafeSearchEnum::None => "strict",
            SearchSafeSearchEnum::Moderate => "strict",
            SearchSafeSearchEnum::Strict => "strict",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchSafeSearchEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SearchSafeSearchEnum {
    fn default() -> SearchSafeSearchEnum {
        SearchSafeSearchEnum::Moderate
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the presence of captions on the videos.
pub enum SearchVideoCaptionEnum {
    
    /// "videoCaptionUnspecified"
    #[serde(rename="videoCaptionUnspecified")]
    VideoCaptionUnspecified,
    

    /// Do not filter results based on caption availability.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only include videos that have captions.
    ///
    /// "closedCaption"
    #[serde(rename="closedCaption")]
    ClosedCaption,
    

    /// Only include videos that do not have captions.
    ///
    /// "none"
    #[serde(rename="none")]
    None,
}

impl AsRef<str> for SearchVideoCaptionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoCaptionEnum::VideoCaptionUnspecified => "none",
            SearchVideoCaptionEnum::Any => "none",
            SearchVideoCaptionEnum::ClosedCaption => "none",
            SearchVideoCaptionEnum::None => "none",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoCaptionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the definition of the videos.
pub enum SearchVideoDefinitionEnum {
    

    /// Return all videos, regardless of their resolution.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve videos in standard definition.
    ///
    /// "standard"
    #[serde(rename="standard")]
    Standard,
    

    /// Only retrieve HD videos.
    ///
    /// "high"
    #[serde(rename="high")]
    High,
}

impl AsRef<str> for SearchVideoDefinitionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDefinitionEnum::Any => "high",
            SearchVideoDefinitionEnum::Standard => "high",
            SearchVideoDefinitionEnum::High => "high",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDefinitionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on 3d videos.
pub enum SearchVideoDimensionEnum {
    

    /// Include both 3D and non-3D videos in returned results. This is the default value.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Restrict search results to exclude 3D videos.
    ///
    /// "2d"
    #[serde(rename="2d")]
    _2d,
    

    /// Restrict search results to only include 3D videos.
    ///
    /// "3d"
    #[serde(rename="3d")]
    _3d,
}

impl AsRef<str> for SearchVideoDimensionEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDimensionEnum::Any => "3d",
            SearchVideoDimensionEnum::_2d => "3d",
            SearchVideoDimensionEnum::_3d => "3d",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDimensionEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the duration of the videos.
pub enum SearchVideoDurationEnum {
    
    /// "videoDurationUnspecified"
    #[serde(rename="videoDurationUnspecified")]
    VideoDurationUnspecified,
    

    /// Do not filter video search results based on their duration. This is the default value.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only include videos that are less than four minutes long.
    ///
    /// "short"
    #[serde(rename="short")]
    Short,
    

    /// Only include videos that are between four and 20 minutes long (inclusive).
    ///
    /// "medium"
    #[serde(rename="medium")]
    Medium,
    

    /// Only include videos longer than 20 minutes.
    ///
    /// "long"
    #[serde(rename="long")]
    Long,
}

impl AsRef<str> for SearchVideoDurationEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoDurationEnum::VideoDurationUnspecified => "long",
            SearchVideoDurationEnum::Any => "long",
            SearchVideoDurationEnum::Short => "long",
            SearchVideoDurationEnum::Medium => "long",
            SearchVideoDurationEnum::Long => "long",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoDurationEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on embeddable videos.
pub enum SearchVideoEmbeddableEnum {
    
    /// "videoEmbeddableUnspecified"
    #[serde(rename="videoEmbeddableUnspecified")]
    VideoEmbeddableUnspecified,
    

    /// Return all videos, embeddable or not.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve embeddable videos.
    ///
    /// "true"
    #[serde(rename="true")]
    True,
}

impl AsRef<str> for SearchVideoEmbeddableEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoEmbeddableEnum::VideoEmbeddableUnspecified => "true",
            SearchVideoEmbeddableEnum::Any => "true",
            SearchVideoEmbeddableEnum::True => "true",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoEmbeddableEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on the license of the videos.
pub enum SearchVideoLicenseEnum {
    

    /// Return all videos, regardless of which license they have, that match the query parameters.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only return videos that have the standard YouTube license.
    ///
    /// "youtube"
    #[serde(rename="youtube")]
    Youtube,
    

    /// Only return videos that have a Creative Commons license. Users can reuse videos with this license in other videos that they create. Learn more.
    ///
    /// "creativeCommon"
    #[serde(rename="creativeCommon")]
    CreativeCommon,
}

impl AsRef<str> for SearchVideoLicenseEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoLicenseEnum::Any => "creativeCommon",
            SearchVideoLicenseEnum::Youtube => "creativeCommon",
            SearchVideoLicenseEnum::CreativeCommon => "creativeCommon",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoLicenseEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on syndicated videos.
pub enum SearchVideoSyndicatedEnum {
    
    /// "videoSyndicatedUnspecified"
    #[serde(rename="videoSyndicatedUnspecified")]
    VideoSyndicatedUnspecified,
    

    /// Return all videos, syndicated or not.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve syndicated videos.
    ///
    /// "true"
    #[serde(rename="true")]
    True,
}

impl AsRef<str> for SearchVideoSyndicatedEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoSyndicatedEnum::VideoSyndicatedUnspecified => "true",
            SearchVideoSyndicatedEnum::Any => "true",
            SearchVideoSyndicatedEnum::True => "true",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoSyndicatedEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Filter on videos of a specific type.
pub enum SearchVideoTypeEnum {
    
    /// "videoTypeUnspecified"
    #[serde(rename="videoTypeUnspecified")]
    VideoTypeUnspecified,
    

    /// Return all videos.
    ///
    /// "any"
    #[serde(rename="any")]
    Any,
    

    /// Only retrieve movies.
    ///
    /// "movie"
    #[serde(rename="movie")]
    Movie,
    

    /// Only retrieve episodes of shows.
    ///
    /// "episode"
    #[serde(rename="episode")]
    Episode,
}

impl AsRef<str> for SearchVideoTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SearchVideoTypeEnum::VideoTypeUnspecified => "episode",
            SearchVideoTypeEnum::Any => "episode",
            SearchVideoTypeEnum::Movie => "episode",
            SearchVideoTypeEnum::Episode => "episode",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SearchVideoTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// The order of the returned subscriptions
pub enum SubscriptionOrderEnum {
    
    /// "subscriptionOrderUnspecified"
    #[serde(rename="subscriptionOrderUnspecified")]
    SubscriptionOrderUnspecified,
    

    /// Sort by relevance.
    ///
    /// "relevance"
    #[serde(rename="relevance")]
    Relevance,
    

    /// Sort by order of activity.
    ///
    /// "unread"
    #[serde(rename="unread")]
    Unread,
    

    /// Sort alphabetically.
    ///
    /// "alphabetical"
    #[serde(rename="alphabetical")]
    Alphabetical,
}

impl AsRef<str> for SubscriptionOrderEnum {
    fn as_ref(&self) -> &str {
        match *self {
            SubscriptionOrderEnum::SubscriptionOrderUnspecified => "alphabetical",
            SubscriptionOrderEnum::Relevance => "alphabetical",
            SubscriptionOrderEnum::Unread => "alphabetical",
            SubscriptionOrderEnum::Alphabetical => "alphabetical",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a SubscriptionOrderEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

impl Default for SubscriptionOrderEnum {
    fn default() -> SubscriptionOrderEnum {
        SubscriptionOrderEnum::Relevance
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Get a third party link of the given type.
pub enum ThirdPartyLinkTypeEnum {
    
    /// "linkUnspecified"
    #[serde(rename="linkUnspecified")]
    LinkUnspecified,
    

    /// A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube.
    ///
    /// "channelToStoreLink"
    #[serde(rename="channelToStoreLink")]
    ChannelToStoreLink,
}

impl AsRef<str> for ThirdPartyLinkTypeEnum {
    fn as_ref(&self) -> &str {
        match *self {
            ThirdPartyLinkTypeEnum::LinkUnspecified => "channelToStoreLink",
            ThirdPartyLinkTypeEnum::ChannelToStoreLink => "channelToStoreLink",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ThirdPartyLinkTypeEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return the videos that are in the specified chart.
pub enum VideoChartEnum {
    
    /// "chartUnspecified"
    #[serde(rename="chartUnspecified")]
    ChartUnspecified,
    

    /// Return the most popular videos for the specified content region and video category.
    ///
    /// "mostPopular"
    #[serde(rename="mostPopular")]
    MostPopular,
}

impl AsRef<str> for VideoChartEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoChartEnum::ChartUnspecified => "mostPopular",
            VideoChartEnum::MostPopular => "mostPopular",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoChartEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
/// Return videos liked/disliked by the authenticated user. Does not support RateType.RATED_TYPE_NONE.
pub enum VideoMyRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoMyRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoMyRatingEnum::None => "dislike",
            VideoMyRatingEnum::Like => "dislike",
            VideoMyRatingEnum::Dislike => "dislike",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoMyRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
pub enum VideoRatingEnum {
    
    /// "none"
    #[serde(rename="none")]
    None,
    

    /// The entity is liked.
    ///
    /// "like"
    #[serde(rename="like")]
    Like,
    

    /// The entity is disliked.
    ///
    /// "dislike"
    #[serde(rename="dislike")]
    Dislike,
}

impl AsRef<str> for VideoRatingEnum {
    fn as_ref(&self) -> &str {
        match *self {
            VideoRatingEnum::None => "dislike",
            VideoRatingEnum::Like => "dislike",
            VideoRatingEnum::Dislike => "dislike",
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a VideoRatingEnum {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}


