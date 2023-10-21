use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, create, and delete its own configuration data in your Google Drive
    DriveAppdata,

    /// Create, edit, and delete your Google Play Games activity
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DriveAppdata => "https://www.googleapis.com/auth/drive.appdata",
            Scope::Full => "https://www.googleapis.com/auth/games",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}

