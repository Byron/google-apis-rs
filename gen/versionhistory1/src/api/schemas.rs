use super::*;
/// Each Channel is owned by a Platform and owns a collection of versions. Possible Channels are listed in the Channel enum below. Not all Channels are available for every Platform (e.g. CANARY does not exist for LINUX).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// Type of channel.
    #[serde(rename="channelType")]
    
    pub channel_type: Option<ChannelChannelTypeEnum>,
    /// Channel name. Format is "{product}/platforms/{platform}/channels/{channel}"
    
    pub name: Option<String>,
}

impl client::Part for Channel {}


/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Interval {
    /// Optional. Exclusive end of the interval. If specified, a Timestamp matching this interval will have to be before the end.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Inclusive start of the interval. If specified, a Timestamp matching this interval will have to be the same or after the start.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Interval {}


/// Response message for ListChannels.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels list platforms](PlatformChannelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListChannelsResponse {
    /// The list of channels.
    
    pub channels: Option<Vec<Channel>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListChannelsResponse {}


/// Response message for ListPlatforms.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list platforms](PlatformListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListPlatformsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of platforms.
    
    pub platforms: Option<Vec<Platform>>,
}

impl client::ResponseResult for ListPlatformsResponse {}


/// Response message for ListReleases.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels versions releases list platforms](PlatformChannelVersionReleaseListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReleasesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of releases.
    
    pub releases: Option<Vec<Release>>,
}

impl client::ResponseResult for ListReleasesResponse {}


/// Response message for ListVersions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels versions list platforms](PlatformChannelVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVersionsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of versions.
    
    pub versions: Option<Vec<Version>>,
}

impl client::ResponseResult for ListVersionsResponse {}


/// Each Platform is owned by a Product and owns a collection of channels. Available platforms are listed in Platform enum below. Not all Channels are available for every Platform (e.g. CANARY does not exist for LINUX).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [channels versions releases list platforms](PlatformChannelVersionReleaseListCall) (none)
/// * [channels versions list platforms](PlatformChannelVersionListCall) (none)
/// * [channels list platforms](PlatformChannelListCall) (none)
/// * [list platforms](PlatformListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Platform {
    /// Platform name. Format is "{product}/platforms/{platform}"
    
    pub name: Option<String>,
    /// Type of platform.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<PlatformPlatformTypeEnum>,
}

impl client::Resource for Platform {}


/// A Release is owned by a Version. A Release contains information about the release(s) of its parent version. This includes when the release began and ended, as well as what percentage it was released at. If the version is released again, or if the serving percentage changes, it will create another release under the version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    /// Rollout fraction. This fraction indicates the fraction of people that should receive this version in this release. If the fraction is not specified in ReleaseManager, the API will assume fraction is 1.
    
    pub fraction: Option<f64>,
    /// Rollout fraction group. Only fractions with the same fraction_group are statistically comparable: there may be non-fractional differences between different fraction groups.
    #[serde(rename="fractionGroup")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub fraction_group: Option<i64>,
    /// Release name. Format is "{product}/platforms/{platform}/channels/{channel}/versions/{version}/releases/{release}"
    
    pub name: Option<String>,
    /// Timestamp interval of when the release was live. If end_time is unspecified, the release is currently live.
    
    pub serving: Option<Interval>,
    /// String containing just the version number. e.g. "84.0.4147.38"
    
    pub version: Option<String>,
}

impl client::Part for Release {}


/// Each Version is owned by a Channel. A Version only displays the Version number (e.g. 84.0.4147.38). A Version owns a collection of releases.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Version name. Format is "{product}/platforms/{platform}/channels/{channel}/versions/{version}" e.g. "chrome/platforms/win/channels/beta/versions/84.0.4147.38"
    
    pub name: Option<String>,
    /// String containing just the version number. e.g. "84.0.4147.38"
    
    pub version: Option<String>,
}

impl client::Part for Version {}


