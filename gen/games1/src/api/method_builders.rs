use super::*;
/// A builder providing access to all methods supported on *achievementDefinition* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.achievement_definitions();
/// # }
/// ```
pub struct AchievementDefinitionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for AchievementDefinitionMethods<'a, S> {}

impl<'a, S> AchievementDefinitionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the achievement definitions for your application.
    pub fn list(&self) -> AchievementDefinitionListCall<'a, S> {
        AchievementDefinitionListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *achievement* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `increment(...)`, `list(...)`, `reveal(...)`, `set_steps_at_least(...)`, `unlock(...)` and `update_multiple(...)`
/// // to build up your call.
/// let rb = hub.achievements();
/// # }
/// ```
pub struct AchievementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for AchievementMethods<'a, S> {}

impl<'a, S> AchievementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Increments the steps of the achievement with the given ID for the currently authenticated player.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    /// * `stepsToIncrement` - The number of steps to increment.
    pub fn increment(&self, achievement_id: &str, steps_to_increment: i32) -> AchievementIncrementCall<'a, S> {
        AchievementIncrementCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _steps_to_increment: steps_to_increment,
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the progress for all your application's achievements for the currently authenticated player.
    /// 
    /// # Arguments
    ///
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    pub fn list(&self, player_id: &str) -> AchievementListCall<'a, S> {
        AchievementListCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _state: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the state of the achievement with the given ID to `REVEALED` for the currently authenticated player.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    pub fn reveal(&self, achievement_id: &str) -> AchievementRevealCall<'a, S> {
        AchievementRevealCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the steps for the currently authenticated player towards unlocking an achievement. If the steps parameter is less than the current number of steps that the player already gained for the achievement, the achievement is not modified.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    /// * `steps` - The minimum value to set the steps to.
    pub fn set_steps_at_least(&self, achievement_id: &str, steps: i32) -> AchievementSetStepsAtLeastCall<'a, S> {
        AchievementSetStepsAtLeastCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _steps: steps,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unlocks this achievement for the currently authenticated player.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    pub fn unlock(&self, achievement_id: &str) -> AchievementUnlockCall<'a, S> {
        AchievementUnlockCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates multiple achievements for the currently authenticated player.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_multiple(&self, request: AchievementUpdateMultipleRequest) -> AchievementUpdateMultipleCall<'a, S> {
        AchievementUpdateMultipleCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *application* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_end_point(...)`, `played(...)` and `verify(...)`
/// // to build up your call.
/// let rb = hub.applications();
/// # }
/// ```
pub struct ApplicationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationMethods<'a, S> {}

impl<'a, S> ApplicationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata of the application with the given ID. If the requested application is not available for the specified `platformType`, the returned response will not include any instance data.
    /// 
    /// # Arguments
    ///
    /// * `applicationId` - The application ID from the Google Play developer console.
    pub fn get(&self, application_id: &str) -> ApplicationGetCall<'a, S> {
        ApplicationGetCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _platform_type: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a URL for the requested end point type.
    pub fn get_end_point(&self) -> ApplicationGetEndPointCall<'a, S> {
        ApplicationGetEndPointCall {
            hub: self.hub,
            _end_point_type: Default::default(),
            _application_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Indicate that the currently authenticated user is playing your application.
    pub fn played(&self) -> ApplicationPlayedCall<'a, S> {
        ApplicationPlayedCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies the auth token provided with this request is for the application with the specified ID, and returns the ID of the player it was granted for.
    /// 
    /// # Arguments
    ///
    /// * `applicationId` - The application ID from the Google Play developer console.
    pub fn verify(&self, application_id: &str) -> ApplicationVerifyCall<'a, S> {
        ApplicationVerifyCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *event* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_by_player(...)`, `list_definitions(...)` and `record(...)`
/// // to build up your call.
/// let rb = hub.events();
/// # }
/// ```
pub struct EventMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for EventMethods<'a, S> {}

impl<'a, S> EventMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list showing the current progress on events in this application for the currently authenticated user.
    pub fn list_by_player(&self) -> EventListByPlayerCall<'a, S> {
        EventListByPlayerCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of the event definitions in this application.
    pub fn list_definitions(&self) -> EventListDefinitionCall<'a, S> {
        EventListDefinitionCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn record(&self, request: EventRecordRequest) -> EventRecordCall<'a, S> {
        EventRecordCall {
            hub: self.hub,
            _request: request,
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *leaderboard* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.leaderboards();
/// # }
/// ```
pub struct LeaderboardMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for LeaderboardMethods<'a, S> {}

impl<'a, S> LeaderboardMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata of the leaderboard with the given ID.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    pub fn get(&self, leaderboard_id: &str) -> LeaderboardGetCall<'a, S> {
        LeaderboardGetCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the leaderboard metadata for your application.
    pub fn list(&self) -> LeaderboardListCall<'a, S> {
        LeaderboardListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *metagame* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_metagame_config(...)` and `list_categories_by_player(...)`
/// // to build up your call.
/// let rb = hub.metagame();
/// # }
/// ```
pub struct MetagameMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for MetagameMethods<'a, S> {}

impl<'a, S> MetagameMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Return the metagame configuration data for the calling application.
    pub fn get_metagame_config(&self) -> MetagameGetMetagameConfigCall<'a, S> {
        MetagameGetMetagameConfigCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List play data aggregated per category for the player corresponding to `playerId`.
    /// 
    /// # Arguments
    ///
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    /// * `collection` - The collection of categories for which data will be returned.
    pub fn list_categories_by_player(&self, player_id: &str, collection: &MetagameCollectionEnum) -> MetagameListCategoriesByPlayerCall<'a, S> {
        MetagameListCategoriesByPlayerCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _collection: collection.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *player* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_scoped_player_ids(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.players();
/// # }
/// ```
pub struct PlayerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for PlayerMethods<'a, S> {}

impl<'a, S> PlayerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set `playerId` to `me`.
    /// 
    /// # Arguments
    ///
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    pub fn get(&self, player_id: &str) -> PlayerGetCall<'a, S> {
        PlayerGetCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _player_id_consistency_token: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves scoped player identifiers for currently authenticated user.
    pub fn get_scoped_player_ids(&self) -> PlayerGetScopedPlayerIdCall<'a, S> {
        PlayerGetScopedPlayerIdCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the collection of players for the currently authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `collection` - Collection of players being retrieved
    pub fn list(&self, collection: &PlayerCollectionEnum) -> PlayerListCall<'a, S> {
        PlayerListCall {
            hub: self.hub,
            _collection: collection.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *revision* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `check(...)`
/// // to build up your call.
/// let rb = hub.revisions();
/// # }
/// ```
pub struct RevisionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for RevisionMethods<'a, S> {}

impl<'a, S> RevisionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Checks whether the games client is out of date.
    /// 
    /// # Arguments
    ///
    /// * `clientRevision` - The revision of the client SDK used by your application. Format: `[PLATFORM_TYPE]:[VERSION_NUMBER]`. Possible values of `PLATFORM_TYPE` are: * `ANDROID` - Client is running the Android SDK. * `IOS` - Client is running the iOS SDK. * `WEB_APP` - Client is running as a Web App.
    pub fn check(&self, client_revision: &str) -> RevisionCheckCall<'a, S> {
        RevisionCheckCall {
            hub: self.hub,
            _client_revision: client_revision.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *score* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)`, `list_window(...)`, `submit(...)` and `submit_multiple(...)`
/// // to build up your call.
/// let rb = hub.scores();
/// # }
/// ```
pub struct ScoreMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for ScoreMethods<'a, S> {}

impl<'a, S> ScoreMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get high scores, and optionally ranks, in leaderboards for the currently authenticated player. For a specific time span, `leaderboardId` can be set to `ALL` to retrieve data for all leaderboards in a given time span. `NOTE: You cannot ask for 'ALL' leaderboards and 'ALL' timeSpans in the same request; only one parameter may be set to 'ALL'.
    /// 
    /// # Arguments
    ///
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    /// * `leaderboardId` - The ID of the leaderboard. Can be set to 'ALL' to retrieve data for all leaderboards for this application.
    /// * `timeSpan` - The time span for the scores and ranks you're requesting.
    pub fn get(&self, player_id: &str, leaderboard_id: &str, time_span: &ScoreTimeSpanEnum) -> ScoreGetCall<'a, S> {
        ScoreGetCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _leaderboard_id: leaderboard_id.to_string(),
            _time_span: time_span.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _include_rank_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the scores in a leaderboard, starting from the top.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    /// * `collection` - The collection of scores you're requesting.
    /// * `timeSpan` - The time span for the scores and ranks you're requesting.
    pub fn list(&self, leaderboard_id: &str, collection: &ScoreCollectionEnum, time_span: &ScoreTimeSpanEnum) -> ScoreListCall<'a, S> {
        ScoreListCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _collection: collection.clone(),
            _time_span: time_span.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the scores in a leaderboard around (and including) a player's score.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    /// * `collection` - The collection of scores you're requesting.
    /// * `timeSpan` - The time span for the scores and ranks you're requesting.
    pub fn list_window(&self, leaderboard_id: &str, collection: &ScoreCollectionEnum, time_span: &ScoreTimeSpanEnum) -> ScoreListWindowCall<'a, S> {
        ScoreListWindowCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _collection: collection.clone(),
            _time_span: time_span.clone(),
            _return_top_if_absent: Default::default(),
            _results_above: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submits a score to the specified leaderboard.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    /// * `score` - The score you're submitting. The submitted score is ignored if it is worse than a previously submitted score, where worse depends on the leaderboard sort order. The meaning of the score value depends on the leaderboard format type. For fixed-point, the score represents the raw value. For time, the score represents elapsed time in milliseconds. For currency, the score represents a value in micro units.
    pub fn submit(&self, leaderboard_id: &str, score: i64) -> ScoreSubmitCall<'a, S> {
        ScoreSubmitCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _score: score,
            _score_tag: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submits multiple scores to leaderboards.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn submit_multiple(&self, request: PlayerScoreSubmissionList) -> ScoreSubmitMultipleCall<'a, S> {
        ScoreSubmitMultipleCall {
            hub: self.hub,
            _request: request,
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *snapshot* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.snapshots();
/// # }
/// ```
pub struct SnapshotMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for SnapshotMethods<'a, S> {}

impl<'a, S> SnapshotMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the metadata for a given snapshot ID.
    /// 
    /// # Arguments
    ///
    /// * `snapshotId` - The ID of the snapshot.
    pub fn get(&self, snapshot_id: &str) -> SnapshotGetCall<'a, S> {
        SnapshotGetCall {
            hub: self.hub,
            _snapshot_id: snapshot_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of snapshots created by your application for the player corresponding to the player ID.
    /// 
    /// # Arguments
    ///
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    pub fn list(&self, player_id: &str) -> SnapshotListCall<'a, S> {
        SnapshotListCall {
            hub: self.hub,
            _player_id: player_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *stat* resources.
/// It is not used directly, but through the [`Games`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_games1 as games1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use games1::{Games, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Games::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.stats();
/// # }
/// ```
pub struct StatMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Games<S>,
}

impl<'a, S> client::MethodsBuilder for StatMethods<'a, S> {}

impl<'a, S> StatMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns engagement and spend statistics in this application for the currently authenticated user.
    pub fn get(&self) -> StatGetCall<'a, S> {
        StatGetCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



