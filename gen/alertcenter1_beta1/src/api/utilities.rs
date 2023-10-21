use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See and delete your domain's G Suite alerts, and send alert feedback
    AppAlert,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppAlert => "https://www.googleapis.com/auth/apps.alerts",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppAlert
    }
}

