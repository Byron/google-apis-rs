use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See, edit, create, and delete all your Google Docs documents
    Document,

    /// See all your Google Docs documents
    DocumentReadonly,

    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    DriveFile,

    /// See and download all your Google Drive files
    DriveReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Document => "https://www.googleapis.com/auth/documents",
            Scope::DocumentReadonly => "https://www.googleapis.com/auth/documents.readonly",
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DocumentReadonly
    }
}

