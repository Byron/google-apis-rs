use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage G Suite licenses for your domain
    AppLicensing,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::AppLicensing => "https://www.googleapis.com/auth/apps.licensing",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppLicensing
    }
}

