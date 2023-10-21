use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// Create, edit, and delete your Google Play Games activity
    Game,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Game => "https://www.googleapis.com/auth/games",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Game
    }
}

