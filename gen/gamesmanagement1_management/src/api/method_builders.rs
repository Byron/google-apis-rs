use super::*;
/// A builder providing access to all methods supported on *achievement* resources.
/// It is not used directly, but through the [`GamesManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gamesmanagement1_management as gamesmanagement1_management;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gamesmanagement1_management::{GamesManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GamesManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `reset(...)`, `reset_all(...)`, `reset_all_for_all_players(...)`, `reset_for_all_players(...)` and `reset_multiple_for_all_players(...)`
/// // to build up your call.
/// let rb = hub.achievements();
/// # }
/// ```
pub struct AchievementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GamesManagement<S>,
}

impl<'a, S> client::MethodsBuilder for AchievementMethods<'a, S> {}

impl<'a, S> AchievementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets the achievement with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    pub fn reset(&self, achievement_id: &str) -> AchievementResetCall<'a, S> {
        AchievementResetCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all achievements for the currently authenticated player for your application. This method is only accessible to whitelisted tester accounts for your application.
    pub fn reset_all(&self) -> AchievementResetAllCall<'a, S> {
        AchievementResetAllCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all draft achievements for all players. This method is only available to user accounts for your developer console.
    pub fn reset_all_for_all_players(&self) -> AchievementResetAllForAllPlayerCall<'a, S> {
        AchievementResetAllForAllPlayerCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets the achievement with the given ID for all players. This method is only available to user accounts for your developer console. Only draft achievements can be reset.
    /// 
    /// # Arguments
    ///
    /// * `achievementId` - The ID of the achievement used by this method.
    pub fn reset_for_all_players(&self, achievement_id: &str) -> AchievementResetForAllPlayerCall<'a, S> {
        AchievementResetForAllPlayerCall {
            hub: self.hub,
            _achievement_id: achievement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets achievements with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft achievements may be reset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reset_multiple_for_all_players(&self, request: AchievementResetMultipleForAllRequest) -> AchievementResetMultipleForAllPlayerCall<'a, S> {
        AchievementResetMultipleForAllPlayerCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *application* resources.
/// It is not used directly, but through the [`GamesManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gamesmanagement1_management as gamesmanagement1_management;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gamesmanagement1_management::{GamesManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GamesManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list_hidden(...)`
/// // to build up your call.
/// let rb = hub.applications();
/// # }
/// ```
pub struct ApplicationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GamesManagement<S>,
}

impl<'a, S> client::MethodsBuilder for ApplicationMethods<'a, S> {}

impl<'a, S> ApplicationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the list of players hidden from the given application. This method is only available to user accounts for your developer console.
    /// 
    /// # Arguments
    ///
    /// * `applicationId` - The application ID from the Google Play developer console.
    pub fn list_hidden(&self, application_id: &str) -> ApplicationListHiddenCall<'a, S> {
        ApplicationListHiddenCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *event* resources.
/// It is not used directly, but through the [`GamesManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gamesmanagement1_management as gamesmanagement1_management;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gamesmanagement1_management::{GamesManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GamesManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `reset(...)`, `reset_all(...)`, `reset_all_for_all_players(...)`, `reset_for_all_players(...)` and `reset_multiple_for_all_players(...)`
/// // to build up your call.
/// let rb = hub.events();
/// # }
/// ```
pub struct EventMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GamesManagement<S>,
}

impl<'a, S> client::MethodsBuilder for EventMethods<'a, S> {}

impl<'a, S> EventMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all player progress on the event with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application.
    /// 
    /// # Arguments
    ///
    /// * `eventId` - The ID of the event.
    pub fn reset(&self, event_id: &str) -> EventResetCall<'a, S> {
        EventResetCall {
            hub: self.hub,
            _event_id: event_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all player progress on all events for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application.
    pub fn reset_all(&self) -> EventResetAllCall<'a, S> {
        EventResetAllCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all draft events for all players. This method is only available to user accounts for your developer console.
    pub fn reset_all_for_all_players(&self) -> EventResetAllForAllPlayerCall<'a, S> {
        EventResetAllForAllPlayerCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets the event with the given ID for all players. This method is only available to user accounts for your developer console. Only draft events can be reset.
    /// 
    /// # Arguments
    ///
    /// * `eventId` - The ID of the event.
    pub fn reset_for_all_players(&self, event_id: &str) -> EventResetForAllPlayerCall<'a, S> {
        EventResetForAllPlayerCall {
            hub: self.hub,
            _event_id: event_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets events with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft events may be reset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reset_multiple_for_all_players(&self, request: EventsResetMultipleForAllRequest) -> EventResetMultipleForAllPlayerCall<'a, S> {
        EventResetMultipleForAllPlayerCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *player* resources.
/// It is not used directly, but through the [`GamesManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gamesmanagement1_management as gamesmanagement1_management;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gamesmanagement1_management::{GamesManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GamesManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `hide(...)` and `unhide(...)`
/// // to build up your call.
/// let rb = hub.players();
/// # }
/// ```
pub struct PlayerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GamesManagement<S>,
}

impl<'a, S> client::MethodsBuilder for PlayerMethods<'a, S> {}

impl<'a, S> PlayerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Hide the given player's leaderboard scores from the given application. This method is only available to user accounts for your developer console.
    /// 
    /// # Arguments
    ///
    /// * `applicationId` - The application ID from the Google Play developer console.
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    pub fn hide(&self, application_id: &str, player_id: &str) -> PlayerHideCall<'a, S> {
        PlayerHideCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _player_id: player_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unhide the given player's leaderboard scores from the given application. This method is only available to user accounts for your developer console.
    /// 
    /// # Arguments
    ///
    /// * `applicationId` - The application ID from the Google Play developer console.
    /// * `playerId` - A player ID. A value of `me` may be used in place of the authenticated player's ID.
    pub fn unhide(&self, application_id: &str, player_id: &str) -> PlayerUnhideCall<'a, S> {
        PlayerUnhideCall {
            hub: self.hub,
            _application_id: application_id.to_string(),
            _player_id: player_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *score* resources.
/// It is not used directly, but through the [`GamesManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gamesmanagement1_management as gamesmanagement1_management;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gamesmanagement1_management::{GamesManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = GamesManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `reset(...)`, `reset_all(...)`, `reset_all_for_all_players(...)`, `reset_for_all_players(...)` and `reset_multiple_for_all_players(...)`
/// // to build up your call.
/// let rb = hub.scores();
/// # }
/// ```
pub struct ScoreMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a GamesManagement<S>,
}

impl<'a, S> client::MethodsBuilder for ScoreMethods<'a, S> {}

impl<'a, S> ScoreMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets scores for the leaderboard with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    pub fn reset(&self, leaderboard_id: &str) -> ScoreResetCall<'a, S> {
        ScoreResetCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets all scores for all leaderboards for the currently authenticated players. This method is only accessible to whitelisted tester accounts for your application.
    pub fn reset_all(&self) -> ScoreResetAllCall<'a, S> {
        ScoreResetAllCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets scores for all draft leaderboards for all players. This method is only available to user accounts for your developer console.
    pub fn reset_all_for_all_players(&self) -> ScoreResetAllForAllPlayerCall<'a, S> {
        ScoreResetAllForAllPlayerCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets scores for the leaderboard with the given ID for all players. This method is only available to user accounts for your developer console. Only draft leaderboards can be reset.
    /// 
    /// # Arguments
    ///
    /// * `leaderboardId` - The ID of the leaderboard.
    pub fn reset_for_all_players(&self, leaderboard_id: &str) -> ScoreResetForAllPlayerCall<'a, S> {
        ScoreResetForAllPlayerCall {
            hub: self.hub,
            _leaderboard_id: leaderboard_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resets scores for the leaderboards with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft leaderboards may be reset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn reset_multiple_for_all_players(&self, request: ScoresResetMultipleForAllRequest) -> ScoreResetMultipleForAllPlayerCall<'a, S> {
        ScoreResetMultipleForAllPlayerCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



