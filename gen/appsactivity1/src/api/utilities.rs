use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View the activity history of your Google apps
    Activity,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Activity => "https://www.googleapis.com/auth/activity",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Activity
    }
}

