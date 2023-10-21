use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and add to the activity record of files in your Google Drive
    DriveActivity,

    /// View the activity record of files in your Google Drive
    DriveActivityReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::DriveActivity => "https://www.googleapis.com/auth/drive.activity",
            Scope::DriveActivityReadonly => "https://www.googleapis.com/auth/drive.activity.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveActivityReadonly
    }
}

