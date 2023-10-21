use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,

    /// View and administer all your Firebase data and settings
    Full,

    /// View all your Firebase data and settings
    Readonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::Full => "https://www.googleapis.com/auth/firebase",
            Scope::Readonly => "https://www.googleapis.com/auth/firebase.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}

