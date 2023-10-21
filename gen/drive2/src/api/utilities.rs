use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Full,

    /// See, create, and delete its own configuration data in your Google Drive
    Appdata,

    /// View your Google Drive apps
    AppReadonly,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    File,

    /// View and manage metadata of files in your Google Drive
    Metadata,

    /// See information about your Google Drive files
    MetadataReadonly,

    /// View the photos, videos and albums in your Google Photos
    PhotoReadonly,

    /// See and download all your Google Drive files
    Readonly,

    /// Modify your Google Apps Script scripts' behavior
    Script,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/drive",
            Scope::Appdata => "https://www.googleapis.com/auth/drive.appdata",
            Scope::AppReadonly => "https://www.googleapis.com/auth/drive.apps.readonly",
            Scope::File => "https://www.googleapis.com/auth/drive.file",
            Scope::Metadata => "https://www.googleapis.com/auth/drive.metadata",
            Scope::MetadataReadonly => "https://www.googleapis.com/auth/drive.metadata.readonly",
            Scope::PhotoReadonly => "https://www.googleapis.com/auth/drive.photos.readonly",
            Scope::Readonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Script => "https://www.googleapis.com/auth/drive.scripts",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AppReadonly
    }
}

