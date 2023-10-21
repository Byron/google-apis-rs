use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, upload, and organize items in your Google Photos library
    Full,

    /// Add to your Google Photos library
    Appendonly,

    /// Edit the info in your photos, videos, and albums created within this app, including titles, descriptions, and covers
    EditAppcreateddata,

    /// View your Google Photos library
    Readonly,

    /// Manage photos added by this app
    ReadonlyAppcreateddata,

    /// Manage and add to shared albums on your behalf
    Sharing,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/photoslibrary",
            Scope::Appendonly => "https://www.googleapis.com/auth/photoslibrary.appendonly",
            Scope::EditAppcreateddata => "https://www.googleapis.com/auth/photoslibrary.edit.appcreateddata",
            Scope::Readonly => "https://www.googleapis.com/auth/photoslibrary.readonly",
            Scope::ReadonlyAppcreateddata => "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
            Scope::Sharing => "https://www.googleapis.com/auth/photoslibrary.sharing",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}

