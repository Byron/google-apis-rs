use super::*;
/// Achievement reset all response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset all achievements](AchievementResetAllCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementResetAllResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetAllResponse`.
    
    pub kind: Option<String>,
    /// The achievement reset results.
    
    pub results: Option<Vec<AchievementResetResponse>>,
}

impl client::ResponseResult for AchievementResetAllResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset multiple for all players achievements](AchievementResetMultipleForAllPlayerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementResetMultipleForAllRequest {
    /// The IDs of achievements to reset.
    
    pub achievement_ids: Option<Vec<String>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetMultipleForAllRequest`.
    
    pub kind: Option<String>,
}

impl client::RequestValue for AchievementResetMultipleForAllRequest {}


/// An achievement reset response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset achievements](AchievementResetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementResetResponse {
    /// The current state of the achievement. This is the same as the initial state of the achievement. Possible values are: - "`HIDDEN`"- Achievement is hidden. - "`REVEALED`" - Achievement is revealed. - "`UNLOCKED`" - Achievement is unlocked. 
    #[serde(rename="currentState")]
    
    pub current_state: Option<String>,
    /// The ID of an achievement for which player state has been updated.
    #[serde(rename="definitionId")]
    
    pub definition_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#achievementResetResponse`.
    
    pub kind: Option<String>,
    /// Flag to indicate if the requested update actually occurred.
    #[serde(rename="updateOccurred")]
    
    pub update_occurred: Option<bool>,
}

impl client::ResponseResult for AchievementResetResponse {}


/// Multiple events reset all request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset multiple for all players events](EventResetMultipleForAllPlayerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventsResetMultipleForAllRequest {
    /// The IDs of events to reset.
    
    pub event_ids: Option<Vec<String>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#eventsResetMultipleForAllRequest`.
    
    pub kind: Option<String>,
}

impl client::RequestValue for EventsResetMultipleForAllRequest {}


/// 1P/3P metadata about the player's experience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesPlayerExperienceInfoResource {
    /// The current number of experience points for the player.
    #[serde(rename="currentExperiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_experience_points: Option<i64>,
    /// The current level of the player.
    #[serde(rename="currentLevel")]
    
    pub current_level: Option<GamesPlayerLevelResource>,
    /// The timestamp when the player was leveled up, in millis since Unix epoch UTC.
    #[serde(rename="lastLevelUpTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_level_up_timestamp_millis: Option<i64>,
    /// The next level of the player. If the current level is the maximum level, this should be same as the current level.
    #[serde(rename="nextLevel")]
    
    pub next_level: Option<GamesPlayerLevelResource>,
}

impl client::Part for GamesPlayerExperienceInfoResource {}


/// 1P/3P metadata about a user's level.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesPlayerLevelResource {
    /// The level for the user.
    
    pub level: Option<i32>,
    /// The maximum experience points for this level.
    #[serde(rename="maxExperiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_experience_points: Option<i64>,
    /// The minimum experience points for this level.
    #[serde(rename="minExperiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_experience_points: Option<i64>,
}

impl client::Part for GamesPlayerLevelResource {}


/// The HiddenPlayer resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HiddenPlayer {
    /// Output only. The time this player was hidden.
    #[serde(rename="hiddenTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub hidden_time_millis: Option<i64>,
    /// Output only. Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayer`.
    
    pub kind: Option<String>,
    /// Output only. The player information.
    
    pub player: Option<Player>,
}

impl client::Part for HiddenPlayer {}


/// A list of hidden players.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list hidden applications](ApplicationListHiddenCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HiddenPlayerList {
    /// The players.
    
    pub items: Option<Vec<HiddenPlayer>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayerList`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for HiddenPlayerList {}


/// A Player resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [hide players](PlayerHideCall) (none)
/// * [unhide players](PlayerUnhideCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    /// The base URL for the image that represents the player.
    #[serde(rename="avatarImageUrl")]
    
    pub avatar_image_url: Option<String>,
    /// The url to the landscape mode player banner image.
    #[serde(rename="bannerUrlLandscape")]
    
    pub banner_url_landscape: Option<String>,
    /// The url to the portrait mode player banner image.
    #[serde(rename="bannerUrlPortrait")]
    
    pub banner_url_portrait: Option<String>,
    /// The name to display for the player.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// An object to represent Play Game experience information for the player.
    #[serde(rename="experienceInfo")]
    
    pub experience_info: Option<GamesPlayerExperienceInfoResource>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#player`.
    
    pub kind: Option<String>,
    /// An object representation of the individual components of the player's name. For some players, these fields may not be present.
    
    pub name: Option<PlayerName>,
    /// The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs.
    #[serde(rename="originalPlayerId")]
    
    pub original_player_id: Option<String>,
    /// The ID of the player.
    #[serde(rename="playerId")]
    
    pub player_id: Option<String>,
    /// The player's profile settings. Controls whether or not the player's profile is visible to other players.
    #[serde(rename="profileSettings")]
    
    pub profile_settings: Option<ProfileSettings>,
    /// The player's title rewarded for their game activities.
    
    pub title: Option<String>,
}

impl client::Resource for Player {}


/// A list of leaderboard reset resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset all scores](ScoreResetAllCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScoreResetAllResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetAllResponse`.
    
    pub kind: Option<String>,
    /// The leaderboard reset results.
    
    pub results: Option<Vec<PlayerScoreResetResponse>>,
}

impl client::ResponseResult for PlayerScoreResetAllResponse {}


/// A list of reset leaderboard entry resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset scores](ScoreResetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScoreResetResponse {
    /// The ID of an leaderboard for which player state has been updated.
    #[serde(rename="definitionId")]
    
    pub definition_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#playerScoreResetResponse`.
    
    pub kind: Option<String>,
    /// The time spans of the updated score. Possible values are: - "`ALL_TIME`" - The score is an all-time score. - "`WEEKLY`" - The score is a weekly score. - "`DAILY`" - The score is a daily score. 
    #[serde(rename="resetScoreTimeSpans")]
    
    pub reset_score_time_spans: Option<Vec<String>>,
}

impl client::ResponseResult for PlayerScoreResetResponse {}


/// Profile settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileSettings {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#profileSettings`.
    
    pub kind: Option<String>,
    /// no description provided
    #[serde(rename="profileVisible")]
    
    pub profile_visible: Option<bool>,
}

impl client::Part for ProfileSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reset multiple for all players scores](ScoreResetMultipleForAllPlayerCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScoresResetMultipleForAllRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#scoresResetMultipleForAllRequest`.
    
    pub kind: Option<String>,
    /// The IDs of leaderboards to reset.
    
    pub leaderboard_ids: Option<Vec<String>>,
}

impl client::RequestValue for ScoresResetMultipleForAllRequest {}


/// An object representation of the individual components of the player's name. For some players, these fields may not be present.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerName {
    /// The family name of this player. In some places, this is known as the last name.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name of this player. In some places, this is known as the first name.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
}

impl client::NestedType for PlayerName {}
impl client::Part for PlayerName {}


