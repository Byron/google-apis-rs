use super::*;
/// An achievement configuration resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete achievement configurations](AchievementConfigurationDeleteCall) (none)
/// * [get achievement configurations](AchievementConfigurationGetCall) (response)
/// * [insert achievement configurations](AchievementConfigurationInsertCall) (request|response)
/// * [list achievement configurations](AchievementConfigurationListCall) (none)
/// * [update achievement configurations](AchievementConfigurationUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementConfiguration {
    /// The type of the achievement.
    #[serde(rename="achievementType")]
    
    pub achievement_type: Option<AchievementConfigurationAchievementTypeEnum>,
    /// The draft data of the achievement.
    
    pub draft: Option<AchievementConfigurationDetail>,
    /// The ID of the achievement.
    
    pub id: Option<String>,
    /// The initial state of the achievement.
    #[serde(rename="initialState")]
    
    pub initial_state: Option<AchievementConfigurationInitialStateEnum>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfiguration`.
    
    pub kind: Option<String>,
    /// The read-only published data of the achievement.
    
    pub published: Option<AchievementConfigurationDetail>,
    /// Steps to unlock. Only applicable to incremental achievements.
    #[serde(rename="stepsToUnlock")]
    
    pub steps_to_unlock: Option<i32>,
    /// The token for this resource.
    
    pub token: Option<String>,
}

impl client::RequestValue for AchievementConfiguration {}
impl client::Resource for AchievementConfiguration {}
impl client::ResponseResult for AchievementConfiguration {}


/// An achievement configuration detail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementConfigurationDetail {
    /// Localized strings for the achievement description.
    
    pub description: Option<LocalizedStringBundle>,
    /// The icon url of this achievement. Writes to this field are ignored.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationDetail`.
    
    pub kind: Option<String>,
    /// Localized strings for the achievement name.
    
    pub name: Option<LocalizedStringBundle>,
    /// Point value for the achievement.
    #[serde(rename="pointValue")]
    
    pub point_value: Option<i32>,
    /// The sort rank of this achievement. Writes to this field are ignored.
    #[serde(rename="sortRank")]
    
    pub sort_rank: Option<i32>,
}

impl client::Part for AchievementConfigurationDetail {}


/// A ListConfigurations response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievement configurations](AchievementConfigurationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementConfigurationListResponse {
    /// The achievement configurations.
    
    pub items: Option<Vec<AchievementConfiguration>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfigurationListResponse`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AchievementConfigurationListResponse {}


/// A number affix resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesNumberAffixConfiguration {
    /// When the language requires special treatment of "small" numbers (as with 2, 3, and 4 in Czech; or numbers ending 2, 3, or 4 but not 12, 13, or 14 in Polish).
    
    pub few: Option<LocalizedStringBundle>,
    /// When the language requires special treatment of "large" numbers (as with numbers ending 11-99 in Maltese).
    
    pub many: Option<LocalizedStringBundle>,
    /// When the language requires special treatment of numbers like one (as with the number 1 in English and most other languages; in Russian, any number ending in 1 but not ending in 11 is in this class).
    
    pub one: Option<LocalizedStringBundle>,
    /// When the language does not require special treatment of the given quantity (as with all numbers in Chinese, or 42 in English).
    
    pub other: Option<LocalizedStringBundle>,
    /// When the language requires special treatment of numbers like two (as with 2 in Welsh, or 102 in Slovenian).
    
    pub two: Option<LocalizedStringBundle>,
    /// When the language requires special treatment of the number 0 (as in Arabic).
    
    pub zero: Option<LocalizedStringBundle>,
}

impl client::Part for GamesNumberAffixConfiguration {}


/// A number format resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesNumberFormatConfiguration {
    /// The curreny code string. Only used for CURRENCY format type.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// The number of decimal places for number. Only used for NUMERIC format type.
    #[serde(rename="numDecimalPlaces")]
    
    pub num_decimal_places: Option<i32>,
    /// The formatting for the number.
    #[serde(rename="numberFormatType")]
    
    pub number_format_type: Option<GamesNumberFormatConfigurationNumberFormatTypeEnum>,
    /// An optional suffix for the NUMERIC format type. These strings follow the same plural rules as all Android string resources.
    
    pub suffix: Option<GamesNumberAffixConfiguration>,
}

impl client::Part for GamesNumberFormatConfiguration {}


/// An leaderboard configuration resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete leaderboard configurations](LeaderboardConfigurationDeleteCall) (none)
/// * [get leaderboard configurations](LeaderboardConfigurationGetCall) (response)
/// * [insert leaderboard configurations](LeaderboardConfigurationInsertCall) (request|response)
/// * [list leaderboard configurations](LeaderboardConfigurationListCall) (none)
/// * [update leaderboard configurations](LeaderboardConfigurationUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardConfiguration {
    /// The draft data of the leaderboard.
    
    pub draft: Option<LeaderboardConfigurationDetail>,
    /// The ID of the leaderboard.
    
    pub id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfiguration`.
    
    pub kind: Option<String>,
    /// The read-only published data of the leaderboard.
    
    pub published: Option<LeaderboardConfigurationDetail>,
    /// Maximum score that can be posted to this leaderboard.
    #[serde(rename="scoreMax")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score_max: Option<i64>,
    /// Minimum score that can be posted to this leaderboard.
    #[serde(rename="scoreMin")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score_min: Option<i64>,
    /// no description provided
    #[serde(rename="scoreOrder")]
    
    pub score_order: Option<LeaderboardConfigurationScoreOrderEnum>,
    /// The token for this resource.
    
    pub token: Option<String>,
}

impl client::RequestValue for LeaderboardConfiguration {}
impl client::Resource for LeaderboardConfiguration {}
impl client::ResponseResult for LeaderboardConfiguration {}


/// A leaderboard configuration detail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardConfigurationDetail {
    /// The icon url of this leaderboard. Writes to this field are ignored.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationDetail`.
    
    pub kind: Option<String>,
    /// Localized strings for the leaderboard name.
    
    pub name: Option<LocalizedStringBundle>,
    /// The score formatting for the leaderboard.
    #[serde(rename="scoreFormat")]
    
    pub score_format: Option<GamesNumberFormatConfiguration>,
    /// The sort rank of this leaderboard. Writes to this field are ignored.
    #[serde(rename="sortRank")]
    
    pub sort_rank: Option<i32>,
}

impl client::Part for LeaderboardConfigurationDetail {}


/// A ListConfigurations response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leaderboard configurations](LeaderboardConfigurationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardConfigurationListResponse {
    /// The leaderboard configurations.
    
    pub items: Option<Vec<LeaderboardConfiguration>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfigurationListResponse`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for LeaderboardConfigurationListResponse {}


/// A localized string resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedString`.
    
    pub kind: Option<String>,
    /// The locale string.
    
    pub locale: Option<String>,
    /// The string value.
    
    pub value: Option<String>,
}

impl client::Part for LocalizedString {}


/// A localized string bundle resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedStringBundle {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#localizedStringBundle`.
    
    pub kind: Option<String>,
    /// The locale strings.
    
    pub translations: Option<Vec<LocalizedString>>,
}

impl client::Part for LocalizedStringBundle {}


