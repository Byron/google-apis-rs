use super::*;
/// An achievement definition object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievement definitions](AchievementDefinitionListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementDefinition {
    /// The type of the achievement.
    #[serde(rename="achievementType")]
    
    pub achievement_type: Option<AchievementDefinitionAchievementTypeEnum>,
    /// The description of the achievement.
    
    pub description: Option<String>,
    /// Experience points which will be earned when unlocking this achievement.
    #[serde(rename="experiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub experience_points: Option<i64>,
    /// The total steps for an incremental achievement as a string.
    #[serde(rename="formattedTotalSteps")]
    
    pub formatted_total_steps: Option<String>,
    /// The ID of the achievement.
    
    pub id: Option<String>,
    /// The initial state of the achievement.
    #[serde(rename="initialState")]
    
    pub initial_state: Option<AchievementDefinitionInitialStateEnum>,
    /// Indicates whether the revealed icon image being returned is a default image, or is provided by the game.
    #[serde(rename="isRevealedIconUrlDefault")]
    
    pub is_revealed_icon_url_default: Option<bool>,
    /// Indicates whether the unlocked icon image being returned is a default image, or is game-provided.
    #[serde(rename="isUnlockedIconUrlDefault")]
    
    pub is_unlocked_icon_url_default: Option<bool>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinition`.
    
    pub kind: Option<String>,
    /// The name of the achievement.
    
    pub name: Option<String>,
    /// The image URL for the revealed achievement icon.
    #[serde(rename="revealedIconUrl")]
    
    pub revealed_icon_url: Option<String>,
    /// The total steps for an incremental achievement.
    #[serde(rename="totalSteps")]
    
    pub total_steps: Option<i32>,
    /// The image URL for the unlocked achievement icon.
    #[serde(rename="unlockedIconUrl")]
    
    pub unlocked_icon_url: Option<String>,
}

impl client::Resource for AchievementDefinition {}


/// A list of achievement definition objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievement definitions](AchievementDefinitionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementDefinitionsListResponse {
    /// The achievement definitions.
    
    pub items: Option<Vec<AchievementDefinition>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinitionsListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for AchievementDefinitionsListResponse {}


/// An achievement increment response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [increment achievements](AchievementIncrementCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementIncrementResponse {
    /// The current steps recorded for this incremental achievement.
    #[serde(rename="currentSteps")]
    
    pub current_steps: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementIncrementResponse`.
    
    pub kind: Option<String>,
    /// Whether the current steps for the achievement has reached the number of steps required to unlock.
    #[serde(rename="newlyUnlocked")]
    
    pub newly_unlocked: Option<bool>,
}

impl client::ResponseResult for AchievementIncrementResponse {}


/// An achievement reveal response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reveal achievements](AchievementRevealCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementRevealResponse {
    /// The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked.
    #[serde(rename="currentState")]
    
    pub current_state: Option<AchievementRevealResponseCurrentStateEnum>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementRevealResponse`.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for AchievementRevealResponse {}


/// An achievement set steps at least response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set steps at least achievements](AchievementSetStepsAtLeastCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementSetStepsAtLeastResponse {
    /// The current steps recorded for this incremental achievement.
    #[serde(rename="currentSteps")]
    
    pub current_steps: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementSetStepsAtLeastResponse`.
    
    pub kind: Option<String>,
    /// Whether the current steps for the achievement has reached the number of steps required to unlock.
    #[serde(rename="newlyUnlocked")]
    
    pub newly_unlocked: Option<bool>,
}

impl client::ResponseResult for AchievementSetStepsAtLeastResponse {}


/// An achievement unlock response
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [unlock achievements](AchievementUnlockCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUnlockResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUnlockResponse`.
    
    pub kind: Option<String>,
    /// Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player).
    #[serde(rename="newlyUnlocked")]
    
    pub newly_unlocked: Option<bool>,
}

impl client::ResponseResult for AchievementUnlockResponse {}


/// A list of achievement update requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update multiple achievements](AchievementUpdateMultipleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUpdateMultipleRequest {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleRequest`.
    
    pub kind: Option<String>,
    /// The individual achievement update requests.
    
    pub updates: Option<Vec<AchievementUpdateRequest>>,
}

impl client::RequestValue for AchievementUpdateMultipleRequest {}


/// Response message for UpdateMultipleAchievements rpc.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update multiple achievements](AchievementUpdateMultipleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUpdateMultipleResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleResponse`.
    
    pub kind: Option<String>,
    /// The updated state of the achievements.
    #[serde(rename="updatedAchievements")]
    
    pub updated_achievements: Option<Vec<AchievementUpdateResponse>>,
}

impl client::ResponseResult for AchievementUpdateMultipleResponse {}


/// A request to update an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUpdateRequest {
    /// The achievement this update is being applied to.
    #[serde(rename="achievementId")]
    
    pub achievement_id: Option<String>,
    /// The payload if an update of type `INCREMENT` was requested for the achievement.
    #[serde(rename="incrementPayload")]
    
    pub increment_payload: Option<GamesAchievementIncrement>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateRequest`.
    
    pub kind: Option<String>,
    /// The payload if an update of type `SET_STEPS_AT_LEAST` was requested for the achievement.
    #[serde(rename="setStepsAtLeastPayload")]
    
    pub set_steps_at_least_payload: Option<GamesAchievementSetStepsAtLeast>,
    /// The type of update being applied.
    #[serde(rename="updateType")]
    
    pub update_type: Option<AchievementUpdateRequestUpdateTypeEnum>,
}

impl client::Part for AchievementUpdateRequest {}


/// An updated achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AchievementUpdateResponse {
    /// The achievement this update is was applied to.
    #[serde(rename="achievementId")]
    
    pub achievement_id: Option<String>,
    /// The current state of the achievement.
    #[serde(rename="currentState")]
    
    pub current_state: Option<AchievementUpdateResponseCurrentStateEnum>,
    /// The current steps recorded for this achievement if it is incremental.
    #[serde(rename="currentSteps")]
    
    pub current_steps: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateResponse`.
    
    pub kind: Option<String>,
    /// Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player).
    #[serde(rename="newlyUnlocked")]
    
    pub newly_unlocked: Option<bool>,
    /// Whether the requested updates actually affected the achievement.
    #[serde(rename="updateOccurred")]
    
    pub update_occurred: Option<bool>,
}

impl client::Part for AchievementUpdateResponse {}


/// The Application resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get applications](ApplicationGetCall) (response)
/// * [get end point applications](ApplicationGetEndPointCall) (none)
/// * [played applications](ApplicationPlayedCall) (none)
/// * [verify applications](ApplicationVerifyCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// The number of achievements visible to the currently authenticated player.
    
    pub achievement_count: Option<i32>,
    /// The assets of the application.
    
    pub assets: Option<Vec<ImageAsset>>,
    /// The author of the application.
    
    pub author: Option<String>,
    /// The category of the application.
    
    pub category: Option<ApplicationCategory>,
    /// The description of the application.
    
    pub description: Option<String>,
    /// A list of features that have been enabled for the application.
    #[serde(rename="enabledFeatures")]
    
    pub enabled_features: Option<Vec<ApplicationEnabledFeaturesEnum>>,
    /// The ID of the application.
    
    pub id: Option<String>,
    /// The instances of the application.
    
    pub instances: Option<Vec<Instance>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#application`.
    
    pub kind: Option<String>,
    /// The last updated timestamp of the application.
    #[serde(rename="lastUpdatedTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_updated_timestamp: Option<i64>,
    /// The number of leaderboards visible to the currently authenticated player.
    
    pub leaderboard_count: Option<i32>,
    /// The name of the application.
    
    pub name: Option<String>,
    /// A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. "E0E0E0").
    #[serde(rename="themeColor")]
    
    pub theme_color: Option<String>,
}

impl client::Resource for Application {}
impl client::ResponseResult for Application {}


/// An application category object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCategory {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationCategory`.
    
    pub kind: Option<String>,
    /// The primary category.
    
    pub primary: Option<String>,
    /// The secondary category.
    
    pub secondary: Option<String>,
}

impl client::Part for ApplicationCategory {}


/// A third party application verification response resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [verify applications](ApplicationVerifyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationVerifyResponse {
    /// An alternate ID that was once used for the player that was issued the auth token used in this request. (This field is not normally populated.)
    
    pub alternate_player_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationVerifyResponse`.
    
    pub kind: Option<String>,
    /// The ID of the player that was issued the auth token used in this request.
    
    pub player_id: Option<String>,
}

impl client::ResponseResult for ApplicationVerifyResponse {}


/// Data related to individual game categories.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// The category name.
    
    pub category: Option<String>,
    /// Experience points earned in this category.
    #[serde(rename="experiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub experience_points: Option<i64>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#category`.
    
    pub kind: Option<String>,
}

impl client::Part for Category {}


/// A third party list metagame categories response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list categories by player metagame](MetagameListCategoriesByPlayerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryListResponse {
    /// The list of categories with usage data.
    
    pub items: Option<Vec<Category>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#categoryListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for CategoryListResponse {}


/// Container for a URL end point of the requested type.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get end point applications](ApplicationGetEndPointCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EndPoint {
    /// A URL suitable for loading in a web browser for the requested endpoint.
    
    pub url: Option<String>,
}

impl client::ResponseResult for EndPoint {}


/// A batch update failure resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventBatchRecordFailure {
    /// The cause for the update failure.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<EventBatchRecordFailureFailureCauseEnum>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventBatchRecordFailure`.
    
    pub kind: Option<String>,
    /// The time range which was rejected; empty for a request-wide failure.
    
    pub range: Option<EventPeriodRange>,
}

impl client::Part for EventBatchRecordFailure {}


/// An event child relationship resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventChild {
    /// The ID of the child event.
    #[serde(rename="childId")]
    
    pub child_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventChild`.
    
    pub kind: Option<String>,
}

impl client::Part for EventChild {}


/// An event definition resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventDefinition {
    /// A list of events that are a child of this event.
    #[serde(rename="childEvents")]
    
    pub child_events: Option<Vec<EventChild>>,
    /// Description of what this event represents.
    
    pub description: Option<String>,
    /// The name to display for the event.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The ID of the event.
    
    pub id: Option<String>,
    /// The base URL for the image that represents the event.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// Indicates whether the icon image being returned is a default image, or is game-provided.
    #[serde(rename="isDefaultImageUrl")]
    
    pub is_default_image_url: Option<bool>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinition`.
    
    pub kind: Option<String>,
    /// The visibility of event being tracked in this definition.
    
    pub visibility: Option<EventDefinitionVisibilityEnum>,
}

impl client::Part for EventDefinition {}


/// A ListDefinitions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list definitions events](EventListDefinitionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventDefinitionListResponse {
    /// The event definitions.
    
    pub items: Option<Vec<EventDefinition>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinitionListResponse`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for EventDefinitionListResponse {}


/// An event period time range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventPeriodRange {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodRange`.
    
    pub kind: Option<String>,
    /// The time when this update period ends, in millis, since 1970 UTC (Unix Epoch).
    #[serde(rename="periodEndMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub period_end_millis: Option<i64>,
    /// The time when this update period begins, in millis, since 1970 UTC (Unix Epoch).
    #[serde(rename="periodStartMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub period_start_millis: Option<i64>,
}

impl client::Part for EventPeriodRange {}


/// An event period update resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventPeriodUpdate {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodUpdate`.
    
    pub kind: Option<String>,
    /// The time period being covered by this update.
    #[serde(rename="timePeriod")]
    
    pub time_period: Option<EventPeriodRange>,
    /// The updates being made for this time period.
    
    pub updates: Option<Vec<EventUpdateRequest>>,
}

impl client::Part for EventPeriodUpdate {}


/// An event update failure resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventRecordFailure {
    /// The ID of the event that was not updated.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// The cause for the update failure.
    #[serde(rename="failureCause")]
    
    pub failure_cause: Option<EventRecordFailureFailureCauseEnum>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordFailure`.
    
    pub kind: Option<String>,
}

impl client::Part for EventRecordFailure {}


/// An event period update resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [record events](EventRecordCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventRecordRequest {
    /// The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch).
    #[serde(rename="currentTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_time_millis: Option<i64>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordRequest`.
    
    pub kind: Option<String>,
    /// The request ID used to identify this attempt to record events.
    #[serde(rename="requestId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_id: Option<i64>,
    /// A list of the time period updates being made in this request.
    #[serde(rename="timePeriods")]
    
    pub time_periods: Option<Vec<EventPeriodUpdate>>,
}

impl client::RequestValue for EventRecordRequest {}


/// An event period update resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventUpdateRequest {
    /// The ID of the event being modified in this update.
    #[serde(rename="definitionId")]
    
    pub definition_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateRequest`.
    
    pub kind: Option<String>,
    /// The number of times this event occurred in this time period.
    #[serde(rename="updateCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub update_count: Option<i64>,
}

impl client::Part for EventUpdateRequest {}


/// An event period update resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [record events](EventRecordCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventUpdateResponse {
    /// Any batch-wide failures which occurred applying updates.
    #[serde(rename="batchFailures")]
    
    pub batch_failures: Option<Vec<EventBatchRecordFailure>>,
    /// Any failures updating a particular event.
    #[serde(rename="eventFailures")]
    
    pub event_failures: Option<Vec<EventRecordFailure>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateResponse`.
    
    pub kind: Option<String>,
    /// The current status of any updated events
    #[serde(rename="playerEvents")]
    
    pub player_events: Option<Vec<PlayerEvent>>,
}

impl client::ResponseResult for EventUpdateResponse {}


/// The payload to request to increment an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesAchievementIncrement {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementIncrement`.
    
    pub kind: Option<String>,
    /// The requestId associated with an increment to an achievement.
    #[serde(rename="requestId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub request_id: Option<i64>,
    /// The number of steps to be incremented.
    
    pub steps: Option<i32>,
}

impl client::Part for GamesAchievementIncrement {}


/// The payload to request to increment an achievement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GamesAchievementSetStepsAtLeast {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementSetStepsAtLeast`.
    
    pub kind: Option<String>,
    /// The minimum number of steps for the achievement to be set to.
    
    pub steps: Option<i32>,
}

impl client::Part for GamesAchievementSetStepsAtLeast {}


/// An image asset object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageAsset {
    /// The height of the asset.
    
    pub height: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#imageAsset`.
    
    pub kind: Option<String>,
    /// The name of the asset.
    
    pub name: Option<String>,
    /// The URL of the asset.
    
    pub url: Option<String>,
    /// The width of the asset.
    
    pub width: Option<i32>,
}

impl client::Part for ImageAsset {}


/// The Instance resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Instance {
    /// URI which shows where a user can acquire this instance.
    #[serde(rename="acquisitionUri")]
    
    pub acquisition_uri: Option<String>,
    /// Platform dependent details for Android.
    #[serde(rename="androidInstance")]
    
    pub android_instance: Option<InstanceAndroidDetails>,
    /// Platform dependent details for iOS.
    #[serde(rename="iosInstance")]
    
    pub ios_instance: Option<InstanceIosDetails>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#instance`.
    
    pub kind: Option<String>,
    /// Localized display name.
    
    pub name: Option<String>,
    /// The platform type.
    #[serde(rename="platformType")]
    
    pub platform_type: Option<InstancePlatformTypeEnum>,
    /// Flag to show if this game instance supports realtime play.
    #[serde(rename="realtimePlay")]
    
    pub realtime_play: Option<bool>,
    /// Flag to show if this game instance supports turn based play.
    #[serde(rename="turnBasedPlay")]
    
    pub turn_based_play: Option<bool>,
    /// Platform dependent details for Web.
    #[serde(rename="webInstance")]
    
    pub web_instance: Option<InstanceWebDetails>,
}

impl client::Part for Instance {}


/// The Android instance details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceAndroidDetails {
    /// Flag indicating whether the anti-piracy check is enabled.
    #[serde(rename="enablePiracyCheck")]
    
    pub enable_piracy_check: Option<bool>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceAndroidDetails`.
    
    pub kind: Option<String>,
    /// Android package name which maps to Google Play URL.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Indicates that this instance is the default for new installations.
    
    pub preferred: Option<bool>,
}

impl client::Part for InstanceAndroidDetails {}


/// The iOS details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceIosDetails {
    /// Bundle identifier.
    #[serde(rename="bundleIdentifier")]
    
    pub bundle_identifier: Option<String>,
    /// iTunes App ID.
    #[serde(rename="itunesAppId")]
    
    pub itunes_app_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceIosDetails`.
    
    pub kind: Option<String>,
    /// Indicates that this instance is the default for new installations on iPad devices.
    #[serde(rename="preferredForIpad")]
    
    pub preferred_for_ipad: Option<bool>,
    /// Indicates that this instance is the default for new installations on iPhone devices.
    #[serde(rename="preferredForIphone")]
    
    pub preferred_for_iphone: Option<bool>,
    /// Flag to indicate if this instance supports iPad.
    #[serde(rename="supportIpad")]
    
    pub support_ipad: Option<bool>,
    /// Flag to indicate if this instance supports iPhone.
    #[serde(rename="supportIphone")]
    
    pub support_iphone: Option<bool>,
}

impl client::Part for InstanceIosDetails {}


/// The Web details resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceWebDetails {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceWebDetails`.
    
    pub kind: Option<String>,
    /// Launch URL for the game.
    #[serde(rename="launchUrl")]
    
    pub launch_url: Option<String>,
    /// Indicates that this instance is the default for new installations.
    
    pub preferred: Option<bool>,
}

impl client::Part for InstanceWebDetails {}


/// The Leaderboard resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get leaderboards](LeaderboardGetCall) (response)
/// * [list leaderboards](LeaderboardListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Leaderboard {
    /// The icon for the leaderboard.
    #[serde(rename="iconUrl")]
    
    pub icon_url: Option<String>,
    /// The leaderboard ID.
    
    pub id: Option<String>,
    /// Indicates whether the icon image being returned is a default image, or is game-provided.
    #[serde(rename="isIconUrlDefault")]
    
    pub is_icon_url_default: Option<bool>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboard`.
    
    pub kind: Option<String>,
    /// The name of the leaderboard.
    
    pub name: Option<String>,
    /// How scores are ordered.
    
    pub order: Option<LeaderboardOrderEnum>,
}

impl client::Resource for Leaderboard {}
impl client::ResponseResult for Leaderboard {}


/// The Leaderboard Entry resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// The localized string for the numerical value of this score.
    #[serde(rename="formattedScore")]
    
    pub formatted_score: Option<String>,
    /// The localized string for the rank of this score for this leaderboard.
    #[serde(rename="formattedScoreRank")]
    
    pub formatted_score_rank: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardEntry`.
    
    pub kind: Option<String>,
    /// The player who holds this score.
    
    pub player: Option<Player>,
    /// The rank of this score for this leaderboard.
    #[serde(rename="scoreRank")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score_rank: Option<i64>,
    /// Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.
    #[serde(rename="scoreTag")]
    
    pub score_tag: Option<String>,
    /// The numerical value of this score.
    #[serde(rename="scoreValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score_value: Option<i64>,
    /// The time span of this high score.
    #[serde(rename="timeSpan")]
    
    pub time_span: Option<LeaderboardEntryTimeSpanEnum>,
    /// The timestamp at which this score was recorded, in milliseconds since the epoch in UTC.
    #[serde(rename="writeTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub write_timestamp_millis: Option<i64>,
}

impl client::Part for LeaderboardEntry {}


/// A list of leaderboard objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list leaderboards](LeaderboardListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardListResponse {
    /// The leaderboards.
    
    pub items: Option<Vec<Leaderboard>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for LeaderboardListResponse {}


/// A score rank in a leaderboard.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardScoreRank {
    /// The number of scores in the leaderboard as a string.
    #[serde(rename="formattedNumScores")]
    
    pub formatted_num_scores: Option<String>,
    /// The rank in the leaderboard as a string.
    #[serde(rename="formattedRank")]
    
    pub formatted_rank: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScoreRank`.
    
    pub kind: Option<String>,
    /// The number of scores in the leaderboard.
    #[serde(rename="numScores")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_scores: Option<i64>,
    /// The rank in the leaderboard.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub rank: Option<i64>,
}

impl client::Part for LeaderboardScoreRank {}


/// A ListScores response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list scores](ScoreListCall) (response)
/// * [list window scores](ScoreListWindowCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardScores {
    /// The scores in the leaderboard.
    
    pub items: Option<Vec<LeaderboardEntry>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScores`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The total number of scores in the leaderboard.
    #[serde(rename="numScores")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_scores: Option<i64>,
    /// The score of the requesting player on the leaderboard. The player's score may appear both here and in the list of scores above. If you are viewing a public leaderboard and the player is not sharing their gameplay information publicly, the `scoreRank`and `formattedScoreRank` values will not be present.
    #[serde(rename="playerScore")]
    
    pub player_score: Option<LeaderboardEntry>,
    /// The pagination token for the previous page of results.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
}

impl client::ResponseResult for LeaderboardScores {}


/// The metagame config resource
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metagame config metagame](MetagameGetMetagameConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetagameConfig {
    /// Current version of the metagame configuration data. When this data is updated, the version number will be increased by one.
    #[serde(rename="currentVersion")]
    
    pub current_version: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#metagameConfig`.
    
    pub kind: Option<String>,
    /// The list of player levels.
    #[serde(rename="playerLevels")]
    
    pub player_levels: Option<Vec<PlayerLevel>>,
}

impl client::ResponseResult for MetagameConfig {}


/// A Player resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get players](PlayerGetCall) (response)
/// * [get scoped player ids players](PlayerGetScopedPlayerIdCall) (none)
/// * [list players](PlayerListCall) (none)
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
    
    pub experience_info: Option<PlayerExperienceInfo>,
    /// The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game.
    #[serde(rename="friendStatus")]
    
    pub friend_status: Option<PlayerFriendStatusEnum>,
    /// Per-application unique player identifier.
    #[serde(rename="gamePlayerId")]
    
    pub game_player_id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#player`
    
    pub kind: Option<String>,
    /// A representation of the individual components of the name.
    
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
impl client::ResponseResult for Player {}


/// An achievement object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAchievement {
    /// The state of the achievement.
    #[serde(rename="achievementState")]
    
    pub achievement_state: Option<PlayerAchievementAchievementStateEnum>,
    /// The current steps for an incremental achievement.
    #[serde(rename="currentSteps")]
    
    pub current_steps: Option<i32>,
    /// Experience points earned for the achievement. This field is absent for achievements that have not yet been unlocked and 0 for achievements that have been unlocked by testers but that are unpublished.
    #[serde(rename="experiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub experience_points: Option<i64>,
    /// The current steps for an incremental achievement as a string.
    #[serde(rename="formattedCurrentStepsString")]
    
    pub formatted_current_steps_string: Option<String>,
    /// The ID of the achievement.
    
    pub id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievement`.
    
    pub kind: Option<String>,
    /// The timestamp of the last modification to this achievement's state.
    #[serde(rename="lastUpdatedTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_updated_timestamp: Option<i64>,
}

impl client::Part for PlayerAchievement {}


/// A list of achievement objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list achievements](AchievementListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerAchievementListResponse {
    /// The achievements.
    
    pub items: Option<Vec<PlayerAchievement>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievementListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for PlayerAchievementListResponse {}


/// An event status resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEvent {
    /// The ID of the event definition.
    #[serde(rename="definitionId")]
    
    pub definition_id: Option<String>,
    /// The current number of times this event has occurred, as a string. The formatting of this string depends on the configuration of your event in the Play Games Developer Console.
    #[serde(rename="formattedNumEvents")]
    
    pub formatted_num_events: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEvent`.
    
    pub kind: Option<String>,
    /// The current number of times this event has occurred.
    #[serde(rename="numEvents")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_events: Option<i64>,
    /// The ID of the player.
    #[serde(rename="playerId")]
    
    pub player_id: Option<String>,
}

impl client::Part for PlayerEvent {}


/// A ListByPlayer response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list by player events](EventListByPlayerCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEventListResponse {
    /// The player events.
    
    pub items: Option<Vec<PlayerEvent>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEventListResponse`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for PlayerEventListResponse {}


/// 1P/3P metadata about the player's experience.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerExperienceInfo {
    /// The current number of experience points for the player.
    #[serde(rename="currentExperiencePoints")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_experience_points: Option<i64>,
    /// The current level of the player.
    #[serde(rename="currentLevel")]
    
    pub current_level: Option<PlayerLevel>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerExperienceInfo`.
    
    pub kind: Option<String>,
    /// The timestamp when the player was leveled up, in millis since Unix epoch UTC.
    #[serde(rename="lastLevelUpTimestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_level_up_timestamp_millis: Option<i64>,
    /// The next level of the player. If the current level is the maximum level, this should be same as the current level.
    #[serde(rename="nextLevel")]
    
    pub next_level: Option<PlayerLevel>,
}

impl client::Part for PlayerExperienceInfo {}


/// A player leaderboard score object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerLeaderboardScore {
    /// The rank of the score in the friends collection for this leaderboard.
    #[serde(rename="friendsRank")]
    
    pub friends_rank: Option<LeaderboardScoreRank>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScore`.
    
    pub kind: Option<String>,
    /// The ID of the leaderboard this score is in.
    
    pub leaderboard_id: Option<String>,
    /// The public rank of the score in this leaderboard. This object will not be present if the user is not sharing their scores publicly.
    #[serde(rename="publicRank")]
    
    pub public_rank: Option<LeaderboardScoreRank>,
    /// The formatted value of this score.
    #[serde(rename="scoreString")]
    
    pub score_string: Option<String>,
    /// Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.
    #[serde(rename="scoreTag")]
    
    pub score_tag: Option<String>,
    /// The numerical value of this score.
    #[serde(rename="scoreValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score_value: Option<i64>,
    /// The social rank of the score in this leaderboard.
    #[serde(rename="socialRank")]
    
    pub social_rank: Option<LeaderboardScoreRank>,
    /// The time span of this score.
    #[serde(rename="timeSpan")]
    
    pub time_span: Option<PlayerLeaderboardScoreTimeSpanEnum>,
    /// The timestamp at which this score was recorded, in milliseconds since the epoch in UTC.
    #[serde(rename="writeTimestamp")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub write_timestamp: Option<i64>,
}

impl client::Part for PlayerLeaderboardScore {}


/// A list of player leaderboard scores.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get scores](ScoreGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerLeaderboardScoreListResponse {
    /// The leaderboard scores.
    
    pub items: Option<Vec<PlayerLeaderboardScore>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScoreListResponse`.
    
    pub kind: Option<String>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The Player resources for the owner of this score.
    
    pub player: Option<Player>,
}

impl client::ResponseResult for PlayerLeaderboardScoreListResponse {}


/// 1P/3P metadata about a user's level.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerLevel {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLevel`.
    
    pub kind: Option<String>,
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

impl client::Part for PlayerLevel {}


/// A third party player list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list players](PlayerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerListResponse {
    /// The players.
    
    pub items: Option<Vec<Player>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for PlayerListResponse {}


/// A player score.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScore {
    /// The formatted score for this player score.
    #[serde(rename="formattedScore")]
    
    pub formatted_score: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScore`.
    
    pub kind: Option<String>,
    /// The numerical value for this player score.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score: Option<i64>,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.
    #[serde(rename="scoreTag")]
    
    pub score_tag: Option<String>,
    /// The time span for this player score.
    #[serde(rename="timeSpan")]
    
    pub time_span: Option<PlayerScoreTimeSpanEnum>,
}

impl client::Part for PlayerScore {}


/// A list of score submission statuses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit multiple scores](ScoreSubmitMultipleCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScoreListResponse {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreListResponse`.
    
    pub kind: Option<String>,
    /// The score submissions statuses.
    #[serde(rename="submittedScores")]
    
    pub submitted_scores: Option<Vec<PlayerScoreResponse>>,
}

impl client::ResponseResult for PlayerScoreListResponse {}


/// A list of leaderboard entry resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit scores](ScoreSubmitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScoreResponse {
    /// The time spans where the submitted score is better than the existing score for that time span.
    #[serde(rename="beatenScoreTimeSpans")]
    
    pub beaten_score_time_spans: Option<Vec<PlayerScoreResponseBeatenScoreTimeSpansEnum>>,
    /// The formatted value of the submitted score.
    #[serde(rename="formattedScore")]
    
    pub formatted_score: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreResponse`.
    
    pub kind: Option<String>,
    /// The leaderboard ID that this score was submitted to.
    #[serde(rename="leaderboardId")]
    
    pub leaderboard_id: Option<String>,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.
    #[serde(rename="scoreTag")]
    
    pub score_tag: Option<String>,
    /// The scores in time spans that have not been beaten. As an example, the submitted score may be better than the player's `DAILY` score, but not better than the player's scores for the `WEEKLY` or `ALL_TIME` time spans.
    #[serde(rename="unbeatenScores")]
    
    pub unbeaten_scores: Option<Vec<PlayerScore>>,
}

impl client::ResponseResult for PlayerScoreResponse {}


/// A list of score submission requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [submit multiple scores](ScoreSubmitMultipleCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerScoreSubmissionList {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreSubmissionList`.
    
    pub kind: Option<String>,
    /// The score submissions.
    
    pub scores: Option<Vec<ScoreSubmission>>,
}

impl client::RequestValue for PlayerScoreSubmissionList {}


/// Profile settings
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileSettings {
    /// no description provided
    #[serde(rename="friendsListVisibility")]
    
    pub friends_list_visibility: Option<ProfileSettingFriendsListVisibilityEnum>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#profileSettings`.
    
    pub kind: Option<String>,
    /// Whether the player's profile is visible to the currently signed in player.
    #[serde(rename="profileVisible")]
    
    pub profile_visible: Option<bool>,
}

impl client::Part for ProfileSettings {}


/// A third party checking a revision response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [check revisions](RevisionCheckCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevisionCheckResponse {
    /// The version of the API this client revision should use when calling API methods.
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#revisionCheckResponse`.
    
    pub kind: Option<String>,
    /// The result of the revision check.
    #[serde(rename="revisionStatus")]
    
    pub revision_status: Option<RevisionCheckResponseRevisionStatusEnum>,
}

impl client::ResponseResult for RevisionCheckResponse {}


/// Scoped player identifiers.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get scoped player ids players](PlayerGetScopedPlayerIdCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScopedPlayerIds {
    /// Identifier of the player across all games of the given developer. Every player has the same developer_player_key in all games of one developer. Developer player key changes for the game if the game is transferred to another developer. Note that game_player_id will stay unchanged.
    #[serde(rename="developerPlayerKey")]
    
    pub developer_player_key: Option<String>,
    /// Game-scoped player identifier. This is the same id that is returned in GetPlayer game_player_id field.
    #[serde(rename="gamePlayerId")]
    
    pub game_player_id: Option<String>,
}

impl client::ResponseResult for ScopedPlayerIds {}


/// A request to submit a score to leaderboards.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScoreSubmission {
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#scoreSubmission`.
    
    pub kind: Option<String>,
    /// The leaderboard this score is being submitted to.
    #[serde(rename="leaderboardId")]
    
    pub leaderboard_id: Option<String>,
    /// The new score being submitted.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub score: Option<i64>,
    /// Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986.
    #[serde(rename="scoreTag")]
    
    pub score_tag: Option<String>,
    /// Signature Values will contain URI-safe characters as defined by section 2.3 of RFC 3986.
    
    pub signature: Option<String>,
}

impl client::Part for ScoreSubmission {}


/// An snapshot object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get snapshots](SnapshotGetCall) (response)
/// * [list snapshots](SnapshotListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snapshot {
    /// The cover image of this snapshot. May be absent if there is no image.
    #[serde(rename="coverImage")]
    
    pub cover_image: Option<SnapshotImage>,
    /// The description of this snapshot.
    
    pub description: Option<String>,
    /// The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller.
    #[serde(rename="driveId")]
    
    pub drive_id: Option<String>,
    /// The duration associated with this snapshot, in millis.
    #[serde(rename="durationMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_millis: Option<i64>,
    /// The ID of the snapshot.
    
    pub id: Option<String>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshot`.
    
    pub kind: Option<String>,
    /// The timestamp (in millis since Unix epoch) of the last modification to this snapshot.
    #[serde(rename="lastModifiedMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_millis: Option<i64>,
    /// The progress value (64-bit integer set by developer) associated with this snapshot.
    #[serde(rename="progressValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub progress_value: Option<i64>,
    /// The title of this snapshot.
    
    pub title: Option<String>,
    /// The type of this snapshot.
    #[serde(rename="type")]
    
    pub type_: Option<SnapshotTypeEnum>,
    /// The unique name provided when the snapshot was created.
    #[serde(rename="uniqueName")]
    
    pub unique_name: Option<String>,
}

impl client::Resource for Snapshot {}
impl client::ResponseResult for Snapshot {}


/// An image of a snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotImage {
    /// The height of the image.
    
    pub height: Option<i32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotImage`.
    
    pub kind: Option<String>,
    /// The MIME type of the image.
    
    pub mime_type: Option<String>,
    /// The URL of the image. This URL may be invalidated at any time and should not be cached.
    
    pub url: Option<String>,
    /// The width of the image.
    
    pub width: Option<i32>,
}

impl client::Part for SnapshotImage {}


/// A third party list snapshots response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list snapshots](SnapshotListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotListResponse {
    /// The snapshots.
    
    pub items: Option<Vec<Snapshot>>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotListResponse`.
    
    pub kind: Option<String>,
    /// Token corresponding to the next page of results. If there are no more results, the token is omitted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SnapshotListResponse {}


/// A third party stats resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get stats](StatGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    /// Average session length in minutes of the player. E.g., 1, 30, 60, ... . Not populated if there is not enough information.
    
    pub avg_session_length_minutes: Option<f32>,
    /// The probability of the player not returning to play the game in the next day. E.g., 0, 0.1, 0.5, ..., 1.0. Not populated if there is not enough information.
    
    pub churn_probability: Option<f32>,
    /// Number of days since the player last played this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information.
    
    pub days_since_last_played: Option<i32>,
    /// The probability of the player going to spend beyond a threshold amount of money. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information.
    
    pub high_spender_probability: Option<f32>,
    /// Uniquely identifies the type of this resource. Value is always the fixed string `games#statsResponse`.
    
    pub kind: Option<String>,
    /// Number of in-app purchases made by the player in this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information.
    
    pub num_purchases: Option<i32>,
    /// The approximate number of sessions of the player within the last 28 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information.
    
    pub num_sessions: Option<i32>,
    /// The approximation of the sessions percentile of the player within the last 30 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information.
    
    pub num_sessions_percentile: Option<f32>,
    /// The approximate spend percentile of the player in this game. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information.
    
    pub spend_percentile: Option<f32>,
    /// The probability of the player going to spend the game in the next seven days. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information.
    
    pub spend_probability: Option<f32>,
    /// The predicted amount of money that the player going to spend in the next 28 days. E.g., 1, 30, 60, ... . Not populated if there is not enough information.
    
    pub total_spend_next_28_days: Option<f32>,
}

impl client::ResponseResult for StatsResponse {}


/// A representation of the individual components of the name.
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


